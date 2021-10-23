use std::sync::Mutex;

pub struct Computers {
    state: Mutex<State>
}

struct State {
    computers: Vec<Computer>
}

struct Computer {
    name: String,
    turn_on_requested: bool
}