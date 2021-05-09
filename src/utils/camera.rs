use rascam::*;

use std::{thread, time};
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use crate::server::server_error::{ServerError, LogicError};

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
    info: CameraInfo
}

impl Camera {
    pub fn new() -> Result<Self, ServerError> {
        let mut info = rascam::info()?;

        if info.cameras.is_empty() {
            return Err(LogicError::CameraNotFound.into());
        }

        let first = info.cameras.remove(0);
        Ok(Camera {
            info: first
        })
    }

    pub fn make_photo(&self) -> Result<Vec<u8>, CameraError> {
        let mut camera = SimpleCamera::new(info.clone())?;
        camera.activate()?;

        let sleep_duration = time::Duration::from_millis(2000);
        thread::sleep(sleep_duration);

        let image = camera.take_one()?;
        Ok(Vec::from(&image))
    }
}

