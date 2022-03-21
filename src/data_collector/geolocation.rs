use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::DataCollector;

const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";

#[derive(Serialize, Deserialize)]
pub struct CurrentIP {
  pub ip: String,
}

impl DataCollector {
  /// Gets the current public IP address
  pub async fn get_current_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get(IP_ADDRESS_URL).await?;
    let cur_ip: CurrentIP = response.json().await?;
    Ok(cur_ip.ip)
  }
}
