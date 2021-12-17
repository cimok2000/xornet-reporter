use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigJSON {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct SignupBody {
    pub two_factor_key: String,
    pub hostname: String,
    pub hardware_uuid: String,
}

#[derive(Deserialize)]
pub struct SignupResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct SignupResponseError {
    pub error: String,
}

#[derive(Debug)]
pub struct AuthManager {
    pub access_token: String,
}

impl AuthManager {
    pub fn new() -> Result<AuthManager> {
        let config = AuthManager::load_config()?;
        return Ok(Self {
            access_token: config.access_token,
        });
    }

    pub fn save_access_token(access_token: &str) -> Result<()> {
        let mut config = AuthManager::load_config()?;
        config.access_token = access_token.to_string();
        AuthManager::save_config(config)?;
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
            return Ok(AuthManager::create_config()?);
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
        AuthManager::save_config(config.clone())?;
        return Ok(config);
    }

    /// The signup function that authenticates the machine into Xornet backend.
    pub async fn signup(
        two_factor_key: &str,
        hostname: &str,
        hardware_uuid: &str,
    ) -> Result<SignupResponse> {
        println!("Signing up to Xornet...");

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8000/machines/@signup")
            .json(&SignupBody {
                two_factor_key: two_factor_key.to_string(),
                hostname: hostname.to_string(),
                hardware_uuid: hardware_uuid.to_string(),
            })
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response_json: SignupResponse = serde_json::from_str(&response.text().await?)?;
                return Ok(response_json);
            }
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::NOT_FOUND
            | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                let response_json: SignupResponseError =
                    serde_json::from_str(&response.text().await?)?;
                return Err(anyhow::anyhow!(response_json.error));
            }
            _ => return Err(anyhow::anyhow!("Unexpected response from Xornet")),
        }
    }
}
