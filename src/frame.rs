//! Frame module
use crate::{Format, error::OrbbecError, sys::frame::OBFrame};

/// Frame trait
pub trait Frame: From<OBFrame> + AsRef<OBFrame> {}

/// Video Frame Implementation Macro
macro_rules! impl_video_frame {
    ($t:ident) => {
        impl $t {
            /// Get the device timestamp of the video frame
            pub fn timestamp_us(&self) -> u64 {
                self.inner.get_timestamp_us().unwrap()
            }

            /// Get the system timestamp of the video frame
            pub fn system_timestamp_us(&self) -> Result<u64, OrbbecError> {
                self.inner
                    .get_system_timestamp_us()
                    .map_err(OrbbecError::from)
            }

            /// Get the global timestamp of the video frame
            pub fn global_timestamp_us(&self) -> Result<u64, OrbbecError> {
                self.inner
                    .get_global_timestamp_us()
                    .map_err(OrbbecError::from)
            }

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

        impl From<OBFrame> for $t {
            fn from(frame: OBFrame) -> Self {
                $t { inner: frame }
            }
        }

        impl AsRef<OBFrame> for $t {
            fn as_ref(&self) -> &OBFrame {
                &self.inner
            }
        }

        impl Frame for $t {}
    };
}

/// Color frame
pub struct ColorFrame {
    inner: OBFrame,
}
impl_video_frame!(ColorFrame);

/// Depth frame
pub struct DepthFrame {
    inner: OBFrame,
}
impl_video_frame!(DepthFrame);

impl DepthFrame {
    /// Get the scale (pixel value * scale = distance in millimeters) of the depth frame
    pub fn depth_scale(&self) -> f32 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a depth frame
        // SDK only returns error for this function if pointer is NULL or not a depth frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L198
        self.inner.get_depth_scale().unwrap()
    }
}

/// Point Cloud frame
pub struct PointCloudFrame {
    inner: OBFrame,
}

impl PointCloudFrame {
    /// Get the raw data of the point cloud frame
    pub fn raw_data(&self) -> &[u8] {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L219
        self.inner.get_data().unwrap()
    }

    /// Get the width of the point cloud frame
    pub fn width(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a point cloud frame
        // SDK only returns error for this function if pointer is NULL or not a point cloud frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L529
        self.inner.get_points_width().map(|w| w as u16).unwrap()
    }

    /// Get the height of the video frame
    pub fn height(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a point cloud frame
        // SDK only returns error for this function if pointer is NULL or not a point cloud frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L538
        self.inner.get_points_height().map(|h| h as u16).unwrap()
    }

    /// Check if the point cloud frame has color information
    pub fn has_color(&self) -> bool {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a video frame
        // SDK only returns error for this function if pointer is NULL or not a video frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L147
        let format = self.inner.get_format().unwrap();

        format == Format::RGBPoint
    }

    /// Get the point cloud coordinate scale
    pub fn coordinate_scale(&self) -> f32 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid and a point cloud frame
        // SDK only returns error for this function if pointer is NULL or not a point cloud frame
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Frame.cpp#L219
        self.inner.get_points_scale().unwrap()
    }
}

impl From<OBFrame> for PointCloudFrame {
    fn from(frame: OBFrame) -> Self {
        PointCloudFrame { inner: frame }
    }
}

impl AsRef<OBFrame> for PointCloudFrame {
    fn as_ref(&self) -> &OBFrame {
        &self.inner
    }
}

impl Frame for PointCloudFrame {}

/// A container of multiple frames.
pub struct FrameSet {
    inner: OBFrame,
}

impl FrameSet {
    /// Get the depth frame from the frameset
    pub fn get_depth_frame(&self) -> Result<Option<DepthFrame>, OrbbecError> {
        self.inner
            .get_depth_frame()
            .map_err(OrbbecError::from)
            .map(|frame| frame.map(DepthFrame::from))
    }

    /// Get the color frame from the frameset
    pub fn get_color_frame(&self) -> Result<Option<ColorFrame>, OrbbecError> {
        self.inner
            .get_color_frame()
            .map_err(OrbbecError::from)
            .map(|frame| frame.map(ColorFrame::from))
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
