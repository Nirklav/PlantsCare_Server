use std::sync::Arc;
use crate::server::InputData;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::services::climate::{WeatherSensor, Conditioner, Climate, Sensors};

#[derive(Deserialize, Debug)]
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

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct ConditionersRequest {
    climate: Arc<Climate>
}

impl ConditionersRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, ConditionersRequest {
            climate: climate.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for ConditionersRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "conditioners"
    }

    fn process(&self, input: Input, _: &InputData) -> Result<Output, ServerError> {
        let sensors = Sensors::new(input.sensors, input.sensor_temp, input.bedroom_temp, input.living_temp);
        let conditioners = self.climate.calculate(sensors)?;
        Ok(Output {
            conditioners
        })
    }
}