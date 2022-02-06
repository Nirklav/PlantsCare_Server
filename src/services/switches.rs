use std::sync::Mutex;
use crate::server::server_error::ServerError;

pub struct Switches {
    state: Mutex<State>
}

struct State {
    switches: Vec<Switch>
}

struct Switch {
    name: String,
    enabled: bool,
    ip: Option<String>,
    port: Option<u16>
}

impl Switches {
    pub fn new() -> Self {
        Switches {
            state: Mutex::new(Switches::new_state())
        }
    }

    fn new_state() -> State {
        State {
            switches: Vec::new()
        }
    }

    pub fn is_enabled(&self, name: &str, ip: &Option<String>, port: &Option<u16>) -> Result<bool, ServerError> {
        let mut guard = self.state.lock()?;

        if let Some(mut switch) = Switches::find_mut(&mut guard, name) {
            switch.ip = ip.clone();
            switch.port = port.clone();
            return Ok(switch.enabled)
        } else {
            let switch = Switch {
                name: name.to_string(),
                ip: ip.clone(),
                port: port.clone(),
                enabled: false
            };
            guard.switches.push(switch);
        }

        Ok(false)
    }

    pub fn set(&self, name: &str, value: bool) -> Result<(bool, Option<String>, Option<u16>), ServerError> {
        let mut guard = self.state.lock()?;

        if let Some(switch) = Switches::find_mut(&mut guard, name) {
            switch.enabled = value;
            Ok((false, switch.ip.clone(), switch.port.clone()))
        } else {
            let switch = Switch {
                name: name.to_string(),
                ip: None,
                port: None,
                enabled: value
            };
            guard.switches.push(switch);
            Ok((true, None, None))
        }
    }

    fn find_mut<'a>(state: &'a mut State, name: &str) -> Option<&'a mut Switch> {
        state.switches
            .iter_mut()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }
}