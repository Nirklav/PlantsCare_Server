use std::fmt::Debug;
use async_trait::async_trait;
use hyper::http::request::Parts;
use hyper::{Body, Response, StatusCode};
use hyper::body::Bytes;

use serde_json;
use serde::{Serialize, de::DeserializeOwned};

use crate::server::request_handler::MethodHandler;
use crate::server::server_error::{LogicError, ServerError};

#[async_trait]
pub trait JsonMethodHandler : Sync + Send {
    type Input : DeserializeOwned + Debug + Send;
    type Output : Serialize + Debug;

    async fn process(&self, parts: Parts, input: Self::Input) -> Result<Self::Output, ServerError>;

    fn read_key<'a>(&self, _input: &'a Self::Input) -> Option<&'a str> {
        None
    }
}

pub struct JsonMethodHandlerAdapter<H: JsonMethodHandler> {
    inner: H,
    key: Option<String>
}

impl<H: 'static + JsonMethodHandler> JsonMethodHandlerAdapter<H> {
    pub fn new(inner: H, key: Option<String>) -> Self {
        JsonMethodHandlerAdapter {
            inner,
            key
        }
    }
}

impl<H: JsonMethodHandler> JsonMethodHandlerAdapter<H> {
    pub fn check_key(&self, parts: &Parts, input: &H::Input) -> Result<(), ServerError> {
        if let Some(key) = &self.key {

            let req_key = if let Some(k) = H::read_key(&self.inner, &input) {
                Some(k)
            } else {
                if let Some(h) = parts.headers.get("Protected-Key") {
                    Some(h.to_str()?)
                } else {
                    None
                }
            };

            match req_key {
                Some(rk) => {
                    if rk != key {
                        return Err(LogicError::InvalidProtectedKey.into())
                    }
                }
                None => return Err(LogicError::InvalidProtectedKey.into())
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<H: JsonMethodHandler> MethodHandler for JsonMethodHandlerAdapter<H> {
    async fn process(&self, parts: Parts, data: Bytes) -> Result<Response<Body>, ServerError> {
        let input : H::Input = serde_json::from_slice(&data)?;

        self.check_key(&parts, &input)?;

        let output = self.inner.process(parts, input).await?;
        let output = serde_json::to_vec(&output)?;

        let resp = Response::builder()
            .status(StatusCode::OK)
            .header(hyper::http::header::CONTENT_TYPE, "application/json")
            .header(hyper::http::header::CONTENT_LENGTH, output.len())
            .body(Body::from(output))?;

        Ok(resp)
    }
}