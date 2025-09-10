//! Frame module
use crate::{Format, error::OrbbecError, sys::frame::OBFrame};

/// Frame trait
pub trait Frame: From<OBFrame> + AsRef<OBFrame> {}

/// Single video frame
pub struct VideoFrame {
    inner: OBFrame,
}

impl VideoFrame {
    /// Get the raw data of the video frame
    pub fn raw_data(&self) -> &[u8] {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L219
        self.inner.get_data().unwrap()
    }

    /// Get the width of the video frame
    pub fn width(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a video frame
        // SDK only returns error for this function if pointer is NULL or not a video frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L129
        self.inner.get_video_width().map(|w| w as u16).unwrap()
    }

    /// Get the height of the video frame
    pub fn height(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a video frame
        // SDK only returns error for this function if pointer is NULL or not a video frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L138
        self.inner.get_video_height().map(|h| h as u16).unwrap()
    }

    /// Get the format of the video frame data
    pub fn format(&self) -> Format {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a video frame
        // SDK only returns error for this function if pointer is NULL or not a video frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L147
        self.inner.get_format().unwrap()
    }
}

impl From<OBFrame> for VideoFrame {
    fn from(frame: OBFrame) -> Self {
        VideoFrame { inner: frame }
    }
}

impl AsRef<OBFrame> for VideoFrame {
    fn as_ref(&self) -> &OBFrame {
        &self.inner
    }
}

impl Frame for VideoFrame {}

/// A container of multiple frames.
pub struct FrameSet {
    inner: OBFrame,
}

impl FrameSet {
    /// Get the depth frame from the frameset
    pub fn get_depth_frame(&self) -> Result<Option<VideoFrame>, OrbbecError> {
        self.inner
            .get_depth_frame()
            .map_err(OrbbecError::from)
            .map(|frame| frame.map(VideoFrame::from))
    }

    /// Get the color frame from the frameset
    pub fn get_color_frame(&self) -> Result<Option<VideoFrame>, OrbbecError> {
        self.inner
            .get_color_frame()
            .map_err(OrbbecError::from)
            .map(|frame| frame.map(VideoFrame::from))
    }
}

impl From<OBFrame> for FrameSet {
    fn from(frame: OBFrame) -> Self {
        FrameSet { inner: frame }
    }
}

impl AsRef<OBFrame> for FrameSet {
    fn as_ref(&self) -> &OBFrame {
        &self.inner
    }
}

impl Frame for FrameSet {}
