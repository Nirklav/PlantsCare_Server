use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::json_request_handler::{JsonRequestHandler, JsonRequestHandlerAdapter};
use crate::server::server_error::{ServerError};
use crate::utils::servo::Servo;

#[derive(Deserialize, Debug)]
pub struct Input {
    duty_cycle: f64
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: String
}

pub struct TurnServoRequest {
    servo: Arc<Servo>
}

impl TurnServoRequest {
    pub fn new(servo: &Arc<Servo>) -> Arc<dyn RequestHandler> {
        JsonRequestHandlerAdapter::new(TurnServoRequest {
            servo: servo.clone()
        })
    }
}

impl JsonRequestHandler for TurnServoRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "turn-servo"
    }

    fn process(&self, input: Input) -> Result<Output, ServerError> {
        self.servo.turn_next(input.duty_cycle)?;
        Ok(Output {
            result: "Ok".to_string()
        })
    }
}