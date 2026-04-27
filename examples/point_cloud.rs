use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use orbbec_sdk::{
    AlignMode, Context, Format, FrameAggregateOutputMode, LogSeverity, PermissionType, SensorType,
    filter::{AlignFilter, Filter, PointCloudFilter},
    logger::Logger,
    pipeline::{Config, Pipeline},
    prop,
};

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const COLOR_WIDTH: u16 = 1280;
const COLOR_HEIGHT: u16 = 720;
const FPS: u8 = 15;

const RGB_POINT_CLOUD: bool = true; // Set to false to disable color in point cloud

const POINT_CLOUD_SCALE: f32 = 1e-3; // 1 mm to meters

/// Convert raw point cloud data to a vector of 3D points with color
fn convert_pointcloud(data: &[u8], with_color: bool) -> rerun::Points3D {
    let point_size = if with_color { 6 } else { 3 }; // Each point has 3 coordinates (x, y, z) and optionally 3 color channels (r, g, b).
    let num_points = (data.len() / (point_size * 4)) as usize; // Each float is 4 bytes
    let mut points = Vec::with_capacity(num_points);
    let mut colors = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let base_index = i * point_size * 4;
        let x = f32::from_le_bytes(data[base_index..base_index + 4].try_into().unwrap())
            * POINT_CLOUD_SCALE;
        let y = f32::from_le_bytes(data[base_index + 4..base_index + 8].try_into().unwrap())
            * POINT_CLOUD_SCALE;
        let z = f32::from_le_bytes(data[base_index + 8..base_index + 12].try_into().unwrap())
            * POINT_CLOUD_SCALE;

        if with_color {
            let r = f32::from_le_bytes(data[base_index + 12..base_index + 16].try_into().unwrap());
            let g = f32::from_le_bytes(data[base_index + 16..base_index + 20].try_into().unwrap());
            let b = f32::from_le_bytes(data[base_index + 20..base_index + 24].try_into().unwrap());
            points.push(rerun::Vec3D::new(x, y, z));
            colors.push(rerun::Color::from_rgb(r as u8, g as u8, b as u8));
        } else {
            points.push(rerun::Vec3D::new(x, y, z));
            colors.push(rerun::Color::from_rgb(128, 128, 128));
        }
    }

    rerun::Points3D::new(points).with_colors(colors)
}

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

    #[arg(long, default_value_t = RGB_POINT_CLOUD)]
    rgb_point_cloud: bool,
}

fn main() -> Result<()> {
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
    if device.is_property_supported::<prop::HwNoiseRemoveFilterEnable>(PermissionType::Write)? {
        // HW filter is supported, use it instead of SW filter
        device.set_property::<prop::HwNoiseRemoveFilterEnable>(true)?;
        device.set_property::<prop::HwNoiseRemoveFilterThreshold>(0.2)?;
        device.set_property::<prop::DepthNoiseRemovalFilter>(false)?;
        println!("Using HW depth noise filter.");
    } else {
        // HW filter not supported, use SW filter
        device.set_property::<prop::DepthNoiseRemovalFilter>(true)?;
        device.set_property::<prop::DepthNoiseRemovalFilterMaxDiff>(256)?;
        device.set_property::<prop::DepthNoiseRemovalFilterMaxSpeckleSize>(80)?;
        println!("Using SW depth noise filter.");
    }

    // Create pipeline
    let mut config = Config::new()?;
    let mut pipeline = Pipeline::new(&device)?;

    // Get depth stream profile
    let depth_profiles = pipeline.get_stream_profiles(SensorType::Depth)?;
    let depth_profile = depth_profiles.get_video_stream_profile(
        args.depth_width,
        args.depth_height,
        Format::Y16,
        args.fps,
    )?;

    // Get color stream profile
    let color_profiles = pipeline.get_stream_profiles(SensorType::Color)?;
    let color_profile = color_profiles.get_video_stream_profile(
        args.color_width,
        args.color_height,
        Format::Mjpg,
        args.fps,
    )?;

    // Enable depth and color streams
    config.enable_stream_with_profile(&depth_profile)?;
    config.enable_stream_with_profile(&color_profile)?;
    config.set_align_mode(AlignMode::D2CHwMode)?;
    config.set_frame_aggregate_output_mode(FrameAggregateOutputMode::AllTypeFrameRequire)?;

    // Create align filter and set to align to color stream
    let mut align_filter = AlignFilter::new()?;
    align_filter.set_gap_fill(false)?;
    align_filter.set_align_to_stream_profile(&color_profile)?;

    // Create point cloud filter
    let mut pc_filter = PointCloudFilter::new()?;
    pc_filter.set_color(args.rgb_point_cloud)?;

    // Enable sync mode
    pipeline.set_frame_sync(true)?;

    // Start streaming
    pipeline.start(&config)?;

    let rr = rerun::RecordingStreamBuilder::new("orbbec_sdk_rs_point_cloud").connect_grpc()?;

    // Make camera point forward instead of up
    rr.log_static(
        "orbbec",
        &rerun::Transform3D::from_rotation(rerun::Rotation3D::AxisAngle(
            rerun::components::RotationAxisAngle::new(
                rerun::Vec3D::new(1.0, 0.0, 0.0),
                -90_f32.to_radians(),
            ),
        )),
    )?;

    let intrinsic = color_profile.get_intrinsic()?;
    rr.log_static(
        "orbbec/camera",
        &rerun::Pinhole::new(
            rerun::components::PinholeProjection::from_focal_length_and_principal_point(
                [intrinsic.fx, intrinsic.fy],
                [intrinsic.cx as f32, intrinsic.cy as f32],
            ),
        ),
    )?;

    // Main loop
    loop {
        // Get frameset
        let frameset = match pipeline.wait_for_frames(Duration::from_millis(100))? {
            Some(frameset) => frameset,
            None => {
                eprintln!("Timeout waiting for frames.");
                continue;
            }
        };

        // Check if color frame is available
        let Some(color_frame) = frameset.get_color_frame()? else {
            eprintln!("No color frame found.");
            continue;
        };

        // Check if depth frame is available
        if frameset.get_depth_frame()?.is_none() {
            eprintln!("No depth frame found.");
            continue;
        };

        // Align color to depth
        let aligned_frame = align_filter.process(&frameset)?;

        let depth_frame = aligned_frame.get_depth_frame()?;
        let Some(depth_frame) = depth_frame else {
            eprintln!("No depth frame found.");
            continue;
        };

        let depth_data = depth_frame.raw_data();

        // Generate point cloud
        let pc_frame = pc_filter.process(&aligned_frame)?;

        // Convert raw data to point cloud
        let points = convert_pointcloud(pc_frame.raw_data(), pc_frame.has_color());

        // Set timestamp and log point cloud
        rr.set_time(
            "camera_clock",
            std::time::Duration::from_micros(color_frame.timestamp_us()),
        );
        rr.log("orbbec/point_cloud", &points)?;

        rr.log(
            "orbbec/camera",
            &rerun::EncodedImage::from_file_contents(color_frame.raw_data().to_owned())
                .with_media_type("image/jpeg"),
        )?;

        rr.log(
            "orbbec/camera/depth",
            &rerun::DepthImage::from_gray16(
                depth_data,
                [depth_frame.width() as u32, depth_frame.height() as u32],
            )
            .with_meter(1000.0 / depth_frame.depth_scale()),
        )?;
    }
}
