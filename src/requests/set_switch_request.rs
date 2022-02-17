use std::sync::Arc;
use crate::commands::command::Command;
use crate::server::InputData;

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

#[derive(Serialize, Debug)]
pub struct EnableCommandInput {
    enabled: bool
}

#[derive(Deserialize, Debug)]
pub struct EnableCommandOutput {

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

    fn process(&self, input: Input, _: &InputData) -> Result<Output, ServerError> {
        let (created, ip, port) = self.switches.set(&input.name, input.value)?;

        if let Some(ip) = ip {
            if let Some(port) = port {
                let r = Command::new((ip, port))?
                    .method_id(0)
                    .input(EnableCommandInput { enabled: input.value })?
                    .execute::<EnableCommandOutput>();

                if let Err(e) = r {
                    error!("error on switch command: {}", &e)
                }
            }
        }

        Ok(Output {
            created
        })
    }
}