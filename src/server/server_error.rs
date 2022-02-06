use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use std::sync::PoisonError;
use std::io;

use hyper;
use hyper::StatusCode;
use serde_json;

use futures::future;
use futures::future::FutureResult;

use crate::utils::camera::CameraError;
use crate::utils::rppal_error::RppalError;

#[derive(Debug)]
pub enum ServerError {
    Json(serde_json::error::Error),
    Io(io::Error),
    Logic(LogicError),
    Hyper(hyper::Error),
    Camera(CameraError),
    Rppal(RppalError),
    Read(ReadError),
    Posion
}

impl From<serde_json::error::Error> for ServerError {
    fn from(e: serde_json::error::Error) -> Self {
        ServerError::Json(e)
    }
}

impl From<io::Error> for ServerError {
    fn from(e: io::Error) -> Self {
        ServerError::Io(e)
    }
}

impl From<LogicError> for ServerError {
    fn from(e: LogicError) -> Self {
        ServerError::Logic(e)
    }
}

impl From<hyper::Error> for ServerError {
    fn from(e: hyper::Error) -> Self {
        ServerError::Hyper(e)
    }
}

impl From<CameraError> for ServerError {
    fn from(e: CameraError) -> Self {
        ServerError::Camera(e)
    }
}

impl From<RppalError> for ServerError {
    fn from(e: RppalError) -> Self {
        ServerError::Rppal(e)
    }
}

impl From<ReadError> for ServerError {
    fn from(e: ReadError) -> Self {
        ServerError::Read(e)
    }
}

impl<T> From<PoisonError<T>> for ServerError {
    fn from(_: PoisonError<T>) -> Self {
        ServerError::Posion
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            ServerError::Json(ref e) => e.fmt(f),
            ServerError::Io(ref e) => e.fmt(f),
            ServerError::Logic(ref e) => e.fmt(f),
            ServerError::Hyper(ref e) => e.fmt(f),
            ServerError::Camera(ref e) => e.fmt(f),
            ServerError::Rppal(ref e) => e.fmt(f),
            ServerError::Read(ref e) => e.fmt(f),
            ServerError::Posion => f.write_str("Poison error")
        }
    }
}

#[derive(Debug)]
pub enum LogicError {
    InvalidProtectedKey = 1,
    CameraNotFound = 2,
    CommandMethodIdNotSet = 3,
    CommandInputNotSet = 4,
    CommandUnsupportedContentType = 5
}

impl Error for LogicError {
    fn description(&self) -> &str {
        match *self {
            LogicError::InvalidProtectedKey => "Invalid protected key",
            LogicError::CameraNotFound => "Camera was not found",
            LogicError::CommandMethodIdNotSet => "Command method is not set",
            LogicError::CommandInputNotSet => "Command input is not set",
            LogicError::CommandUnsupportedContentType => "Command has unsupported content type"
        }
    }
}

impl Display for LogicError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "ServerError: {}", self)
    }
}

#[derive(Debug)]
pub struct ReadError {
    pub code: ReadErrorCode,
    pub status_code: StatusCode
}

#[derive(Debug)]
pub enum ReadErrorCode {
    ServerMethodHeaderNotFound,
    ServerMethodNotFound(String)
}

impl ReadError {
    fn new(code: ReadErrorCode, status_code: StatusCode) -> Self {
        ReadError {
            code,
            status_code
        }
    }

    fn into_server_error_future<T>(self) -> FutureResult<T, ServerError> {
        future::err::<T, ServerError>(self.into())
    }

    pub fn server_method_header_not_found<T>() -> FutureResult<T, ServerError> {
        let e = Self::new(ReadErrorCode::ServerMethodHeaderNotFound, StatusCode::NotFound);
        e.into_server_error_future::<T>()
    }

    pub fn server_method_not_found<T>(method_name: &str) -> FutureResult<T, ServerError> {
        let e = Self::new(ReadErrorCode::ServerMethodNotFound(method_name.to_owned()), StatusCode::NotFound);
        e.into_server_error_future::<T>()
    }
}

impl Error for ReadError {
    fn description(&self) -> &str {
        match (*self).code {
            ReadErrorCode::ServerMethodHeaderNotFound => "Server method header not found",
            ReadErrorCode::ServerMethodNotFound(_) => "Server method not found",
        }
    }
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match (*self).code {
            ReadErrorCode::ServerMethodNotFound(ref name) => write!(f, "ReadError: {} \"{}\"", self, name),
            _ => write!(f, "ReadError: {}", self)
        }
    }
}