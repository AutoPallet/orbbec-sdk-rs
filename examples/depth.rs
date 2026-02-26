use std::time::Duration;

use clap::Parser;
mod utils;
use orbbec_sdk::{
    Context, Format, LogSeverity, SensorType,
    logger::Logger,
    pipeline::{Config, Pipeline},
};

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const FPS: u8 = 15;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 0)]
    device_index: usize,

    #[arg(long, default_value_t = DEPTH_WIDTH)]
    depth_width: u16,
    #[arg(long, default_value_t = DEPTH_HEIGHT)]
    depth_height: u16,

    #[arg(long, default_value_t = FPS)]
    fps: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Don't create a ./Log directory in the current working directory
    Logger::set_directory(LogSeverity::Off, None)?;

    // Create context and get device list
    let context = Context::new()?;
    let devices = context.query_device_list()?;

    if devices.is_empty() {
        anyhow::bail!("no Orbbec devices found");
    }
    let device = devices.get(args.device_index)?;

    // Create pipeline
    let mut config = Config::new()?;
    let mut pipeline = Pipeline::new(&device)?;

    // Get depth stream profile
    let depth_profiles = pipeline.get_stream_profiles(SensorType::Depth)?;
    let depth_profile = utils::get_video_stream_profile(
        &depth_profiles,
        args.depth_width,
        args.depth_height,
        Format::Y16,
        args.fps,
    )?;

    // Enable depth stream
    config.enable_stream_with_profile(&depth_profile)?;

    // Start streaming
    pipeline.start(&config)?;

    let rr = rerun::RecordingStreamBuilder::new("orbbec_sdk_rs_depth").connect_grpc()?;

    loop {
        // Get frameset
        let frameset = match pipeline.wait_for_frames(Duration::from_millis(100))? {
            Some(frameset) => frameset,
            None => {
                eprintln!("Timeout waiting for frames.");
                continue;
            }
        };

        // Get depth frame data
        let Some(depth_frame) = frameset.get_depth_frame()? else {
            eprintln!("No depth frame found.");
            continue;
        };

        let depth_data = depth_frame.raw_data();

        rr.set_time(
            "camera_clock",
            std::time::Duration::from_micros(depth_frame.timestamp_us()),
        );

        rr.log(
            "orbbec/depth_frame",
            &rerun::DepthImage::from_gray16(
                depth_data,
                [depth_frame.width() as u32, depth_frame.height() as u32],
            )
            .with_meter(1000.0 / depth_frame.depth_scale()),
        )?;
    }
}
