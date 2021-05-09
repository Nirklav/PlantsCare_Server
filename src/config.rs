use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub db_connection: String,
    pub log_config_path: String
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Config, Error> {
        let metadata = fs::metadata(&file_path)?;
        let file_size = metadata.len() as usize;
        let mut result_buf = String::with_capacity(file_size);

        let mut file = File::open(&file_path)?;
        let _ = file.read_to_string(&mut result_buf)?;

        let config : Config = serde_json::from_str(&result_buf)?;

        Ok(config)
    }
}