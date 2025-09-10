//! Context management and device enumeration
use super::device::OBDeviceList;
use super::{OBError, drop_ob_object, orb};

/// Context is a management class that describes the runtime of the SDK and is responsible for resource allocation and release of the SDK.
/// Context has the ability to manage multiple devices. It is responsible for enumerating devices, monitoring device callbacks, and enabling multi-device synchronization.
pub struct OBContext {
    inner: *mut orb::ob_context,
}

drop_ob_object!(OBContext, ob_delete_context);

impl OBContext {
    /// Create a context object with the default configuration file
    pub fn new() -> Result<Self, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let context = unsafe { orb::ob_create_context(&mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBContext { inner: context })
    }

    /// Get a list of enumerated devices
    pub fn query_device_list(&self) -> Result<OBDeviceList, OBError> {
        let mut err_ptr = std::ptr::null_mut();

        let device_list = unsafe { orb::ob_query_device_list(self.inner, &mut err_ptr) };

        OBError::consume(err_ptr)?;

        Ok(OBDeviceList::new(device_list))
    }
}
