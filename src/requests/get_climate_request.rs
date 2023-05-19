use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::services::climate::{Conditioner, Climate, Sensors};

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    conditioners: Vec<Conditioner>,
    sensors: Sensors
}

pub struct GetClimateRequest {
    climate: Arc<Climate>
}

impl GetClimateRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("get-climate")
            .set_post(JsonMethodHandlerAdapter::new(GetClimateRequest {
                climate: climate.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for GetClimateRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, _input: Input) -> Result<Output, ServerError> {
        let conditioners = self.climate.conditioners()?;
        let sensors = self.climate.sensors()?;
        Ok(Output {
            conditioners,
            sensors
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}