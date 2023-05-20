#[cfg(target_os = "linux")]
use rppal::gpio;
#[cfg(target_os = "linux")]
use rppal::pwm;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RppalError {
    #[cfg(target_os = "linux")]
    #[error("Gpio error: {0}")]
    Gpio(#[from] gpio::Error),
    #[cfg(target_os = "linux")]
    #[error("Pwm error: {0}")]
    Pwm(#[from] pwm::Error)
}