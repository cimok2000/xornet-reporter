use anyhow::Result;
use serde_json::Value;
use std::fs::{self, File};
use std::io::ErrorKind;

#[derive(Debug)]
pub struct AuthManager {
    pub access_token: String,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        let config = AuthManager::load_config()?;

        let access_token = match config.get("access_token") {
            Some(access_token) => access_token.to_string(),
            None => "".to_string(),
        };

        return Ok(Self { access_token });
    }

    fn load_config() -> Result<Value> {
        let config_file = File::open("config.json");

        match config_file {
            Ok(config_file) => config_file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => File::create("config.json")?,
                _ => panic!("failed to create config.json"),
            },
        };

        let config = serde_json::from_str(&fs::read_to_string("config.json")?)?;

        return Ok(config);
    }
}
