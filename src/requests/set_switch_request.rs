use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;
use crate::commands::command::Command;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::Switches;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

pub struct SwitchRequest;

impl SwitchRequest {
    pub fn new(key: &str, switches: &Arc<Switches>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("set-switch")
            .set_get(JsonMethodHandlerAdapter::new(GetSwitchMethod {
                switches: switches.clone()
            }, key.clone()))
            .set_post(JsonMethodHandlerAdapter::new(PostSwitchMethod {
                switches: switches.clone()
            }, key.clone())))
    }
}

#[derive(Deserialize, Debug)]
pub struct GetInput {
    name: String
}

#[derive(Serialize, Debug)]
pub struct GetOutput {
    enabled: bool
}

pub struct GetSwitchMethod {
    switches: Arc<Switches>
}

#[async_trait]
impl JsonMethodHandler for GetSwitchMethod {
    type Input = GetInput;
    type Output = GetOutput;

    async fn process(&self, _parts: Parts, input: GetInput) -> Result<GetOutput, ServerError> {
        Ok(GetOutput {
            enabled: self.switches.is_enabled(&input.name, &None, &None)?
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct PostInput {
    key: Option<String>,
    name: String,
    value: bool
}

#[derive(Serialize, Debug)]
pub struct PostOutput {
    created: bool
}

#[derive(Serialize, Debug)]
pub struct EnableCommandInput {
    enabled: bool
}

#[derive(Deserialize, Debug)]
pub struct EnableCommandOutput {

}

pub struct PostSwitchMethod {
    switches: Arc<Switches>
}

#[async_trait]
impl JsonMethodHandler for PostSwitchMethod {
    type Input = PostInput;
    type Output = PostOutput;

    async fn process(&self, _parts: Parts, input: PostInput) -> Result<PostOutput, ServerError> {
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

        Ok(PostOutput {
            created
        })
    }

    fn read_key<'a>(&self, input: &'a PostInput) -> Option<&'a str> {
        input.key.as_ref().map(|x| x.as_str())
    }
}