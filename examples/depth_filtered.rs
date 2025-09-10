mod utils;
use std::time::Duration;

use orbbec_sdk::{
    Context, Format, HoleFillMode, PermissionType, SensorType,
    device::DeviceProperty,
    filter::{
        DecimationFilter, Filter, HoleFillingFilter, SpatialModerateFilter, TemporalFilter,
        ThresholdFilter,
    },
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

    // Enable depth stream
    config.enable_stream_with_profile(&depth_profile).unwrap();

    // Create decimation filter
    let mut decimation_filter: DecimationFilter = DecimationFilter::new().unwrap();
    decimation_filter.set_factor(2).unwrap();

    // Create spatial filter (Moderate variant)
    let mut spatial_filter = SpatialModerateFilter::new().unwrap();
    spatial_filter.set_threshold(160).unwrap();
    spatial_filter.set_magnitude(3).unwrap();
    spatial_filter.set_radius(5).unwrap();

    // Create temporal filter
    let mut temporal_filter = TemporalFilter::new().unwrap();
    temporal_filter.set_threshold(0.1).unwrap();
    temporal_filter.set_weight(0.4).unwrap();

    // Hole filling filter
    let mut hole_filling_filter = HoleFillingFilter::new().unwrap();
    hole_filling_filter
        .set_mode(HoleFillMode::Farthest)
        .unwrap();

    // Threshold filter
    let mut threshold_filter = ThresholdFilter::new().unwrap();
    threshold_filter.set_min_depth(200).unwrap();
    threshold_filter.set_max_depth(4000).unwrap();

    // Start streaming
    pipeline.start(&config).unwrap();

    // Create window to display both depth images (original and filtered)
    let window_config = WindowOptions {
        preserve_aspect_ratio: true,
        default_controls: false,
        ..Default::default()
    };
    let orig_window = create_window("Original Depth", window_config.clone())
        .expect("Failed to create original depth window");
    let filter_window = create_window("Filtered Depth", window_config.clone())
        .expect("Failed to create filtered depth window");

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

        // Get depth frame
        let depth_frame = frameset.get_depth_frame().unwrap().unwrap();

        // Get original depth frame resolution
        let orig_depth_res = (depth_frame.width(), depth_frame.height());

        // Get original depth frame data
        let orig_depth_data = depth_frame.raw_data();

        // Apply filters
        let depth_frame = decimation_filter.process(&depth_frame).unwrap();
        let depth_frame = spatial_filter.process(&depth_frame).unwrap();
        let depth_frame = temporal_filter.process(&depth_frame).unwrap();
        let depth_frame = hole_filling_filter.process(&depth_frame).unwrap();
        let depth_frame = threshold_filter.process(&depth_frame).unwrap();

        // Get filtered depth frame resolution
        let filtered_depth_res = (depth_frame.width(), depth_frame.height());

        // Get depth filtered frame data
        let depth_filtered_data = depth_frame.raw_data();

        // Convert original depth to color image
        let orig_depth_image = utils::depth2image(orig_depth_data).unwrap();

        // Convert filtered depth to color image
        let depth_image = utils::depth2image(depth_filtered_data).unwrap();

        // Display the original depth image
        let orig_image = ImageView::new(
            ImageInfo::rgb8(orig_depth_res.0 as u32, orig_depth_res.1 as u32),
            &orig_depth_image,
        );
        orig_window.set_image("Original Depth", orig_image).unwrap();

        // Display the filtered depth image
        let depth_image = ImageView::new(
            ImageInfo::rgb8(filtered_depth_res.0 as u32, filtered_depth_res.1 as u32),
            &depth_image,
        );
        filter_window
            .set_image("Filtered Depth", depth_image)
            .unwrap();
    }
}
