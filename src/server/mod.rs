use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;

use futures::Future;
use futures::Stream;
use futures::future;
use futures::future::FutureResult;

use hyper;
use hyper::{Headers, StatusCode, Method};
use hyper::server::{Service, Request, Response};
use hyper::header::{ContentEncoding, ContentType, Allow, Encoding, ContentLength};

use self::request_handler::RequestHandler;
use self::headers::ServerMethod;
use self::server_error::{ServerError, ReadError};

pub mod headers;
pub mod server_error;
pub mod request_handler;
pub mod json_request_handler;
pub mod protected_json_request_handler;

pub struct PlantsCareService {
    requests: HashMap<&'static str, Arc<dyn RequestHandler>>
}

impl PlantsCareService {
    pub fn new() -> PlantsCareService {
        PlantsCareService {
            requests : HashMap::new()
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn RequestHandler>) {
        let method = handler.method();
        self.requests.insert(method, handler);
    }

    fn get_handler(&self, method: &ServerMethod) -> Option<Arc<dyn RequestHandler>> {
        match self.requests.get(method.name()) {
            Some(h) => Some(h.clone()),
            None => None
        }
    }

    fn read_info(&self, request: Request) -> FutureResult<InputInfo, ServerError> {
        let request_handler : Arc<dyn RequestHandler> = {
            let headers : &Headers = request.headers();
            let method: &ServerMethod = match headers.get::<ServerMethod>() {
                Some(m) => m,
                None => return ReadError::server_method_header_not_found::<InputInfo>()
            };

            match self.get_handler(method) {
                Some(h) => h,
                None => return ReadError::server_method_not_found::<InputInfo>(method.name())
            }
        };

        let info = InputInfo {
            request,
            request_handler
        };

        future::ok::<InputInfo, ServerError>(info)
    }

    fn read_data(info: InputInfo) -> Box<dyn Future<Item=InputData, Error=ServerError>> {
        let request_handler = info.request_handler;
        let remote_addr = info.request.remote_addr();
        let f = info.request
            .body()
            .concat2()
            .and_then(move |b| {
                let data = InputData {
                    request_handler,
                    remote_addr,
                    str: match String::from_utf8(b.to_owned()) {
                        Ok(s) => s,
                        Err(e) => return future::err(e.into())
                    },
                };

                future::ok(data)
            })
            .map_err(|he| ServerError::from(he));

        Box::new(f)
    }

    fn process_data(data: InputData) -> FutureResult<Response, ServerError> {
        let output = match data.request_handler.process(&data) {
            Ok(o) => o,
            Err(e) => {
                error!("error on request process: {}", &e);

                if let Ok(output_err) = data.request_handler.on_error(e) {
                    return Self::error(Some(output_err), StatusCode::BadRequest);
                }
                return Self::error(None, StatusCode::InternalServerError);
            }
        };

        Self::success(output)
    }

    fn process_error(server_error: ServerError) -> FutureResult<Response, hyper::Error> {
        error!("error: {}", &server_error);
        match server_error {
            ServerError::Json(_) => Self::error(None, StatusCode::BadRequest),
            ServerError::Io(_) => Self::error(Some("Io error".to_owned()), StatusCode::InternalServerError),
            ServerError::Logic(_) => Self::error(Some("Logic error".to_owned()), StatusCode::InternalServerError),
            ServerError::Hyper(e) => future::err(e),
            ServerError::Camera(_) => Self::error(Some("Camera error".to_owned()), StatusCode::InternalServerError),
            ServerError::Rppal(_) => Self::error(Some("Rppal lib error".to_owned()), StatusCode::InternalServerError),
            ServerError::Read(_) => Self::error(None, StatusCode::BadRequest),
            ServerError::Posion => Self::error(Some("Poison error".to_owned()), StatusCode::InternalServerError),
        }
    }

    fn success(output: String) -> FutureResult<Response, ServerError> {
        let res = Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentEncoding(vec![Encoding::Identity]))
            .with_header(ContentType::json())
            .with_header(ContentLength(output.as_bytes().len() as u64))
            .with_body(output);

        future::ok::<Response, ServerError>(res)
    }

    fn error<TError: 'static>(output: Option<String>, status_code: StatusCode) -> FutureResult<Response, TError> {
        let mut res = Response::new()
            .with_status(status_code)
            .with_header(Allow(vec![Method::Post]))
            .with_header(ContentEncoding(vec![Encoding::Identity]))
            .with_header(ContentType::json());

        if let Some(o) = output {
            res.set_body(o)
        }

        future::ok::<Response, TError>(res)
    }
}

impl Service for PlantsCareService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match *req.method() {
            Method::Post => Box::new(self.read_info(req)
                .and_then(|info| Self::read_data(info))
                .and_then(|data| Self::process_data(data))
                .or_else(|se| Self::process_error(se))),
            _ => Box::new(Self::error(None, StatusCode::MethodNotAllowed))
        }
    }
}

struct InputInfo {
    request: Request,
    request_handler: Arc<dyn RequestHandler>
}

pub struct InputData {
    request_handler: Arc<dyn RequestHandler>,
    pub remote_addr: Option<SocketAddr>,
    pub str: String,
}