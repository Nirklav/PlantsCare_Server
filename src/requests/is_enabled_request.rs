use std::sync::Arc;
use crate::server::InputData;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::Switches;

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
    name: String,
    ip: Option<String>,
    port: Option<u16>
}

#[derive(Serialize, Debug)]
pub struct Output {
    enabled: bool
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct IsEnabledRequest {
    switches: Arc<Switches>
}

impl IsEnabledRequest {
    pub fn new(key: &str, switches: &Arc<Switches>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, IsEnabledRequest {
            switches: switches.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for IsEnabledRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "is-enabled"
    }

    fn process(&self, input: Input, _: &InputData) -> Result<Output, ServerError> {
        Ok(Output {
            enabled: self.switches.is_enabled(&input.name, &input.ip, &input.port)?
        })
    }
}