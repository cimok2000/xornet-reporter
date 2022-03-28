use parking_lot::Mutex;
use std::sync::Arc;

pub fn arcmutex<T>(item: T) -> Arc<Mutex<T>> {
  return Arc::new(Mutex::new(item));
}

/// Returns the speed in megabytes per second
/// # Arguments
/// * `number` - The number to convert
/// * `speed` - The speed multiplier of the number
pub fn parse_speed(number: f32, speed: &str) -> f32 {
  match speed {
    "bps" => return number / 1000000f32,
    "Kbps" => return number / 1000f32,
    "Mbps" => return number,
    "Gbps" => return number * 1000f32,
    "Tbps" => return number * 1000000f32,
    _ => return number,
  }
}
