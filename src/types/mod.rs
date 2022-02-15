use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticData {
  pub hostname: Option<String>,
  pub os_version: Option<String>,
  pub os_name: Option<String>,
  pub cpu_cores: Option<usize>,
  pub public_ip: String,
  pub cpu_model: String,
  pub cpu_threads: usize,
  pub total_mem: u64,
  pub reporter_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInterfaceStats {
  pub n: String,
  pub tx: u64,
  pub rx: u64,
  pub s: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CPUStats {
  pub usage: Vec<u16>,
  pub freq: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RAMStats {
  pub used: u64,
  pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPUStats {
  pub brand: String,
  pub gpu_usage: u32,
  pub power_usage: u32,
  pub mem_used: u64,
  pub mem_total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskStats {
  pub name: String,
  pub mount: String,
  pub fs: String,
  pub r#type: String,
  pub total: u64,
  pub used: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempStats {
  pub label: String,
  pub value: f32,
}
