use std::sync::Arc;
use base64;
use crate::server::InputData;

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
    pub fn new(key: &str, camera: &Arc<Camera>) -> Arc<dyn RequestHandler> {
        ProtectedJsonRequestHandlerAdapter::new(key, GetCameraImageRequest {
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

    fn process(&self, _: Input, _: &InputData) -> Result<Output, ServerError> {
        let photo = self.camera.make_photo()?;
        let photo_len = photo.len();

        info!("Encoding photo to base64...");
        let photo_encoded = base64::encode(photo);
        info!("Photo converted to base64 (before: {}, after: {})", photo_len, photo_encoded.len());

        info!("Sending photo...");
        Ok(Output {
            image_base64: photo_encoded
        })
    }
}