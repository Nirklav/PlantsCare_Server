use std::sync::Arc;
use std::fmt::Debug;

use serde_json;
use serde::{Serialize};
use serde::de::{DeserializeOwned};

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError, LogicError};

pub trait JsonRequestHandler: Sync + Send {
    type Input : DeserializeOwned + Debug;
    type Output : Serialize + Debug;

    fn method(&self) -> &'static str;
    fn process(&self, input: Self::Input) -> Result<Self::Output, ServerError>;
}

pub struct JsonRequestHandlerAdapter<H: JsonRequestHandler> {
    inner: H
}

impl<H: 'static + JsonRequestHandler> JsonRequestHandlerAdapter<H> {
    pub fn new(inner: H) -> Arc<dyn RequestHandler> {
        Arc::new(JsonRequestHandlerAdapter {
            inner
        })
    }
}

impl<H: JsonRequestHandler> RequestHandler for JsonRequestHandlerAdapter<H> {
    fn method(&self) -> &'static str {
        self.inner.method()
    }

    fn process(&self, input_json: String) -> Result<String, ServerError> {
        let input : H::Input = serde_json::from_str(&input_json)?;
        let output = self.inner.process(input)?;
        let output_json = serde_json::to_string(&output)?;
        Ok(output_json)
    }

    fn on_error(&self, e: ServerError) -> Result<String, ServerError> {
        if let ServerError::Logic(se) = e {
            let output_json = serde_json::to_string(&ErrorOutput::new(se))?;
            return Ok(output_json);
        }

        Err(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorOutput {
    code: i32
}

impl ErrorOutput {
    fn new(error: LogicError) -> Self {
        ErrorOutput {
            code: error as i32
        }
    }
}