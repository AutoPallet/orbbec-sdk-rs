//! Frame and FrameSet related operations
use super::enums::OBFormat;
use super::{OBError, drop_ob_object, orb};

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
        let mut err_ptr = std::ptr::null_mut();

        let size = unsafe { orb::ob_frame_get_data_size(self.inner, &mut err_ptr) } as usize;
        OBError::consume(err_ptr)?;

        let data = unsafe { orb::ob_frame_get_data(self.inner, &mut err_ptr) };
        OBError::consume(err_ptr)?;

        Ok(unsafe { std::slice::from_raw_parts(data as *const u8, size) })
    }

    /// Get the format of the frame
    pub fn get_format(&self) -> Result<OBFormat, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let format = unsafe { orb::ob_frame_get_format(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(format.into())
    }

    /// Get the depth frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_depth_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let frame = unsafe { orb::ob_frameset_get_depth_frame(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    /// Get the color frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_color_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let frame = unsafe { orb::ob_frameset_get_color_frame(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    /// Get the point cloud frame from the frameset.
    /// Only valid for frameset frames.
    pub fn get_points_frame(&self) -> Result<Option<OBFrame>, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let frame = unsafe { orb::ob_frameset_get_points_frame(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }

    /// Get the width of the video frame.
    /// Only valid for video frames.
    pub fn get_video_width(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let width = unsafe { orb::ob_video_frame_get_width(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(width)
    }

    /// Get the height of the video frame.
    /// Only valid for video frames.
    pub fn get_video_height(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let height = unsafe { orb::ob_video_frame_get_height(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(height)
    }

    /// Get the width of the point cloud frame.
    /// Only valid for point cloud frames.
    pub fn get_points_width(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let width = unsafe { orb::ob_point_cloud_frame_get_width(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(width)
    }

    /// Get the height of the point cloud frame.
    /// Only valid for point cloud frames.
    pub fn get_points_height(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let height = unsafe { orb::ob_point_cloud_frame_get_height(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(height)
    }

    /// Get the point cloud coordinate scale.
    /// Only valid for point cloud frames.
    pub fn get_points_scale(&self) -> Result<f32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let scale =
            unsafe { orb::ob_points_frame_get_coordinate_value_scale(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(scale)
    }
}
