#[cfg(target_os = "linux")]
use rascam::*;
#[cfg(target_os = "linux")]
use std::thread;
#[cfg(target_os = "linux")]
use std::time::Duration;

use thiserror::Error;

pub struct Camera {
    #[cfg(target_os="linux")]
    info: CameraInfo
}

impl Camera {
    #[cfg(not(target_os = "linux"))]
    pub fn new() -> Result<Self, CameraError> {
        Ok(Camera {
        })
    }

    #[cfg(not(target_os = "linux"))]
    pub fn make_photo(&self) -> Result<Vec<u8>, CameraError> {
        Ok(Vec::new())
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Self, CameraError> {
        let mut info = rascam::info()?;

        if info.cameras.is_empty() {
            return Err(CameraError::NotFound);
        }

        let first = info.cameras.remove(0);
        Ok(Camera {
            info: first
        })
    }

    #[cfg(target_os = "linux")]
    pub fn make_photo(&self) -> Result<Vec<u8>, CameraError> {
        let mut camera = SimpleCamera::new(self.info.clone())?;
        camera.activate()?;

        let sleep_duration = Duration::from_millis(2000);
        thread::sleep(sleep_duration);

        let image = camera.take_one()?;

        info!("copying photo to own memory...");
        Ok(Vec::from(image.as_slice()))
    }
}

#[derive(Error, Debug)]
pub enum CameraError {
    #[cfg(target_os = "linux")]
    Rascam(#[from] rascam::CameraError),
    #[cfg(target_os = "linux")]
    NotFound
}
