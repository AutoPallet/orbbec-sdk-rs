mod utils;
use std::time::Duration;

use orbbec_sdk::{
    Context, Format, SensorType,
    pipeline::{Config, Pipeline},
};
use show_image::{ImageInfo, ImageView, WindowOptions, create_window, run_context};

const DEPTH_WIDTH: u16 = 848;
const DEPTH_HEIGHT: u16 = 480;
const FPS: u8 = 15;

fn main() {
    // Run the user task in the show-image context
    run_context(user_task);
}

fn user_task() {
    // Create context and get device list
    let context = Context::new().unwrap();
    let devices = context.query_device_list().unwrap();

    if devices.is_empty() {
        eprintln!("No Orbbec devices found.");
        return;
    }

    // Get the first device available
    let device = devices.get(0).unwrap();

    // Create pipeline
    let mut config = Config::new().unwrap();
    let mut pipeline = Pipeline::new(&device).unwrap();

    // Get depth stream profile
    let depth_profiles = pipeline.get_stream_profiles(SensorType::Depth).unwrap();
    let depth_profile = depth_profiles
        .get_video_stream_profile(DEPTH_WIDTH, DEPTH_HEIGHT, Format::Y16, FPS)
        .unwrap();

    // Enable depth stream
    config.enable_stream_with_profile(&depth_profile).unwrap();

    // Start streaming
    pipeline.start(&config).unwrap();

    // Create window to display depth image
    let window_config = WindowOptions {
        preserve_aspect_ratio: true,
        default_controls: false,
        ..Default::default()
    };
    let window = create_window("Depth", window_config.clone()).expect("Failed to create window");

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

        // Get depth frame data
        let depth_frame = frameset.get_depth_frame().unwrap().unwrap();
        let depth_data = depth_frame.raw_data();

        // Convert depth to color image
        let depth_image = utils::depth2image(depth_data).unwrap();

        // Display the depth image
        let image = ImageView::new(
            ImageInfo::rgb8(depth_frame.width() as u32, depth_frame.height() as u32),
            &depth_image,
        );
        window.set_image("Depth", image).unwrap();
    }
}
