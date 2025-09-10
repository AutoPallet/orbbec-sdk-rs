//! Utility functions for examples
#![allow(dead_code)]

use colorous::TURBO;

/// Convert depth data to a color image for visualization
pub fn depth2image(depth: &[u8]) -> Option<Vec<u8>> {
    // Depth data is in u16 format
    assert!(depth.len() % 2 == 0);
    let depth_u16 =
        unsafe { std::slice::from_raw_parts(depth.as_ptr() as *const u16, depth.len() / 2) };

    // Find min and max depth values for normalization
    let (min, max) = depth_u16
        .iter()
        .fold((u16::MAX, u16::MIN), |(min, max), &d| {
            (if d != 0 { min.min(d) } else { min }, max.max(d))
        });

    // Map depth values to colors using the TURBO colormap
    let bgr = depth_u16
        .iter()
        .flat_map(|&d| {
            if d == 0 {
                [0, 0, 0]
            } else {
                let normalized = (d - min) as f64 / (max - min) as f64;
                let color = TURBO.eval_continuous(normalized);
                [color.b, color.g, color.r]
            }
        })
        .collect::<Vec<u8>>();

    Some(bgr)
}

/// Add two images together
pub fn add_images(img1: &[u8], img2: &[u8]) -> Vec<u8> {
    // Both images must be the same size
    assert!(img1.len() == img2.len());

    // Add images together (50% each)

    img1.iter()
        .zip(img2.iter())
        .map(|(&a, &b)| (a / 2) + (b / 2))
        .collect::<Vec<u8>>()
}
