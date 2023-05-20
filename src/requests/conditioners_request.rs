use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::services::climate::{WeatherSensor, Conditioner, Climate, Sensors};

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug, Default)]
pub struct Input {
    key: String,
    sensors: Vec<WeatherSensor>,
    sensor_temp: f32,
    bedroom_temp: f32,
    living_temp: f32
}

#[derive(Serialize, Debug)]
pub struct Output {
    conditioners: Vec<Conditioner>
}

pub struct ConditionersRequest {
    climate: Arc<Climate>
}

impl ConditionersRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("conditioners")
            .set_post(JsonMethodHandlerAdapter::new(ConditionersRequest {
                climate: climate.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for ConditionersRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, input: Input) -> Result<Output, ServerError> {
        let sensors = Sensors::new(input.sensors, input.sensor_temp, input.bedroom_temp, input.living_temp);
        let conditioners = self.climate.calculate(sensors)?;
        Ok(Output {
            conditioners
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}