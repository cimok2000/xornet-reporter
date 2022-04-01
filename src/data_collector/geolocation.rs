use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::DataCollector;

const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";
const COUNTRY_URL: &str = "https://ipwhois.app/json/";

#[derive(Serialize, Deserialize)]
pub struct GeolocationInfo {
  pub ip: String,
  pub country: String,
}
#[derive(Serialize, Deserialize)]
struct CurrentIP {
  ip: String,
}

#[derive(Serialize, Deserialize)]
struct CurrentCountry {
  country_code: String,
}

impl DataCollector {
  /// Gets the current public IP address
  pub async fn get_current_ip() -> Result<String, reqwest::Error> {
    let cur_ip: String = reqwest::get(IP_ADDRESS_URL)
      .await?
      .json::<CurrentIP>()
      .await?
      .ip;
    Ok(cur_ip)
  }
  /// Gets the current country
  pub async fn get_current_country(ip: Option<String>) -> Result<String, reqwest::Error> {
    let cur_ip = match ip {
      Some(ip) => ip,
      None => DataCollector::get_current_ip().await?,
    };
    let cur_country: String = reqwest::get(COUNTRY_URL.to_owned() + &cur_ip)
      .await?
      .json::<CurrentCountry>()
      .await?
      .country_code;
    Ok(cur_country)
  }
}
