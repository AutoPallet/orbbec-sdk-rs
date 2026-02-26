use std::time::Duration;

use clap::Parser;
mod utils;
use orbbec_sdk::{
    Context, ConvertType, Format, LogSeverity, PermissionType, SensorType,
    device::DeviceProperty,
    filter::{AlignFilter, Filter, FormatConvertFilter},
    logger::Logger,
    pipeline::{Config, Pipeline},
};

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const COLOR_WIDTH: u16 = 1280;
const COLOR_HEIGHT: u16 = 720;
const FPS: u8 = 15;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 0)]
    device_index: usize,

    #[arg(long, default_value_t = DEPTH_WIDTH)]
    depth_width: u16,
    #[arg(long, default_value_t = DEPTH_HEIGHT)]
    depth_height: u16,

    #[arg(long, default_value_t = COLOR_WIDTH)]
    color_width: u16,
    #[arg(long, default_value_t = COLOR_HEIGHT)]
    color_height: u16,

    #[arg(long, default_value_t = FPS)]
    fps: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Don't create a ./Log directory in the current working directory
    Logger::set_directory(LogSeverity::Off, None)?;

    // Create context and get device list
    let context = Context::new()?;

    let _logger_handle = Logger::set_callback(LogSeverity::Warn, |_severity, message| {
        println!("[orbbec]{message}");
    })?;

    let devices = context.query_device_list()?;

    if devices.is_empty() {
        anyhow::bail!("no Orbbec devices found");
    }

    let mut device = devices.get(args.device_index)?;

    // Load the "High Accuracy" preset (if it's supported)
    if let Err(err) = device.load_preset("High Accuracy") {
        eprintln!("Failed to load preset (may not be supported on this device): {err}");
    }

    // Enable depth noise filter
    let hw_noise = DeviceProperty::HWNoiseRemoveFilterEnable(true);
    if device.is_property_supported(hw_noise, PermissionType::Write)? {
        // HW filter is supported, use it instead of SW filter
        device.set_property(hw_noise)?;
        device.set_property(DeviceProperty::HWNoiseRemoveFilterThreshold(0.2))?;
        device.set_property(DeviceProperty::DepthNoiseRemovalFilter(false))?;
        println!("Using HW depth noise filter.");
    } else {
        // HW filter not supported, use SW filter
        device.set_property(DeviceProperty::DepthNoiseRemovalFilter(true))?;
        device.set_property(DeviceProperty::DepthNoiseRemovalFilterMaxDiff(256))?;
        device.set_property(DeviceProperty::DepthNoiseRemovalFilterMaxSpeckleSize(80))?;
        println!("Using SW depth noise filter.");
    }

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

    // Get color stream profile
    let color_profiles = pipeline.get_stream_profiles(SensorType::Color)?;
    let color_profile = utils::get_video_stream_profile(
        &color_profiles,
        args.color_width,
        args.color_height,
        Format::MJPG,
        args.fps,
    )?;

    // Enable depth and color streams
    config.enable_stream_with_profile(&depth_profile)?;
    config.enable_stream_with_profile(&color_profile)?;

    // Create align filter and set to align to color stream
    let mut align_filter = AlignFilter::new()?;
    align_filter.set_align_to_stream_profile(&color_profile)?;

    // Create format conversion filter to convert MJPG to RGB
    let mut convert_filter = FormatConvertFilter::new()?;
    convert_filter.set_convert_type(ConvertType::MJPGToRGB)?;

    // Enable sync mode
    pipeline.set_frame_sync(true)?;

    // Start streaming
    pipeline.start(&config)?;

    let rr = rerun::RecordingStreamBuilder::new("orbbec_sdk_rs_depth_aligned").connect_grpc()?;

    loop {
        // Get frameset
        let frameset = match pipeline.wait_for_frames(Duration::from_millis(100))? {
            Some(frameset) => frameset,
            None => {
                eprintln!("Timeout waiting for frames.");
                continue;
            }
        };

        // Get color frame
        let Some(color_frame) = frameset.get_color_frame()? else {
            eprintln!("No color frame found.");
            continue;
        };

        // Example usage: convert color frame from MJPG to RGB
        // We don't use the converted frame since we use the JPEG data directly
        let color_frame_converted = convert_filter.process(&color_frame)?;
        let _color_data = color_frame_converted.raw_data();

        // Get depth frame
        let Some(depth_frame) = frameset.get_depth_frame()? else {
            eprintln!("No depth frame found.");
            continue;
        };

        // Align depth frame to color frame
        let depth_frame = align_filter.process(&depth_frame)?;
        let depth_data = depth_frame.raw_data();

        rr.set_time(
            "camera_clock",
            std::time::Duration::from_micros(color_frame.timestamp_us()),
        );

        rr.log(
            "orbbec/color_frame",
            &rerun::EncodedImage::from_file_contents(color_frame.raw_data().to_owned())
                .with_media_type("image/jpeg"),
        )?;

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
