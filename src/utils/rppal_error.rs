#[cfg(target_os = "linux")]
use rppal::gpio;
#[cfg(target_os = "linux")]
use rppal::pwm;
#[cfg(target_os = "linux")]
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RppalError {
    #[cfg(target_os = "linux")]
    Gpio(#[from] gpio::Error),
    #[cfg(target_os = "linux")]
    Pwm(#[from] pwm::Error)
}