#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;
#[cfg(target_os = "linux")]
use std::thread;
#[cfg(target_os = "linux")]
use std::time::Duration;

use crate::utils::rppal_error::RppalError;

pub struct WaterSensor {
    #[cfg(target_os = "linux")]
    gpio: Gpio
}

#[cfg(target_os = "linux")]
const WATER_SENSOR_POWER_PIN : u8 = 17;

#[cfg(target_os = "linux")]
const WATER_SENSOR_IN : u8 = 27;

impl WaterSensor {
    #[cfg(target_os = "windows")]
    pub fn new() -> Result<Self, RppalError> {
        Ok(WaterSensor {
        })
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Self, RppalError>{
        let gpio = Gpio::new()?;

        let mut power_pin = gpio.get(WATER_SENSOR_POWER_PIN)?
            .into_output();

        power_pin.set_low();

        Ok(WaterSensor {
            gpio
        })
    }

    #[cfg(target_os = "windows")]
    pub fn is_enough(&self) -> Result<bool, RppalError> {
        Ok(false)
    }

    #[cfg(target_os = "linux")]
    pub fn is_enough(&self) -> Result<bool, RppalError> {
        let mut power_pin = self.gpio.get(WATER_SENSOR_POWER_PIN)?
            .into_output();

        let mut in_pin = self.gpio.get(WATER_SENSOR_IN)?
            .into_input();

        power_pin.set_high();
        thread::sleep(Duration::from_millis(1000));

        let in_pin_high = in_pin.is_high();

        power_pin.set_low();
        Ok(in_pin_high)
    }
}