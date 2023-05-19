use std::sync::Arc;
use async_trait::async_trait;
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::Switches;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: Option<String>,
    name: String,
    ip: Option<String>,
    port: Option<u16>
}

#[derive(Serialize, Debug)]
pub struct Output {
    enabled: bool
}

pub struct IsEnabledRequest {
    switches: Arc<Switches>
}

impl IsEnabledRequest {
    pub fn new(key: &str, switches: &Arc<Switches>) -> Arc<RequestHandler> {
        Arc::new(RequestHandler::new("is-enabled")
            .set_get(Self::adapter(key, switches))
            .set_post(Self::adapter(key, switches)))
    }

    fn adapter(key: &str, switches: &Arc<Switches>) -> JsonMethodHandlerAdapter<IsEnabledRequest> {
        let key = Some(key.to_string());
        let request = IsEnabledRequest {
            switches: switches.clone()
        };
        JsonMethodHandlerAdapter::new(request, key)
    }
}

#[async_trait]
impl JsonMethodHandler for IsEnabledRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _parts: Parts, input: Input) -> Result<Output, ServerError> {
        Ok(Output {
            enabled: self.switches.is_enabled(&input.name, &input.ip, &input.port)?
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        input.key.as_ref().map(|x| x.as_str())
    }
}