#[cfg(target_os = "linux")]
use rppal::*;
#[cfg(target_os = "linux")]
use rppal::pwm::{Channel, Pwm};
use crate::utils::rppal_error::RppalError;
#[cfg(target_os = "linux")]
use std::time::Duration;

#[cfg(target_os = "linux")]
const DUTY_CYCLE_START : f64 = 0.03;
#[cfg(target_os = "linux")]
const DUTY_CYCLE_ZERO : f64 = 0.08;
#[cfg(target_os = "linux")]
const DUTY_CYCLE_LENGTH : f64 = 0.1;
#[cfg(target_os = "linux")]
const DEGREE_START : f32 = -90.0;
#[cfg(target_os = "linux")]
const DEGREE_END : f32 = 90.0;

pub struct Servo {
    #[cfg(target_os = "linux")]
    pwm: Pwm
}

impl Servo {
    #[cfg(target_os = "windows")]
    pub fn new() -> Result<Servo, RppalError> {
        Ok(Servo {
        })
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Servo, RppalError> {
        let pwm = pwm::Pwm::new(Channel::Pwm0)?;
        pwm.set_period(Duration::from_millis(20))?;
        pwm.set_duty_cycle(DUTY_CYCLE_ZERO)?;
        pwm.enable()?;

        Ok(Servo {
            pwm
        })
    }

    #[cfg(target_os = "windows")]
    pub fn turn_to(&self, _: f32) -> Result<(), RppalError> {
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn turn_to(&self, angle: f32) -> Result<(), RppalError> {
        let corrected_angle = (angle.clamp(DEGREE_START, DEGREE_END) + 90.0) as f64;
        let duty_cycle = DUTY_CYCLE_START + corrected_angle * DUTY_CYCLE_LENGTH / 180.0;
        self.pwm.set_duty_cycle(duty_cycle)?;
        Ok(())
    }
}