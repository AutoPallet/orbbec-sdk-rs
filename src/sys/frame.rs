//! Frame and FrameSet related operations
use super::orb::OBFormat;
use super::{OBError, call_ob_function, drop_ob_object, impl_ob_method, orb};

/// A container of one or multiple frames
pub struct OBFrame {
    inner: *mut orb::ob_frame,
}

drop_ob_object!(OBFrame, ob_delete_frame);

impl OBFrame {
    pub fn new(inner: *mut orb::ob_frame) -> Self {
        OBFrame { inner }
    }

    pub(crate) fn inner(&self) -> *mut orb::ob_frame {
        self.inner
    }

    /// Get the data buffer of a frame
    pub fn get_data(&self) -> Result<&[u8], OBError> {
        let size = call_ob_function!(orb::ob_frame_get_data_size, self.inner)? as usize;
        let data = call_ob_function!(orb::ob_frame_get_data, self.inner)?;
        Ok(unsafe { std::slice::from_raw_parts(data as *const u8, size) })
    }

    /// Get the format of the frame
    pub fn get_format(&self) -> Result<OBFormat, OBError> {
        Ok(call_ob_function!(orb::ob_frame_get_format, self.inner)?.into())
    }

    /// Get the depth frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_depth_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let frame = call_ob_function!(orb::ob_frameset_get_depth_frame, self.inner)?;
        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    /// Get the color frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_color_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let frame = call_ob_function!(orb::ob_frameset_get_color_frame, self.inner)?;
        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    /// Get the point cloud frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_points_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let frame = call_ob_function!(orb::ob_frameset_get_points_frame, self.inner)?;
        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    impl_ob_method!(
        /// Get the width of the video frame.
        /// Only valid for video frames.
        get_video_width => u32,
        orb::ob_video_frame_get_width,
    );

    impl_ob_method!(
        /// Get the height of the video frame.
        /// Only valid for video frames.
        get_video_height => u32,
        orb::ob_video_frame_get_height,
    );

    impl_ob_method!(
        /// Get the width of the point cloud frame.
        /// Only valid for point cloud frames.
        get_points_width => u32,
        orb::ob_point_cloud_frame_get_width,
    );

    impl_ob_method!(
        /// Get the height of the point cloud frame.
        /// Only valid for point cloud frames.
        get_points_height => u32,
        orb::ob_point_cloud_frame_get_height,
    );

    impl_ob_method!(
        /// Get the point cloud coordinate scale.
        /// Only valid for point cloud frames.
        get_points_scale => f32,
        orb::ob_points_frame_get_coordinate_value_scale,
    );
}
