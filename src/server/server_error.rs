use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;

use hyper;
use hyper::StatusCode;
use serde_json;
use rascam::CameraError;

use futures::future;
use futures::future::FutureResult;

#[derive(Debug)]
pub enum ServerError {
    Json(serde_json::error::Error),
    Logic(LogicError),
    Hyper(hyper::Error),
    Camera(CameraError),
    Read(ReadError)
}

impl From<serde_json::error::Error> for ServerError {
    fn from(e: serde_json::error::Error) -> Self {
        ServerError::Json(e)
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

impl From<ReadError> for ServerError {
    fn from(e: ReadError) -> Self {
        ServerError::Read(e)
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            ServerError::Json(ref e) => e.fmt(f),
            ServerError::Logic(ref e) => e.fmt(f),
            ServerError::Hyper(ref e) => e.fmt(f),
            ServerError::Camera(ref e) => e.fmt(f),
            ServerError::Read(ref e) => e.fmt(f)
        }
    }
}

#[derive(Debug)]
pub enum LogicError {
    InvalidProtectedKey = 1,
    CameraNotFound = 2,
}

impl Error for LogicError {
    fn description(&self) -> &str {
        match *self {
            LogicError::InvalidProtectedKey => "Invalid protected key",
            LogicError::CameraNotFound => "Camera was not found"
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