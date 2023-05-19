use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;

use hyper::{StatusCode, Request, Response, Body};

use crate::server::error_output::ErrorOutput;

use self::request_handler::RequestHandler;
use self::server_error::ServerError;

pub mod server_error;
pub mod request_handler;
pub mod json_request_handler;
pub mod error_output;

pub struct RpiHomeContext {
    requests: HashMap<&'static str, Arc<RequestHandler>>
}

impl RpiHomeContext {
    pub fn new() -> RpiHomeContext {
        RpiHomeContext {
            requests : HashMap::new()
        }
    }

    pub fn add_handler(&mut self, handler: Arc<RequestHandler>) {
        let path = handler.path();
        self.requests.insert(path, handler);
    }

    pub async fn handle(context: Arc<RpiHomeContext>, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let (parts, body) = req.into_parts();

        let path = if let Some(header) = parts.headers.get("Server-Method") {
            if let Ok(h) = header.to_str() {
                h.to_string()
            } else {
                return Self::error_message("Not found", StatusCode::NOT_FOUND);
            }
        } else {
            parts.uri.path()[1..].to_string()
        };

        let data = match hyper::body::to_bytes(body).await {
            Ok(d) => d,
            Err(_) => return Self::error_message("Cannon read", StatusCode::BAD_REQUEST)
        };

        let handler = {
            match context.requests.get(path.as_str()) {
                Some(r) => r.clone(),
                None => return Self::error_message("Not found", StatusCode::NOT_FOUND)
            }
        };

        let result = handler.process(parts, data).await;

        Ok(match result {
            Ok(r) => r,
            Err(ServerError::Logic(le)) => {
                let output = ErrorOutput::new(le);
                let output = match serde_json::to_vec(&output) {
                    Ok(o) => o,
                    Err(e) => {
                        error!("error on error serialization: {}", &e);
                        return Self::error_message(e, StatusCode::INTERNAL_SERVER_ERROR)
                    }
                };

                return Self::error(Body::from(output), StatusCode::BAD_REQUEST)
            }
            Err(e) => {
                error!("error on request process: {}", &e);
                return Self::error_message(e, StatusCode::BAD_REQUEST)
            }
        })
    }

    fn error_message<T: ToString>(message: T, code: StatusCode) -> Result<Response<Body>, Infallible> {
        let body = Body::from(message.to_string());
        Self::error(body, code)
    }

    fn error(body: Body, code: StatusCode) -> Result<Response<Body>, Infallible> {
        let r = Response::builder()
            .status(code)
            .body(body);

        Ok(unwrap(r))
    }
}

pub fn unwrap(r: Result<Response<Body>, hyper::http::Error>) -> Response<Body> {
    match r {
        Ok(r) => r,
        Err(e) => {
            let response = Response::new(Body::from("Internal server error"));
            error!("Internal server error: {}", &e);
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::INTERNAL_SERVER_ERROR;
            Response::from_parts(parts, body)
        }
    }
}

