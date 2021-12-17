use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigJSON {
    pub access_token: String,
}

pub struct ConfigManager {
    pub config: ConfigJSON,
}

impl ConfigManager {
    pub fn new() -> Result<ConfigManager> {
        let config = ConfigManager::load_config()?;
        return Ok(Self { config });
    }

    pub fn save_access_token(access_token: &str) -> Result<()> {
        let mut config = ConfigManager::load_config()?;
        config.access_token = access_token.to_string();
        ConfigManager::save_config(config)?;
        return Ok(());
    }

    /// Saves the modified config to the config file
    pub fn save_config(config: ConfigJSON) -> Result<()> {
        let file = File::create("config.json")?;
        serde_json::to_writer(file, &config)?;
        return Ok(());
    }

    /// Loads the config file from disk or creates a new one if it doesn't exist.
    pub fn load_config() -> Result<ConfigJSON> {
        if !Path::new("config.json").exists() {
            return Ok(ConfigManager::create_config()?);
        } else {
            let file = File::open("config.json")?;
            return Ok(serde_json::from_reader(file)?);
        }
    }

    /// Creates a new config file with an empty access token.
    pub fn create_config() -> Result<ConfigJSON> {
        let config = ConfigJSON {
            access_token: String::new(),
        };
        ConfigManager::save_config(config.clone())?;
        return Ok(config);
    }
}
