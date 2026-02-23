//! Device management and properties
use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::prop::StructProperty;

use super::orb::{OBDeviceType, OBPermissionType, OBPropertyID};
use super::{OBError, call_ob_function, drop_ob_object, impl_ob_method, orb};

/// A class describing device information, representing the name, id, serial number and other basic information of an RGBD camera.
pub struct OBDeviceInfo {
    inner: *mut orb::ob_device_info,
}

drop_ob_object!(OBDeviceInfo, ob_delete_device_info);

impl OBDeviceInfo {
    /// Create a new device info object
    pub(crate) fn new(inner: *mut orb::ob_device_info) -> Self {
        OBDeviceInfo { inner }
    }

    /// Get device name
    pub fn get_name(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_name, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    impl_ob_method!(
        /// Get device PID
        get_pid => i32,
        orb::ob_device_info_get_pid,
    );

    impl_ob_method!(
        /// Get device VID
        get_vid => i32,
        orb::ob_device_info_get_vid,
    );

    /// Get device UID
    pub fn get_uid(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_uid, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device serial number
    pub fn get_serial_number(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_serial_number, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device firmware version
    pub fn get_firmware_version(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_firmware_version, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device hardware version
    pub fn get_hardware_version(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_hardware_version, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device connection type
    pub fn get_connection_type(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_connection_type, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device minimum supported SDK version
    pub fn get_min_supported_sdk_version(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(
            orb::ob_device_info_get_supported_min_sdk_version,
            self.inner
        )?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    /// Get device ASIC name
    pub fn get_asic_name(&self) -> Result<&CStr, OBError> {
        let ptr = call_ob_function!(orb::ob_device_info_get_asicName, self.inner)?;
        Ok(unsafe { CStr::from_ptr(ptr) })
    }

    impl_ob_method!(
        /// Get device type
        get_device_type => OBDeviceType,
        orb::ob_device_info_get_device_type,
    );
}

/// Class representing a single device
pub struct OBDevice {
    inner: *mut orb::ob_device,
}

drop_ob_object!(OBDevice, ob_delete_device);

impl OBDevice {
    pub(crate) fn new(inner: *mut orb::ob_device) -> Self {
        OBDevice { inner }
    }

    pub(crate) fn inner(&self) -> *mut orb::ob_device {
        self.inner
    }

    /// Get device information
    pub fn get_info(&self) -> Result<OBDeviceInfo, OBError> {
        let info = call_ob_function!(orb::ob_device_get_device_info, self.inner)?;
        Ok(OBDeviceInfo::new(info))
    }

    impl_ob_method!(
        /// Check if a device property is supported
        is_property_supported => bool,
        orb::ob_device_is_property_supported,
        property_id: OBPropertyID,
        permission: OBPermissionType,
    );

    impl_ob_method!(
        /// Set boolean property
        set_bool_property => (),
        orb::ob_device_set_bool_property,
        property_id: OBPropertyID,
        value: bool,
    );

    impl_ob_method!(
        /// Get boolean property
        get_bool_property => bool,
        orb::ob_device_get_bool_property,
        property_id: OBPropertyID,
    );

    impl_ob_method!(
        /// Set integer property
        set_int_property => (),
        orb::ob_device_set_int_property,
        property_id: OBPropertyID,
        value: i32,
    );

    impl_ob_method!(
        /// Get integer property
        get_int_property => i32,
        orb::ob_device_get_int_property,
        property_id: OBPropertyID,
    );

    impl_ob_method!(
        /// Set float property
        set_float_property => (),
        orb::ob_device_set_float_property,
        property_id: OBPropertyID,
        value: f32,
    );

    impl_ob_method!(
        /// Get float property
        get_float_property => f32,
        orb::ob_device_get_float_property,
        property_id: OBPropertyID,
    );

    pub fn get_struct_property<T: StructProperty>(&self) -> Result<T::Value, OBError> {
        let mut value = MaybeUninit::<T::Value>::uninit();
        let mut data_size: u32 = size_of::<T::Value>() as u32;

        call_ob_function!(
            orb::ob_device_get_structured_data,
            self.inner,
            T::ID,
            value.as_mut_ptr() as *mut u8,
            &mut data_size
        )?;

        if data_size != size_of::<T>() as u32 {
            panic!(
                "unexpected size: got {}, expected {}",
                data_size,
                size_of::<T>()
            );
        }

        Ok(unsafe { value.assume_init() })
    }

    /// Load the device preset
    /// After loading the preset, the settings in the preset will set to the device immediately. Therefore, it is recommended to re-read the device settings to update the user program temporarily.
    pub fn load_preset(&self, preset_name: &CStr) -> Result<(), OBError> {
        call_ob_function!(orb::ob_device_load_preset, self.inner, preset_name.as_ptr())
    }

    impl_ob_method!(
        /// Check if the device supports global timestamp
        is_global_timestamp_supported => bool,
        orb::ob_device_is_global_timestamp_supported,
    );

    impl_ob_method!(
        /// Enable or disable global timestamp
        enable_global_timestamp => (),
        orb::ob_device_enable_global_timestamp,
        enabled: bool,
    );
}

/// List of devices
pub struct OBDeviceList {
    inner: *mut orb::ob_device_list,
}

drop_ob_object!(OBDeviceList, ob_delete_device_list);

impl OBDeviceList {
    pub(crate) fn new(inner: *mut orb::ob_device_list) -> Self {
        OBDeviceList { inner }
    }

    impl_ob_method!(
        /// Get the number of devices in the list
        get_count => u32,
        orb::ob_device_list_get_count,
    );

    /// Get the device object at the specified index
    pub fn get_device(&self, index: u32) -> Result<OBDevice, OBError> {
        let device = call_ob_function!(orb::ob_device_list_get_device, self.inner, index)?;
        Ok(OBDevice::new(device))
    }
}
