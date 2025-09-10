//! Device properties
use crate::sys::enums::OBPropertyID;

/// Property value
#[derive(Debug, PartialEq)]
pub enum PropertyValue<'a> {
    Bool(&'a mut bool),
    Int(&'a mut i32),
    Float(&'a mut f32),
}

/// Device property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceProperty {
    /// LDP switch
    LDP(bool),
    /// Laser switch
    Laser(bool),
    /// Laser pulse width
    LaserPulseWidth(i32),
    /// Laser current (unit: mA)
    LaserCurrent(f32),
    /// IR flood switch
    Flood(bool),
    /// IR flood level
    FloodLevel(i32),
    /// Enable/disable temperature compensation
    TemperatureCompensation(bool),
    /// Depth mirror
    DepthMirror(bool),
    /// Depth flip
    DepthFlip(bool),
    /// Depth Postfilter
    DepthPostfilter(bool),
    /// Depth Holefilter
    DepthHolefilter(bool),
    /// IR mirror
    IRMirror(bool),
    /// IR flip
    IRFlip(bool),
    /// Minimum depth threshold
    MinDepth(i32),
    /// Maximum depth threshold
    MaxDepth(i32),
    /// Software filter switch
    DepthNoiseRemovalFilter(bool),
    /// LDP status
    LDPStatus(bool),
    /// Maxdiff for depth noise removal filter
    DepthNoiseRemovalFilterMaxDiff(i32),
    /// MaxSpeckleSize for depth noise removal filter
    DepthNoiseRemovalFilterMaxSpeckleSize(i32),
    /// Hardware d2c is on
    DepthAlignHardware(bool),
    /// Timestamp adjustment
    TimestampOffset(i32),
    /// Hardware distortion switch Rectify
    HardwareDistortionSwitch(bool),
    /// Fan mode switch
    FanWorkMode(i32),
    /// Multi-resolution D2C mode
    DepthAlignHardwareMode(i32),
    /// Anti-collusion activation status
    AntiCollusionActivationStatus(bool),
    /// Depth precision level
    DepthPrecisionLevel(i32),
    /// TOF filter range configuration
    TofFilterRange(i32),
    /// Laser mode
    LaserMode(i32),
    /// brt2r-rectify function switch
    Rectify2(bool),
    /// Color mirror
    ColorMirror(bool),
    /// Color flip
    ColorFlip(bool),
    /// Indicator switch
    IndicatorLight(bool),
    /// Disparity to depth switch
    DisparityToDepth(bool),
    /// BRT function switch
    BRT(bool),
    /// Watchdog function switch
    Watchdog(bool),
    /// External signal trigger restart function switch
    ExternalSignalReset(bool),
    /// Heartbeat monitoring function switch
    Heartbeat(bool),
    /// Depth cropping mode device
    DepthCroppingMode(i32),
    /// D2C preprocessing switch
    D2CPreprocess(bool),
    /// Enable/disable GPM function
    GPM(bool),
    /// Custom RGB cropping switch
    RGBCustomCrop(bool),
    /// Device operating mode (power consumption)
    DeviceWorkMode(i32),
    /// Device communication type
    DeviceCommunicationType(i32),
    /// Switch infrared imaging mode
    SwitchIRMode(i32),
    /// Laser power level
    LaserPowerLevelControl(i32),
    /// LDP's measure distance
    LDPMeasureDistance(i32),
    /// Reset device time to zero
    TimerResetSignal(bool),
    /// Enable send reset device time signal to other device
    TimerResetTriggerOutEnable(bool),
    /// Delay to reset device time
    TimerResetDelayUs(i32),
    /// Signal to capture image
    CaptureImageSignal(bool),
    /// Right IR sensor mirror state
    IRRightMirror(bool),
    /// Number frame to capture once a 'OB_PROP_CAPTURE_IMAGE_SIGNAL_BOOL' effect
    CaptureImageFrameNumber(i32),
    /// Right IR sensor flip state
    IRRightFlip(bool),
    /// Color sensor rotation
    ColorRotate(i32),
    /// IR/Left-IR sensor rotation
    IRRotate(i32),
    /// Right IR sensor rotation
    IRRightRotate(i32),
    /// Depth sensor rotation
    DepthRotate(i32),
    /// Get hardware laser power actual level
    LaserPowerActualLevel(i32),
    /// USB's power state
    USBPowerState(i32),
    /// DC's power state
    DCPowerState(i32),
    /// Device development mode switch
    DeviceDevelopmentMode(i32),
    /// Multi-DeviceSync synchronized signal trigger out is enable state
    SyncSignalTriggerOut(bool),
    /// Restore factory settings and factory parameters
    RestoreFactorySettings(bool),
    /// Enter recovery mode when boot the device
    BootIntoRecoveryMode(bool),
    /// Query whether the current device is running in recovery mode
    DeviceInRecoveryMode(bool),
    /// Capture interval mode
    CaptureIntervalMode(i32),
    /// Capture time interval
    CaptureImageTimeInterval(i32),
    /// Capture number interval
    CaptureImageNumberInterval(i32),
    /// Timer reset enable
    TimerResetEnable(bool),
    /// Enable or disable the device to retry USB2.0 re-identification
    DeviceUSB2RepeatIdentify(bool),
    /// Reboot device delay mode
    DeviceRebootDelay(i32),
    /// Query the status of laser overcurrent protection
    LaserOvercurrentProtectionStatus(bool),
    /// Query the status of laser pulse width protection
    LaserPulseWidthProtectionStatus(bool),
    /// Laser always on
    LaserAlwaysOn(bool),
    /// Laser on/off alternate mode
    LaserOnOffPattern(i32),
    /// Depth unit flexible adjustment
    DepthUnitFlexibleAdjustment(f32),
    /// Laser control
    LaserControl(i32),
    /// IR brightness
    IRBrightness(i32),
    /// Slave/secondary device synchronization status
    SlaveDeviceSyncStatus(bool),
    /// Color AE max exposure
    ColorAEMaxExposure(i32),
    /// Max exposure time of IR auto exposure
    IRAEMaxExposure(i32),
    /// Disparity search range mode
    DispSearchRangeMode(i32),
    /// Laser high temperature protection
    LaserHighTemperatureProtect(bool),
    /// Low exposure laser control
    LowExposureLaserControl(bool),
    /// Check pps sync in signal
    CheckPPSSyncInSignal(bool),
    /// Disparity search range offset
    DispSearchOffset(i32),
    /// Repower device
    DeviceRepower(bool),
    /// Frame interleave config index
    FrameInterleaveConfigIndex(i32),
    /// Frame interleave enable
    FrameInterleaveEnable(bool),
    /// Laser pattern sync with delay
    FrameInterleaveLaserPatternSyncDelay(i32),
    /// Get the health check result from device
    OnChipCalibrationHealthCheck(f32),
    /// Enable or disable on-chip calibration
    OnChipCalibrationEnable(bool),
    /// Hardware noise remove filter switch
    HWNoiseRemoveFilterEnable(bool),
    /// Hardware noise remove filter threshold
    HWNoiseRemoveFilterThreshold(f32),
    /// Soft trigger auto capture enable
    DeviceAutoCaptureEnable(bool),
    /// Soft trigger auto capture interval time
    DeviceAutoCaptureIntervalTime(i32),
    /// PTP time synchronization enable
    DevicePTPClockSyncEnable(bool),
    /// Depth with confidence stream enable
    DepthWithConfidenceStreamEnable(bool),
    /// Enable or disable confidence stream filter
    ConfidenceStreamFilter(bool),
    /// Confidence stream filter threshold
    ConfidenceStreamFilterThreshold(i32),
    /// Confidence stream mirror enable
    ConfidenceMirror(bool),
    /// Confidence stream flip enable
    ConfidenceFlip(bool),
    /// Confidence stream rotate
    ConfidenceRotate(i32),
    /// Color camera auto exposure
    ColorAutoExposure(bool),
    /// Color camera exposure adjustment
    ColorExposure(i32),
    /// Color camera gain adjustment
    ColorGain(i32),
    /// Color camera automatic white balance
    ColorAutoWhiteBalance(bool),
    /// Color camera white balance adjustment
    ColorWhiteBalance(i32),
    /// Color camera brightness adjustment
    ColorBrightness(i32),
    /// Color camera sharpness adjustment
    ColorSharpness(i32),
    /// Color camera shutter adjustment
    ColorShutter(i32),
    /// Color camera saturation adjustment
    ColorSaturation(i32),
    /// Color camera contrast adjustment
    ColorContrast(i32),
    /// Color camera gamma adjustment
    ColorGamma(i32),
    /// Color camera image rotation
    ColorRoll(i32),
    /// Color camera auto exposure priority
    ColorAutoExposurePriority(i32),
    /// Color camera brightness compensation
    ColorBacklightCompensation(i32),
    /// Color camera color tint
    ColorHue(i32),
    /// Color Camera Power Line Frequency
    ColorPowerLineFrequency(i32),
    /// Automatic exposure of depth camera
    DepthAutoExposure(bool),
    /// Depth camera exposure adjustment
    DepthExposure(i32),
    /// Depth camera gain adjustment
    DepthGain(i32),
    /// Infrared camera auto exposure
    IRAutoExposure(bool),
    /// Infrared camera exposure adjustment
    IRExposure(i32),
    /// Infrared camera gain adjustment
    IRGain(i32),
    /// Select Infrared camera data source channel
    IRChannelDataSource(i32),
    /// Depth effect dedistortion
    DepthRMFilter(bool),
    /// Color camera maximal gain
    ColorMaximalGain(i32),
    /// Color camera shutter gain
    ColorMaximalShutter(i32),
    /// Enable/disable IR short exposure function
    IRShortExposure(bool),
    /// Color camera HDR
    ColorHDR(bool),
    /// IR long exposure mode switch
    IRLongExposure(bool),
    /// Setting and getting the USB device frame skipping mode status
    SkipFrame(bool),
    /// Depth HDR merge
    HDRMerge(bool),
    /// Color camera FOCUS
    ColorFocus(i32),
    /// IR rectify status
    IRRectify(bool),
    /// Depth camera priority
    DepthAutoExposurePriority(i32),
    /// Software disparity to depth
    SDKDisparityToDepth(bool),
    /// Depth data unpacking function switch
    SDKDepthFrameUnpack(bool),
    /// IR data unpacking function switch
    SDKIRFrameUnpack(bool),
    /// Accel data conversion function switch
    SDKAccelFrameTransformed(bool),
    /// Gyro data conversion function switch
    SDKGyroFrameTransformed(bool),
    /// Left IR frame data unpacking function switch
    SDKIRLeftFrameUnpack(bool),
    /// Right IR frame data unpacking function switch
    SDKIRRightFrameUnpack(bool),
    /// Read the current network bandwidth type of the network device
    NetworkBandwidthType(i32),
    /// Switch device performance mode
    DevicePerformanceMode(i32),
    /// Calibration JSON file read from device
    RawDataCameraCalibJsonFile(i32),
    /// Confidence degree
    DebugESGMConfidence(f32),
}

impl<'a> DeviceProperty {
    /// Decompose to (property_id, property_value)
    pub(crate) fn decompose(&'a mut self) -> (OBPropertyID, PropertyValue<'a>) {
        match self {
            DeviceProperty::LDP(value) => (OBPropertyID::LDPBool, PropertyValue::Bool(value)),
            DeviceProperty::Laser(value) => (OBPropertyID::LaserBool, PropertyValue::Bool(value)),
            DeviceProperty::LaserPulseWidth(value) => {
                (OBPropertyID::LaserPulseWidthInt, PropertyValue::Int(value))
            }
            DeviceProperty::LaserCurrent(value) => {
                (OBPropertyID::LaserCurrentFloat, PropertyValue::Float(value))
            }
            DeviceProperty::Flood(value) => (OBPropertyID::FloodBool, PropertyValue::Bool(value)),
            DeviceProperty::FloodLevel(value) => {
                (OBPropertyID::FloodLevelInt, PropertyValue::Int(value))
            }
            DeviceProperty::TemperatureCompensation(value) => (
                OBPropertyID::TemperatureCompensationBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DepthMirror(value) => {
                (OBPropertyID::DepthMirrorBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DepthFlip(value) => {
                (OBPropertyID::DepthFlipBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DepthPostfilter(value) => (
                OBPropertyID::DepthPostfilterBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DepthHolefilter(value) => (
                OBPropertyID::DepthHolefilterBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::IRMirror(value) => {
                (OBPropertyID::IRMirrorBool, PropertyValue::Bool(value))
            }
            DeviceProperty::IRFlip(value) => (OBPropertyID::IRFlipBool, PropertyValue::Bool(value)),
            DeviceProperty::MinDepth(value) => {
                (OBPropertyID::MinDepthInt, PropertyValue::Int(value))
            }
            DeviceProperty::MaxDepth(value) => {
                (OBPropertyID::MaxDepthInt, PropertyValue::Int(value))
            }
            DeviceProperty::DepthNoiseRemovalFilter(value) => (
                OBPropertyID::DepthNoiseRemovalFilterBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::LDPStatus(value) => {
                (OBPropertyID::LDPStatusBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DepthNoiseRemovalFilterMaxDiff(value) => (
                OBPropertyID::DepthNoiseRemovalFilterMaxDiffInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DepthNoiseRemovalFilterMaxSpeckleSize(value) => (
                OBPropertyID::DepthNoiseRemovalFilterMaxSpeckleSizeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DepthAlignHardware(value) => (
                OBPropertyID::DepthAlignHardwareBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::TimestampOffset(value) => {
                (OBPropertyID::TimestampOffsetInt, PropertyValue::Int(value))
            }
            DeviceProperty::HardwareDistortionSwitch(value) => (
                OBPropertyID::HardwareDistortionSwitchBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::FanWorkMode(value) => {
                (OBPropertyID::FanWorkModeInt, PropertyValue::Int(value))
            }
            DeviceProperty::DepthAlignHardwareMode(value) => (
                OBPropertyID::DepthAlignHardwareModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::AntiCollusionActivationStatus(value) => (
                OBPropertyID::AntiCollusionActivationStatusBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DepthPrecisionLevel(value) => (
                OBPropertyID::DepthPrecisionLevelInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::TofFilterRange(value) => {
                (OBPropertyID::TofFilterRangeInt, PropertyValue::Int(value))
            }
            DeviceProperty::LaserMode(value) => {
                (OBPropertyID::LaserModeInt, PropertyValue::Int(value))
            }
            DeviceProperty::Rectify2(value) => {
                (OBPropertyID::Rectify2Bool, PropertyValue::Bool(value))
            }
            DeviceProperty::ColorMirror(value) => {
                (OBPropertyID::ColorMirrorBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ColorFlip(value) => {
                (OBPropertyID::ColorFlipBool, PropertyValue::Bool(value))
            }
            DeviceProperty::IndicatorLight(value) => {
                (OBPropertyID::IndicatorLightBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DisparityToDepth(value) => (
                OBPropertyID::DisparityToDepthBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::BRT(value) => (OBPropertyID::BRTBool, PropertyValue::Bool(value)),
            DeviceProperty::Watchdog(value) => {
                (OBPropertyID::WatchdogBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ExternalSignalReset(value) => (
                OBPropertyID::ExternalSignalResetBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::Heartbeat(value) => {
                (OBPropertyID::HeartbeatBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DepthCroppingMode(value) => (
                OBPropertyID::DepthCroppingModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::D2CPreprocess(value) => {
                (OBPropertyID::D2CPreprocessBool, PropertyValue::Bool(value))
            }
            DeviceProperty::GPM(value) => (OBPropertyID::GPMBool, PropertyValue::Bool(value)),
            DeviceProperty::RGBCustomCrop(value) => {
                (OBPropertyID::RGBCustomCropBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DeviceWorkMode(value) => {
                (OBPropertyID::DeviceWorkModeInt, PropertyValue::Int(value))
            }
            DeviceProperty::DeviceCommunicationType(value) => (
                OBPropertyID::DeviceCommunicationTypeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::SwitchIRMode(value) => {
                (OBPropertyID::SwitchIRModeInt, PropertyValue::Int(value))
            }
            DeviceProperty::LaserPowerLevelControl(value) => (
                OBPropertyID::LaserPowerLevelControlInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::LDPMeasureDistance(value) => (
                OBPropertyID::LDPMeasureDistanceInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::TimerResetSignal(value) => (
                OBPropertyID::TimerResetSignalBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::TimerResetTriggerOutEnable(value) => (
                OBPropertyID::TimerResetTriggerOutEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::TimerResetDelayUs(value) => (
                OBPropertyID::TimerResetDelayUsInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::CaptureImageSignal(value) => (
                OBPropertyID::CaptureImageSignalBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::IRRightMirror(value) => {
                (OBPropertyID::IRRightMirrorBool, PropertyValue::Bool(value))
            }
            DeviceProperty::CaptureImageFrameNumber(value) => (
                OBPropertyID::CaptureImageFrameNumberInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::IRRightFlip(value) => {
                (OBPropertyID::IRRightFlipBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ColorRotate(value) => {
                (OBPropertyID::ColorRotateInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRRotate(value) => {
                (OBPropertyID::IRRotateInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRRightRotate(value) => {
                (OBPropertyID::IRRightRotateInt, PropertyValue::Int(value))
            }
            DeviceProperty::DepthRotate(value) => {
                (OBPropertyID::DepthRotateInt, PropertyValue::Int(value))
            }
            DeviceProperty::LaserPowerActualLevel(value) => (
                OBPropertyID::LaserPowerActualLevelInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::USBPowerState(value) => {
                (OBPropertyID::USBPowerStateInt, PropertyValue::Int(value))
            }
            DeviceProperty::DCPowerState(value) => {
                (OBPropertyID::DCPowerStateInt, PropertyValue::Int(value))
            }
            DeviceProperty::DeviceDevelopmentMode(value) => (
                OBPropertyID::DeviceDevelopmentModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::SyncSignalTriggerOut(value) => (
                OBPropertyID::SyncSignalTriggerOutBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::RestoreFactorySettings(value) => (
                OBPropertyID::RestoreFactorySettingsBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::BootIntoRecoveryMode(value) => (
                OBPropertyID::BootIntoRecoveryModeBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DeviceInRecoveryMode(value) => (
                OBPropertyID::DeviceInRecoveryModeBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::CaptureIntervalMode(value) => (
                OBPropertyID::CaptureIntervalModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::CaptureImageTimeInterval(value) => (
                OBPropertyID::CaptureImageTimeIntervalInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::CaptureImageNumberInterval(value) => (
                OBPropertyID::CaptureImageNumberIntervalInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::TimerResetEnable(value) => (
                OBPropertyID::TimerResetEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DeviceUSB2RepeatIdentify(value) => (
                OBPropertyID::DeviceUSB2RepeatIdentifyBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DeviceRebootDelay(value) => (
                OBPropertyID::DeviceRebootDelayInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::LaserOvercurrentProtectionStatus(value) => (
                OBPropertyID::LaserOvercurrentProtectionStatusBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::LaserPulseWidthProtectionStatus(value) => (
                OBPropertyID::LaserPulseWidthProtectionStatusBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::LaserAlwaysOn(value) => {
                (OBPropertyID::LaserAlwaysOnBool, PropertyValue::Bool(value))
            }
            DeviceProperty::LaserOnOffPattern(value) => (
                OBPropertyID::LaserOnOffPatternInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DepthUnitFlexibleAdjustment(value) => (
                OBPropertyID::DepthUnitFlexibleAdjustmentFloat,
                PropertyValue::Float(value),
            ),
            DeviceProperty::LaserControl(value) => {
                (OBPropertyID::LaserControlInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRBrightness(value) => {
                (OBPropertyID::IRBrightnessInt, PropertyValue::Int(value))
            }
            DeviceProperty::SlaveDeviceSyncStatus(value) => (
                OBPropertyID::SlaveDeviceSyncStatusBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ColorAEMaxExposure(value) => (
                OBPropertyID::ColorAEMaxExposureInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::IRAEMaxExposure(value) => {
                (OBPropertyID::IRAEMaxExposureInt, PropertyValue::Int(value))
            }
            DeviceProperty::DispSearchRangeMode(value) => (
                OBPropertyID::DispSearchRangeModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::LaserHighTemperatureProtect(value) => (
                OBPropertyID::LaserHighTemperatureProtectBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::LowExposureLaserControl(value) => (
                OBPropertyID::LowExposureLaserControlBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::CheckPPSSyncInSignal(value) => (
                OBPropertyID::CheckPPSSyncInSignalBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DispSearchOffset(value) => {
                (OBPropertyID::DispSearchOffsetInt, PropertyValue::Int(value))
            }
            DeviceProperty::DeviceRepower(value) => {
                (OBPropertyID::DeviceRepowerBool, PropertyValue::Bool(value))
            }
            DeviceProperty::FrameInterleaveConfigIndex(value) => (
                OBPropertyID::FrameInterleaveConfigIndexInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::FrameInterleaveEnable(value) => (
                OBPropertyID::FrameInterleaveEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::FrameInterleaveLaserPatternSyncDelay(value) => (
                OBPropertyID::FrameInterleaveLaserPatternSyncDelayInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::OnChipCalibrationHealthCheck(value) => (
                OBPropertyID::OnChipCalibrationHealthCheckFloat,
                PropertyValue::Float(value),
            ),
            DeviceProperty::OnChipCalibrationEnable(value) => (
                OBPropertyID::OnChipCalibrationEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::HWNoiseRemoveFilterEnable(value) => (
                OBPropertyID::HWNoiseRemoveFilterEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::HWNoiseRemoveFilterThreshold(value) => (
                OBPropertyID::HWNoiseRemoveFilterThresholdFloat,
                PropertyValue::Float(value),
            ),
            DeviceProperty::DeviceAutoCaptureEnable(value) => (
                OBPropertyID::DeviceAutoCaptureEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DeviceAutoCaptureIntervalTime(value) => (
                OBPropertyID::DeviceAutoCaptureIntervalTimeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DevicePTPClockSyncEnable(value) => (
                OBPropertyID::DevicePTPClockSyncEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DepthWithConfidenceStreamEnable(value) => (
                OBPropertyID::DepthWithConfidenceStreamEnableBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ConfidenceStreamFilter(value) => (
                OBPropertyID::ConfidenceStreamFilterBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ConfidenceStreamFilterThreshold(value) => (
                OBPropertyID::ConfidenceStreamFilterThresholdInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::ConfidenceMirror(value) => (
                OBPropertyID::ConfidenceMirrorBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ConfidenceFlip(value) => {
                (OBPropertyID::ConfidenceFlipBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ConfidenceRotate(value) => {
                (OBPropertyID::ConfidenceRotateInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorAutoExposure(value) => (
                OBPropertyID::ColorAutoExposureBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ColorExposure(value) => {
                (OBPropertyID::ColorExposureInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorGain(value) => {
                (OBPropertyID::ColorGainInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorAutoWhiteBalance(value) => (
                OBPropertyID::ColorAutoWhiteBalanceBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ColorWhiteBalance(value) => (
                OBPropertyID::ColorWhiteBalanceInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::ColorBrightness(value) => {
                (OBPropertyID::ColorBrightnessInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorSharpness(value) => {
                (OBPropertyID::ColorSharpnessInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorShutter(value) => {
                (OBPropertyID::ColorShutterInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorSaturation(value) => {
                (OBPropertyID::ColorSaturationInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorContrast(value) => {
                (OBPropertyID::ColorContrastInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorGamma(value) => {
                (OBPropertyID::ColorGammaInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorRoll(value) => {
                (OBPropertyID::ColorRollInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorAutoExposurePriority(value) => (
                OBPropertyID::ColorAutoExposurePriorityInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::ColorBacklightCompensation(value) => (
                OBPropertyID::ColorBacklightCompensationInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::ColorHue(value) => {
                (OBPropertyID::ColorHueInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorPowerLineFrequency(value) => (
                OBPropertyID::ColorPowerLineFrequencyInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DepthAutoExposure(value) => (
                OBPropertyID::DepthAutoExposureBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::DepthExposure(value) => {
                (OBPropertyID::DepthExposureInt, PropertyValue::Int(value))
            }
            DeviceProperty::DepthGain(value) => {
                (OBPropertyID::DepthGainInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRAutoExposure(value) => {
                (OBPropertyID::IRAutoExposureBool, PropertyValue::Bool(value))
            }
            DeviceProperty::IRExposure(value) => {
                (OBPropertyID::IRExposureInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRGain(value) => (OBPropertyID::IRGainInt, PropertyValue::Int(value)),
            DeviceProperty::IRChannelDataSource(value) => (
                OBPropertyID::IRChannelDataSourceInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DepthRMFilter(value) => {
                (OBPropertyID::DepthRMFilterBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ColorMaximalGain(value) => {
                (OBPropertyID::ColorMaximalGainInt, PropertyValue::Int(value))
            }
            DeviceProperty::ColorMaximalShutter(value) => (
                OBPropertyID::ColorMaximalShutterInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::IRShortExposure(value) => (
                OBPropertyID::IRShortExposureBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::ColorHDR(value) => {
                (OBPropertyID::ColorHDRBool, PropertyValue::Bool(value))
            }
            DeviceProperty::IRLongExposure(value) => {
                (OBPropertyID::IRLongExposureBool, PropertyValue::Bool(value))
            }
            DeviceProperty::SkipFrame(value) => {
                (OBPropertyID::SkipFrameBool, PropertyValue::Bool(value))
            }
            DeviceProperty::HDRMerge(value) => {
                (OBPropertyID::HDRMergeBool, PropertyValue::Bool(value))
            }
            DeviceProperty::ColorFocus(value) => {
                (OBPropertyID::ColorFocusInt, PropertyValue::Int(value))
            }
            DeviceProperty::IRRectify(value) => {
                (OBPropertyID::IRRectifyBool, PropertyValue::Bool(value))
            }
            DeviceProperty::DepthAutoExposurePriority(value) => (
                OBPropertyID::DepthAutoExposurePriorityInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::SDKDisparityToDepth(value) => (
                OBPropertyID::SDKDisparityToDepthBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKDepthFrameUnpack(value) => (
                OBPropertyID::SDKDepthFrameUnpackBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKIRFrameUnpack(value) => (
                OBPropertyID::SDKIRFrameUnpackBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKAccelFrameTransformed(value) => (
                OBPropertyID::SDKAccelFrameTransformedBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKGyroFrameTransformed(value) => (
                OBPropertyID::SDKGyroFrameTransformedBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKIRLeftFrameUnpack(value) => (
                OBPropertyID::SDKIRLeftFrameUnpackBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::SDKIRRightFrameUnpack(value) => (
                OBPropertyID::SDKIRRightFrameUnpackBool,
                PropertyValue::Bool(value),
            ),
            DeviceProperty::NetworkBandwidthType(value) => (
                OBPropertyID::NetworkBandwidthTypeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DevicePerformanceMode(value) => (
                OBPropertyID::DevicePerformanceModeInt,
                PropertyValue::Int(value),
            ),
            DeviceProperty::RawDataCameraCalibJsonFile(value) => (
                OBPropertyID::RawDataCameraCalibJsonFile,
                PropertyValue::Int(value),
            ),
            DeviceProperty::DebugESGMConfidence(value) => (
                OBPropertyID::DebugESGMConfidenceFloat,
                PropertyValue::Float(value),
            ),
        }
    }
}
