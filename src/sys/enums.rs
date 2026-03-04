//! This module defines various enums used in the Orbbec SDK.
use super::orb;

/// Sensor Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBSensorType {
    /// Unknown Sensor
    Unknown = orb::OBSensorType_OB_SENSOR_UNKNOWN as isize,
    /// Infrared Sensor
    Infrared = orb::OBSensorType_OB_SENSOR_IR as isize,
    /// Depth Sensor
    Depth = orb::OBSensorType_OB_SENSOR_DEPTH as isize,
    /// Color Sensor
    Color = orb::OBSensorType_OB_SENSOR_COLOR as isize,
    /// Accelerometer Sensor
    Accelerometer = orb::OBSensorType_OB_SENSOR_ACCEL as isize,
    /// Gyroscope Sensor
    Gyroscope = orb::OBSensorType_OB_SENSOR_GYRO as isize,
    /// Left IR Sensor
    LeftInfrared = orb::OBSensorType_OB_SENSOR_IR_LEFT as isize,
    /// Right IR Sensor
    RightInfrared = orb::OBSensorType_OB_SENSOR_IR_RIGHT as isize,
    /// Confidence Sensor
    Confidence = orb::OBSensorType_OB_SENSOR_CONFIDENCE as isize,
}

/// Pixel Format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBFormat {
    /// Unknown format
    Unknown = orb::OBFormat_OB_FORMAT_UNKNOWN as isize,
    /// YUYV format
    YUYV = orb::OBFormat_OB_FORMAT_YUYV as isize,
    /// YUY2 format (the actual format is the same as YUYV)
    YUY2 = orb::OBFormat_OB_FORMAT_YUY2 as isize,
    /// UYVY format
    UYVY = orb::OBFormat_OB_FORMAT_UYVY as isize,
    /// NV12 format
    NV12 = orb::OBFormat_OB_FORMAT_NV12 as isize,
    /// NV21 format
    NV21 = orb::OBFormat_OB_FORMAT_NV21 as isize,
    /// MJPEG encoding format
    MJPG = orb::OBFormat_OB_FORMAT_MJPG as isize,
    /// H.264 encoding format
    H264 = orb::OBFormat_OB_FORMAT_H264 as isize,
    /// H.265 encoding format
    H265 = orb::OBFormat_OB_FORMAT_H265 as isize,
    /// Y16 format, 16-bit per pixel, single-channel
    Y16 = orb::OBFormat_OB_FORMAT_Y16 as isize,
    /// Y8 format, 8-bit per pixel, single-channel
    Y8 = orb::OBFormat_OB_FORMAT_Y8 as isize,
    /// Y10 format, 10-bit per pixel, single-channel (SDK will unpack into Y16 by default)
    Y10 = orb::OBFormat_OB_FORMAT_Y10 as isize,
    /// Y11 format, 11-bit per pixel, single-channel (SDK will unpack into Y16 by default)
    Y11 = orb::OBFormat_OB_FORMAT_Y11 as isize,
    /// Y12 format, 12-bit per pixel, single-channel (SDK will unpack into Y16 by default)
    Y12 = orb::OBFormat_OB_FORMAT_Y12 as isize,
    /// GRAY (the actual format is the same as YUYV)
    Gray = orb::OBFormat_OB_FORMAT_GRAY as isize,
    /// HEVC encoding format (the actual format is the same as H265)
    HEVC = orb::OBFormat_OB_FORMAT_HEVC as isize,
    /// I420 format
    I420 = orb::OBFormat_OB_FORMAT_I420 as isize,
    /// Acceleration data format
    Accel = orb::OBFormat_OB_FORMAT_ACCEL as isize,
    /// Gyroscope data format
    Gyro = orb::OBFormat_OB_FORMAT_GYRO as isize,
    /// XYZ 3D coordinate point format, @ref OBPoint
    Point = orb::OBFormat_OB_FORMAT_POINT as isize,
    /// XYZ 3D coordinate point format with RGB information, @ref OBColorPoint
    RGBPoint = orb::OBFormat_OB_FORMAT_RGB_POINT as isize,
    /// RLE pressure test format (SDK will be unpacked into Y16 by default)
    RLE = orb::OBFormat_OB_FORMAT_RLE as isize,
    /// RGB format (actual RGB888)
    RGB = orb::OBFormat_OB_FORMAT_RGB as isize,
    /// BGR format (actual BGR888)
    BGR = orb::OBFormat_OB_FORMAT_BGR as isize,
    /// Y14 format, 14-bit per pixel, single-channel (SDK will unpack into Y16 by default)
    Y14 = orb::OBFormat_OB_FORMAT_Y14 as isize,
    /// BGRA format
    BGRA = orb::OBFormat_OB_FORMAT_BGRA as isize,
    /// Compression format
    Compressed = orb::OBFormat_OB_FORMAT_COMPRESSED as isize,
    /// RVL pressure test format (SDK will be unpacked into Y16 by default)
    RVL = orb::OBFormat_OB_FORMAT_RVL as isize,
    /// Is same as Y16
    Z16 = orb::OBFormat_OB_FORMAT_Z16 as isize,
    /// Is same as Y12, using for right ir stream
    YV12 = orb::OBFormat_OB_FORMAT_YV12 as isize,
    /// Is same as Y8, using for right ir stream
    BA81 = orb::OBFormat_OB_FORMAT_BA81 as isize,
    /// RGBA format
    RGBA = orb::OBFormat_OB_FORMAT_RGBA as isize,
    /// byr2 format
    BYR2 = orb::OBFormat_OB_FORMAT_BYR2 as isize,
    /// RAW16 format
    RW16 = orb::OBFormat_OB_FORMAT_RW16 as isize,
    /// Y12C4 format
    Y12C4 = orb::OBFormat_OB_FORMAT_Y12C4 as isize,
}

impl From<orb::OBFormat> for OBFormat {
    fn from(format: orb::OBFormat) -> Self {
        match format {
            orb::OBFormat_OB_FORMAT_UNKNOWN => OBFormat::Unknown,
            orb::OBFormat_OB_FORMAT_YUYV => OBFormat::YUYV,
            orb::OBFormat_OB_FORMAT_YUY2 => OBFormat::YUY2,
            orb::OBFormat_OB_FORMAT_UYVY => OBFormat::UYVY,
            orb::OBFormat_OB_FORMAT_NV12 => OBFormat::NV12,
            orb::OBFormat_OB_FORMAT_NV21 => OBFormat::NV21,
            orb::OBFormat_OB_FORMAT_MJPG => OBFormat::MJPG,
            orb::OBFormat_OB_FORMAT_H264 => OBFormat::H264,
            orb::OBFormat_OB_FORMAT_H265 => OBFormat::H265,
            orb::OBFormat_OB_FORMAT_Y16 => OBFormat::Y16,
            orb::OBFormat_OB_FORMAT_Y8 => OBFormat::Y8,
            orb::OBFormat_OB_FORMAT_Y10 => OBFormat::Y10,
            orb::OBFormat_OB_FORMAT_Y11 => OBFormat::Y11,
            orb::OBFormat_OB_FORMAT_Y12 => OBFormat::Y12,
            orb::OBFormat_OB_FORMAT_GRAY => OBFormat::Gray,
            orb::OBFormat_OB_FORMAT_HEVC => OBFormat::HEVC,
            orb::OBFormat_OB_FORMAT_I420 => OBFormat::I420,
            orb::OBFormat_OB_FORMAT_ACCEL => OBFormat::Accel,
            orb::OBFormat_OB_FORMAT_GYRO => OBFormat::Gyro,
            orb::OBFormat_OB_FORMAT_POINT => OBFormat::Point,
            orb::OBFormat_OB_FORMAT_RGB_POINT => OBFormat::RGBPoint,
            orb::OBFormat_OB_FORMAT_RLE => OBFormat::RLE,
            orb::OBFormat_OB_FORMAT_RGB => OBFormat::RGB,
            orb::OBFormat_OB_FORMAT_BGR => OBFormat::BGR,
            orb::OBFormat_OB_FORMAT_Y14 => OBFormat::Y14,
            orb::OBFormat_OB_FORMAT_BGRA => OBFormat::BGRA,
            orb::OBFormat_OB_FORMAT_COMPRESSED => OBFormat::Compressed,
            orb::OBFormat_OB_FORMAT_RVL => OBFormat::RVL,
            orb::OBFormat_OB_FORMAT_Z16 => OBFormat::Z16,
            orb::OBFormat_OB_FORMAT_YV12 => OBFormat::YV12,
            orb::OBFormat_OB_FORMAT_BA81 => OBFormat::BA81,
            orb::OBFormat_OB_FORMAT_RGBA => OBFormat::RGBA,
            orb::OBFormat_OB_FORMAT_BYR2 => OBFormat::BYR2,
            orb::OBFormat_OB_FORMAT_RW16 => OBFormat::RW16,
            orb::OBFormat_OB_FORMAT_Y12C4 => OBFormat::Y12C4,
            _ => OBFormat::Unknown,
        }
    }
}

/// Property ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBPropertyID {
    /// LDP switch
    LDPBool = orb::OBPropertyID_OB_PROP_LDP_BOOL as isize,
    /// Laser switch
    LaserBool = orb::OBPropertyID_OB_PROP_LASER_BOOL as isize,
    /// Laser pulse width
    LaserPulseWidthInt = orb::OBPropertyID_OB_PROP_LASER_PULSE_WIDTH_INT as isize,
    /// Laser current (unit: mA)
    LaserCurrentFloat = orb::OBPropertyID_OB_PROP_LASER_CURRENT_FLOAT as isize,
    /// IR flood switch
    FloodBool = orb::OBPropertyID_OB_PROP_FLOOD_BOOL as isize,
    /// IR flood level
    FloodLevelInt = orb::OBPropertyID_OB_PROP_FLOOD_LEVEL_INT as isize,
    /// Enable/disable temperature compensation
    TemperatureCompensationBool = orb::OBPropertyID_OB_PROP_TEMPERATURE_COMPENSATION_BOOL as isize,
    /// Depth mirror
    DepthMirrorBool = orb::OBPropertyID_OB_PROP_DEPTH_MIRROR_BOOL as isize,
    /// Depth flip
    DepthFlipBool = orb::OBPropertyID_OB_PROP_DEPTH_FLIP_BOOL as isize,
    /// Depth Postfilter
    DepthPostfilterBool = orb::OBPropertyID_OB_PROP_DEPTH_POSTFILTER_BOOL as isize,
    /// Depth Holefilter
    DepthHolefilterBool = orb::OBPropertyID_OB_PROP_DEPTH_HOLEFILTER_BOOL as isize,
    /// IR mirror
    IRMirrorBool = orb::OBPropertyID_OB_PROP_IR_MIRROR_BOOL as isize,
    /// IR flip
    IRFlipBool = orb::OBPropertyID_OB_PROP_IR_FLIP_BOOL as isize,
    /// Minimum depth threshold
    MinDepthInt = orb::OBPropertyID_OB_PROP_MIN_DEPTH_INT as isize,
    /// Maximum depth threshold
    MaxDepthInt = orb::OBPropertyID_OB_PROP_MAX_DEPTH_INT as isize,
    /// Software filter switch
    DepthNoiseRemovalFilterBool =
        orb::OBPropertyID_OB_PROP_DEPTH_NOISE_REMOVAL_FILTER_BOOL as isize,
    /// LDP status
    LDPStatusBool = orb::OBPropertyID_OB_PROP_LDP_STATUS_BOOL as isize,
    /// Maxdiff for depth noise removal filter
    DepthNoiseRemovalFilterMaxDiffInt =
        orb::OBPropertyID_OB_PROP_DEPTH_NOISE_REMOVAL_FILTER_MAX_DIFF_INT as isize,
    /// MaxSpeckleSize for depth noise removal filter
    DepthNoiseRemovalFilterMaxSpeckleSizeInt =
        orb::OBPropertyID_OB_PROP_DEPTH_NOISE_REMOVAL_FILTER_MAX_SPECKLE_SIZE_INT as isize,
    /// Hardware d2c is on
    DepthAlignHardwareBool = orb::OBPropertyID_OB_PROP_DEPTH_ALIGN_HARDWARE_BOOL as isize,
    /// Timestamp adjustment
    TimestampOffsetInt = orb::OBPropertyID_OB_PROP_TIMESTAMP_OFFSET_INT as isize,
    /// Hardware distortion switch Rectify
    HardwareDistortionSwitchBool =
        orb::OBPropertyID_OB_PROP_HARDWARE_DISTORTION_SWITCH_BOOL as isize,
    /// Fan mode switch
    FanWorkModeInt = orb::OBPropertyID_OB_PROP_FAN_WORK_MODE_INT as isize,
    /// Multi-resolution D2C mode
    DepthAlignHardwareModeInt = orb::OBPropertyID_OB_PROP_DEPTH_ALIGN_HARDWARE_MODE_INT as isize,
    /// Anti-collusion activation status
    AntiCollusionActivationStatusBool =
        orb::OBPropertyID_OB_PROP_ANTI_COLLUSION_ACTIVATION_STATUS_BOOL as isize,
    /// Depth precision level
    DepthPrecisionLevelInt = orb::OBPropertyID_OB_PROP_DEPTH_PRECISION_LEVEL_INT as isize,
    /// TOF filter range configuration
    TofFilterRangeInt = orb::OBPropertyID_OB_PROP_TOF_FILTER_RANGE_INT as isize,
    /// Laser mode
    LaserModeInt = orb::OBPropertyID_OB_PROP_LASER_MODE_INT as isize,
    /// brt2r-rectify function switch
    Rectify2Bool = orb::OBPropertyID_OB_PROP_RECTIFY2_BOOL as isize,
    /// Color mirror
    ColorMirrorBool = orb::OBPropertyID_OB_PROP_COLOR_MIRROR_BOOL as isize,
    /// Color flip
    ColorFlipBool = orb::OBPropertyID_OB_PROP_COLOR_FLIP_BOOL as isize,
    /// Indicator switch
    IndicatorLightBool = orb::OBPropertyID_OB_PROP_INDICATOR_LIGHT_BOOL as isize,
    /// Disparity to depth switch
    DisparityToDepthBool = orb::OBPropertyID_OB_PROP_DISPARITY_TO_DEPTH_BOOL as isize,
    /// BRT function switch
    BRTBool = orb::OBPropertyID_OB_PROP_BRT_BOOL as isize,
    /// Watchdog function switch
    WatchdogBool = orb::OBPropertyID_OB_PROP_WATCHDOG_BOOL as isize,
    /// External signal trigger restart function switch
    ExternalSignalResetBool = orb::OBPropertyID_OB_PROP_EXTERNAL_SIGNAL_RESET_BOOL as isize,
    /// Heartbeat monitoring function switch
    HeartbeatBool = orb::OBPropertyID_OB_PROP_HEARTBEAT_BOOL as isize,
    /// Depth cropping mode device
    DepthCroppingModeInt = orb::OBPropertyID_OB_PROP_DEPTH_CROPPING_MODE_INT as isize,
    /// D2C preprocessing switch
    D2CPreprocessBool = orb::OBPropertyID_OB_PROP_D2C_PREPROCESS_BOOL as isize,
    /// Enable/disable GPM function
    GPMBool = orb::OBPropertyID_OB_PROP_GPM_BOOL as isize,
    /// Custom RGB cropping switch
    RGBCustomCropBool = orb::OBPropertyID_OB_PROP_RGB_CUSTOM_CROP_BOOL as isize,
    /// Device operating mode (power consumption)
    DeviceWorkModeInt = orb::OBPropertyID_OB_PROP_DEVICE_WORK_MODE_INT as isize,
    /// Device communication type
    DeviceCommunicationTypeInt = orb::OBPropertyID_OB_PROP_DEVICE_COMMUNICATION_TYPE_INT as isize,
    /// Switch infrared imaging mode
    SwitchIRModeInt = orb::OBPropertyID_OB_PROP_SWITCH_IR_MODE_INT as isize,
    /// Laser power level
    LaserPowerLevelControlInt = orb::OBPropertyID_OB_PROP_LASER_POWER_LEVEL_CONTROL_INT as isize,
    /// LDP's measure distance
    LDPMeasureDistanceInt = orb::OBPropertyID_OB_PROP_LDP_MEASURE_DISTANCE_INT as isize,
    /// Reset device time to zero
    TimerResetSignalBool = orb::OBPropertyID_OB_PROP_TIMER_RESET_SIGNAL_BOOL as isize,
    /// Enable send reset device time signal to other device
    TimerResetTriggerOutEnableBool =
        orb::OBPropertyID_OB_PROP_TIMER_RESET_TRIGGER_OUT_ENABLE_BOOL as isize,
    /// Delay to reset device time
    TimerResetDelayUsInt = orb::OBPropertyID_OB_PROP_TIMER_RESET_DELAY_US_INT as isize,
    /// Signal to capture image
    CaptureImageSignalBool = orb::OBPropertyID_OB_PROP_CAPTURE_IMAGE_SIGNAL_BOOL as isize,
    /// Right IR sensor mirror state
    IRRightMirrorBool = orb::OBPropertyID_OB_PROP_IR_RIGHT_MIRROR_BOOL as isize,
    /// Number frame to capture once a 'OB_PROP_CAPTURE_IMAGE_SIGNAL_BOOL' effect
    CaptureImageFrameNumberInt = orb::OBPropertyID_OB_PROP_CAPTURE_IMAGE_FRAME_NUMBER_INT as isize,
    /// Right IR sensor flip state
    IRRightFlipBool = orb::OBPropertyID_OB_PROP_IR_RIGHT_FLIP_BOOL as isize,
    /// Color sensor rotation
    ColorRotateInt = orb::OBPropertyID_OB_PROP_COLOR_ROTATE_INT as isize,
    /// IR/Left-IR sensor rotation
    IRRotateInt = orb::OBPropertyID_OB_PROP_IR_ROTATE_INT as isize,
    /// Right IR sensor rotation
    IRRightRotateInt = orb::OBPropertyID_OB_PROP_IR_RIGHT_ROTATE_INT as isize,
    /// Depth sensor rotation
    DepthRotateInt = orb::OBPropertyID_OB_PROP_DEPTH_ROTATE_INT as isize,
    /// Get hardware laser power actual level
    LaserPowerActualLevelInt = orb::OBPropertyID_OB_PROP_LASER_POWER_ACTUAL_LEVEL_INT as isize,
    /// USB's power state
    USBPowerStateInt = orb::OBPropertyID_OB_PROP_USB_POWER_STATE_INT as isize,
    /// DC's power state
    DCPowerStateInt = orb::OBPropertyID_OB_PROP_DC_POWER_STATE_INT as isize,
    /// Device development mode switch
    DeviceDevelopmentModeInt = orb::OBPropertyID_OB_PROP_DEVICE_DEVELOPMENT_MODE_INT as isize,
    /// Multi-DeviceSync synchronized signal trigger out is enable state
    SyncSignalTriggerOutBool = orb::OBPropertyID_OB_PROP_SYNC_SIGNAL_TRIGGER_OUT_BOOL as isize,
    /// Restore factory settings and factory parameters
    RestoreFactorySettingsBool = orb::OBPropertyID_OB_PROP_RESTORE_FACTORY_SETTINGS_BOOL as isize,
    /// Enter recovery mode when boot the device
    BootIntoRecoveryModeBool = orb::OBPropertyID_OB_PROP_BOOT_INTO_RECOVERY_MODE_BOOL as isize,
    /// Query whether the current device is running in recovery mode
    DeviceInRecoveryModeBool = orb::OBPropertyID_OB_PROP_DEVICE_IN_RECOVERY_MODE_BOOL as isize,
    /// Capture interval mode
    CaptureIntervalModeInt = orb::OBPropertyID_OB_PROP_CAPTURE_INTERVAL_MODE_INT as isize,
    /// Capture time interval
    CaptureImageTimeIntervalInt =
        orb::OBPropertyID_OB_PROP_CAPTURE_IMAGE_TIME_INTERVAL_INT as isize,
    /// Capture number interval
    CaptureImageNumberIntervalInt =
        orb::OBPropertyID_OB_PROP_CAPTURE_IMAGE_NUMBER_INTERVAL_INT as isize,
    /// Timer reset enable
    TimerResetEnableBool = orb::OBPropertyID_OB_PROP_TIMER_RESET_ENABLE_BOOL as isize,
    /// Enable or disable the device to retry USB2.0 re-identification
    DeviceUSB2RepeatIdentifyBool =
        orb::OBPropertyID_OB_PROP_DEVICE_USB2_REPEAT_IDENTIFY_BOOL as isize,
    /// Reboot device delay mode
    DeviceRebootDelayInt = orb::OBPropertyID_OB_PROP_DEVICE_REBOOT_DELAY_INT as isize,
    /// Query the status of laser overcurrent protection
    LaserOvercurrentProtectionStatusBool =
        orb::OBPropertyID_OB_PROP_LASER_OVERCURRENT_PROTECTION_STATUS_BOOL as isize,
    /// Query the status of laser pulse width protection
    LaserPulseWidthProtectionStatusBool =
        orb::OBPropertyID_OB_PROP_LASER_PULSE_WIDTH_PROTECTION_STATUS_BOOL as isize,
    /// Laser always on
    LaserAlwaysOnBool = orb::OBPropertyID_OB_PROP_LASER_ALWAYS_ON_BOOL as isize,
    /// Laser on/off alternate mode
    LaserOnOffPatternInt = orb::OBPropertyID_OB_PROP_LASER_ON_OFF_PATTERN_INT as isize,
    /// Depth unit flexible adjustment
    DepthUnitFlexibleAdjustmentFloat =
        orb::OBPropertyID_OB_PROP_DEPTH_UNIT_FLEXIBLE_ADJUSTMENT_FLOAT as isize,
    /// Laser control
    LaserControlInt = orb::OBPropertyID_OB_PROP_LASER_CONTROL_INT as isize,
    /// IR brightness
    IRBrightnessInt = orb::OBPropertyID_OB_PROP_IR_BRIGHTNESS_INT as isize,
    /// Slave/secondary device synchronization status
    SlaveDeviceSyncStatusBool = orb::OBPropertyID_OB_PROP_SLAVE_DEVICE_SYNC_STATUS_BOOL as isize,
    /// Color AE max exposure
    ColorAEMaxExposureInt = orb::OBPropertyID_OB_PROP_COLOR_AE_MAX_EXPOSURE_INT as isize,
    /// Max exposure time of IR auto exposure
    IRAEMaxExposureInt = orb::OBPropertyID_OB_PROP_IR_AE_MAX_EXPOSURE_INT as isize,
    /// Disparity search range mode
    DispSearchRangeModeInt = orb::OBPropertyID_OB_PROP_DISP_SEARCH_RANGE_MODE_INT as isize,
    /// Laser high temperature protection
    LaserHighTemperatureProtectBool =
        orb::OBPropertyID_OB_PROP_LASER_HIGH_TEMPERATURE_PROTECT_BOOL as isize,
    /// Low exposure laser control
    LowExposureLaserControlBool =
        orb::OBPropertyID_OB_PROP_LOW_EXPOSURE_LASER_CONTROL_BOOL as isize,
    /// Check pps sync in signal
    CheckPPSSyncInSignalBool = orb::OBPropertyID_OB_PROP_CHECK_PPS_SYNC_IN_SIGNAL_BOOL as isize,
    /// Disparity search range offset
    DispSearchOffsetInt = orb::OBPropertyID_OB_PROP_DISP_SEARCH_OFFSET_INT as isize,
    /// Repower device
    DeviceRepowerBool = orb::OBPropertyID_OB_PROP_DEVICE_REPOWER_BOOL as isize,
    /// Frame interleave config index
    FrameInterleaveConfigIndexInt =
        orb::OBPropertyID_OB_PROP_FRAME_INTERLEAVE_CONFIG_INDEX_INT as isize,
    /// Frame interleave enable
    FrameInterleaveEnableBool = orb::OBPropertyID_OB_PROP_FRAME_INTERLEAVE_ENABLE_BOOL as isize,
    /// Laser pattern sync with delay
    FrameInterleaveLaserPatternSyncDelayInt =
        orb::OBPropertyID_OB_PROP_FRAME_INTERLEAVE_LASER_PATTERN_SYNC_DELAY_INT as isize,
    /// Get the health check result from device
    OnChipCalibrationHealthCheckFloat =
        orb::OBPropertyID_OB_PROP_ON_CHIP_CALIBRATION_HEALTH_CHECK_FLOAT as isize,
    /// Enable or disable on-chip calibration
    OnChipCalibrationEnableBool =
        orb::OBPropertyID_OB_PROP_ON_CHIP_CALIBRATION_ENABLE_BOOL as isize,
    /// Hardware noise remove filter switch
    HWNoiseRemoveFilterEnableBool =
        orb::OBPropertyID_OB_PROP_HW_NOISE_REMOVE_FILTER_ENABLE_BOOL as isize,
    /// Hardware noise remove filter threshold
    HWNoiseRemoveFilterThresholdFloat =
        orb::OBPropertyID_OB_PROP_HW_NOISE_REMOVE_FILTER_THRESHOLD_FLOAT as isize,
    /// Soft trigger auto capture enable
    DeviceAutoCaptureEnableBool = orb::OBPropertyID_OB_DEVICE_AUTO_CAPTURE_ENABLE_BOOL as isize,
    /// Soft trigger auto capture interval time
    DeviceAutoCaptureIntervalTimeInt =
        orb::OBPropertyID_OB_DEVICE_AUTO_CAPTURE_INTERVAL_TIME_INT as isize,
    /// PTP time synchronization enable
    DevicePTPClockSyncEnableBool = orb::OBPropertyID_OB_DEVICE_PTP_CLOCK_SYNC_ENABLE_BOOL as isize,
    /// Depth with confidence stream enable
    DepthWithConfidenceStreamEnableBool =
        orb::OBPropertyID_OB_PROP_DEPTH_WITH_CONFIDENCE_STREAM_ENABLE_BOOL as isize,
    /// Enable or disable confidence stream filter
    ConfidenceStreamFilterBool = orb::OBPropertyID_OB_PROP_CONFIDENCE_STREAM_FILTER_BOOL as isize,
    /// Confidence stream filter threshold
    ConfidenceStreamFilterThresholdInt =
        orb::OBPropertyID_OB_PROP_CONFIDENCE_STREAM_FILTER_THRESHOLD_INT as isize,
    /// Confidence stream mirror enable
    ConfidenceMirrorBool = orb::OBPropertyID_OB_PROP_CONFIDENCE_MIRROR_BOOL as isize,
    /// Confidence stream flip enable
    ConfidenceFlipBool = orb::OBPropertyID_OB_PROP_CONFIDENCE_FLIP_BOOL as isize,
    /// Confidence stream rotate
    ConfidenceRotateInt = orb::OBPropertyID_OB_PROP_CONFIDENCE_ROTATE_INT as isize,
    /// Color camera auto exposure
    ColorAutoExposureBool = orb::OBPropertyID_OB_PROP_COLOR_AUTO_EXPOSURE_BOOL as isize,
    /// Color camera exposure adjustment
    ColorExposureInt = orb::OBPropertyID_OB_PROP_COLOR_EXPOSURE_INT as isize,
    /// Color camera gain adjustment
    ColorGainInt = orb::OBPropertyID_OB_PROP_COLOR_GAIN_INT as isize,
    /// Color camera automatic white balance
    ColorAutoWhiteBalanceBool = orb::OBPropertyID_OB_PROP_COLOR_AUTO_WHITE_BALANCE_BOOL as isize,
    /// Color camera white balance adjustment
    ColorWhiteBalanceInt = orb::OBPropertyID_OB_PROP_COLOR_WHITE_BALANCE_INT as isize,
    /// Color camera brightness adjustment
    ColorBrightnessInt = orb::OBPropertyID_OB_PROP_COLOR_BRIGHTNESS_INT as isize,
    /// Color camera sharpness adjustment
    ColorSharpnessInt = orb::OBPropertyID_OB_PROP_COLOR_SHARPNESS_INT as isize,
    /// Color camera shutter adjustment
    ColorShutterInt = orb::OBPropertyID_OB_PROP_COLOR_SHUTTER_INT as isize,
    /// Color camera saturation adjustment
    ColorSaturationInt = orb::OBPropertyID_OB_PROP_COLOR_SATURATION_INT as isize,
    /// Color camera contrast adjustment
    ColorContrastInt = orb::OBPropertyID_OB_PROP_COLOR_CONTRAST_INT as isize,
    /// Color camera gamma adjustment
    ColorGammaInt = orb::OBPropertyID_OB_PROP_COLOR_GAMMA_INT as isize,
    /// Color camera image rotation
    ColorRollInt = orb::OBPropertyID_OB_PROP_COLOR_ROLL_INT as isize,
    /// Color camera auto exposure priority
    ColorAutoExposurePriorityInt =
        orb::OBPropertyID_OB_PROP_COLOR_AUTO_EXPOSURE_PRIORITY_INT as isize,
    /// Color camera brightness compensation
    ColorBacklightCompensationInt =
        orb::OBPropertyID_OB_PROP_COLOR_BACKLIGHT_COMPENSATION_INT as isize,
    /// Color camera color tint
    ColorHueInt = orb::OBPropertyID_OB_PROP_COLOR_HUE_INT as isize,
    /// Color Camera Power Line Frequency
    ColorPowerLineFrequencyInt = orb::OBPropertyID_OB_PROP_COLOR_POWER_LINE_FREQUENCY_INT as isize,
    /// Automatic exposure of depth camera
    DepthAutoExposureBool = orb::OBPropertyID_OB_PROP_DEPTH_AUTO_EXPOSURE_BOOL as isize,
    /// Depth camera exposure adjustment
    DepthExposureInt = orb::OBPropertyID_OB_PROP_DEPTH_EXPOSURE_INT as isize,
    /// Depth camera gain adjustment
    DepthGainInt = orb::OBPropertyID_OB_PROP_DEPTH_GAIN_INT as isize,
    /// Infrared camera auto exposure
    IRAutoExposureBool = orb::OBPropertyID_OB_PROP_IR_AUTO_EXPOSURE_BOOL as isize,
    /// Infrared camera exposure adjustment
    IRExposureInt = orb::OBPropertyID_OB_PROP_IR_EXPOSURE_INT as isize,
    /// Infrared camera gain adjustment
    IRGainInt = orb::OBPropertyID_OB_PROP_IR_GAIN_INT as isize,
    /// Select Infrared camera data source channel
    IRChannelDataSourceInt = orb::OBPropertyID_OB_PROP_IR_CHANNEL_DATA_SOURCE_INT as isize,
    /// Depth effect dedistortion
    DepthRMFilterBool = orb::OBPropertyID_OB_PROP_DEPTH_RM_FILTER_BOOL as isize,
    /// Color camera maximal gain
    ColorMaximalGainInt = orb::OBPropertyID_OB_PROP_COLOR_MAXIMAL_GAIN_INT as isize,
    /// Color camera shutter gain
    ColorMaximalShutterInt = orb::OBPropertyID_OB_PROP_COLOR_MAXIMAL_SHUTTER_INT as isize,
    /// Enable/disable IR short exposure function
    IRShortExposureBool = orb::OBPropertyID_OB_PROP_IR_SHORT_EXPOSURE_BOOL as isize,
    /// Color camera HDR
    ColorHDRBool = orb::OBPropertyID_OB_PROP_COLOR_HDR_BOOL as isize,
    /// IR long exposure mode switch
    IRLongExposureBool = orb::OBPropertyID_OB_PROP_IR_LONG_EXPOSURE_BOOL as isize,
    /// Setting and getting the USB device frame skipping mode status
    SkipFrameBool = orb::OBPropertyID_OB_PROP_SKIP_FRAME_BOOL as isize,
    /// Depth HDR merge
    HDRMergeBool = orb::OBPropertyID_OB_PROP_HDR_MERGE_BOOL as isize,
    /// Color camera FOCUS
    ColorFocusInt = orb::OBPropertyID_OB_PROP_COLOR_FOCUS_INT as isize,
    /// IR rectify status
    IRRectifyBool = orb::OBPropertyID_OB_PROP_IR_RECTIFY_BOOL as isize,
    /// Depth camera priority
    DepthAutoExposurePriorityInt =
        orb::OBPropertyID_OB_PROP_DEPTH_AUTO_EXPOSURE_PRIORITY_INT as isize,
    /// Software disparity to depth
    SDKDisparityToDepthBool = orb::OBPropertyID_OB_PROP_SDK_DISPARITY_TO_DEPTH_BOOL as isize,
    /// Depth data unpacking function switch
    SDKDepthFrameUnpackBool = orb::OBPropertyID_OB_PROP_SDK_DEPTH_FRAME_UNPACK_BOOL as isize,
    /// IR data unpacking function switch
    SDKIRFrameUnpackBool = orb::OBPropertyID_OB_PROP_SDK_IR_FRAME_UNPACK_BOOL as isize,
    /// Accel data conversion function switch
    SDKAccelFrameTransformedBool =
        orb::OBPropertyID_OB_PROP_SDK_ACCEL_FRAME_TRANSFORMED_BOOL as isize,
    /// Gyro data conversion function switch
    SDKGyroFrameTransformedBool =
        orb::OBPropertyID_OB_PROP_SDK_GYRO_FRAME_TRANSFORMED_BOOL as isize,
    /// Left IR frame data unpacking function switch
    SDKIRLeftFrameUnpackBool = orb::OBPropertyID_OB_PROP_SDK_IR_LEFT_FRAME_UNPACK_BOOL as isize,
    /// Right IR frame data unpacking function switch
    SDKIRRightFrameUnpackBool = orb::OBPropertyID_OB_PROP_SDK_IR_RIGHT_FRAME_UNPACK_BOOL as isize,
    /// Read the current network bandwidth type of the network device
    NetworkBandwidthTypeInt = orb::OBPropertyID_OB_PROP_NETWORK_BANDWIDTH_TYPE_INT as isize,
    /// Switch device performance mode
    DevicePerformanceModeInt = orb::OBPropertyID_OB_PROP_DEVICE_PERFORMANCE_MODE_INT as isize,
    /// Calibration JSON file read from device
    RawDataCameraCalibJsonFile = orb::OBPropertyID_OB_RAW_DATA_CAMERA_CALIB_JSON_FILE as isize,
    /// Confidence degree
    DebugESGMConfidenceFloat = orb::OBPropertyID_OB_PROP_DEBUG_ESGM_CONFIDENCE_FLOAT as isize,
}

/// Filter configuration value type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBFilterConfigValueType {
    /// Invalid Type
    Invalid = orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_INVALID as isize,
    /// Integer Type
    Int = orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_INT as isize,
    /// Float Type
    Float = orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_FLOAT as isize,
    /// Boolean Type
    Bool = orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_BOOLEAN as isize,
}

impl From<orb::OBFilterConfigValueType> for OBFilterConfigValueType {
    fn from(value: orb::OBFilterConfigValueType) -> Self {
        match value {
            orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_INVALID => {
                OBFilterConfigValueType::Invalid
            }
            orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_INT => {
                OBFilterConfigValueType::Int
            }
            orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_FLOAT => {
                OBFilterConfigValueType::Float
            }
            orb::OBFilterConfigValueType_OB_FILTER_CONFIG_VALUE_TYPE_BOOLEAN => {
                OBFilterConfigValueType::Bool
            }
            _ => OBFilterConfigValueType::Invalid,
        }
    }
}

/// Format conversion types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBConvertFormat {
    /// YUYV to RGB
    YUYVToRGB = orb::OBConvertFormat_FORMAT_YUYV_TO_RGB as isize,
    /// I420 to RGB
    I420ToRGB = orb::OBConvertFormat_FORMAT_I420_TO_RGB as isize,
    /// NV21 to RGB
    NV21ToRGB = orb::OBConvertFormat_FORMAT_NV21_TO_RGB as isize,
    /// NV12 to RGB
    NV12ToRGB = orb::OBConvertFormat_FORMAT_NV12_TO_RGB as isize,
    /// MJPG to I420
    MJPGToI420 = orb::OBConvertFormat_FORMAT_MJPG_TO_I420 as isize,
    /// RGB to BGR
    RGBToBGR = orb::OBConvertFormat_FORMAT_RGB_TO_BGR as isize,
    /// MJPG to NV21
    MJPGToNV21 = orb::OBConvertFormat_FORMAT_MJPG_TO_NV21 as isize,
    /// MJPG to RGB
    MJPGToRGB = orb::OBConvertFormat_FORMAT_MJPG_TO_RGB as isize,
    /// MJPG to BGR
    MJPGToBGR = orb::OBConvertFormat_FORMAT_MJPG_TO_BGR as isize,
    /// MJPG to BGRA
    MJPGToBGRA = orb::OBConvertFormat_FORMAT_MJPG_TO_BGRA as isize,
    /// UYVY to RGB
    UYVYToRGB = orb::OBConvertFormat_FORMAT_UYVY_TO_RGB as isize,
    /// BGR to RGB
    BGRToRGB = orb::OBConvertFormat_FORMAT_BGR_TO_RGB as isize,
    /// MJPG to NV12
    MJPGToNV12 = orb::OBConvertFormat_FORMAT_MJPG_TO_NV12 as isize,
    /// YUYV to BGR
    YUYVToBGR = orb::OBConvertFormat_FORMAT_YUYV_TO_BGR as isize,
    /// YUYV to RGBA
    YUYVToRGBA = orb::OBConvertFormat_FORMAT_YUYV_TO_RGBA as isize,
    /// YUYV to BGRA
    YUYVToBGRA = orb::OBConvertFormat_FORMAT_YUYV_TO_BGRA as isize,
    /// YUYV to Y16
    YUYVToY16 = orb::OBConvertFormat_FORMAT_YUYV_TO_Y16 as isize,
    /// YUYV to Y8
    YUYVToY8 = orb::OBConvertFormat_FORMAT_YUYV_TO_Y8 as isize,
    /// RGBA to RGB
    RGBAToRGB = orb::OBConvertFormat_FORMAT_RGBA_TO_RGB as isize,
    /// BGRA to BGR
    BGRAToBGR = orb::OBConvertFormat_FORMAT_BGRA_TO_BGR as isize,
    /// Y16 to RGB
    Y16ToRGB = orb::OBConvertFormat_FORMAT_Y16_TO_RGB as isize,
    /// Y8 to RGB
    Y8ToRGB = orb::OBConvertFormat_FORMAT_Y8_TO_RGB as isize,
}

/// Exception types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBExceptionType {
    /// Unknown error, an error not clearly defined by the SDK
    Unknown = orb::OBExceptionType_OB_EXCEPTION_TYPE_UNKNOWN as isize,
    /// Standard exception, an error caused by the standard library
    StdException = orb::OBExceptionType_OB_EXCEPTION_STD_EXCEPTION as isize,
    /// Camera/Device has been disconnected, the camera/device is not available
    CameraDisconnected = orb::OBExceptionType_OB_EXCEPTION_TYPE_CAMERA_DISCONNECTED as isize,
    /// An error in the SDK adaptation platform layer, which means an error in the implementation of a specific system platform
    PlatformException = orb::OBExceptionType_OB_EXCEPTION_TYPE_PLATFORM as isize,
    /// Invalid parameter type exception, need to check input parameter
    InvalidValue = orb::OBExceptionType_OB_EXCEPTION_TYPE_INVALID_VALUE as isize,
    /// Wrong API call sequence, the API is called in the wrong order or the wrong parameter is passed
    WrongAPICallSequence = orb::OBExceptionType_OB_EXCEPTION_TYPE_WRONG_API_CALL_SEQUENCE as isize,
    /// SDK and firmware have not yet implemented this function or feature
    NotImplemented = orb::OBExceptionType_OB_EXCEPTION_TYPE_NOT_IMPLEMENTED as isize,
    /// SDK access I/O exception error
    IOException = orb::OBExceptionType_OB_EXCEPTION_TYPE_IO as isize,
    /// SDK access and use memory errors. For example, the frame fails to allocate memory
    MemoryException = orb::OBExceptionType_OB_EXCEPTION_TYPE_MEMORY as isize,
    /// Unsupported operation type error by SDK or device
    UnsupportedOperation = orb::OBExceptionType_OB_EXCEPTION_TYPE_UNSUPPORTED_OPERATION as isize,
}

impl From<orb::OBExceptionType> for OBExceptionType {
    fn from(value: orb::OBExceptionType) -> Self {
        match value {
            orb::OBExceptionType_OB_EXCEPTION_TYPE_UNKNOWN => OBExceptionType::Unknown,
            orb::OBExceptionType_OB_EXCEPTION_STD_EXCEPTION => OBExceptionType::StdException,
            orb::OBExceptionType_OB_EXCEPTION_TYPE_CAMERA_DISCONNECTED => {
                OBExceptionType::CameraDisconnected
            }
            orb::OBExceptionType_OB_EXCEPTION_TYPE_PLATFORM => OBExceptionType::PlatformException,
            orb::OBExceptionType_OB_EXCEPTION_TYPE_INVALID_VALUE => OBExceptionType::InvalidValue,
            orb::OBExceptionType_OB_EXCEPTION_TYPE_WRONG_API_CALL_SEQUENCE => {
                OBExceptionType::WrongAPICallSequence
            }
            orb::OBExceptionType_OB_EXCEPTION_TYPE_NOT_IMPLEMENTED => {
                OBExceptionType::NotImplemented
            }
            orb::OBExceptionType_OB_EXCEPTION_TYPE_IO => OBExceptionType::IOException,
            orb::OBExceptionType_OB_EXCEPTION_TYPE_MEMORY => OBExceptionType::MemoryException,
            orb::OBExceptionType_OB_EXCEPTION_TYPE_UNSUPPORTED_OPERATION => {
                OBExceptionType::UnsupportedOperation
            }
            _ => OBExceptionType::Unknown,
        }
    }
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBDeviceType {
    /// Unknown device type
    Unknown = orb::OBDeviceType_OB_DEVICE_TYPE_UNKNOWN as isize,
    /// Monocular structured light camera
    SLMonocularCamera = orb::OBDeviceType_OB_STRUCTURED_LIGHT_MONOCULAR_CAMERA as isize,
    /// Binocular structured light camera
    SLBinocularCamera = orb::OBDeviceType_OB_STRUCTURED_LIGHT_BINOCULAR_CAMERA as isize,
    /// Time-of-Flight camera
    TOFCamera = orb::OBDeviceType_OB_TOF_CAMERA as isize,
}

impl From<orb::OBDeviceType> for OBDeviceType {
    fn from(value: orb::OBDeviceType) -> Self {
        match value {
            orb::OBDeviceType_OB_DEVICE_TYPE_UNKNOWN => OBDeviceType::Unknown,
            orb::OBDeviceType_OB_STRUCTURED_LIGHT_MONOCULAR_CAMERA => {
                OBDeviceType::SLMonocularCamera
            }
            orb::OBDeviceType_OB_STRUCTURED_LIGHT_BINOCULAR_CAMERA => {
                OBDeviceType::SLBinocularCamera
            }
            orb::OBDeviceType_OB_TOF_CAMERA => OBDeviceType::TOFCamera,
            _ => OBDeviceType::Unknown,
        }
    }
}

/// Stream Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBStreamType {
    /// Unknown stream type
    Unknown = orb::OBStreamType_OB_STREAM_UNKNOWN as isize,
    /// Video stream
    Video = orb::OBStreamType_OB_STREAM_VIDEO as isize,
    /// Infrared stream
    Infrared = orb::OBStreamType_OB_STREAM_IR as isize,
    /// Color stream
    Color = orb::OBStreamType_OB_STREAM_COLOR as isize,
    /// Depth stream
    Depth = orb::OBStreamType_OB_STREAM_DEPTH as isize,
    /// Accelerometer stream
    Accelerometer = orb::OBStreamType_OB_STREAM_ACCEL as isize,
    /// Gyroscope stream
    Gyroscope = orb::OBStreamType_OB_STREAM_GYRO as isize,
    /// Left infrared stream
    LeftInfrared = orb::OBStreamType_OB_STREAM_IR_LEFT as isize,
    /// Right infrared stream
    RightInfrared = orb::OBStreamType_OB_STREAM_IR_RIGHT as isize,
    /// Raw phase stream
    RawPhase = orb::OBStreamType_OB_STREAM_RAW_PHASE as isize,
    /// Confidence stream
    Confidence = orb::OBStreamType_OB_STREAM_CONFIDENCE as isize,
}

/// Permission type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBPermissionType {
    /// Read permission
    Read = orb::OBPermissionType_OB_PERMISSION_READ as isize,
    /// Write permission
    Write = orb::OBPermissionType_OB_PERMISSION_WRITE as isize,
    /// Read and write permission
    ReadWrite = orb::OBPermissionType_OB_PERMISSION_READ_WRITE as isize,
}

/// Hole Fill Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBHoleFillMode {
    /// Fill with the top pixel value
    Top = orb::OBHoleFillingMode_OB_HOLE_FILL_TOP as isize,
    /// Fill with the nearest pixel (distance to camera) value
    Nearest = orb::OBHoleFillingMode_OB_HOLE_FILL_NEAREST as isize,
    /// Fill with the farthest pixel (distance to camera) value
    Farthest = orb::OBHoleFillingMode_OB_HOLE_FILL_FAREST as isize,
}

/// Point Cloud Coordinate System
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBCoordinateSystem {
    /// Left Handed Coordinate System
    LeftHanded = orb::OB_COORDINATE_SYSTEM_TYPE_OB_LEFT_HAND_COORDINATE_SYSTEM as isize,
    /// Right Handed Coordinate System
    RightHanded = orb::OB_COORDINATE_SYSTEM_TYPE_OB_RIGHT_HAND_COORDINATE_SYSTEM as isize,
}

/// Align Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBAlignMode {
    /// Turn off alignment
    Disable = orb::OBAlignMode_ALIGN_DISABLE as isize,
    /// Hardware D2C alignment mode
    Hardware = orb::OBAlignMode_ALIGN_D2C_HW_MODE as isize,
    /// Software D2C alignment mode
    Software = orb::OBAlignMode_ALIGN_D2C_SW_MODE as isize,
}

/// Frame Aggregate Output Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OBFrameAggregateOutputMode {
    /// Only FrameSet that contains all types of data frames will be output
    AllTypeFrameRequire =
        orb::OB_FRAME_AGGREGATE_OUTPUT_MODE_OB_FRAME_AGGREGATE_OUTPUT_ALL_TYPE_FRAME_REQUIRE
            as isize,
    /// Color Frame Require output mode
    ColorFrameRequire =
        orb::OB_FRAME_AGGREGATE_OUTPUT_MODE_OB_FRAME_AGGREGATE_OUTPUT_COLOR_FRAME_REQUIRE as isize,
    /// FrameSet for any case will be output
    AnySituation =
        orb::OB_FRAME_AGGREGATE_OUTPUT_MODE_OB_FRAME_AGGREGATE_OUTPUT_ANY_SITUATION as isize,
    /// Disable Frame Aggreate
    Disable = orb::OB_FRAME_AGGREGATE_OUTPUT_MODE_OB_FRAME_AGGREGATE_OUTPUT_DISABLE as isize,
}
