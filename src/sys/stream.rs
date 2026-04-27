//! Stream profiles and related operations
use super::{OBError, call_ob_function, drop_ob_object, impl_ob_method, orb};
use crate::sys::orb::{OBCameraDistortion, OBCameraIntrinsic, OBD2CTransform, OBFormat};

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

    impl_ob_method!(
        /// Get video stream profile intrinsic
        get_video_intrinsic => OBCameraIntrinsic,
        orb::ob_video_stream_profile_get_intrinsic,
    );

    impl_ob_method!(
        /// Get video stream profile distortion
        get_video_distortion => OBCameraDistortion,
        orb::ob_video_stream_profile_get_distortion,
    );

    /// Get the extrinsic for source stream to target stream
    pub fn get_extrinsic_to(&self, target: &OBStreamProfile) -> Result<OBD2CTransform, OBError> {
        let transform = call_ob_function!(
            orb::ob_stream_profile_get_extrinsic_to,
            self.inner,
            target.inner(),
        )?;
        Ok(transform)
    }

    impl_ob_method!(
        /// Get stream profile format
        get_format => OBFormat,
        orb::ob_stream_profile_get_format,
    );

    impl_ob_method!(
        /// Get the frame rate of the video stream.
        /// Returns error if the profile is not a video stream profile.
        get_video_fps => u32,
        orb::ob_video_stream_profile_get_fps,
    );

    impl_ob_method!(
        /// Get the width of the video stream.
        /// Returns error if the profile is not a video stream profile.
        get_video_width => u32,
        orb::ob_video_stream_profile_get_width,
    );

    impl_ob_method!(
        /// Get the height of the video stream.
        /// Returns error if the profile is not a video stream profile.
        get_video_height => u32,
        orb::ob_video_stream_profile_get_height,
    );
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

    impl_ob_method!(
        /// Get the number of stream profiles in the list
        get_count => u32,
        orb::ob_stream_profile_list_get_count,
    );

    /// Get the stream profile at the specified index
    pub fn get_stream_profile(&self, index: u32) -> Result<OBStreamProfile, OBError> {
        let profile = call_ob_function!(
            orb::ob_stream_profile_list_get_profile,
            self.inner,
            index as i32
        )?;
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
        let profile = call_ob_function!(
            orb::ob_stream_profile_list_get_video_stream_profile,
            self.inner,
            width,
            height,
            format,
            fps
        )?;
        Ok(OBStreamProfile::new(profile))
    }
}
