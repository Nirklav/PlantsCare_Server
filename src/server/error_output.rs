use crate::server::server_error::LogicError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorOutput {
    code: i32
}

impl ErrorOutput {
    pub fn new(error: LogicError) -> Self {
        ErrorOutput {
            code: error as i32
        }
    }
}