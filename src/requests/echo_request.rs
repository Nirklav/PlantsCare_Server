use std::sync::Arc;
use crate::server::InputData;

use crate::server::request_handler::RequestHandler;
use crate::server::json_request_handler::{JsonRequestHandler, JsonRequestHandlerAdapter};
use crate::server::server_error::{ServerError, LogicError};

#[derive(Deserialize, Debug)]
pub struct Input {
    str: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    str: String
}

pub struct EchoRequest;

impl EchoRequest {
    pub fn new() -> Arc<dyn RequestHandler> {
        JsonRequestHandlerAdapter::new(EchoRequest)
    }
}

impl JsonRequestHandler for EchoRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "echo"
    }

    fn process(&self, input: Input, _: &InputData) -> Result<Output, ServerError> {
        if input.str.eq("logic error test") {
            return Err(LogicError::InvalidProtectedKey.into());
        }

        Ok(Output {
            str: input.str
        })
    }
}