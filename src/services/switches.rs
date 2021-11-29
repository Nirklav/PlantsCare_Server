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
    enabled: bool
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

    pub fn is_enabled(&self, name: &str) -> Result<bool, ServerError> {
        let mut guard = self.state.lock()?;

        if let Some(switch) = Switches::find_mut(&mut guard, name) {
            return Ok(switch.enabled)
        }
        return Ok(false)
    }

    pub fn set(&self, name: &str, value: bool) -> Result<bool, ServerError> {
        let mut guard = self.state.lock()?;

        if let Some(switch) = Switches::find_mut(&mut guard, name) {
            switch.enabled = value;
            Ok(false)
        } else {
            let switch = Switch {
                name: name.to_string(),
                enabled: value
            };
            guard.switches.push(switch);
            Ok(true)
        }
    }

    fn find_mut<'a>(state: &'a mut State, name: &str) -> Option<&'a mut Switch> {
        state.switches
            .iter_mut()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }
}