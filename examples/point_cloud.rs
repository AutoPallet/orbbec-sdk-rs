mod utils;
use std::time::Duration;

use kiss3d::window::Window;
use nalgebra::Point3;
use orbbec_sdk::{
    Context, Format, PermissionType, SensorType,
    device::DeviceProperty,
    filter::{AlignFilter, Filter, PointCloudFilter},
    pipeline::{Config, Pipeline},
};
use std::sync::mpsc::channel;
use std::thread;

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const COLOR_WIDTH: u16 = 1280;
const COLOR_HEIGHT: u16 = 720;
const FPS: u8 = 15;

const RGB_POINT_CLOUD: bool = true; // Set to false to disable color in point cloud

/// Convert raw point cloud data to a vector of 3D points with color
fn convert_pointcloud(data: &[u8], with_color: bool) -> Vec<(Point3<f32>, Point3<f32>)> {
    let point_size = if with_color { 6 } else { 3 }; // Each point has 3 coordinates (x, y, z) and optionally 3 color channels (r, g, b).
    let num_points = (data.len() / (point_size * 4)) as usize; // Each float is 4 bytes
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let base_index = i * point_size * 4;
        let x = f32::from_le_bytes(data[base_index..base_index + 4].try_into().unwrap());
        let y = f32::from_le_bytes(data[base_index + 4..base_index + 8].try_into().unwrap());
        let z = f32::from_le_bytes(data[base_index + 8..base_index + 12].try_into().unwrap());

        if with_color {
            let r = f32::from_le_bytes(data[base_index + 12..base_index + 16].try_into().unwrap())
                / 255.0;
            let g = f32::from_le_bytes(data[base_index + 16..base_index + 20].try_into().unwrap())
                / 255.0;
            let b = f32::from_le_bytes(data[base_index + 20..base_index + 24].try_into().unwrap())
                / 255.0;
            points.push((Point3::new(x, y, z), Point3::new(r, g, b)));
        } else {
            points.push((Point3::new(x, y, z), Point3::new(0.5, 0.5, 0.5))); // Default color (gray)
        }
    }

    points
}

fn main() {
    // Create context and get device list
    let context = Context::new().unwrap();
    let devices = context.query_device_list().unwrap();

    if devices.is_empty() {
        eprintln!("No Orbbec devices found.");
        return;
    }

    // Get the first device available
    let mut device = devices.get(0).unwrap();

    // Load the "High Accuracy" preset
    device.load_preset("High Accuracy").unwrap();

    // Enable depth noise filter
    let hw_noise = DeviceProperty::HWNoiseRemoveFilterEnable(true);
    if device
        .is_property_supported(hw_noise, PermissionType::Write)
        .unwrap()
    {
        // HW filter is supported, use it instead of SW filter
        device.set_property(hw_noise).unwrap();
        device
            .set_property(DeviceProperty::HWNoiseRemoveFilterThreshold(0.2))
            .unwrap();
        device
            .set_property(DeviceProperty::DepthNoiseRemovalFilter(false))
            .unwrap();
        println!("Using HW depth noise filter.");
    } else {
        // HW filter not supported, use SW filter
        device
            .set_property(DeviceProperty::DepthNoiseRemovalFilter(true))
            .unwrap();
        device
            .set_property(DeviceProperty::DepthNoiseRemovalFilterMaxDiff(256))
            .unwrap();
        device
            .set_property(DeviceProperty::DepthNoiseRemovalFilterMaxSpeckleSize(80))
            .unwrap();
        println!("Using SW depth noise filter.");
    }

    // Create pipeline
    let mut config = Config::new().unwrap();
    let mut pipeline = Pipeline::new(&device).unwrap();

    // Get depth stream profile
    let depth_profiles = pipeline.get_stream_profiles(SensorType::Depth).unwrap();
    let depth_profile = depth_profiles
        .get_video_stream_profile(DEPTH_WIDTH, DEPTH_HEIGHT, Format::Y16, FPS)
        .unwrap();

    // Get color stream profile
    let color_profiles = pipeline.get_stream_profiles(SensorType::Color).unwrap();
    let color_profile = color_profiles
        .get_video_stream_profile(COLOR_WIDTH, COLOR_HEIGHT, Format::MJPG, FPS)
        .unwrap();

    // Enable depth and color streams
    config.enable_stream_with_profile(&depth_profile).unwrap();
    config.enable_stream_with_profile(&color_profile).unwrap();

    // Create align filter and set to align to color stream
    let mut align_filter = AlignFilter::new().unwrap();
    align_filter
        .set_align_to_stream_profile(&color_profile)
        .unwrap();

    // Create point cloud filter
    let mut pc_filter = PointCloudFilter::new().unwrap();
    pc_filter.set_color(RGB_POINT_CLOUD).unwrap();

    // Enable sync mode
    pipeline.set_frame_sync(true).unwrap();

    // Start streaming
    pipeline.start(&config).unwrap();

    let (tx, rx) = channel();

    // Spawn a thread for rendering
    thread::spawn(move || {
        let mut window = Window::new("Point Cloud");
        window.set_point_size(1.0);

        // Rendering loop
        while window.render() {
            // Receive points from the main thread
            let points = rx.recv().unwrap();

            // Draw all points
            for (point, color) in points {
                window.draw_point(&point, &color);
            }
        }
    });

    // Main loop
    loop {
        // Get frameset
        let frameset = match pipeline
            .wait_for_frames(Duration::from_millis(100))
            .unwrap()
        {
            Some(frameset) => frameset,
            None => {
                eprintln!("Timeout waiting for frames.");
                continue;
            }
        };

        // Check if color frame is available
        if let None = frameset.get_color_frame().unwrap() {
            eprintln!("No color frame found.");
            continue;
        }

        // Check if depth frame is available
        if let None = frameset.get_depth_frame().unwrap() {
            eprintln!("No depth frame found.");
            continue;
        }

        // Align depth to color
        let aligned_frame = align_filter.process(&frameset).unwrap();

        // Generate point cloud
        let pc_frame = pc_filter.process(&aligned_frame).unwrap();

        // Convert raw data to point cloud
        let points = convert_pointcloud(pc_frame.raw_data(), pc_frame.has_color());

        // Send points to rendering thread
        tx.send(points).unwrap();
    }
}
