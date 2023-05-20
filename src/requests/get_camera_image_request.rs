use std::sync::Arc;
use async_trait::async_trait;
use base64::engine::{Engine, general_purpose};
use hyper::http::request::Parts;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::utils::camera::Camera;

use serde::{Deserialize, Serialize};
use crate::server::json_request_handler::{JsonMethodHandler, JsonMethodHandlerAdapter};

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    image_base64: String
}

pub struct GetCameraImageRequest {
    camera: Arc<Camera>
}

impl GetCameraImageRequest {
    pub fn new(key: &str, camera: &Arc<Camera>) -> Arc<RequestHandler> {
        let key = Some(key.to_string());
        Arc::new(RequestHandler::new("get-camera-image")
            .set_post(JsonMethodHandlerAdapter::new(GetCameraImageRequest {
                camera: camera.clone()
            }, key)))
    }
}

#[async_trait]
impl JsonMethodHandler for GetCameraImageRequest {
    type Input = Input;
    type Output = Output;

    async fn process(&self, _: Parts, _: Input) -> Result<Output, ServerError> {
        let photo = self.camera.make_photo()?;
        let photo_len = photo.len();

        info!("Encoding photo to base64...");
        let photo_encoded = general_purpose::STANDARD.encode(photo);
        info!("Photo converted to base64 (before: {}, after: {})", photo_len, photo_encoded.len());

        info!("Sending photo...");
        Ok(Output {
            image_base64: photo_encoded
        })
    }

    fn read_key<'a>(&self, input: &'a Input) -> Option<&'a str> {
        Some(&input.key)
    }
}