use std::sync::Arc;
use base64;

use crate::server::request_handler::RequestHandler;
use crate::server::server_error::{ServerError};
use crate::server::protected_json_request_handler::{ProtectedJsonRequestHandler, ProtectedJsonRequestHandlerAdapter, ProtectedInput};
use crate::utils::camera::Camera;

#[derive(Deserialize, Debug)]
pub struct Input {
    key: String
}

#[derive(Serialize, Debug)]
pub struct Output {
    image_base64: String
}

impl ProtectedInput for Input {
    fn get_protected_key(&self) -> &str {
        &self.key
    }
}

pub struct GetCameraImageRequest {
    camera: Arc<Camera>
}

impl GetCameraImageRequest {
    pub fn new(camera: &Arc<Camera>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new("hJasd123SDm1l_12!", GetCameraImageRequest {
            camera: camera.clone()
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

        info!("encoding photo to base64...");
        let photo_encoded = base64::encode(photo);

        info!("sending photo...");
        Ok(Output {
            image_base64: photo_encoded
        })
    }
}