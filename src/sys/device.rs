//! Device management and properties
use std::ffi::CStr;

use super::enums::{OBDeviceType, OBPermissionType, OBPropertyID};
use super::{OBError, drop_ob_object, orb};

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
        let mut err_ptr = std::ptr::null_mut();

        let name = unsafe { orb::ob_device_info_get_name(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(name) })
    }

    /// Get device PID
    pub fn get_pid(&self) -> Result<i32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let pid = unsafe { orb::ob_device_info_get_pid(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(pid)
    }

    /// Get device VID
    pub fn get_vid(&self) -> Result<i32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let vid = unsafe { orb::ob_device_info_get_vid(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(vid)
    }

    /// Get device UID
    pub fn get_uid(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let uid = unsafe { orb::ob_device_info_get_uid(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(uid) })
    }

    /// Get device serial number
    pub fn get_serial_number(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let sn = unsafe { orb::ob_device_info_get_serial_number(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(sn) })
    }

    /// Get device firmware version
    pub fn get_firmware_version(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let fw = unsafe { orb::ob_device_info_get_firmware_version(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(fw) })
    }

    /// Get device hardware version
    pub fn get_hardware_version(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let hw = unsafe { orb::ob_device_info_get_hardware_version(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(hw) })
    }

    /// Get device connection type
    pub fn get_connection_type(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let conn = unsafe { orb::ob_device_info_get_connection_type(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(conn) })
    }

    /// Get device minimum supported SDK version
    pub fn get_min_supported_sdk_version(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let ver =
            unsafe { orb::ob_device_info_get_supported_min_sdk_version(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(ver) })
    }

    /// Get device ASIC name
    pub fn get_asic_name(&self) -> Result<&CStr, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let asic = unsafe { orb::ob_device_info_get_asicName(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(unsafe { std::ffi::CStr::from_ptr(asic) })
    }

    /// Get device type
    pub fn get_device_type(&self) -> Result<OBDeviceType, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let dtype = unsafe { orb::ob_device_info_get_device_type(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(dtype.into())
    }
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
        let mut err_ptr = std::ptr::null_mut();

        let info = unsafe { orb::ob_device_get_device_info(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBDeviceInfo::new(info))
    }

    /// Check if a device property is supported
    pub fn is_property_supported(
        &self,
        property_id: OBPropertyID,
        permission: OBPermissionType,
    ) -> Result<bool, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let supported = unsafe {
            orb::ob_device_is_property_supported(
                self.inner,
                property_id as i32,
                permission as i32,
                &mut err_ptr,
            )
        };

        OBError::consume(err_ptr)?;

        Ok(supported)
    }

    /// Set boolean property
    pub fn set_bool_property(&self, property_id: OBPropertyID, value: bool) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_device_set_bool_property(self.inner, property_id as i32, value, &mut err_ptr)
        };

        OBError::consume(err_ptr)
    }

    /// Get boolean property
    pub fn get_bool_property(&self, property_id: OBPropertyID) -> Result<bool, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let value = unsafe {
            orb::ob_device_get_bool_property(self.inner, property_id as i32, &mut err_ptr)
        };

        OBError::consume(err_ptr)?;

        Ok(value)
    }

    /// Set integer property
    pub fn set_int_property(&self, property_id: OBPropertyID, value: i32) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_device_set_int_property(self.inner, property_id as i32, value, &mut err_ptr)
        };

        OBError::consume(err_ptr)
    }

    /// Get integer property
    pub fn get_int_property(&self, property_id: OBPropertyID) -> Result<i32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let value = unsafe {
            orb::ob_device_get_int_property(self.inner, property_id as i32, &mut err_ptr)
        };

        OBError::consume(err_ptr)?;

        Ok(value)
    }

    /// Set float property
    pub fn set_float_property(&self, property_id: OBPropertyID, value: f32) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe {
            orb::ob_device_set_float_property(self.inner, property_id as i32, value, &mut err_ptr)
        };

        OBError::consume(err_ptr)
    }

    /// Get float property
    pub fn get_float_property(&self, property_id: OBPropertyID) -> Result<f32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let value = unsafe {
            orb::ob_device_get_float_property(self.inner, property_id as i32, &mut err_ptr)
        };

        OBError::consume(err_ptr)?;

        Ok(value)
    }

    /// Load the device preset
    /// After loading the preset, the settings in the preset will set to the device immediately. Therefore, it is recommended to re-read the device settings to update the user program temporarily.
    pub fn load_preset(&self, preset_name: &CStr) -> Result<(), OBError> {
        let mut err_ptr = std::ptr::null_mut();

        unsafe { orb::ob_device_load_preset(self.inner, preset_name.as_ptr(), &mut err_ptr) };

        OBError::consume(err_ptr)
    }
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

    /// Get the number of devices in the list
    pub fn get_count(&self) -> Result<u32, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let count = unsafe { orb::ob_device_list_get_count(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(count)
    }

    /// Get the device object at the specified index
    pub fn get_device(&self, index: u32) -> Result<OBDevice, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let device = unsafe { orb::ob_device_list_get_device(self.inner, index, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBDevice::new(device))
    }
}
