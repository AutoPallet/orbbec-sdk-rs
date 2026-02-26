use std::time::Duration;

use clap::Parser;
mod utils;
use orbbec_sdk::{
    Context, Format, HoleFillMode, LogSeverity, PermissionType, SensorType,
    device::DeviceProperty,
    filter::{
        DecimationFilter, Filter, HoleFillingFilter, SpatialModerateFilter, TemporalFilter,
        ThresholdFilter,
    },
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

    #[arg(long, default_value_t = 2)]
    decimation_factor: u8,
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

    // Enable depth stream
    config.enable_stream_with_profile(&depth_profile)?;

    // Create decimation filter
    let mut decimation_filter: DecimationFilter = DecimationFilter::new()?;
    decimation_filter.set_factor(args.decimation_factor)?;

    // Create spatial filter (Moderate variant)
    let mut spatial_filter = SpatialModerateFilter::new()?;
    spatial_filter.set_threshold(160)?;
    spatial_filter.set_magnitude(3)?;
    spatial_filter.set_radius(5)?;

    // Create temporal filter
    let mut temporal_filter = TemporalFilter::new()?;
    temporal_filter.set_threshold(0.1)?;
    temporal_filter.set_weight(0.4)?;

    // Hole filling filter
    let mut hole_filling_filter = HoleFillingFilter::new()?;
    hole_filling_filter.set_mode(HoleFillMode::Farthest)?;

    // Threshold filter
    let mut threshold_filter = ThresholdFilter::new()?;
    threshold_filter.set_min_depth(200)?;
    threshold_filter.set_max_depth(4000)?;

    // Start streaming
    pipeline.start(&config)?;

    let rr = rerun::RecordingStreamBuilder::new("orbbec_sdk_rs_depth_filtered").connect_grpc()?;

    let intrinsic = depth_profile.get_intrinsic()?;
    rr.log_static(
        "orbbec/depth_raw",
        &rerun::Pinhole::from_focal_length_and_resolution(
            [intrinsic.fx, intrinsic.fy],
            [intrinsic.width as f32, intrinsic.height as f32],
        ),
    )?;

    rr.log_static("orbbec/depth_filtered", &{
        // Scale down the intrinsic by the decimation factor
        let f = args.decimation_factor as f32;
        rerun::Pinhole::from_focal_length_and_resolution(
            [intrinsic.fx / f, intrinsic.fy / f],
            [intrinsic.width as f32 / f, intrinsic.height as f32 / f],
        )
    })?;

    loop {
        // Get frameset
        let frameset = match pipeline.wait_for_frames(Duration::from_millis(100))? {
            Some(frameset) => frameset,
            None => {
                eprintln!("Timeout waiting for frames.");
                continue;
            }
        };

        // Get depth frame
        let Some(depth_frame) = frameset.get_depth_frame()? else {
            eprintln!("No depth frame found.");
            continue;
        };

        let orig_depth_data = depth_frame.raw_data();
        let orig_width = depth_frame.width() as u32;
        let orig_height = depth_frame.height() as u32;
        let orig_depth_scale = depth_frame.depth_scale();
        let timestamp_us = depth_frame.timestamp_us();

        // Apply filters
        let depth_frame = decimation_filter.process(&depth_frame)?;
        let depth_frame = spatial_filter.process(&depth_frame)?;
        let depth_frame = temporal_filter.process(&depth_frame)?;
        let depth_frame = hole_filling_filter.process(&depth_frame)?;
        let depth_frame = threshold_filter.process(&depth_frame)?;

        let depth_filtered_data = depth_frame.raw_data();

        rr.set_time(
            "camera_clock",
            std::time::Duration::from_micros(timestamp_us),
        );

        rr.log(
            "orbbec/depth_raw",
            &rerun::DepthImage::from_gray16(orig_depth_data, [orig_width, orig_height])
                .with_meter(1000.0 / orig_depth_scale),
        )?;

        rr.log(
            "orbbec/depth_filtered",
            &rerun::DepthImage::from_gray16(
                depth_filtered_data,
                [depth_frame.width() as u32, depth_frame.height() as u32],
            )
            .with_meter(1000.0 / depth_frame.depth_scale()),
        )?;
    }
}
