use std::sync::Arc;
use crate::server::InputData;

use crate::server::request_handler::RequestHandler;
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::server::server_error::{ServerError};
use crate::utils::servo::Servo;

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
    angle: f32
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

pub struct TurnServoRequest {
    servo: Arc<Servo>
}

impl TurnServoRequest {
    pub fn new(key: &str, servo: &Arc<Servo>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, TurnServoRequest {
            servo: servo.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for TurnServoRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "turn-servo"
    }

    fn process(&self, input: Input, _: &InputData) -> Result<Output, ServerError> {
        self.servo.turn_to(input.angle)?;
        Ok(Output {
            result: "Ok".to_string()
        })
    }
}