//! Device module
use std::ffi::c_void;
use std::fmt;

use crate::error::{OrbbecError, OrbbecErrorData};
use crate::sys::prop::{GetProperty, Property, SetProperty};
use crate::{Context, DeviceType, PermissionType, sys};

/// Coefficients of the global timestamp linear fit: `host_us = a * device_us + b`.
///
/// Returned by [`Device::global_timestamp_linear_param`]. All fields are zero
/// until the fitter has collected enough samples (typically a few seconds).
#[derive(Clone, Copy, Debug)]
pub struct LinearFuncParam {
    /// Slope of the fit.
    pub coefficient_a: f64,
    /// Intercept of the fit.
    pub constant_b: f64,
    /// Most recent device-time sample fed into the fit (`x`).
    pub check_data_x: u64,
    /// Host-time partner of `check_data_x` (`y`).
    pub check_data_y: u64,
}

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
    // Drop order matters: `inner` must drop first so the SDK joins the
    // global-timestamp fitter thread before we free `host_clock_fn`. Rust drops
    // fields in declaration order. This only joins the thread if dropping
    // `inner` releases the *last* shared_ptr ref to the underlying ob_device;
    // see the invariant on `host_clock_fn` below.
    pub(crate) inner: sys::device::OBDevice,
    /// Owns the heap storage that the SDK's global-timestamp fitter calls
    /// through. `None` means the SDK is using its built-in clock source.
    ///
    /// Invariant: a `Device` with `host_clock_fn.is_some()` must outlive
    /// every `Pipeline` created from it. A `Pipeline` holds its own
    /// shared_ptr ref to the same underlying ob_device, so if the `Pipeline`
    /// outlives this `Device`, dropping this wrapper does not stop the
    /// fitter thread; the C side keeps calling back into the freed
    /// `host_clock_fn` box, a use-after-free.
    host_clock_fn: Option<Box<dyn FnMut() -> u64 + Send>>,
}

impl Device {
    pub(crate) fn new(inner: sys::device::OBDevice) -> Self {
        Device {
            inner,
            host_clock_fn: None,
        }
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

    /// Check if a device property is supported.
    ///
    /// ### Type Parameters
    /// * `P` - The struct implementing [`Property`] to check (e.g., `DepthAlignHardware`).
    ///
    /// ### Arguments
    /// * `permission` - The permission type to check (read, write, or read/write).
    ///
    /// ### Returns
    /// Returns `Ok(true)` if supported, `Ok(false)` otherwise, or an [`OrbbecError`].
    pub fn is_property_supported<P: Property>(
        &self,
        permission: PermissionType,
    ) -> Result<bool, OrbbecError> {
        self.inner
            .is_property_supported(P::ID, permission)
            .map_err(OrbbecError::from)
    }

    /// Set a property value on the device
    ///
    /// ### Type Parameters
    /// * `P` - The struct implementing [`SetProperty`] to set (e.g., `DepthAlignHardware`).
    ///
    /// ### Arguments
    /// * `property` - The property to set and its value
    pub fn set_property<P: SetProperty>(&mut self, value: P::Value) -> Result<(), OrbbecError> {
        P::set_on(self, value)
    }

    /// Get a property value from the device
    ///
    /// ### Type Parameters
    /// * `P` - The struct implementing [`GetProperty`] to get (e.g., `DepthAlignHardware`).
    ///
    /// ### Returns
    /// Returns the property value, or an [`OrbbecError`].
    /// * `property` - The property to get
    pub fn get_property<P: GetProperty>(&self) -> Result<P::Value, OrbbecError> {
        P::get_from(self)
    }

    /// Check if the device supports global timestamp
    pub fn is_global_timestamp_supported(&self) -> Result<bool, OrbbecError> {
        self.inner
            .is_global_timestamp_supported()
            .map_err(OrbbecError::from)
    }

    /// Enable or disable global timestamp
    pub fn enable_global_timestamp(&self, enabled: bool) -> Result<(), OrbbecError> {
        self.inner
            .enable_global_timestamp(enabled)
            .map_err(OrbbecError::from)
    }

    /// Install a host clock source for the global-timestamp fitter.
    ///
    /// The SDK normally pairs each device-time query with its built-in
    /// `std::chrono::system_clock`-based host clock. Callers that want
    /// `frame.global_timestamp_us()` in a different domain (typically a
    /// monotonic application clock) supply that domain here.
    ///
    /// The closure is invoked from the SDK's fitter thread, twice per sample
    /// interval (default 1 s), bracketing a device-time property query. It
    /// must not block on, and must not call back into, the SDK.
    ///
    /// Safe to call before or after `enable_global_timestamp(true)`. If the
    /// fitter is currently running, the call blocks briefly (~one sample's
    /// RTT) so the SDK can swap atomically; any samples collected with the
    /// previous clock domain are discarded.
    pub fn set_global_timestamp_host_clock_fn<F>(&mut self, f: F) -> Result<(), OrbbecError>
    where
        F: FnMut() -> u64 + Send + 'static,
    {
        // Heap-allocate F so its address is stable. We pass `&mut *boxed` (a
        // thin `*mut F`) as the C-side `user_data`; the trampoline casts it
        // back to invoke F.
        let mut boxed: Box<F> = Box::new(f);
        let user_data = &mut *boxed as *mut F as *mut c_void;

        extern "C" fn trampoline<F: FnMut() -> u64>(user_data: *mut c_void) -> u64 {
            // SAFETY: `user_data` is the pointer we registered alongside
            // `trampoline::<F>`. Its lifetime is tied to the `Box<F>` stored
            // on the `Device`, which outlives the SDK's fitter thread (Drop
            // order: `inner` joins the thread first, then `host_clock_fn`
            // frees this box).
            let f = unsafe { &mut *(user_data as *mut F) };
            f()
        }

        // SAFETY: `user_data` points into `boxed`, which we move into
        // `self.host_clock_fn` immediately below. The C side holds the
        // pointer until either this method is called again or the device is
        // destroyed; in both cases we ensure no fitter call is mid-flight
        // before freeing the previous box.
        unsafe {
            self.inner
                .set_global_timestamp_host_clock_fn(Some(trampoline::<F>), user_data)
                .map_err(OrbbecError::from)?;
        }

        // The C side has returned, meaning no in-flight sample is still using
        // the previous fn pointer. Now it's safe to drop the old box (if any)
        // by replacing it with the new one.
        self.host_clock_fn = Some(boxed);

        Ok(())
    }

    /// Set the maximum acceptable round-trip time of a fitter sample.
    /// Samples whose host-side bracket exceeds this are discarded.
    ///
    /// The SDK default is 20 ms (suitable for USB or wired-gigabit). For
    /// Wi-Fi or congested links, bump this to ~50 ms.
    pub fn set_global_timestamp_max_rtt_us(&self, max_rtt_us: u64) -> Result<(), OrbbecError> {
        self.inner
            .set_global_timestamp_max_rtt_us(max_rtt_us)
            .map_err(OrbbecError::from)
    }

    /// Read the current linear fit from the global-timestamp fitter.
    /// Returns a value whose `coefficient_a` is `0.0` if the fitter has not
    /// collected enough samples yet.
    pub fn global_timestamp_linear_param(&self) -> Result<LinearFuncParam, OrbbecError> {
        let raw = self
            .inner
            .get_global_timestamp_linear_param()
            .map_err(OrbbecError::from)?;
        Ok(LinearFuncParam {
            coefficient_a: raw.coefficient_a,
            constant_b: raw.constant_b,
            check_data_x: raw.check_data_x,
            check_data_y: raw.check_data_y,
        })
    }
}

// SAFETY: the underlying ob_device is a shared_ptr-backed SDK object whose
// property server (a recursive mutex) and other components take their own
// per-device locks, so calls into it are safe from any thread; `host_clock_fn`
// is already `Send`. Note this only covers the C object's own locking: two
// `Device` wrappers can alias the same underlying ob_device (e.g.
// `Pipeline::get_device()` mints a second wrapper from the shared_ptr held by
// a `Pipeline`), and callers must not drive one underlying device
// concurrently through aliased wrappers, since per-wrapper `&mut self`
// exclusivity does not imply per-C-object exclusivity.
unsafe impl Send for Device {}

/// A list of Orbbec devices available
pub struct DeviceList<'a> {
    inner: sys::device::OBDeviceList,
    /// We hold a reference to the context to ensure it lives as long as the device list
    _context: &'a Context,
}

/// Serializes [`DeviceList::get`] across threads. Device creation in the SDK
/// shares unlocked device-manager state, and its slow step (the vendor TCP
/// connect) is globally serialized inside the SDK anyway, so callers lose no
/// parallelism. If a thread holding this lock panics inside `get`, the mutex
/// is poisoned and every later `lock().unwrap()` in `get` panics too.
static DEVICE_OPEN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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

    /// Get the serial number stored in the device-list metadata at `index`.
    ///
    /// Unlike [`Self::get`], this does not create or initialize a [`Device`].
    ///
    /// Returns an error if `index` is outside the device list.
    pub fn serial_number(&self, index: usize) -> Result<String, OrbbecError> {
        let index = u32::try_from(index).map_err(|_| {
            OrbbecError::InvalidValue(OrbbecErrorData {
                message: "Device index exceeds the SDK index range".to_string(),
                function: "DeviceList::serial_number".to_string(),
                args: index.to_string(),
            })
        })?;
        let serial_number = self
            .inner
            .get_serial_number(index)
            .map_err(OrbbecError::from)?;

        Ok(serial_number.to_string_lossy().into_owned())
    }

    /// Get the device at the specified index
    /// ### Arguments
    /// * `index` - The index of the device to get
    pub fn get(&self, index: usize) -> Result<Device, OrbbecError> {
        let _guard = DEVICE_OPEN_LOCK.lock().unwrap();
        let device = self.inner.get_device(index as u32);

        device.map(Device::new).map_err(OrbbecError::from)
    }

    /// Get an iterator over the devices in the list.
    pub fn iter(&self) -> DeviceListIterator<'a, '_> {
        DeviceListIterator::new(self)
    }
}

// SAFETY: the underlying ob_device_list is immutable snapshot metadata plus
// `get_device`, which is serialized by DEVICE_OPEN_LOCK. The `_context`
// reference is only held to keep the SDK context alive.
unsafe impl Send for DeviceList<'_> {}
unsafe impl Sync for DeviceList<'_> {}

/// An iterator over the devices in a device list
pub struct DeviceListIterator<'a, 'b> {
    device_list: &'b DeviceList<'a>,
    index: usize,
    count: usize,
}

impl<'a, 'b> DeviceListIterator<'a, 'b> {
    fn new(device_list: &'b DeviceList<'a>) -> Self {
        DeviceListIterator {
            device_list,
            index: 0,
            count: device_list.len(),
        }
    }
}

impl<'a, 'b> Iterator for DeviceListIterator<'a, 'b> {
    type Item = Result<Device, OrbbecError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }

        let device = self.device_list.get(self.index);
        self.index += 1;
        Some(device)
    }
}

impl<'a, 'b> IntoIterator for &'b DeviceList<'a>
where
    'b: 'a,
{
    type Item = Result<Device, OrbbecError>;
    type IntoIter = DeviceListIterator<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
