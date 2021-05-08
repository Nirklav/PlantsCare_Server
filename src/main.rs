extern crate sysfs_gpio;

use sysfs_gpio::*;
use std::thread;
use std::time::Duration;

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
}
