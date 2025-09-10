//! Device module
mod device_property;
use std::fmt;

use crate::device::device_property::PropertyValue;
use crate::error::{OrbbecError, OrbbecErrorData};
use crate::{Context, DeviceType, PermissionType, sys};

#[doc(inline)]
pub use device_property::DeviceProperty;

/// Device information
pub struct DeviceInfo {
    inner: sys::device::OBDeviceInfo,
}

impl DeviceInfo {
    pub(crate) fn new(inner: sys::device::OBDeviceInfo) -> Self {
        DeviceInfo { inner }
    }

    /// Get the device name
    pub fn name(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L464
        let cstr = self.inner.get_name().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device USB product ID
    pub fn pid(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L471
        self.inner.get_pid().unwrap() as u16
    }

    /// Get the device USB vendor ID
    pub fn vid(&self) -> u16 {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L477
        self.inner.get_vid().unwrap() as u16
    }

    /// Get the device unique identifier (based on the port it is connected, platform specific)
    pub fn uid(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L482
        let cstr = self.inner.get_uid().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device serial number
    pub fn serial_number(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L488
        let cstr = self.inner.get_serial_number().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device firmware version
    pub fn firmware_version(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L494
        let cstr = self.inner.get_firmware_version().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device hardware version
    pub fn hardware_version(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L500
        let cstr = self.inner.get_hardware_version().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device connection type
    pub fn connection_type(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L506
        let cstr = self.inner.get_connection_type().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device minimum supported SDK version
    pub fn minimum_supported_sdk_version(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L522
        let cstr = self.inner.get_min_supported_sdk_version().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device ASIC name
    pub fn asic_name(&self) -> String {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L528
        let cstr = self.inner.get_asic_name().unwrap();

        cstr.to_string_lossy().into_owned()
    }

    /// Get the device type
    pub fn device_type(&self) -> DeviceType {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L534
        self.inner.get_device_type().unwrap()
    }
}

impl fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DeviceInfo")
            .field("name", &self.name())
            .field("pid", &format_args!("{:04X}", self.pid()))
            .field("vid", &format_args!("{:04X}", self.vid()))
            .field("uid", &self.uid())
            .field("serial_number", &self.serial_number())
            .field("firmware_version", &self.firmware_version())
            .field("hardware_version", &self.hardware_version())
            .field("connection_type", &self.connection_type())
            .field(
                "minimum_supported_sdk_version",
                &self.minimum_supported_sdk_version(),
            )
            .field("asic_name", &self.asic_name())
            .field("device_type", &self.device_type())
            .finish()
    }
}

/// A single Orbbec device
pub struct Device {
    inner: sys::device::OBDevice,
}

impl Device {
    pub(crate) fn new(inner: sys::device::OBDevice) -> Self {
        Device { inner }
    }

    pub(crate) fn inner(&self) -> &sys::device::OBDevice {
        &self.inner
    }

    /// Get the device information
    pub fn info(&self) -> Result<DeviceInfo, OrbbecError> {
        let info = self.inner.get_info().map_err(OrbbecError::from)?;

        Ok(DeviceInfo::new(info))
    }

    /// Load a preset configuration to the device
    /// ### Arguments
    /// * `preset_name` - The name of the preset to load
    pub fn load_preset(&mut self, preset_name: &str) -> Result<(), OrbbecError> {
        let cstr = std::ffi::CString::new(preset_name).map_err(|e| {
            let err_data = OrbbecErrorData {
                message: format!("Invalid preset name: {e}"),
                function: "Device::load_preset".to_string(),
                args: preset_name.to_string(),
            };

            OrbbecError::InvalidValue(err_data)
        })?;

        self.inner.load_preset(&cstr).map_err(OrbbecError::from)
    }

    /// Check if a device property is supported
    /// ### Arguments
    /// * `property_id` - The property to check
    /// * `permission` - The permission type to check (read, write, or read/write)
    pub fn is_property_supported(
        &self,
        property: DeviceProperty,
        permission: PermissionType,
    ) -> Result<bool, OrbbecError> {
        let mut property = property;
        let (prop_id, _) = property.decompose();

        self.inner
            .is_property_supported(prop_id, permission)
            .map_err(OrbbecError::from)
    }

    /// Set a property value on the device
    /// ### Arguments
    /// * `property` - The property to set and its value
    pub fn set_property(&mut self, property: DeviceProperty) -> Result<(), OrbbecError> {
        let mut property = property;
        let (prop_id, prop_type) = property.decompose();

        match prop_type {
            PropertyValue::Bool(value) => self
                .inner
                .set_bool_property(prop_id, *value)
                .map_err(OrbbecError::from),
            PropertyValue::Int(value) => self
                .inner
                .set_int_property(prop_id, *value)
                .map_err(OrbbecError::from),
            PropertyValue::Float(value) => self
                .inner
                .set_float_property(prop_id, *value)
                .map_err(OrbbecError::from),
        }
    }

    /// Get a property value from the device
    /// ### Arguments
    /// * `property` - The property to get
    pub fn get_property(&self, property: &mut DeviceProperty) -> Result<(), OrbbecError> {
        let (prop_id, prop_type) = property.decompose();

        match prop_type {
            PropertyValue::Bool(value) => {
                *value = self
                    .inner
                    .get_bool_property(prop_id)
                    .map_err(OrbbecError::from)?;
            }
            PropertyValue::Int(value) => {
                *value = self
                    .inner
                    .get_int_property(prop_id)
                    .map_err(OrbbecError::from)?;
            }
            PropertyValue::Float(value) => {
                *value = self
                    .inner
                    .get_float_property(prop_id)
                    .map_err(OrbbecError::from)?;
            }
        }
        Ok(())
    }
}

/// A list of Orbbec devices available
pub struct DeviceList<'a> {
    inner: sys::device::OBDeviceList,
    /// We hold a reference to the context to ensure it lives as long as the device list
    _context: &'a Context,
}

impl<'a> DeviceList<'a> {
    pub(crate) fn new(inner: sys::device::OBDeviceList, context: &'a Context) -> Self {
        DeviceList {
            inner,
            _context: context,
        }
    }

    /// Get the number of devices in the list
    pub fn len(&self) -> usize {
        // Unwrap is safe here because internal pointer is guaranteed to be valid
        // SDK only returns error for this function if pointer is NULL
        // Ref: https://github.com/orbbec/OrbbecSDK_v2/blob/815ae047cc977a1f7edd2b97b69ff6cd29f510b3/src/impl/Device.cpp#L27
        self.inner.get_count().unwrap() as usize
    }

    /// Check if the device list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the device at the specified index
    /// ### Arguments
    /// * `index` - The index of the device to get
    pub fn get(&self, index: usize) -> Result<Device, OrbbecError> {
        let device = self.inner.get_device(index as u32);

        device.map(Device::new).map_err(OrbbecError::from)
    }
}

pub struct DeviceListIterator<'a, 'b> {
    device_list: &'b DeviceList<'a>,
    index: usize,
    count: usize,
}

impl<'a, 'b> DeviceListIterator<'a, 'b> {
    pub fn new(device_list: &'a DeviceList<'b>) -> Result<Self, OrbbecError> {
        Ok(DeviceListIterator {
            device_list,
            index: 0,
            count: device_list.len(),
        })
    }
}

impl<'a, 'b> Iterator for DeviceListIterator<'a, 'b> {
    type Item = Result<Device, OrbbecError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            let device = self.device_list.get(self.index);
            self.index += 1;
            Some(device)
        }
    }
}

impl<'a, 'b> IntoIterator for &'b DeviceList<'a>
where
    'b: 'a,
{
    type Item = Result<Device, OrbbecError>;
    type IntoIter = DeviceListIterator<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        DeviceListIterator::<'a, 'b>::new(self).unwrap()
    }
}
