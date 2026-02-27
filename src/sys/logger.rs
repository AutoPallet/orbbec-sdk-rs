//! Logger configuration and callbacks.
use std::os::raw::{c_char, c_void};

use crate::sys::call_ob_function;
use crate::sys::orb::OBDeviceLogSeverityLevel;

use super::{OBError, orb};

/// Raw log callback type from the C API.
pub(crate) type OBLogCallback = orb::ob_log_callback;

/// Set the global log severity.
pub fn set_logger_severity(severity: OBDeviceLogSeverityLevel) -> Result<(), OBError> {
    call_ob_function!(orb::ob_set_logger_severity, severity)
}

/// Set the logger to output to a file with the given log severity and directory.
pub fn set_logger_directory(
    severity: OBDeviceLogSeverityLevel,
    directory: *const c_char,
) -> Result<(), OBError> {
    call_ob_function!(orb::ob_set_logger_to_file, severity, directory)
}

/// Set the logger to call a callback with the given log severity.
pub fn set_logger_callback(
    severity: OBDeviceLogSeverityLevel,
    callback: OBLogCallback,
    user_data: *mut c_void,
) -> Result<(), OBError> {
    call_ob_function!(
        orb::ob_set_logger_to_callback,
        severity,
        callback,
        user_data
    )
}

/// Set the logger to output to the console with the given log severity.
pub fn set_logger_console(severity: OBDeviceLogSeverityLevel) -> Result<(), OBError> {
    call_ob_function!(orb::ob_set_logger_to_console, severity)
}
