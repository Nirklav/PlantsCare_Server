use std::sync::Arc;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::Switches;

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String,
    name: String,
    value: bool
}

#[derive(Serialize, Debug)]
pub struct Output {
    created: bool
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct SetSwitchRequest {
    switches: Arc<Switches>
}

impl SetSwitchRequest {
    pub fn new(key: &str, switches: &Arc<Switches>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, SetSwitchRequest {
            switches: switches.clone()
        })
    }
}

impl ProtectedJsonRequestHandler for SetSwitchRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "set-switch"
    }

    fn process(&self, input: Input) -> Result<Output, ServerError> {
        let created = self.switches.set(&input.name, input.value)?;
        Ok(Output {
            created
        })
    }
}