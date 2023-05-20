use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError, LogicError};

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug, Default)]
pub struct Input {
    str: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    str: String
}

pub struct EchoRequest;

impl EchoRequest {
    pub fn new() -> Arc<RequestHandler> {
        Arc::new(RequestHandler::new("echo")
            .set_post(JsonMethodHandlerAdapter::new(EchoRequest, None)))
    }
}

#[async_trait]
impl JsonMethodHandler for EchoRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, input: Input) -> Result<Self::Output, ServerError> {
        if input.str.eq("logic error test") {
            return Err(LogicError::InvalidProtectedKey.into());
        }

        Ok(Output {
            str: input.str
        })
    }
}