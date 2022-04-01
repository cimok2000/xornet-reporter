use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::DataCollector;

const GEOLOCATION_URL: &str = "https://ipwhois.app/json/";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GeolocationInfo {
  pub ip: String,
  pub country_code: String,
  pub isp: String,
  pub city: String,
  pub timezone_gmtOffset: usize,
}

impl DataCollector {
  /// Gets the geolocation information
  pub async fn get_geolocation_info() -> Result<GeolocationInfo, reqwest::Error> {
    let geolocation_info: GeolocationInfo = reqwest::get(GEOLOCATION_URL)
      .await?
      .json::<GeolocationInfo>()
      .await?;
    Ok(geolocation_info)
  }
}
