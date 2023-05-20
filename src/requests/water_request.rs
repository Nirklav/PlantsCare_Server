use std::sync::Arc;
use async_trait::async_trait;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::utils::water_sensor::WaterSensor;
use crate::utils::water_pump::WaterPump;
use std::time::Duration;
use hyper::http::request::Parts;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug, Default)]
pub struct Input {
    key: String,
    duration_seconds: u64,
    force: bool
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: bool,
    message: String
}

pub struct WaterRequest {
    water_sensor: Arc<WaterSensor>,
    water_pump: Arc<WaterPump>
}

impl WaterRequest {
    pub fn new(key: &str, water_sensor: &Arc<WaterSensor>, water_pump: &Arc<WaterPump>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("water")
            .set_post(JsonMethodHandlerAdapter::new(WaterRequest {
                water_sensor: water_sensor.clone(),
                water_pump: water_pump.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for WaterRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _: Parts, i: Input) -> Result<Output, ServerError> {
        info!("water request: duration {}s, force {}", &i.duration_seconds, &i.force);

        let is_enough_water = self.water_sensor.is_enough()?;
        if !i.force && !is_enough_water {
            return Ok(Output {
                result: false,
                message: "Not enough water".to_owned()
            });
        }

        let duration = Duration::from_secs(i.duration_seconds);
        self.water_pump.enable(duration)?;

        Ok(Output {
            result: true,
            message: "Plant was watered".to_owned()
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}