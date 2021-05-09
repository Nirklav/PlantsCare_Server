#[cfg(target_os = "linux")]
use rascam::*;

use std::{thread, time, fmt};
use std::time::Duration;
use std::fmt::{Display, Formatter};

/*
use sysfs_gpio::*;

fn main() {
    let pin = Pin::new(2);
    pin.with_exported(|| {
        pin.set_direction(Direction::Out).unwrap();
        loop {
            pin.set_value(0).unwrap();
            thread::sleep(Duration::from_millis(200));
            pin.set_value(1).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    }).unwrap();
}*/

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

        writeln!("copying photo to own memory...");
        Ok(Vec::from(image.as_slice()))
    }
}

#[derive(Debug)]
pub enum CameraError {
    #[cfg(target_os = "linux")]
    Rascam(rascam::CameraError),
    NotFound
}

impl Display for CameraError {
    #[cfg(not(target_os = "linux"))]
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            CameraError::Rascam(ref e) => e.fmt(f),
            NotFound => write!(f, "Camera not found")
        }
    }
}

#[cfg(target_os = "linux")]
impl From<rascam::CameraError> for CameraError {
    fn from(e: rascam::CameraError) -> Self {
        CameraError::Rascam(e)
    }
}
