use rppal::*;
use rppal::pwm::{Channel, Pwm};
use crate::utils::rppal_error::RppalError;
use std::time::Duration;

pub const MINUS_90 : f64 = 5.0;
pub const ZERO : f64 = 7.5;
pub const PLUS_90 : f64 = 10.0;

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

    pub fn turn_next(&self) -> Result<(), RppalError> {
        let duty_cycle = self.pwm.duty_cycle()?;
        info!("duty_cycle: {}", &duty_cycle);

        if (duty_cycle - MINUS_90).abs() < 0.01 {
            info!("turn_next: ZERO");
            self.pwm.set_duty_cycle(ZERO)?;
        }
        if (duty_cycle - ZERO).abs() < 0.01 {
            info!("turn_next: PLUS_90");
            self.pwm.set_duty_cycle(PLUS_90)?;
        }
        if (duty_cycle - PLUS_90).abs() < 0.01 {
            info!("turn_next: MINUS_90");
            self.pwm.set_duty_cycle(MINUS_90)?;
        }

        Ok(())
    }
}