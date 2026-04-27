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

    /// Get the frame timestamp (also known as device timestamp, hardware timestamp) of the frame in microseconds.
    /// The hardware timestamp is the time point when the frame was captured by the device (Typically in the mid-exposure, unless otherwise stated), on device clock domain.
    pub fn get_timestamp_us(&self) -> Result<u64, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let timestamp = unsafe { orb::ob_frame_get_timestamp_us(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(timestamp)
    }

    /// Get the system timestamp of the frame in microseconds.
    /// The system timestamp is the time point when the frame was received by the host, on host clock domain.
    pub fn get_system_timestamp_us(&self) -> Result<u64, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let timestamp = unsafe { orb::ob_frame_get_system_timestamp_us(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(timestamp)
    }

    /// Get the global timestamp of the frame in microseconds.
    /// The global timestamp is the time point when the frame was captured by the device, and has been converted to the host clock domain. The conversion process base on the frame timestamp and can eliminate the timer drift of the device
    pub fn get_global_timestamp_us(&self) -> Result<u64, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let timestamp = unsafe { orb::ob_frame_get_global_timestamp_us(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(timestamp)
    }

    /// Get the data buffer of a frame
    pub fn get_data(&self) -> Result<&[u8], OBError> {
        let size = call_ob_function!(orb::ob_frame_get_data_size, self.inner)? as usize;
        let data = call_ob_function!(orb::ob_frame_get_data, self.inner)?;
        Ok(unsafe { std::slice::from_raw_parts(data as *const u8, size) })
    }

    /// Get the format of the frame
    pub fn get_format(&self) -> Result<OBFormat, OBError> {
        Ok(call_ob_function!(orb::ob_frame_get_format, self.inner)?)
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

    impl_ob_method!(
        /// Get the depth frame scale.
        /// Only valid for depth frames.
        get_depth_scale => f32,
        orb::ob_depth_frame_get_value_scale,
    );
}
