use std::sync::Arc;
use base64;

use crate::server::request_handler::RequestHandler;
use crate::server::json_request_handler::{JsonRequestHandler, JsonRequestHandlerAdapter};
use crate::server::server_error::{ServerError, LogicError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter};
use crate::utils::camera::Camera;

#[derive(Deserialize, Debug)]
pub struct Input;

#[derive(Serialize, Debug)]
pub struct Output {
    image_base64: String
}

pub struct GetCameraImageRequest<'a> {
    camera: &'a Camera
}

impl GetCameraImageRequest {
    pub fn new(camera: &Camera) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new("hJasd123SDm1l_12!", GetCameraImageRequest {
            camera
        })
    }
}

impl ProtectedJsonRequestHandler for GetCameraImageRequest {
    type Input = Input;
    type Output = Output;

    fn method(&self) -> &'static str {
        "get-camera-image"
    }

    fn process(&self, _: Input) -> Result<Output, ServerError> {
        let photo = self.camera.make_photo()?;
        let photo_encoded = base64::encode(photo);

        Ok(Output {
            image_base64: photo_encoded
        })
    }
}