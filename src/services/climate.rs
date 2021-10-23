use std::sync::Mutex;
use crate::server::server_error::ServerError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conditioner {
    enabled: bool,
    temperature: i32,
    mode: ConditionerMode
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ConditionerMode {
    Auto = 0,
    Cool = 1,
    Dry  = 2,
    Fan  = 3,
    Heat = 4
}

#[derive(Deserialize, Debug)]
pub struct WeatherSensor {
    channel: i32,
    temperature: i32,
    humidity: i32,
    low_battery: bool
}

pub struct Climate {
    state: Mutex<State>
}

pub struct Sensors {
    weather_sensors: Vec<WeatherSensor>,
    sensor_temp: i32,
    bedroom_temp: i32,
    living_temp: i32
}

struct State {
    conditioners: Vec<Conditioner>,
    sensors: Sensors
}

impl Sensors {
    pub fn empty() -> Self {
        Sensors {
            weather_sensors: Vec::new(),
            sensor_temp: 0,
            bedroom_temp: 0,
            living_temp: 0
        }
    }

    pub fn new(weather_sensors: Vec<WeatherSensor>, sensor_temp: i32, bedroom_temp: i32, living_temp: i32) -> Self {
        Sensors {
            weather_sensors,
            sensor_temp,
            bedroom_temp,
            living_temp
        }
    }
}

impl Climate {
    pub fn new() -> Self {
        Climate {
            state: Mutex::new(Climate::new_state())
        }
    }

    fn new_state() -> State {
        State {
            conditioners: vec![Climate::new_conditioner(), Climate::new_conditioner()],
            sensors: Sensors::empty()
        }
    }

    fn new_conditioner() -> Conditioner {
        Conditioner {
            enabled: false,
            temperature: 20,
            mode: ConditionerMode::Cool
        }
    }

    pub fn set(&self, conditioners: &[Conditioner]) -> Result<(), ServerError> {
        let mut guard = self.state.lock()?;

        for i in 0..conditioners.len() {
            if let Some(conditioner) = conditioners.get(i) {
                if let Some (to_set) = guard.conditioners.get_mut(i) {
                    to_set.enabled = conditioner.enabled;
                    to_set.temperature = conditioner.temperature;
                    to_set.mode = conditioner.mode;
                }
            }
        }

        Ok(())
    }

    pub fn conditioners(&self) -> Result<Vec<Conditioner>, ServerError> {
        let mut guard = self.state.lock()?;
        Ok(guard.conditioners
            .into_iter()
            .map(|c| c.clone())
            .collect())
    }

    pub fn calculate(&self, sensors: Sensors) -> Result<Vec<Conditioner>, ServerError> {
        let mut guard = self.state.lock()?;
        guard.sensors = sensors;

        Ok(guard.conditioners
            .into_iter()
            .map(|c| c.clone())
            .collect())
    }
}