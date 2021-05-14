use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::utils::water_sensor::WaterSensor;

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: bool
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct IsEnoughWaterRequest {
    water_sensor: Arc<WaterSensor>
}

impl IsEnoughWaterRequest {
    pub fn new(key: &str, water_sensor: &Arc<WaterSensor>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, IsEnoughWaterRequest {
            water_sensor: water_sensor.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for IsEnoughWaterRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "is-enough-water"
    }

    fn process(&self, _: Input) -> Result<Output, ServerError> {
        Ok(Output {
            result: self.water_sensor.is_enough()?
        })
    }
}