//! Logger configuration and callbacks.
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::path::Path;

use crate::error::{OrbbecError, OrbbecErrorData};
use crate::{LogSeverity, sys};

pub(crate) type LoggerCallback = Box<Box<dyn FnMut(LogSeverity, &str) + Send>>;

/// Handle that keeps a logger callback alive.
///
/// The callback remains valid only while this handle is alive!
#[must_use = "dropping the handle will disable the callback"]
pub struct LoggerCallbackHandle {
    _callback: LoggerCallback,
}

impl Drop for LoggerCallbackHandle {
    fn drop(&mut self) {
        sys::logger::set_logger_callback(LogSeverity::Off, None, std::ptr::null_mut())
            .map_err(OrbbecError::from)
            .unwrap();
    }
}
/// Logger configuration utilities.
pub struct Logger;

impl Logger {
    /// Set the global log severity.
    pub fn set_severity(severity: LogSeverity) -> Result<(), OrbbecError> {
        sys::logger::set_logger_severity(severity).map_err(OrbbecError::from)
    }

    /// Set log output to the console.
    pub fn set_console(severity: LogSeverity) -> Result<(), OrbbecError> {
        sys::logger::set_logger_console(severity).map_err(OrbbecError::from)
    }

    /// Set log output directory.
    ///
    /// If `directory` is `None`, the existing settings will continue to be used (if the existing
    /// configuration is also empty, the log will not be output to the file)
    pub fn set_directory(
        severity: LogSeverity,
        directory: Option<&Path>,
    ) -> Result<(), OrbbecError> {
        let dir = directory
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default();
        let c_dir = CString::new(dir).map_err(|err| {
            OrbbecError::InvalidValue(OrbbecErrorData {
                message: format!("directory contains NUL byte: {}", err),
                function: "Logger::set_directory".to_string(),
                args: "directory".to_string(),
            })
        })?;

        sys::logger::set_logger_directory(severity, c_dir.as_ptr()).map_err(OrbbecError::from)
    }

    /// Set a log callback.
    ///
    /// The callback remains valid for the lifetime of the returned handle.
    #[must_use = "dropping the handle will disable the callback"]
    pub fn set_callback<F>(
        severity: LogSeverity,
        callback: F,
    ) -> Result<LoggerCallbackHandle, OrbbecError>
    where
        F: FnMut(LogSeverity, &str) + Send + 'static,
    {
        let boxed: LoggerCallback = Box::new(Box::new(callback));
        let user_data = boxed.as_ref() as *const _ as *mut c_void;

        sys::logger::set_logger_callback(severity, Some(log_trampoline), user_data)
            .map_err(OrbbecError::from)?;

        let handle = LoggerCallbackHandle { _callback: boxed };
        Ok(handle)
    }
}

unsafe extern "C" fn log_trampoline(severity: i32, message: *const c_char, user_data: *mut c_void) {
    let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(LogSeverity, &str) + Send>) };
    let message = if message.is_null() {
        std::borrow::Cow::Borrowed("")
    } else {
        unsafe { std::ffi::CStr::from_ptr(message) }.to_string_lossy()
    };

    callback(LogSeverity::from(severity), message.as_ref());
}
