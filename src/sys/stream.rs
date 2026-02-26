//! Stream profiles and related operations
use super::enums::OBFormat;
use super::{OBError, drop_ob_object, orb};

/// Camera intrinsic parameters
pub struct OBCameraIntrinsic {
    inner: orb::OBCameraIntrinsic,
}

impl OBCameraIntrinsic {
    /// Focal length in pixels along X axis
    pub fn fx(&self) -> f32 {
        self.inner.fx
    }

    /// Focal length in pixels along Y axis
    pub fn fy(&self) -> f32 {
        self.inner.fy
    }

    /// Optical center abscissa
    pub fn cx(&self) -> f32 {
        self.inner.cx
    }

    /// Optical center ordinate
    pub fn cy(&self) -> f32 {
        self.inner.cy
    }

    /// Image width in pixels
    pub fn width(&self) -> i16 {
        self.inner.width
    }

    /// Image height in pixels
    pub fn height(&self) -> i16 {
        self.inner.height
    }
}

impl From<orb::OBCameraIntrinsic> for OBCameraIntrinsic {
    fn from(intrinsic: orb::OBCameraIntrinsic) -> Self {
        OBCameraIntrinsic { inner: intrinsic }
    }
}

/// Stream profile
pub struct OBStreamProfile {
    inner: *mut orb::ob_stream_profile,
}

drop_ob_object!(OBStreamProfile, ob_delete_stream_profile);

impl OBStreamProfile {
    pub(crate) fn new(inner: *mut orb::ob_stream_profile) -> Self {
        OBStreamProfile { inner }
    }

    pub(crate) fn inner(&self) -> *mut orb::ob_stream_profile {
        self.inner
    }

    /// Get video stream profile intrinsic
    pub fn get_video_intrinsic(&self) -> Result<OBCameraIntrinsic, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let intrinsics =
            unsafe { orb::ob_video_stream_profile_get_intrinsic(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBCameraIntrinsic::from(intrinsics))
    }

    /// Get stream profile format
    pub fn get_format(&self) -> Result<OBFormat, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let format = unsafe { orb::ob_stream_profile_get_format(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(format.into())
    }

    /// Get the frame rate of the video stream.
    /// Returns error if the profile is not a video stream profile.
    pub fn get_video_fps(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let fps = unsafe { orb::ob_video_stream_profile_get_fps(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(fps)
    }

    /// Get the width of the video stream.
    /// Returns error if the profile is not a video stream profile.
    pub fn get_video_width(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let width = unsafe { orb::ob_video_stream_profile_get_width(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(width)
    }

    /// Get the height of the video stream.
    /// Returns error if the profile is not a video stream profile.
    pub fn get_video_height(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let height = unsafe { orb::ob_video_stream_profile_get_height(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(height)
    }
}

/// List of video stream profiles
pub struct OBStreamProfileList {
    inner: *mut orb::ob_stream_profile_list,
}

drop_ob_object!(OBStreamProfileList, ob_delete_stream_profile_list);

impl OBStreamProfileList {
    pub(crate) fn new(inner: *mut orb::ob_stream_profile_list) -> Self {
        OBStreamProfileList { inner }
    }

    /// Get the number of stream profiles in the list
    pub fn get_count(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let count = unsafe { orb::ob_stream_profile_list_get_count(self.inner, &mut err_ptr) };
        OBError::consume(err_ptr)?;

        Ok(count)
    }

    /// Get the stream profile at the specified index
    pub fn get_stream_profile(&self, index: u32) -> Result<OBStreamProfile, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let profile = unsafe {
            orb::ob_stream_profile_list_get_profile(self.inner, index as i32, &mut err_ptr)
        };
        OBError::consume(err_ptr)?;

        Ok(OBStreamProfile::new(profile))
    }

    /// Match the corresponding ob_stream_profile through the passed parameters. If there are multiple matches, the first one in the list will be returned by default.
    pub fn get_video_stream_profile(
        &self,
        width: i32,
        height: i32,
        format: OBFormat,
        fps: i32,
    ) -> Result<OBStreamProfile, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let profile = unsafe {
            orb::ob_stream_profile_list_get_video_stream_profile(
                self.inner,
                width,
                height,
                format as i32,
                fps,
                &mut err_ptr,
            )
        };

        OBError::consume(err_ptr)?;

        Ok(OBStreamProfile::new(profile))
    }
}
