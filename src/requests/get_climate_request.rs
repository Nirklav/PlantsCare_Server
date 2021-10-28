use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::services::climate::{Conditioner, Climate, Sensors};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    conditioners: Vec<Conditioner>,
    sensors: Sensors
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct GetClimateRequest {
    climate: Arc<Climate>
}

impl GetClimateRequest {
    pub fn new(key: &str, climate: &Arc<Climate>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, GetClimateRequest {
            climate: climate.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for GetClimateRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "get-climate"
    }

    fn process(&self, _: Input) -> Result<Output, ServerError> {
        let conditioners = self.climate.conditioners()?;
        let sensors = self.climate.sensors()?;
        Ok(Output {
            conditioners,
            sensors
        })
    }
}