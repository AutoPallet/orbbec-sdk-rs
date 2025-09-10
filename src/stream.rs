//! Stream module
use crate::{
    Format,
    error::OrbbecError,
    sys::stream::{OBCameraIntrinsic, OBStreamProfile, OBStreamProfileList},
};

/// Stream profile trait
pub trait StreamProfile: AsRef<OBStreamProfile> {}

/// Camera intrinsic for a stream
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraIntrinsic {
    /// Focal length in pixels along X axis
    pub fx: f32,
    /// Focal length in pixels along Y axis
    pub fy: f32,
    /// Optical center abscissa
    pub cx: f32,
    /// Optical center ordinate
    pub cy: f32,
    /// Image width in pixels
    pub width: u16,
    /// Image height in pixels
    pub height: u16,
}

impl From<OBCameraIntrinsic> for CameraIntrinsic {
    fn from(intrinsic: OBCameraIntrinsic) -> Self {
        CameraIntrinsic {
            fx: intrinsic.fx(),
            fy: intrinsic.fy(),
            cx: intrinsic.cx(),
            cy: intrinsic.cy(),
            width: intrinsic.width() as u16,
            height: intrinsic.height() as u16,
        }
    }
}

/// Video Stream profile
pub struct VideoStreamProfile {
    inner: OBStreamProfile,
}

impl VideoStreamProfile {
    pub(crate) fn new(inner: OBStreamProfile) -> Self {
        VideoStreamProfile { inner }
    }

    pub(crate) fn inner(&self) -> &OBStreamProfile {
        &self.inner
    }

    /// Get the camera intrinsic parameters for this video stream profile
    pub fn get_intrinsic(&self) -> Result<CameraIntrinsic, OrbbecError> {
        self.inner
            .get_video_intrinsic()
            .map(|intrinsic| intrinsic.into())
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBStreamProfile> for VideoStreamProfile {
    fn as_ref(&self) -> &OBStreamProfile {
        &self.inner
    }
}
impl StreamProfile for VideoStreamProfile {}

/// List of video stream profiles
pub struct StreamProfileList {
    inner: OBStreamProfileList,
}

impl StreamProfileList {
    pub(crate) fn new(inner: OBStreamProfileList) -> Self {
        StreamProfileList { inner }
    }

    /// Get the corresponding video stream profile through the passed parameters.
    /// ### Arguments
    /// * `width` - Width of the stream in pixels
    /// * `height` - Height of the stream in pixels
    /// * `format` - Pixel format of the stream
    /// * `fps` - Frame rate of the stream in frames per second
    pub fn get_video_stream_profile(
        &self,
        width: u16,
        height: u16,
        format: Format,
        fps: u8,
    ) -> Result<VideoStreamProfile, OrbbecError> {
        self.inner
            .get_video_stream_profile(width as i32, height as i32, format, fps as i32)
            .map(VideoStreamProfile::new)
            .map_err(OrbbecError::from)
    }
}
