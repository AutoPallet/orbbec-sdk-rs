use super::structs::OBDeviceTime;
use crate::sys::orb::{
    OBBaselineCalibrationParam, OBDeviceSerialNumber, OBDeviceTemperature, OBDispOffsetConfig,
    OBMultiDeviceSyncConfig, OBPresetResolutionConfig, OBRegionOfInterest,
};
define_bool_property!(
    AntiCollusionActivationStatus,
    "@brief Anti_collusion activation status"
);
///This property is a struct, but we don't have a struct value type
struct AsicSerialNumber;
define_bool_property!(
    AutoCaptureEnable,
    "@brief soft trigger auto capture enable, use in OB_MULTI_DEVICE_SYNC_MODE_SOFTWARE_TRIGGERING mode"
);
define_int_property!(
    AutoCaptureIntervalTime,
    "@brief soft trigger auto capture interval time, use in OB_MULTI_DEVICE_SYNC_MODE_SOFTWARE_TRIGGERING mode"
);
define_struct_property!(
    BaselineCalibrationParam,
    OBBaselineCalibrationParam,
    "@brief Baseline calibration parameters"
);
define_bool_property!(
    BootIntoRecoveryMode,
    "@brief Enter recovery mode (flashing mode) when boot the device\n @attention The device will take effect after rebooting with the enable option. After entering recovery mode, you can upgrade the device system. Upgrading\n the system may cause system damage, please use it with caution."
);
define_bool_property!(
    Brt,
    "@brief BRT function switch (anti-background interference), 0: Disable, 1: Enable"
);
define_int_property!(
    CaptureImageFrameNumber,
    "@brief Number frame to capture once a 'OB_PROP_CAPTURE_IMAGE_SIGNAL_BOOL' effect. range: [1, 255]"
);
define_int_property!(CaptureImageNumberInterval, "@brief Capture number interval");
define_bool_property!(CaptureImageSignal, "@brief Signal to capture image");
define_int_property!(CaptureImageTimeInterval, "@brief Capture time interval");
define_int_property!(
    CaptureIntervalMode,
    "@brief Capture interval mode, 0:time interval, 1:number interval"
);
define_bool_property!(CheckPpsSyncInSignal, "@brief check pps sync in signal");
define_int_property!(ColorAeMaxExposure, "@brief Color AE max exposure");
define_int_property!(ColorAeMaxGain, "@brief Color AE max gain");
define_struct_property!(
    ColorAeRoi,
    OBRegionOfInterest,
    "@brief Color Sensor AE ROI configuration\n @brief The Value type is @ref OBRegionOfInterest"
);
define_bool_property!(ColorAntiFlicker, "@brief Color anti-flicker switch");
define_bool_property!(ColorAutoExposure, "@brief Color camera auto exposure");
define_int_property!(
    ColorAutoExposurePriority,
    "@brief Color camera auto exposure priority"
);
define_bool_property!(
    ColorAutoWhiteBalance,
    "@brief Color camera automatic white balance"
);
define_int_property!(
    ColorBacklightCompensation,
    "@brief Color camera brightness compensation"
);
define_int_property!(ColorBrightness, "@brief Color camera brightness adjustment");
define_int_property!(ColorContrast, "@brief Color camera contrast adjustment");
define_int_property!(
    ColorDenoisingLevel,
    "@brief Color camera CCI denoising level. 0: Auto; 1-8: higher values indicate stronger denoising.\n @note This setting has no effect when AE (Auto Exposure) is disabled."
);
define_int_property!(ColorExposure, "@brief Color camera exposure adjustment");
define_bool_property!(ColorFlip, "@brief Color flip");
define_int_property!(ColorFocus, "@brief Color camera FOCUS");
define_int_property!(ColorGain, "@brief Color camera gain adjustment");
define_int_property!(ColorGamma, "@brief Color camera gamma adjustment");
define_bool_property!(ColorHdr, "@brief Color camera HDR");
define_int_property!(ColorHue, "@brief Color camera color tint");
define_bool_property!(ColorLeftFlip, "@brief Left Color flip");
define_bool_property!(ColorLeftMirror, "@brief Left Color mirror");
define_int_property!(
    ColorLeftRotate,
    "@brief Left Color sensor rotation, angle{0, 90, 180, 270}"
);
define_int_property!(ColorMaximalShutter, "@brief Color camera shutter gain");
define_bool_property!(ColorMirror, "@brief Color mirror");
define_int_property!(
    ColorPowerLineFrequency,
    "@brief Color Camera Power Line Frequency"
);
define_int_property!(ColorPresetPriority, "@brief Color camera preset priority");
define_bool_property!(ColorRightFlip, "@brief Right Color flip");
define_bool_property!(ColorRightMirror, "@brief Right Color mirror");
define_int_property!(
    ColorRightRotate,
    "@brief Right Color sensor rotation, angle{0, 90, 180, 270}"
);
define_int_property!(
    ColorRoiBrightness,
    "@brief Color camera ROI brightness adjustment"
);
define_int_property!(ColorRoll, "@brief Color camera image rotation");
define_int_property!(
    ColorRotate,
    "@brief Color sensor rotation, angle{0, 90, 180, 270}"
);
define_int_property!(ColorSaturation, "@brief Color camera saturation adjustment");
define_int_property!(ColorSharpness, "@brief Color camera sharpness adjustment");
define_int_property!(ColorShutter, "@brief Color camera shutter adjustment");
///This property is a struct, but we don't have a struct value type
struct ColorSyncedExposureParam;
define_int_property!(
    ColorWhiteBalance,
    "@brief Color camera white balance adjustment"
);
define_bool_property!(ConfidenceFlip, "@brief Confidence stream flip enable");
define_bool_property!(ConfidenceMirror, "@brief Confidence stream mirror enable");
define_int_property!(
    ConfidenceRotate,
    "@brief Confidence stream rotate angle{0, 90, 180, 270}"
);
define_bool_property!(
    ConfidenceStreamFilter,
    "@brief Enable or disable confidence stream filter"
);
define_int_property!(
    ConfidenceStreamFilterThreshold,
    "@brief Confidence stream filter threshold, range [0, 255]"
);
define_bool_property!(
    CpuTemperatureCalibration,
    "@brief cpu temperature correction . true: calibrate temperature"
);
///This property is a struct, but we don't have a struct value type
struct CurrentDepthAlgMode;
define_bool_property!(
    D2CPreprocess,
    "@brief D2C preprocessing switch (such as RGB cropping), 0: off, 1: on"
);
define_int_property!(
    DcPowerState,
    "@brief DC's power state, enum type: OBDCPowerState"
);
define_float_property!(DebugEsgmConfidence, "@brief Confidence degree");
define_struct_property!(
    DepthAeRoi,
    OBRegionOfInterest,
    "@brief Depth Sensor AE ROI configuration\n @brief The Value type is @ref OBRegionOfInterest\n @brief Since the ir sensor is the same physical sensor as the depth sensor, this property will also effect the ir sensor."
);
define_bool_property!(DepthAlignHardware, "@brief Hardware d2c is on");
define_int_property!(DepthAlignHardwareMode, "@brief Multi-resolution D2C mode");
define_bool_property!(
    DepthAutoExposure,
    "@brief Automatic exposure of depth camera (infrared camera will be set synchronously under some models of devices)"
);
define_int_property!(DepthAutoExposurePriority, "@brief Depth camera priority");
define_int_property!(
    DepthCroppingMode,
    "@brief Depth cropping mode device: OB_DEPTH_CROPPING_MODE"
);
define_int_property!(
    DepthExposure,
    "@brief Depth camera exposure adjustment (infrared cameras will be set synchronously under some models of devices)"
);
define_bool_property!(DepthFlip, "@brief Depth flip");
define_int_property!(
    DepthGain,
    "@brief Depth camera gain adjustment (infrared cameras will be set synchronously under some models of devices)"
);
///This property is a struct, but we don't have a struct value type
struct DepthHdrConfig;
define_bool_property!(DepthHolefilter, "@brief Depth Holefilter");
define_int_property!(
    DepthIndustryMode,
    "@brief Depth Stream Industry Working Mode Settings, currently only supported by DCW2."
);
define_bool_property!(DepthMirror, "@brief Depth mirror");
define_bool_property!(DepthNoiseRemovalFilter, "@brief Software filter switch");
define_int_property!(
    DepthNoiseRemovalFilterMaxDiff,
    "@brief maxdiff for depth noise removal filter"
);
define_int_property!(
    DepthNoiseRemovalFilterMaxSpeckleSize,
    "@brief maxSpeckleSize for depth noise removal filter"
);
define_bool_property!(DepthPostfilter, "@brief Depth Postfilter");
define_int_property!(
    DepthPrecisionLevel,
    "@brief the depth precision level, which may change the depth frame data unit, needs to be confirmed through the ValueScale interface of\n DepthFrame"
);
///This property is a struct, but we don't have a struct value type
struct DepthPrecisionSupportList;
define_bool_property!(
    DepthRmFilter,
    "@brief Depth effect dedistortion, true: on, false: off. mutually exclusive with D2C function, RM_Filter disable When hardware or software D2C is enabled."
);
define_int_property!(
    DepthRotate,
    "@brief Depth sensor rotation, angle{0, 90, 180, 270}"
);
define_float_property!(
    DepthUnitFlexibleAdjustment,
    "@brief Depth unit flexible adjustment\\\n @brief This property allows continuous adjustment of the depth unit, unlike @ref OB_PROP_DEPTH_PRECISION_LEVEL_INT must be set to some fixed value."
);
define_bool_property!(
    DepthWithConfidenceStreamEnable,
    "@brief Depth with confidence stream enable"
);
define_int_property!(
    DeviceAeReference,
    "@brief Device AE reference source\n - 0: Depth based\n - 1: Color based"
);
define_int_property!(
    DeviceAeStrategy,
    "@brief Device AE strategy\n - 0: Default\n - 1: Motion"
);
define_int_property!(
    DeviceCommunicationType,
    "@brief Device communication type, 0: USB; 1: Ethernet(RTSP)"
);
define_int_property!(
    DeviceDevelopmentMode,
    "@brief Device development mode switch, optional modes can refer to the definition in @ref OBDeviceDevelopmentMode,the default mode is\n @ref OB_USER_MODE\n @attention The device takes effect after rebooting when switching modes."
);
define_bool_property!(
    DeviceInRecoveryMode,
    "@brief Query whether the current device is running in recovery mode (read-only)"
);
///This property is a struct, but we don't have a struct value type
struct DeviceIpAddrConfig;
///This property is a struct, but we don't have a struct value type
struct DeviceIpAddrConfigV2;
define_int_property!(
    DeviceIpMode,
    "@brief Device IP mode\n @param value\n   - 0: AMR Sensor Mode.\n        Typically configured for ehternet interface sensors for AMRs.\n        When DHCP is enabled and the device fails to obtain a valid IP address, it falls back to Persistent IP.\n        If neither of Persistent IP and DHCP is specified, Persistent IP is enabled by default.\n\n   - 1: Industrial Sensor Mode.\n        Typically configured for ehternet interface sensors for industrial applications.\n        When DHCP is enabled and the device fails to obtain a valid IP address, it falls back to LLA (Link-Local Address).\n        If Persistent IP and DHCP are both enabled, the sensor starts with the attemp to used the specified persistent IP\n        and falls back to DHCP if Persistent IP fails."
);
define_bool_property!(
    DeviceNetworkLla,
    "@brief LLA (Link Local Address) switch\n\n @deprecated The property is deprecated"
);
define_bool_property!(
    DeviceOfflineAfterIpConfigApply,
    "@brief Indicates whether the device will go offline after applying IP configuration.\n This property does not represent an actual command; it is a capability flag only,\n used to identify whether the current device has the behavior of going offline after IP config is applied."
);
define_int_property!(
    DevicePerformanceMode,
    "@brief Switch device performance mode, currently available in Adaptive Mode and High Performance Mode, such as G335LE."
);
define_int_property!(
    DeviceRebootDelay,
    "@brief Reboot device delay mode. Delay time unit: ms, range: [0, 8000)."
);
define_bool_property!(
    DeviceRepower,
    "@brief Repower device (cut off power and power on again)\n\n @brief Currently using for GMSL device, cut off power and power on again by GMSL host driver."
);
define_struct_property!(
    DeviceSerialNumber,
    OBDeviceSerialNumber,
    "@brief get/set serial number"
);
///This property is a struct, but we don't have a struct value type
struct DeviceStaticIpConfigRecord;
define_struct_property!(
    DeviceTemperature,
    OBDeviceTemperature,
    "@brief Device temperature information"
);
define_struct_property!(DeviceTime, OBDeviceTime, "@brief get/set device time");
define_bool_property!(
    DeviceUsb2RepeatIdentify,
    "@brief Enable or disable the device to retry USB2.0 re-identification when the device is connected to a USB2.0 port.\n @brief This feature ensures that the device is not mistakenly identified as a USB 2.0 device when connected to a USB 3.0 port."
);
define_int_property!(
    DeviceWorkMode,
    "@brief Device operating mode (power consumption)"
);
define_int_property!(
    DhcpAssignIpTimeout,
    "@brief DHCP assign IP timeout, unit: second"
);
define_struct_property!(
    DispOffsetConfig,
    OBDispOffsetConfig,
    "@brief Disparity offset interleaving"
);
define_int_property!(
    DispSearchOffset,
    "@brief Disparity search range offset, range: [0, 127]"
);
define_int_property!(
    DispSearchRangeMode,
    "@brief Disparity search range mode, 1: 128, 2: 256"
);
define_bool_property!(
    DisparityToDepth,
    "@brief Disparity to depth switch, false: switch to software disparity convert to depth, true: switch to hardware disparity convert to depth"
);
define_bool_property!(
    ExternalSignalReset,
    "@brief External signal trigger restart function switch, 0: Disable, 1: Enable"
);
define_int_property!(FanWorkMode, "@brief Fan mode switch");
define_bool_property!(Flood, "@brief IR flood switch");
define_int_property!(FloodLevel, "@brief IR flood level");
define_int_property!(
    FrameInterleaveConfigIndex,
    "@brief frame interleave config index"
);
define_bool_property!(
    FrameInterleaveEnable,
    "@brief frame interleave enable (true:enable,false:disable)"
);
define_int_property!(
    FrameInterleaveLaserPatternSyncDelay,
    "@brief laser pattern sync with delay(us)"
);
define_bool_property!(Gpm, "@brief Enable/disable GPM function");
define_bool_property!(
    HardwareDistortionSwitch,
    "@brief Hardware distortion switch Rectify"
);
define_bool_property!(HdrMerge, "@brief Depth HDR merge, true: on, false: off.");
define_bool_property!(
    Heartbeat,
    "@brief Heartbeat monitoring function switch, 0: Disable, 1: Enable"
);
define_bool_property!(
    HwNoiseRemoveFilterEnable,
    "@brief hardware noise remove filter switch"
);
define_float_property!(
    HwNoiseRemoveFilterThreshold,
    "@brief hardware noise remove filter threshold ,range [0.0 - 1.0]"
);
define_bool_property!(
    IndicatorLight,
    "@brief Indicator switch, 0: Disable, 1: Enable"
);
define_int_property!(
    IntraCameraSyncReference,
    "@brief Intra-camera Sync Reference based on the exposure start time, the exposure middle time, or the exposure end time. the definition in @ref\n OBIntraCameraSyncReference"
);
define_int_property!(
    IrAeMaxExposure,
    "@brief Max exposure time of IR auto exposure"
);
define_bool_property!(
    IrAutoExposure,
    "@brief Infrared camera auto exposure (depth camera will be set synchronously under some models of devices)"
);
define_int_property!(IrBrightness, "@brief IR brightness");
define_int_property!(
    IrChannelDataSource,
    "@brief Select Infrared camera data source channel. If not support throw exception. 0 : IR stream from IR Left sensor; 1 : IR stream from IR Right sensor;"
);
define_int_property!(
    IrExposure,
    "@brief Infrared camera exposure adjustment (some models of devices will set the depth camera synchronously)"
);
define_bool_property!(IrFlip, "@brief IR flip");
define_int_property!(
    IrGain,
    "@brief Infrared camera gain adjustment (the depth camera will be set synchronously under some models of devices)"
);
define_bool_property!(
    IrLongExposure,
    "@brief IR long exposure mode switch read and write."
);
define_bool_property!(IrMirror, "@brief IR mirror");
define_bool_property!(
    IrRectify,
    "@brief ir rectify status,true: ir rectify, false: no rectify"
);
define_bool_property!(
    IrRightFlip,
    "@brief Right IR sensor flip state. true: flip image, false: origin, default: false"
);
define_bool_property!(IrRightMirror, "@brief Right IR sensor mirror state");
define_int_property!(
    IrRightRotate,
    "@brief Right IR sensor rotation, angle{0, 90, 180, 270}"
);
define_int_property!(
    IrRotate,
    "@brief IR/Left-IR sensor rotation, angle{0, 90, 180, 270}"
);
define_bool_property!(
    IrShortExposure,
    "@brief The enable/disable switch for IR short exposure function, supported only by a few devices."
);
define_bool_property!(Laser, "@brief Laser switch");
define_bool_property!(
    LaserAlwaysOn,
    "@brief Laser always on, true: always on, false: off, laser will be turned off when out of exposure time"
);
define_int_property!(LaserControl, "@brief Laser control, 0: off, 1: on, 2: auto");
define_float_property!(LaserCurrent, "@brief Laser current (uint: mA)");
define_bool_property!(
    LaserHighTemperatureProtect,
    "@brief Laser high temperature protection"
);
define_int_property!(
    LaserMode,
    "@brief laser mode, the firmware terminal currently only return 1: IR Drive, 2: Torch"
);
define_int_property!(
    LaserOnOffPattern,
    "@brief Laser on/off alternate mode, 0: off, 1: on-off alternate, 2: off-on alternate\n @attention When turn on this mode, the laser will turn on and turn off alternately each frame."
);
define_bool_property!(
    LaserOvercurrentProtectionStatus,
    "@brief Query the status of laser overcurrent protection (read-only)"
);
define_int_property!(
    LaserPowerActualLevel,
    "@brief Get hardware laser power actual level which real state of laser element. OB_PROP_LASER_POWER_LEVEL_CONTROL_INT99 will effect this command\n which it setting and changed the hardware laser energy level."
);
define_int_property!(LaserPowerLevelControl, "@brief Laser power level");
define_int_property!(LaserPulseWidth, "@brief laser pulse width");
define_bool_property!(
    LaserPulseWidthProtectionStatus,
    "@brief Query the status of laser pulse width protection (read-only)"
);
define_bool_property!(Ldp, "@brief LDP switch");
define_int_property!(
    LdpMeasureDistance,
    "@brief LDP's measure distance, unit: mm"
);
define_bool_property!(LdpStatus, "@brief LDP status");
define_int_property!(
    LidarApdTemperature,
    "@brief LiDAR: get apd temperature, uint: 0.01degrees delsius"
);
define_int_property!(LidarApplyConfigs, "@brief LiDAR: apply configs");
define_int_property!(
    LidarMcuTemperature,
    "@brief LiDAR: get mcu temperature, uint: 0.01degrees delsius"
);
define_float_property!(LidarMemsFovSize, "@brief LiDAR: set/get mems fov size");
define_float_property!(LidarMemsFrenquency, "@brief LiDAR: set/get mems frequency");
define_int_property!(
    LidarMotorSpinSpeed,
    "@brief LiDAR: get realtime motor spin speed, unit:0.01rpm"
);
define_int_property!(LidarPort, "@brief LiDAR: set/get port");
define_int_property!(
    LidarRepetitiveScanMode,
    "@brief LiDAR: get/set repetitive scan mode"
);
define_int_property!(LidarSpecificMode, "@brief LiDAR: get/set specific mode");
define_int_property!(
    LidarTailFilterLevel,
    "@brief LiDAR: set/get tail filter level"
);
define_int_property!(LidarWarningInfo, "@brief LiDAR: get warning info");
define_int_property!(LidarWorkMode, "@brief LiDAR: set/get work mode");
define_bool_property!(
    LowExposureLaserControl,
    "@brief low exposure laser control\n\n @brief Currently using for DabaiA device,if the exposure value is lower than a certain threshold, the laser is turned off;\n if it exceeds another threshold, the laser is turned on again."
);
define_int_property!(MaxDepth, "@brief Maximum depth threshold");
define_int_property!(MinDepth, "@brief Minimum depth threshold");
define_struct_property!(
    MultiDeviceSyncConfig,
    OBMultiDeviceSyncConfig,
    "@brief Multi-device synchronization mode and parameter configuration"
);
define_int_property!(
    NetworkBandwidthType,
    "@brief Read the current network bandwidth type of the network device, whether it is Gigabit Ethernet or Fast Ethernet, such as G335LE."
);
define_bool_property!(
    OnChipCalibrationEnable,
    "@brief Enable or disable on-chip calibration"
);
define_float_property!(
    OnChipCalibrationHealthCheck,
    "@brief Get the health check result from device,range is [0.0f,1.5f]"
);
define_struct_property!(
    PresetResolutionConfig,
    OBPresetResolutionConfig,
    "@brief Preset resolution ratio configuration"
);
define_bool_property!(PtpClockSyncEnable, "@brief PTP time synchronization enable");
define_bool_property!(
    Rectify2,
    "@brief brt2r-rectify function switch (brt2r is a special module on mx6600), 0: Disable, 1: Rectify Enable"
);
define_bool_property!(
    RestoreFactorySettings,
    "@brief Restore factory settings and factory parameters\n @attention This command can only be written, and the parameter value must be true. The command takes effect after restarting the device."
);
///This property is a struct, but we don't have a struct value type
struct RgbCropRoi;
define_bool_property!(
    RgbCustomCrop,
    "@brief Custom RGB cropping switch, 0 is off, 1 is on custom cropping, and the ROI cropping area is issued"
);
define_bool_property!(
    SdkAccelFrameTransformed,
    "@brief Accel data conversion function switch (on by default)"
);
define_bool_property!(
    SdkDepthFrameUnpack,
    "@brief Depth data unpacking function switch (each open stream will be turned on by default, support RLE/Y10/Y11/Y12/Y14 format)"
);
define_bool_property!(SdkDisparityToDepth, "@brief Software disparity to depth");
define_bool_property!(
    SdkGyroFrameTransformed,
    "@brief Gyro data conversion function switch (on by default)"
);
define_bool_property!(
    SdkIrFrameUnpack,
    "@brief IR data unpacking function switch (each current will be turned on by default, support RLE/Y10/Y11/Y12/Y14 format)"
);
define_bool_property!(
    SdkIrLeftFrameUnpack,
    "@brief Left IR frame data unpacking function switch (each current will be turned on by default, support RLE/Y10/Y11/Y12/Y14 format)"
);
define_bool_property!(
    SdkIrRightFrameUnpack,
    "@brief Right IR frame data unpacking function switch (each current will be turned on by default, support RLE/Y10/Y11/Y12/Y14 format)"
);
define_bool_property!(
    SkipFrame,
    "@brief Setting and getting the USB device frame skipping mode status, true: frame skipping mode, false: non-frame skipping mode."
);
define_bool_property!(
    SlaveDeviceSyncStatus,
    "@brief Slave/secondary device synchronization status (read-only)"
);
define_int_property!(
    SwitchIrMode,
    "@brief Switch infrared imaging mode, 0: positive IR mode, 1: passive IR mode"
);
define_bool_property!(
    SyncSignalTriggerOut,
    "@brief Multi-DeviceSync synchronized signal trigger out is enable state. true: enable, false: disable"
);
define_bool_property!(
    TemperatureCompensation,
    "@brief Enable/disable temperature compensation"
);
define_int_property!(
    TimerResetDelayUs,
    "@brief Delay to reset device time, unit: us"
);
define_bool_property!(TimerResetEnable, "");
define_bool_property!(TimerResetSignal, "@brief Reset device time to zero");
define_bool_property!(
    TimerResetTriggerOutEnable,
    "@brief Enable send reset device time signal to other device. true: enable, false: disable"
);
define_int_property!(TimestampOffset, "@brief Timestamp adjustment");
///This property is a struct, but we don't have a struct value type
struct TofExposureThresholdControl;
define_int_property!(TofFilterRange, "@brief tof filter range configuration");
define_int_property!(
    UsbPowerState,
    "@brief USB's power state, enum type: OBUSBPowerState"
);
define_bool_property!(
    Watchdog,
    "@brief Watchdog function switch, 0: Disable, 1: Enable"
);
