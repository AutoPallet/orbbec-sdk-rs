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

    pub fn set_struct_property<T: StructProperty>(&self, value: T::Value) -> Result<(), OBError> {
        let data_size: u32 = size_of::<T::Value>() as u32;
        let value_ptr = (&value as *const T::Value).cast::<u8>() as *mut u8;

        call_ob_function!(
            orb::ob_device_set_structured_data,
            self.inner,
            T::ID,
            value_ptr,
            data_size,
        )?;

        Ok(())
    }

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

        if data_size != size_of::<T::Value>() as u32 {
            panic!(
                "unexpected size: got {}, expected {}",
                data_size,
                size_of::<T::Value>()
            );
        }

        Ok(unsafe { value.assume_init() })
    }

    /// Get the multi-device sync configuration via the dedicated SDK API.
    ///
    /// The generic structured-data path (`ob_device_get_structured_data` with
    /// `OB_STRUCT_MULTI_DEVICE_SYNC_CONFIG`) is unsafe to use here: on devices
    /// that speak the old sync protocol the SDK writes back a 16-byte
    /// `OBDeviceSyncConfig` instead of the new 25-byte
    /// `ob_multi_device_sync_config`. The dedicated function performs the
    /// protocol negotiation and always returns the new struct.
    pub fn get_multi_device_sync_config(
        &self,
    ) -> Result<orb::ob_multi_device_sync_config, OBError> {
        call_ob_function!(orb::ob_device_get_multi_device_sync_config, self.inner)
    }

    /// Set the multi-device sync configuration via the dedicated SDK API.
    /// See [`Self::get_multi_device_sync_config`] for why this bypasses the
    /// generic structured-data path.
    pub fn set_multi_device_sync_config(
        &self,
        config: &orb::ob_multi_device_sync_config,
    ) -> Result<(), OBError> {
        call_ob_function!(
            orb::ob_device_set_multi_device_sync_config,
            self.inner,
            config as *const _,
        )
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

    /// Install a host clock callback for the global timestamp fitter.
    ///
    /// The fitter pairs each device-time query with the value returned by
    /// `fn_` and fits a linear `device_us -> host_us` model. Passing `None`
    /// restores the SDK's default `std::chrono::system_clock`-based source.
    ///
    /// # Safety
    ///
    /// `user_data` must remain valid (pointing at live storage) for as long as
    /// the SDK might invoke `fn_` — i.e. until the device is destroyed or this
    /// function is called again with a different `user_data` / `None`.
    pub unsafe fn set_global_timestamp_host_clock_fn(
        &self,
        fn_: orb::ob_host_clock_fn,
        user_data: *mut std::ffi::c_void,
    ) -> Result<(), OBError> {
        call_ob_function!(
            orb::ob_device_set_global_timestamp_host_clock_fn,
            self.inner,
            fn_,
            user_data,
        )
    }

    impl_ob_method!(
        /// Set the maximum acceptable round-trip time of a fitter sample, in microseconds.
        set_global_timestamp_max_rtt_us => (),
        orb::ob_device_set_global_timestamp_max_rtt_us,
        max_rtt_us: u64,
    );

    /// Read the current linear fit `host_us = a * device_us + b` from the fitter.
    /// All fields are zero until the fitter has collected enough samples to fit.
    pub fn get_global_timestamp_linear_param(
        &self,
    ) -> Result<orb::ob_linear_func_param, OBError> {
        let mut out = MaybeUninit::<orb::ob_linear_func_param>::uninit();
        call_ob_function!(
            orb::ob_device_get_global_timestamp_linear_param,
            self.inner,
            out.as_mut_ptr(),
        )?;
        Ok(unsafe { out.assume_init() })
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
