mod utils;
use std::time::Duration;

use orbbec_sdk::{
    Context, ConvertType, Format, PermissionType, SensorType,
    device::DeviceProperty,
    filter::{AlignFilter, Filter, FormatConvertFilter},
    pipeline::{Config, Pipeline},
};
use show_image::{ImageInfo, ImageView, WindowOptions, create_window, run_context};

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const COLOR_WIDTH: u16 = 1280;
const COLOR_HEIGHT: u16 = 720;
const FPS: u8 = 15;

fn main() {
    // Run the user task in the show-image context
    run_context(user_task);
}

/// User task to run in the show-image context
fn user_task() {
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

    // Create format conversion filter to convert MJPG to RGB
    let mut convert_filter = FormatConvertFilter::new().unwrap();
    convert_filter
        .set_convert_type(ConvertType::MJPGToRGB)
        .unwrap();

    // Enable sync mode
    pipeline.set_frame_sync(true).unwrap();

    // Start streaming
    pipeline.start(&config).unwrap();

    // Create window to display color + depth image
    let window_config = WindowOptions {
        preserve_aspect_ratio: true,
        default_controls: false,
        ..Default::default()
    };
    let window =
        create_window("Color + Depth", window_config.clone()).expect("Failed to create window");

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

        // Align depth to color
        let aligned_frame = align_filter.process(&frameset).unwrap();

        // Get color frame data
        let color_frame = aligned_frame.get_color_frame().unwrap();
        let color_frame = match color_frame {
            Some(frame) => frame,
            None => {
                eprintln!("No color frame found.");
                continue;
            }
        };

        // Convert color frame from MJPG to RGB
        let color_frame = convert_filter.process(&color_frame).unwrap();
        let color_data = color_frame.raw_data();

        // Get depth frame data
        let depth_frame = aligned_frame.get_depth_frame().unwrap();
        let depth_frame = match depth_frame {
            Some(frame) => frame,
            None => {
                eprintln!("No depth frame found.");
                continue;
            }
        };
        let depth_data = depth_frame.raw_data();

        // Convert depth to color image
        let depth_image = utils::depth2image(depth_data).unwrap();

        // Add color and depth images together
        let added_image = utils::add_images(&depth_image, color_data);

        // Display the combined image
        let image = ImageView::new(
            ImageInfo::rgb8(depth_frame.width() as u32, depth_frame.height() as u32),
            &added_image,
        );
        window.set_image("Color + Depth", image).unwrap();
    }
}
