//! Pipeline configuration and management
use crate::sys::enums::OBFrameAggregateOutputMode;

use super::device::OBDevice;
use super::enums::{OBAlignMode, OBSensorType};
use super::frame::OBFrame;
use super::stream::{OBStreamProfile, OBStreamProfileList};
use super::{OBError, drop_ob_object, orb};

/// Pipeline Configuration
pub struct OBConfig {
    inner: *mut orb::ob_config,
}

drop_ob_object!(OBConfig, ob_delete_config);

impl OBConfig {
    /// Create a configuration object with default parameters
    pub fn new() -> Result<Self, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let config = unsafe { orb::ob_create_config(&mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBConfig { inner: config })
    }

    /// Enable the specified stream with the given profile
    pub fn enable_stream_with_profile(&self, profile: &OBStreamProfile) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_config_enable_stream_with_stream_profile(
                self.inner,
                profile.inner(),
                &mut err_ptr,
            )
        };

        OBError::consume(err_ptr)
    }

    /// Set the alignment mode for the pipeline configuration
    pub fn set_align_mode(&self, mode: OBAlignMode) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe { orb::ob_config_set_align_mode(self.inner, mode as i32, &mut err_ptr) };

        OBError::consume(err_ptr)
    }

    /// Set whether depth scaling is required after enable depth to color alignment
    pub fn set_depth_scale_after_align_require(&self, enable: bool) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_config_set_depth_scale_after_align_require(self.inner, enable, &mut err_ptr)
        };

        OBError::consume(err_ptr)
    }

    /// Set the frame aggregation output mode for the pipeline configuration
    pub fn set_frame_aggregate_output_mode(
        &self,
        mode: OBFrameAggregateOutputMode,
    ) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_config_set_frame_aggregate_output_mode(self.inner, mode as i32, &mut err_ptr)
        };

        OBError::consume(err_ptr)
    }
}

/// Camera pipeline class
pub struct OBPipeline {
    inner: *mut orb::ob_pipeline,
}

drop_ob_object!(OBPipeline, ob_delete_pipeline);

impl OBPipeline {
    pub fn new(device: &OBDevice) -> Result<Self, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let pipeline = unsafe { orb::ob_create_pipeline_with_device(device.inner(), &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBPipeline { inner: pipeline })
    }

    /// Get the device object associated with the pipeline
    pub fn get_device(&self) -> Result<OBDevice, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let device = unsafe { orb::ob_pipeline_get_device(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBDevice::new(device))
    }

    /// Get a list of stream profiles supported by the specified sensor
    pub fn get_stream_profile_list(
        &self,
        sensor: OBSensorType,
    ) -> Result<OBStreamProfileList, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let profile_list = unsafe {
            orb::ob_pipeline_get_stream_profile_list(self.inner, sensor as i32, &mut err_ptr)
        };

        OBError::consume(err_ptr)?;

        Ok(OBStreamProfileList::new(profile_list))
    }

    /// Get the list of D2C-enabled depth sensor resolutions corresponding to the input color sensor resolution
    pub fn get_d2c_depth_profile_list(
        &self,
        color_profile: &OBStreamProfile,
        align_mode: OBAlignMode,
    ) -> Result<OBStreamProfileList, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let profile_list = unsafe {
            orb::ob_get_d2c_depth_profile_list(
                self.inner,
                color_profile.inner(),
                align_mode as i32,
                &mut err_ptr,
            )
        };

        OBError::consume(err_ptr)?;

        Ok(OBStreamProfileList::new(profile_list))
    }

    /// Enable frame synchronization
    pub fn enable_frame_sync(&self) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe { orb::ob_pipeline_enable_frame_sync(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)
    }

    /// Disable frame synchronization
    pub fn disable_frame_sync(&self) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe { orb::ob_pipeline_disable_frame_sync(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)
    }

    /// Start the pipeline with configuration parameters
    pub fn start_with_config(&self, config: &OBConfig) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe { orb::ob_pipeline_start_with_config(self.inner, config.inner, &mut err_ptr) };

        OBError::consume(err_ptr)
    }

    /// Wait for a set of frames to be returned synchronously
    pub fn wait_for_frameset(&self, timeout_ms: u32) -> Result<Option<OBFrame>, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let frame =
            unsafe { orb::ob_pipeline_wait_for_frameset(self.inner, timeout_ms, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(if frame.is_null() {
            None
        } else {
            Some(OBFrame::new(frame))
        })
    }
}
