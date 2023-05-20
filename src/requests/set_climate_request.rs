use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::services::climate::{Conditioner, Climate};

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug, Default)]
pub struct Input {
    key: String,
    conditioners: Vec<Conditioner>
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: String
}

pub struct SetClimateRequest {
    climate: Arc<Climate>
}

impl SetClimateRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("set-climate")
            .set_post(JsonMethodHandlerAdapter::new(SetClimateRequest {
                climate: climate.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for SetClimateRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, input: Input) -> Result<Output, ServerError> {
        self.climate.set(&input.conditioners)?;
        Ok(Output {
            result: "Success".to_owned()
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}