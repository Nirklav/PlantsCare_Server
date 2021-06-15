use rppal::*;
use rppal::pwm::{Channel, Pwm};
use crate::utils::rppal_error::RppalError;
use std::time::Duration;

pub const MINUS_90 : f64 = 0.05;
pub const ZERO : f64 = 0.075;
pub const PLUS_90 : f64 = 0.1;

pub struct Servo {
    pwm: Pwm
}

impl Servo {
    pub fn new() -> Result<Servo, RppalError> {
        let pwm = pwm::Pwm::new(Channel::Pwm0)?;
        pwm.set_period(Duration::from_millis(20))?;
        pwm.set_duty_cycle(ZERO)?;
        pwm.enable()?;

        Ok(Servo {
            pwm
        })
    }

    pub fn turn_next(&self, duty_cycle: f64) -> Result<(), RppalError> {
        self.pwm.set_duty_cycle(duty_cycle)?;
        Ok(())
    }
}