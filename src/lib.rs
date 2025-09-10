pub mod device;
pub mod error;
pub mod filter;
pub mod frame;
pub mod pipeline;
pub mod stream;
pub(crate) mod sys;

use std::sync::atomic::AtomicBool;

use sys::context::OBContext;

#[doc(inline)]
pub use crate::sys::enums::OBDeviceType as DeviceType;

#[doc(inline)]
pub use crate::sys::enums::OBFormat as Format;

#[doc(inline)]
pub use crate::sys::enums::OBSensorType as SensorType;

#[doc(inline)]
pub use crate::sys::enums::OBStreamType as StreamType;

#[doc(inline)]
pub use crate::sys::enums::OBPermissionType as PermissionType;

#[doc(inline)]
pub use crate::sys::enums::OBHoleFillMode as HoleFillMode;

#[doc(inline)]
pub use crate::sys::enums::OBConvertFormat as ConvertType;

/// There can only be a single context at a time
/// C API does not enforce this, but having multiple contexts
/// will lead to crashes and undefined behavior
static CONTEXT_CREATED: AtomicBool = AtomicBool::new(false);

/// Context Manager
pub struct Context {
    inner: OBContext,
}

impl Context {
    /// Create a new context
    pub fn new() -> Result<Self, error::OrbbecError> {
        if CONTEXT_CREATED
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .is_err()
        {
            let err_data = error::OrbbecErrorData {
                message: "A context has already been created".to_string(),
                function: "Context::new".to_string(),
                args: "".to_string(),
            };

            return Err(error::OrbbecError::WrongAPICallSequence(err_data));
        }

        let ctx = OBContext::new().map_err(error::OrbbecError::from)?;

        Ok(Context { inner: ctx })
    }

    /// Query the list of connected devices
    pub fn query_device_list<'a>(&'a self) -> Result<device::DeviceList<'a>, error::OrbbecError> {
        let list = self
            .inner
            .query_device_list()
            .map_err(error::OrbbecError::from)?;

        Ok(device::DeviceList::new(list, self))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        CONTEXT_CREATED.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
