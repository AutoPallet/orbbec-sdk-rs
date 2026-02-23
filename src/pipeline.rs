//! Pipeline module
use std::time::Duration;

use crate::{
    AlignMode, FrameAggregateOutputMode, SensorType,
    device::Device,
    frame::FrameSet,
    stream::{StreamProfile, StreamProfileList, VideoStreamProfile},
    sys::pipeline::{OBConfig, OBPipeline},
};

/// Pipeline Configuration
pub struct Config {
    inner: OBConfig,
}

impl Config {
    /// Create a new configuration with default parameters
    pub fn new() -> Result<Self, crate::error::OrbbecError> {
        let config = OBConfig::new().map_err(crate::error::OrbbecError::from)?;

        Ok(Config { inner: config })
    }

    /// Enable a stream with the given profile
    /// ### Arguments
    /// * `profile` - Stream profile to enable
    pub fn enable_stream_with_profile<S: StreamProfile>(
        &mut self,
        profile: &S,
    ) -> Result<(), crate::error::OrbbecError> {
        self.inner
            .enable_stream_with_profile(profile.as_ref())
            .map_err(crate::error::OrbbecError::from)
    }

    /// Set the alignment mode for the pipeline configuration
    /// ### Arguments
    /// * `mode` - Alignment mode to set
    pub fn set_align_mode(&mut self, mode: AlignMode) -> Result<(), crate::error::OrbbecError> {
        self.inner
            .set_align_mode(mode)
            .map_err(crate::error::OrbbecError::from)
    }

    /// Set whether depth scaling is required after enable depth to color alignment
    /// ### Arguments
    /// * `enable` - `true` to enable depth scaling, `false` to disable it
    pub fn set_depth_scale_after_align_require(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::error::OrbbecError> {
        self.inner
            .set_depth_scale_after_align_require(enable)
            .map_err(crate::error::OrbbecError::from)
    }

    /// Set the frame aggregation output mode for the pipeline configuration
    /// ### Arguments
    /// * `mode` - Frame aggregation output mode to set
    pub fn set_frame_aggregate_output_mode(
        &mut self,
        mode: FrameAggregateOutputMode,
    ) -> Result<(), crate::error::OrbbecError> {
        self.inner
            .set_frame_aggregate_output_mode(mode)
            .map_err(crate::error::OrbbecError::from)
    }
}

/// Pipeline
pub struct Pipeline {
    inner: OBPipeline,
}

impl Pipeline {
    /// Create a new pipeline for the given device
    /// ### Arguments
    /// * `device` - Device to create the pipeline for
    pub fn new(device: &Device) -> Result<Self, crate::error::OrbbecError> {
        let pipeline = OBPipeline::new(device.inner()).map_err(crate::error::OrbbecError::from)?;

        Ok(Pipeline { inner: pipeline })
    }

    /// Get the device associated with the pipeline
    pub fn get_device(&mut self) -> Result<Device, crate::error::OrbbecError> {
        let device = self
            .inner
            .get_device()
            .map_err(crate::error::OrbbecError::from)?;

        Ok(Device::new(device))
    }

    /// Get a list of stream profiles supported by the specified sensor
    /// ### Arguments
    /// * `sensor` - Sensor type to get the stream profiles for
    pub fn get_stream_profiles(
        &mut self,
        sensor: SensorType,
    ) -> Result<StreamProfileList, crate::error::OrbbecError> {
        let profile_list = self
            .inner
            .get_stream_profile_list(sensor)
            .map(StreamProfileList::new)
            .map_err(crate::error::OrbbecError::from)?;

        Ok(profile_list)
    }

    /// Get the list of D2C-enabled depth sensor resolutions corresponding to the input color sensor resolution
    /// ### Arguments
    /// * `color_profile` - Color sensor profile
    /// * `align_mode` - Depth alignment mode
    pub fn get_d2c_depth_profiles(
        &mut self,
        color_profile: &VideoStreamProfile,
        align_mode: AlignMode,
    ) -> Result<StreamProfileList, crate::error::OrbbecError> {
        let profile_list = self
            .inner
            .get_d2c_depth_profile_list(color_profile.as_ref(), align_mode)
            .map(StreamProfileList::new)
            .map_err(crate::error::OrbbecError::from)?;

        Ok(profile_list)
    }

    /// Start the pipeline with the given configuration
    /// ### Arguments
    /// * `config` - Configuration to use for the pipeline
    pub fn start(&mut self, config: &Config) -> Result<(), crate::error::OrbbecError> {
        self.inner
            .start_with_config(&config.inner)
            .map_err(crate::error::OrbbecError::from)
    }

    /// Set if frames should be synchronized
    /// ### Arguments
    /// * `enable` - `true` to enable frame synchronization, `false` to disable it
    pub fn set_frame_sync(&mut self, enable: bool) -> Result<(), crate::error::OrbbecError> {
        let res = if enable {
            self.inner.enable_frame_sync()
        } else {
            self.inner.disable_frame_sync()
        };

        res.map_err(crate::error::OrbbecError::from)
    }

    /// Wait for the next set of frames from the pipeline
    /// ### Arguments
    /// * `timeout` - Maximum time to wait for frames
    pub fn wait_for_frames(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<FrameSet>, crate::error::OrbbecError> {
        self.inner
            .wait_for_frameset(timeout.as_millis() as u32)
            .map(|frame| frame.map(FrameSet::from))
            .map_err(crate::error::OrbbecError::from)
    }
}
