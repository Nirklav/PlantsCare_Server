use std::sync::Arc;
use std::fmt::Debug;

use serde::{Serialize};
use serde::de::{DeserializeOwned};
use crate::server::InputData;

use crate::server::server_error::{ServerError, LogicError};
use crate::server::request_handler::RequestHandler;
use crate::server::json_request_handler::{JsonRequestHandlerAdapter, JsonRequestHandler};

pub trait ProtectedJsonRequestHandler: Send + Sync {
    type Input : DeserializeOwned + ProtectedInput + Debug;
    type Output : Serialize + Debug;

    fn method(&self) -> &'static str;
    fn process(&self, input: Self::Input, input_data: &InputData) -> Result<Self::Output, ServerError>;
}

pub trait ProtectedInput {
    fn get_protected_key(&self) -> &str;
}

pub struct ProtectedJsonRequestHandlerAdapter<H: ProtectedJsonRequestHandler> {
    protected_key: String,
    inner: H
}

impl<H: 'static + ProtectedJsonRequestHandler> ProtectedJsonRequestHandlerAdapter<H> {
    pub fn new(protected_key: &str, inner: H) -> Arc<dyn RequestHandler> {
        JsonRequestHandlerAdapter::new(ProtectedJsonRequestHandlerAdapter {
            protected_key: protected_key.to_owned(),
            inner
        })
    }
}

impl<H: ProtectedJsonRequestHandler> JsonRequestHandler for ProtectedJsonRequestHandlerAdapter<H> {
    type Input = H::Input;
    type Output = H::Output;

    fn method(&self) -> &'static str {
        self.inner.method()
    }

    fn process(&self, input: Self::Input, input_data: &InputData) -> Result<Self::Output, ServerError> {
        if self.protected_key.ne(input.get_protected_key()) {
            return Err(LogicError::InvalidProtectedKey.into());
        }

        self.inner.process(input, input_data)
    }
}

