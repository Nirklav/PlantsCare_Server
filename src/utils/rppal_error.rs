#[cfg(target_os = "linux")]
use rppal::gpio;
#[cfg(target_os = "linux")]
use rppal::pwm;
#[cfg(target_os = "linux")]
use std::io;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum RppalError {
    Gpio(Box<dyn Error>),
    Pwm(Box<dyn Error>)
}

impl Display for RppalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RppalError::Gpio(ref e) => write!(f, "Gpio error: {}", e),
            RppalError::Pwm(ref e) => write!(f, "Pwm error: {}", e)
        }
    }
}

#[cfg(target_os = "linux")]
impl From<gpio::Error> for RppalError {
    fn from(e: gpio::Error) -> Self {
        RppalError::Gpio(Box::new(e))
    }
}

#[cfg(target_os = "linux")]
impl From<pwm::Error> for RppalError {
    fn from(e: pwm::Error) -> Self {
        RppalError::Pwm(Box::new(e))
    }
}