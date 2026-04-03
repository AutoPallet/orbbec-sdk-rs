//! Context management and device enumeration
use super::device::OBDeviceList;
use super::{OBError, call_ob_function, drop_ob_object, orb};

/// Context is a management class that describes the runtime of the SDK and is responsible for resource allocation and release of the SDK.
/// Context has the ability to manage multiple devices. It is responsible for enumerating devices, monitoring device callbacks, and enabling multi-device synchronization.
pub struct OBContext {
    inner: *mut orb::ob_context,
}

drop_ob_object!(OBContext, ob_delete_context);

impl OBContext {
    /// Create a context object with the default configuration file
    pub fn new() -> Result<Self, OBError> {
        let context = call_ob_function!(orb::ob_create_context)?;
        Ok(OBContext { inner: context })
    }

    /// Get a list of enumerated devices
    pub fn query_device_list(&self) -> Result<OBDeviceList, OBError> {
        let list = call_ob_function!(orb::ob_query_device_list, self.inner)?;
        Ok(OBDeviceList::new(list))
    }
}
