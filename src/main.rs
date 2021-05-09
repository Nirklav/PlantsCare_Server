use sysfs_gpio::*;
use rascam::*;

use std::{thread, time};
use std::time::Duration;
use std::fs::File;
use std::io::Write;

fn main() {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
    }
    else {
        println!("{}", info);
        simple_sync(&info.cameras[0]);
    }

    /*let pin = Pin::new(2);
    pin.with_exported(|| {
        pin.set_direction(Direction::Out).unwrap();
        loop {
            pin.set_value(0).unwrap();
            thread::sleep(Duration::from_millis(200));
            pin.set_value(1).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    }).unwrap();*/
}

fn simple_sync(info: &CameraInfo) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let b = camera.take_one().unwrap();
    File::create("image.jpg").unwrap().write_all(&b).unwrap();

    println!("Saved image as image.jpg");
}
