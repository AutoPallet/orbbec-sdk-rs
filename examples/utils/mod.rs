//! Utility functions for examples
#![allow(dead_code)]

use anyhow::Context;
use colorous::TURBO;
use orbbec_sdk::{
    Format,
    stream::{StreamProfileList, VideoStreamProfile},
};

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

pub fn get_video_stream_profile(
    profiles: &StreamProfileList,
    width: u16,
    height: u16,
    format: Format,
    fps: u8,
) -> anyhow::Result<VideoStreamProfile> {
    profiles.get_video_stream_profile(width, height, format, fps).with_context(|| 
        anyhow::anyhow!(
            "Requested video stream profile {}x{} @ {}fps ({:?}) is not supported. Available video stream profiles:\n{}",
            width,
            height,
            fps,
            format,
            describe_video_profiles(profiles)
        )
    )
}

pub fn describe_video_profiles(profiles: &StreamProfileList) -> String {
    let mut entries = Vec::new();

    for profile in profiles {
        match profile {
            Ok(profile) => {
                let format = profile
                    .format()
                    .map(|format| format!("{format:?}"))
                    .unwrap_or_else(|_| "Unknown".to_string());
                entries.push(format!(
                    "{}x{} @ {}fps ({format})",
                    profile.width(),
                    profile.height(),
                    profile.fps()
                ));
            }
            Err(err) => entries.push(format!("<error reading profile: {err}>")),
        }
    }

    if entries.is_empty() {
        "none reported".to_string()
    } else {
        entries.join("\n")
    }
}
