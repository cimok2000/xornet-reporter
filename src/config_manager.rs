use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub access_token: String,
  pub backend_hostname: String,
}

/// Manages the config.json for the reporter
pub struct ConfigManager {
  pub config: Config,
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
  pub fn save_config(config: Config) -> Result<()> {
    let file = File::create("config.json")?;
    serde_json::to_writer(file, &config)?;
    return Ok(());
  }

  /// Loads the config file from disk or creates a new one if it doesn't exist.
  pub fn load_config() -> Result<Config> {
    if !Path::new("config.json").exists() {
      return Ok(ConfigManager::create_config()?);
    } else {
      let file = File::open("config.json")?;
      return Ok(serde_json::from_reader(file)?);
    }
  }

  /// Creates a new config file with an empty access token and default backend address.
  pub fn create_config() -> Result<Config> {
    let config = Config {
      access_token: String::new(),
      backend_hostname: "backend.xornet.cloud".to_string(),
    };
    ConfigManager::save_config(config.clone())?;
    return Ok(config);
  }
}
