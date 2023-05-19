use std::io;
use std::sync::PoisonError;
use thiserror::Error;

use hyper;
use hyper::header::ToStrError;
use serde_json;

use crate::utils::camera::CameraError;
use crate::utils::rppal_error::RppalError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Json error: {0}")]
    Json(#[from] serde_json::error::Error),
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Logic error: {0}")]
    Logic(#[from] LogicError),
    #[error("Hyper server error: {0}")]
    Hyper(#[from] hyper::http::Error),
    #[error("Camera error: {0}")]
    Camera(#[from] CameraError),
    #[error("Rppal error: {0}")]
    Rppal(#[from] RppalError),
    #[error("To string error: {0}")]
    ToStr(#[from] ToStrError),
    #[error("Mutex is poison")]
    Poison
}

#[derive(Error, Debug)]
pub enum LogicError {
    #[error("Invalid protected key")]
    InvalidProtectedKey = 1,
    #[error("Camera was not found")]
    CameraNotFound = 2,
    #[error("Command method is not set")]
    CommandMethodIdNotSet = 3,
    #[error("Command input is not set")]
    CommandInputNotSet = 4,
    #[error("Command has unsupported content type")]
    CommandUnsupportedContentType = 5,
    #[error("Socket address not found")]
    CommandSocketAddressNotFound = 6
}

impl<T> From<PoisonError<T>> for ServerError {
    fn from(_: PoisonError<T>) -> Self {
        ServerError::Poison
    }
}