use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::utils::water_sensor::WaterSensor;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: bool
}

pub struct IsEnoughWaterRequest {
    water_sensor: Arc<WaterSensor>
}

impl IsEnoughWaterRequest {
    pub fn new(key: &str, water_sensor: &Arc<WaterSensor>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("is-enough-water")
            .set_post(JsonMethodHandlerAdapter::new(IsEnoughWaterRequest {
                water_sensor: water_sensor.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for IsEnoughWaterRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _: Parts, _: Input) -> Result<Output, ServerError> {
        Ok(Output {
            result: self.water_sensor.is_enough()?
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}