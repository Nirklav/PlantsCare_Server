use rppal::*;
use rppal::pwm::{Channel, Pwm};
use crate::utils::rppal_error::RppalError;
use std::time::Duration;

const DUTY_CYCLE_START : f64 = 0.03;
const DUTY_CYCLE_ZERO : f64 = 0.08;
const DUTY_CYCLE_LENGTH : f64 = 0.1;
const DEGREE_START : f32 = -90.0;
const DEGREE_END : f32 = 90.0;

pub struct Servo {
    pwm: Pwm
}

impl Servo {
    pub fn new() -> Result<Servo, RppalError> {
        let pwm = pwm::Pwm::new(Channel::Pwm0)?;
        pwm.set_period(Duration::from_millis(20))?;
        pwm.set_duty_cycle(DUTY_CYCLE_ZERO)?;
        pwm.enable()?;

        Ok(Servo {
            pwm
        })
    }

    pub fn turn_to(&self, angle: f32) -> Result<(), RppalError> {
        let corrected_angle = (angle.clamp(DEGREE_START, DEGREE_END) + 90.0) as f64;
        let duty_cycle = DUTY_CYCLE_START + corrected_angle * DUTY_CYCLE_LENGTH / 180.0;
        self.pwm.set_duty_cycle(duty_cycle)?;
        Ok(())
    }
}