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

    /// Get the width of this video stream profile in pixels
    pub fn width(&self) -> u32 {
        self.inner
            .get_video_width()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the height of this video stream profile in pixels
    pub fn height(&self) -> u32 {
        self.inner
            .get_video_height()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the frame rate of this video stream profile
    pub fn fps(&self) -> u32 {
        self.inner
            .get_video_fps()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the pixel format of this stream profile
    pub fn format(&self) -> Result<Format, OrbbecError> {
        self.inner.get_format().map_err(OrbbecError::from)
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

    /// Get the number of stream profiles in this list
    pub fn len(&self) -> Result<usize, OrbbecError> {
        self.inner
            .get_count()
            .map(|count| count as usize)
            .map_err(OrbbecError::from)
    }

    /// Return whether this list has no profiles
    pub fn is_empty(&self) -> Result<bool, OrbbecError> {
        Ok(self.len()? == 0)
    }

    /// Get the video stream profile at `index`
    pub fn get(&self, index: usize) -> Result<VideoStreamProfile, OrbbecError> {
        self.inner
            .get_stream_profile(index as u32)
            .map(VideoStreamProfile::new)
            .map_err(OrbbecError::from)
    }

    /// Get an iterator over the stream profiles in the list.
    pub fn iter(&self) -> StreamProfileListIterator<'_> {
        StreamProfileListIterator::new(self)
    }
}

/// An iterator over video stream profiles in a stream profile list
pub struct StreamProfileListIterator<'a> {
    profile_list: &'a StreamProfileList,
    index: usize,
    count: usize,
}

impl<'a> StreamProfileListIterator<'a> {
    fn new(profile_list: &'a StreamProfileList) -> Self {
        StreamProfileListIterator {
            profile_list,
            index: 0,
            count: profile_list.len().unwrap(),
        }
    }
}

impl<'a> Iterator for StreamProfileListIterator<'a> {
    type Item = Result<VideoStreamProfile, OrbbecError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            let profile = self.profile_list.get(self.index);
            self.index += 1;
            Some(profile)
        }
    }
}

impl<'a> IntoIterator for &'a StreamProfileList {
    type Item = Result<VideoStreamProfile, OrbbecError>;
    type IntoIter = StreamProfileListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
