//! Logger configuration and callbacks.
use std::os::raw::{c_char, c_void};

use super::enums::OBLogSeverity;
use super::{OBError, orb};

/// Raw log callback type from the C API.
pub(crate) type OBLogCallback = orb::ob_log_callback;

/// Set the global log severity.
pub fn set_logger_severity(severity: OBLogSeverity) -> Result<(), OBError> {
    let mut err_ptr = std::ptr::null_mut();

    unsafe { orb::ob_set_logger_severity(severity as i32, &mut err_ptr) };

    OBError::consume(err_ptr)
}

/// Set the logger to output to a file with the given log severity and directory.
pub fn set_logger_directory(
    severity: OBLogSeverity,
    directory: *const c_char,
) -> Result<(), OBError> {
    let mut err_ptr = std::ptr::null_mut();

    unsafe { orb::ob_set_logger_to_file(severity as i32, directory, &mut err_ptr) };

    OBError::consume(err_ptr)
}

/// Set the logger to call a callback with the given log severity.
pub fn set_logger_callback(
    severity: OBLogSeverity,
    callback: OBLogCallback,
    user_data: *mut c_void,
) -> Result<(), OBError> {
    let mut err_ptr = std::ptr::null_mut();

    unsafe { orb::ob_set_logger_to_callback(severity as i32, callback, user_data, &mut err_ptr) };

    OBError::consume(err_ptr)
}

/// Set the logger to output to the console with the given log severity.
pub fn set_logger_console(severity: OBLogSeverity) -> Result<(), OBError> {
    let mut err_ptr = std::ptr::null_mut();

    unsafe { orb::ob_set_logger_to_console(severity as i32, &mut err_ptr) };

    OBError::consume(err_ptr)
}
