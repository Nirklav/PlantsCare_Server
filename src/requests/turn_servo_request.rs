use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::utils::servo::Servo;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug, Default)]
pub struct Input {
    key: String,
    angle: f32
}

#[derive(Serialize, Debug)]
pub struct Output {
    result: String
}

pub struct TurnServoRequest {
    servo: Arc<Servo>
}

impl TurnServoRequest {
    pub fn new(key: &str, servo: &Arc<Servo>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("turn-servo")
            .set_post(JsonMethodHandlerAdapter::new(TurnServoRequest {
                servo: servo.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for TurnServoRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, input: Input) -> Result<Output, ServerError> {
        self.servo.turn_to(input.angle)?;
        Ok(Output {
            result: "Ok".to_string()
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}