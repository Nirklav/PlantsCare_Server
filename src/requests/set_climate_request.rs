use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::services::climate::{WeatherSensor, Conditioner, Climate, Sensors};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
    conditioners: Vec<Conditioner>
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: String
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct SetClimateRequest {
    climate: Arc<Climate>
}

impl SetClimateRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, SetClimateRequest {
            climate: climate.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for SetClimateRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "set-climate"
    }

    fn process(&self, input: Input) -> Result<Output, ServerError> {;
        self.climate.set(&input.conditioners);
        Ok(Output {
            result: "Success".to_owned()
        })
    }
}