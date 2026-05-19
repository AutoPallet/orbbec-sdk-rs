//! Hand-written `GetProperty` / `SetProperty` impls for properties whose
//! generic structured-data path is unsafe or wrong. The codegen emits a bare
//! `define_property_base!` for these (see `codegen/src/custom.rs`'s
//! `has_custom_property_impl`); this module fills in the accessor traits.

use crate::device::Device;
use crate::error::OrbbecError;
use crate::sys::prop::{GetProperty, Property, SetProperty, property_id_types::MultiDeviceSyncConfig};

// The generic `ob_device_get_structured_data` path returns a 16-byte legacy
// `OBDeviceSyncConfig` on devices that speak the old sync protocol, which
// trips the size check in `OBDevice::get_struct_property`. The dedicated SDK
// functions negotiate the protocol and always exchange the new 25-byte
// `ob_multi_device_sync_config`.

impl GetProperty for MultiDeviceSyncConfig {
    fn get_from(device: &Device) -> Result<<Self as Property>::Value, OrbbecError> {
        device
            .inner
            .get_multi_device_sync_config()
            .map_err(OrbbecError::from)
    }
}

impl SetProperty for MultiDeviceSyncConfig {
    fn set_on(device: &mut Device, value: <Self as Property>::Value) -> Result<(), OrbbecError> {
        device
            .inner
            .set_multi_device_sync_config(&value)
            .map_err(OrbbecError::from)
    }
}
