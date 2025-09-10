//! Error module
use crate::sys::OBError;
use crate::sys::enums::OBExceptionType;

use std::{error::Error, fmt};

/// Detailed error information
#[derive(Debug)]
pub struct OrbbecErrorData {
    /// Detailed error log
    pub message: String,
    /// Function where the error occurred
    pub function: String,
    /// Arguments passed to the function
    pub args: String,
}

/// Orbbec errors
#[derive(Debug)]
pub enum OrbbecError {
    /// Unknown error, an error not clearly defined by the SDK
    Unknown(OrbbecErrorData),
    /// Standard exception, an error caused by the standard library (cpp)
    StdException(OrbbecErrorData),
    /// Camera/Device has been disconnected, the camera/device is not available
    CameraDisconnected(OrbbecErrorData),
    /// An error in the SDK adaptation platform layer, which means an error in the implementation of a specific system platform
    PlatformException(OrbbecErrorData),
    /// Invalid parameter type exception, need to check input parameter
    InvalidValue(OrbbecErrorData),
    /// Wrong API call sequence, the API is called in the wrong order or the wrong parameter is passed
    WrongAPICallSequence(OrbbecErrorData),
    /// SDK and firmware have not yet implemented this function or feature
    NotImplemented(OrbbecErrorData),
    /// SDK access I/O exception error
    IOException(OrbbecErrorData),
    /// SDK access and use memory errors. For example, the frame fails to allocate memory
    MemoryException(OrbbecErrorData),
    /// Unsupported operation type error by SDK or device
    UnsupportedOperation(OrbbecErrorData),
}

impl fmt::Display for OrbbecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrbbecError::Unknown(data) => write!(f, "Unknown Error: {}", data.message),
            OrbbecError::StdException(data) => write!(f, "Standard Exception: {}", data.message),
            OrbbecError::CameraDisconnected(data) => {
                write!(f, "Camera Disconnected: {}", data.message)
            }
            OrbbecError::PlatformException(data) => {
                write!(f, "Platform Exception: {}", data.message)
            }
            OrbbecError::InvalidValue(data) => write!(f, "Invalid Value: {}", data.message),
            OrbbecError::WrongAPICallSequence(data) => {
                write!(f, "Wrong API Call Sequence: {}", data.message)
            }
            OrbbecError::NotImplemented(data) => write!(f, "Not Implemented: {}", data.message),
            OrbbecError::IOException(data) => write!(f, "I/O Exception: {}", data.message),
            OrbbecError::MemoryException(data) => write!(f, "Memory Exception: {}", data.message),
            OrbbecError::UnsupportedOperation(data) => {
                write!(f, "Unsupported Operation: {}", data.message)
            }
        }
    }
}

impl Error for OrbbecError {}

impl From<&OBError> for OrbbecError {
    fn from(err: &OBError) -> Self {
        let message = err.message();
        let function = err.function();
        let args = err.args();
        let exception_type = err.exception_type();

        let data = OrbbecErrorData {
            message,
            function,
            args,
        };

        match exception_type {
            OBExceptionType::Unknown => OrbbecError::Unknown(data),
            OBExceptionType::StdException => OrbbecError::StdException(data),
            OBExceptionType::CameraDisconnected => OrbbecError::CameraDisconnected(data),
            OBExceptionType::PlatformException => OrbbecError::PlatformException(data),
            OBExceptionType::InvalidValue => OrbbecError::InvalidValue(data),
            OBExceptionType::WrongAPICallSequence => OrbbecError::WrongAPICallSequence(data),
            OBExceptionType::NotImplemented => OrbbecError::NotImplemented(data),
            OBExceptionType::IOException => OrbbecError::IOException(data),
            OBExceptionType::MemoryException => OrbbecError::MemoryException(data),
            OBExceptionType::UnsupportedOperation => OrbbecError::UnsupportedOperation(data),
        }
    }
}

impl From<OBError> for OrbbecError {
    fn from(err: OBError) -> Self {
        OrbbecError::from(&err)
    }
}
