use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::utils::water_sensor::WaterSensor;
use crate::utils::water_pump::WaterPump;
use std::time::Duration;

#[derive(Deserialize, Debug)]
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

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct WaterRequest {
    water_sensor: Arc<WaterSensor>,
    water_pump: Arc<WaterPump>
}

impl WaterRequest {
    pub fn new(key: &str, water_sensor: &Arc<WaterSensor>, water_pump: &Arc<WaterPump>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, WaterRequest {
            water_sensor: water_sensor.clone(),
            water_pump: water_pump.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for WaterRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "water"
    }

    fn process(&self, i: Input) -> Result<Output, ServerError> {
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
}