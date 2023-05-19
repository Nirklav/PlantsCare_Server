use async_trait::async_trait;
use hyper::body::Bytes;
use hyper::http::request::Parts;
use hyper::{Body, Method, Response, StatusCode};
use crate::server::server_error::ServerError;

pub struct RequestHandler {
    path: &'static str,
    get: Option<Box<dyn MethodHandler>>,
    post: Option<Box<dyn MethodHandler>>,
    put: Option<Box<dyn MethodHandler>>,
    delete: Option<Box<dyn MethodHandler>>
}

impl RequestHandler {
    pub fn new(path: &'static str) -> Self {
        RequestHandler {
            path,
            get: None,
            post: None,
            put: None,
            delete: None,
        }
    }

    pub fn set_get<T: MethodHandler + 'static>(mut self, handler: T) -> Self {
        self.get = Some(Box::new(handler));
        self
    }

    pub fn set_post<T: MethodHandler + 'static>(mut self, handler: T) -> Self {
        self.post = Some(Box::new(handler));
        self
    }

    pub fn set_put<T: MethodHandler + 'static>(mut self, handler: T) -> Self {
        self.put = Some(Box::new(handler));
        self
    }

    pub fn set_delete<T: MethodHandler + 'static>(mut self, handler: T) -> Self {
        self.delete = Some(Box::new(handler));
        self
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub async fn process(&self, parts: Parts, data: Bytes) -> Result<Response<Body>, ServerError> {
        let handler = match parts.method {
            Method::GET => &self.get,
            Method::POST => &self.post,
            Method::PUT => &self.put,
            Method::DELETE => &self.delete,
            _ => &None
        };

        let handler = match handler {
            Some(h) => h,
            None => return not_found()
        };

        handler.process(parts, data).await
    }
}

#[async_trait]
pub trait MethodHandler: Sync + Send {
    async fn process(&self, _parts: Parts, _data: Bytes) -> Result<Response<Body>, ServerError>;
}

fn not_found() -> Result<Response<Body>, ServerError> {
    let r = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not found"));
    Ok(super::unwrap(r))
}

