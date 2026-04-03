//! Pipeline configuration and management
use crate::sys::orb::OBSensorType;

use super::device::OBDevice;
use super::frame::OBFrame;
use super::stream::{OBStreamProfile, OBStreamProfileList};
use super::{OBError, call_ob_function, drop_ob_object, impl_ob_method, orb};

/// Pipeline Configuration
pub struct OBConfig {
    inner: *mut orb::ob_config,
}

drop_ob_object!(OBConfig, ob_delete_config);

impl OBConfig {
    /// Create a configuration object with default parameters
    pub fn new() -> Result<Self, OBError> {
        let config = call_ob_function!(orb::ob_create_config)?;
        Ok(OBConfig { inner: config })
    }

    /// Enable the specified stream with the given profile
    pub fn enable_stream_with_profile(&self, profile: &OBStreamProfile) -> Result<(), OBError> {
        call_ob_function!(
            orb::ob_config_enable_stream_with_stream_profile,
            self.inner,
            profile.inner()
        )
    }
}

/// Camera pipeline class
pub struct OBPipeline {
    inner: *mut orb::ob_pipeline,
}

drop_ob_object!(OBPipeline, ob_delete_pipeline);

impl OBPipeline {
    pub fn new(device: &OBDevice) -> Result<Self, OBError> {
        let pipeline = call_ob_function!(orb::ob_create_pipeline_with_device, device.inner())?;
        Ok(OBPipeline { inner: pipeline })
    }

    /// Get the device object associated with the pipeline
    pub fn get_device(&self) -> Result<OBDevice, OBError> {
        let device = call_ob_function!(orb::ob_pipeline_get_device, self.inner)?;
        Ok(OBDevice::new(device))
    }

    /// Get a list of stream profiles supported by the specified sensor
    pub fn get_stream_profile_list(
        &self,
        sensor: OBSensorType,
    ) -> Result<OBStreamProfileList, OBError> {
        let list = call_ob_function!(orb::ob_pipeline_get_stream_profile_list, self.inner, sensor)?;
        Ok(OBStreamProfileList::new(list))
    }

    impl_ob_method!(
        /// Enable frame synchronization
        enable_frame_sync => (),
        orb::ob_pipeline_enable_frame_sync,
    );

    impl_ob_method!(
        /// Disable frame synchronization
        disable_frame_sync => (),
        orb::ob_pipeline_disable_frame_sync,
    );

    /// Start the pipeline with configuration parameters
    pub fn start_with_config(&self, config: &OBConfig) -> Result<(), OBError> {
        call_ob_function!(orb::ob_pipeline_start_with_config, self.inner, config.inner)
    }

    /// Wait for a set of frames to be returned synchronously
    pub fn wait_for_frameset(&self, timeout_ms: u32) -> Result<Option<OBFrame>, OBError> {
        let frame = call_ob_function!(orb::ob_pipeline_wait_for_frameset, self.inner, timeout_ms)?;
        Ok(if frame.is_null() { None } else { Some(OBFrame::new(frame)) })
    }
}
