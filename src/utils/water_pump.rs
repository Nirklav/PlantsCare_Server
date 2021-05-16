#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;
#[cfg(target_os = "linux")]
use std::thread;
use std::time::Duration;

use crate::utils::rppal_error::RppalError;

#[cfg(target_os = "linux")]
const WATER_PUMP_POWER_PIN : u8 = 5;

pub struct WaterPump {
    #[cfg(target_os = "linux")]
    gpio: Gpio
}

impl WaterPump {
    #[cfg(target_os = "windows")]
    pub fn new() -> Result<Self, RppalError> {
        Ok(WaterPump {
        })
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Self, RppalError> {
        let gpio = Gpio::new()?;

        let mut power_pin = gpio.get(WATER_PUMP_POWER_PIN)?
            .into_output();

        power_pin.set_low();

        Ok(WaterPump {
            gpio
        })
    }

    #[cfg(target_os = "windows")]
    pub fn enable(&self, _: Duration) -> Result<(), RppalError> {
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn enable(&self, time: Duration) -> Result<(), RppalError> {
        let mut power_pin = self.gpio.get(WATER_PUMP_POWER_PIN)?
            .into_output();

        power_pin.set_high();
        thread::sleep(time);
        power_pin.set_low();

        Ok(())
    }
}