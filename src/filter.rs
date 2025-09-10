//! Filter module
use crate::{
    ConvertType, HoleFillMode, StreamType,
    error::{OrbbecError, OrbbecErrorData},
    frame::Frame,
    stream::VideoStreamProfile,
    sys::filter::OBFilter,
};

/// Filter trait
pub trait Filter<F: Frame>: AsRef<OBFilter> {
    /// Process a frame with the filter
    fn process(&self, frame: &F) -> Result<F, OrbbecError> {
        self.as_ref()
            .process(frame.as_ref())
            .map(|f| F::from(f))
            .map_err(OrbbecError::from)
    }
}

/// Decimation Filter
///
/// This filter reduces the resolution of the depth frame by an integer factor.
pub struct DecimationFilter {
    inner: OBFilter,
}

impl DecimationFilter {
    /// Create a new decimation filter.
    pub fn new() -> Result<Self, OrbbecError> {
        // Unwrap is safe because Decimation filter is always available (public filter)
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/filter/publicfilters/publicFilterLoader.cpp#L32
        let filter = OBFilter::new(c"DecimationFilter")?.unwrap();
        Ok(DecimationFilter { inner: filter })
    }

    /// Set the decimation factor.
    ///
    /// ### Arguments
    /// * `factor` - The decimation factor. Usually between 1 and 8.
    pub fn set_factor(&mut self, factor: u8) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"decimate", factor as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for DecimationFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for DecimationFilter {}

/// Format Convert Filter
///
/// This filter converts the format of the input frame to a different format.
pub struct FormatConvertFilter {
    inner: OBFilter,
}

impl FormatConvertFilter {
    /// Create a new format convert filter.
    pub fn new() -> Result<Self, OrbbecError> {
        // Unwrap is safe because FormatConvert filter is always available (public filter)
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/filter/publicfilters/publicFilterLoader.cpp#L32
        let filter = OBFilter::new(c"FormatConverter")?.unwrap();
        Ok(FormatConvertFilter { inner: filter })
    }

    /// Set the format conversion type
    /// ### Arguments
    /// * `convert_type` - The desired format conversion type.
    pub fn set_convert_type(&mut self, convert_type: ConvertType) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"convertType", convert_type as u32 as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for FormatConvertFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for FormatConvertFilter {}

/// Hole Filling Filter
///
/// This filter fills holes in the depth frame using a specified hole filling mode.
pub struct HoleFillingFilter {
    inner: OBFilter,
}

impl HoleFillingFilter {
    /// Create a new hole filling filter.
    pub fn new() -> Result<Self, OrbbecError> {
        match OBFilter::new(c"HoleFillingFilter")? {
            Some(f) => Ok(HoleFillingFilter { inner: f }),
            None => {
                let err_data = OrbbecErrorData {
                    message: "HoleFillingFilter is not available".to_string(),
                    function: "HoleFillingFilter::new".to_string(),
                    args: "".to_string(),
                };

                Err(OrbbecError::NotImplemented(err_data))
            }
        }
    }

    /// Set the hole filling mode.
    ///
    /// ### Arguments
    /// * `mode` - The hole filling mode.
    pub fn set_mode(&mut self, mode: HoleFillMode) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"hole_filling_mode", mode as u32 as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for HoleFillingFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for HoleFillingFilter {}

/// Temporal Filter
///
/// This filter smooths the depth frame over time to reduce noise and temporal artifacts.
pub struct TemporalFilter {
    inner: OBFilter,
}

impl TemporalFilter {
    /// Create a new temporal filter.
    pub fn new() -> Result<Self, OrbbecError> {
        match OBFilter::new(c"TemporalFilter")? {
            Some(f) => Ok(TemporalFilter { inner: f }),
            None => {
                let err_data = OrbbecErrorData {
                    message: "TemporalFilter is not available".to_string(),
                    function: "TemporalFilter::new".to_string(),
                    args: "".to_string(),
                };

                Err(OrbbecError::NotImplemented(err_data))
            }
        }
    }

    /// Set the threshold parameter.
    ///
    /// Threshold is the maximum change (in percentage) for a pixel to still be considered static (without motion).
    /// ### Arguments
    /// * `threshold` - The threshold value. Usually between 0.1 and 1.0.
    pub fn set_threshold(&mut self, threshold: f32) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"diff_scale", threshold as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the weight parameter.
    ///
    /// Weight is how much the current frame affects the output frame.
    /// ### Arguments
    /// * `weight` - The weight value. Usually between 0.1 and 1.0.
    pub fn set_weight(&mut self, weight: f32) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"weight", weight as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for TemporalFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for TemporalFilter {}

/// Spatial Fast Filter
///
/// This filter smooths the depth frame while preserving edges.
/// It uses an enhanced median smoothing algorithm and is the simplest spatial filter available.
pub struct SpatialFastFilter {
    inner: OBFilter,
}

impl SpatialFastFilter {
    /// Create a new spatial fast filter.
    pub fn new() -> Result<Self, OrbbecError> {
        match OBFilter::new(c"SpatialFastFilter")? {
            Some(f) => Ok(SpatialFastFilter { inner: f }),
            None => {
                let err_data = OrbbecErrorData {
                    message: "SpatialFastFilter is not available".to_string(),
                    function: "SpatialFastFilter::new".to_string(),
                    args: "".to_string(),
                };

                Err(OrbbecError::NotImplemented(err_data))
            }
        }
    }

    /// Set the filter radius parameter.
    ///
    /// Radius is how many neighbors will be considered for smoothing.
    /// ### Arguments
    /// * `radius` - The radius value. Usually between 3 and 5.
    pub fn set_radius(&mut self, radius: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"radius", radius as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for SpatialFastFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for SpatialFastFilter {}

/// Spatial Moderate Filter
///
/// This filter smooths the depth frame while preserving edges.
/// It uses an optimized average smoothing algorithm and is a balance between speed and quality.
pub struct SpatialModerateFilter {
    inner: OBFilter,
}

impl SpatialModerateFilter {
    /// Create a new spatial moderate filter.
    pub fn new() -> Result<Self, OrbbecError> {
        match OBFilter::new(c"SpatialModerateFilter")? {
            Some(f) => Ok(SpatialModerateFilter { inner: f }),
            None => {
                let err_data = OrbbecErrorData {
                    message: "SpatialModerateFilter is not available".to_string(),
                    function: "SpatialModerateFilter::new".to_string(),
                    args: "".to_string(),
                };

                Err(OrbbecError::NotImplemented(err_data))
            }
        }
    }

    /// Set the filter radius parameter.
    ///
    /// Radius is how many neighbors will be considered for smoothing.
    /// ### Arguments
    /// * `radius` - The radius value. Usually between 3 and 7.
    pub fn set_radius(&mut self, radius: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"radius", radius as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the filter magnitude parameter.
    ///
    /// Magnitude is the strength of the filter.
    /// ### Arguments
    /// * `magnitude` - The magnitude value. Usually between 1 and 3.
    pub fn set_magnitude(&mut self, magnitude: u8) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"magnitude", magnitude as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the filter threshold parameter.
    ///
    /// Threshold is the maximum difference between pixels to be considered part of the same surface.
    /// ### Arguments
    /// * `threshold` - The threshold value. Usually between 1 and 10000.
    pub fn set_threshold(&mut self, threshold: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"disp_diff", threshold as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for SpatialModerateFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for SpatialModerateFilter {}

///  Spatial Advanced Filter
///
/// This filter smooths the depth frame while preserving edges.
/// It uses a more advanced algorithm with more parameters to tune.
pub struct SpatialAdvancedFilter {
    inner: OBFilter,
}

impl SpatialAdvancedFilter {
    /// Create a new spatial advanced filter.
    pub fn new() -> Result<Self, OrbbecError> {
        match OBFilter::new(c"SpatialAdvancedFilter")? {
            Some(f) => Ok(SpatialAdvancedFilter { inner: f }),
            None => {
                let err_data = OrbbecErrorData {
                    message: "SpatialAdvancedFilter is not available".to_string(),
                    function: "SpatialAdvancedFilter::new".to_string(),
                    args: "".to_string(),
                };

                Err(OrbbecError::NotImplemented(err_data))
            }
        }
    }

    /// Set the filter alpha parameter.
    ///
    /// Alpha is the weight of a pixel vs neighbors of the same surface.
    /// ### Arguments
    /// * `alpha` - The alpha value. Usually between 0.1 and 1.0.
    pub fn set_alpha(&mut self, alpha: f32) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"alpha", alpha as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the filter threshold parameter.
    ///
    /// Threshold is the maximum difference between pixels to be considered part of the same surface.
    /// ### Arguments
    /// * `threshold` - The threshold value. Usually between 1 and 10000.
    pub fn set_threshold(&mut self, threshold: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"disp_diff", threshold as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the filter radius parameter.
    ///
    /// Radius is how many missing pixels will be filled in.
    /// ### Arguments
    /// * `radius` - The radius value. Usually between 0 and 8.
    pub fn set_radius(&mut self, radius: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"radius", radius as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the filter magnitude parameter.
    ///
    /// Magnitude is the strength of the filter.
    /// ### Arguments
    /// * `magnitude` - The magnitude value. Usually between 1 and 5.
    pub fn set_magnitude(&mut self, magnitude: u8) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"magnitude", magnitude as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for SpatialAdvancedFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for SpatialAdvancedFilter {}

/// Threshold filter
///
/// This filter removes depth values outside the specified range.
pub struct ThresholdFilter {
    inner: OBFilter,
}

impl ThresholdFilter {
    /// Create a new threshold filter.
    pub fn new() -> Result<Self, OrbbecError> {
        // Unwrap is safe because Threshold filter is always available (public filter)
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/filter/publicfilters/publicFilterLoader.cpp#L32
        let filter = OBFilter::new(c"ThresholdFilter")?.unwrap();
        Ok(ThresholdFilter { inner: filter })
    }

    /// Set the minimum depth value (in millimeters).
    /// ### Arguments
    /// * `min_depth` - The minimum depth value to keep, must be between 0 and 16000.
    pub fn set_min_depth(&mut self, min_depth: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"min", min_depth as f64)
            .map_err(OrbbecError::from)
    }

    /// Set the maximum depth value (in millimeters).
    /// ### Arguments
    /// * `max_depth` - The maximum depth value to keep, must be between 0 and 16000.
    pub fn set_max_depth(&mut self, max_depth: u16) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"max", max_depth as f64)
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for ThresholdFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for ThresholdFilter {}

/// Align filter
///
/// This filter aligns a stream (usually depth) to another stream (usually color).
pub struct AlignFilter {
    inner: OBFilter,
}

impl AlignFilter {
    /// Create a new align filter.
    /// By default, it aligns to the color stream and resizes the aligned frame to match the target stream resolution.
    pub fn new() -> Result<Self, OrbbecError> {
        // Unwrap is safe because Align filter is always available (public filter)
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/filter/publicfilters/publicFilterLoader.cpp#L32
        let filter = OBFilter::new(c"Align")?.unwrap();
        Ok(AlignFilter { inner: filter })
    }

    /// Align to the specified stream type.
    /// ### Arguments
    /// * `stream_type` - The stream type to align to.
    pub fn set_align_to_stream_type(&mut self, stream_type: StreamType) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"AlignType", stream_type as u32 as f64)
            .map_err(OrbbecError::from)
    }

    /// Add target stream distortion to the aligned frame.
    /// ### Arguments
    /// * `enable` - Whether to enable distortion.
    pub fn set_add_distortion(&mut self, enable: bool) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"TargetDistortion", if enable { 1.0 } else { 0.0 })
            .map_err(OrbbecError::from)
    }

    /// Set gap fill mode.
    /// ### Arguments
    /// * `enable` - Whether to enable gap fill.
    pub fn set_gap_fill(&mut self, enable: bool) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"GapFillCopy", if enable { 1.0 } else { 0.0 })
            .map_err(OrbbecError::from)
    }

    /// Set match resolution mode. When enabled, the aligned frame will have the same resolution as the target stream.
    /// ### Arguments
    /// * `enable` - Whether to enable match resolution.
    pub fn set_match_resolution(&mut self, enable: bool) -> Result<(), OrbbecError> {
        self.inner
            .set_config_value(c"MatchTargetRes", if enable { 1.0 } else { 0.0 })
            .map_err(OrbbecError::from)
    }

    /// Align to the specified stream profile.
    /// It is useful when the align target stream has not started (without any frame to get intrinsics and extrinsics).
    /// ### Arguments
    /// * `profile` - The video stream profile to align to.
    pub fn set_align_to_stream_profile(
        &mut self,
        profile: &VideoStreamProfile,
    ) -> Result<(), OrbbecError> {
        self.inner
            .set_align_to(profile.inner())
            .map_err(OrbbecError::from)
    }
}

impl AsRef<OBFilter> for AlignFilter {
    fn as_ref(&self) -> &OBFilter {
        &self.inner
    }
}

impl<F: Frame> Filter<F> for AlignFilter {}
