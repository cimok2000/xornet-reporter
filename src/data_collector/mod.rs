mod cpu;
mod disks;
mod geolocation;
mod gpu;
mod nics;
mod ram;
mod temps;
mod uptimes;

use crate::types::{DynamicData, StaticData};
use anyhow::{anyhow, Result};
use nvml::NVML;
use std::{collections::HashMap, time::SystemTime};
use sysinfo::{ProcessRefreshKind, ProcessorExt, System, SystemExt};
use thiserror::Error;

use self::gpu::GPUFetcher;

#[derive(Error, Debug)]
pub enum DataCollectorError {
  #[error("GPU usage unavailable")]
  NoGPU,
  #[error("Temperature unavailable")]
  NoTemp,
}

#[derive(Debug)]
pub struct DataCollector {
  pub gpu_fetcher: GPUFetcher,
  pub fetcher: System,
  pub program_iterations: usize,
  iterator_index: usize,
  network_interface_speeds: HashMap<String, f32>,
  start_timestamp: u128,
}

impl DataCollector {
  /// Creates a new data collector
  pub fn new() -> Result<Self> {
    let (fetcher, gpu_fetcher) = (
      System::new_all(),
      GPUFetcher {
        nvidia: NVML::init().ok(),
      },
    );

    Ok(Self {
      gpu_fetcher,
      fetcher,
      iterator_index: 0,
      program_iterations: 60,
      network_interface_speeds: HashMap::new(),
      start_timestamp: SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis(),
    })
  }

  /// Increments the iterator index by one or resets it to 0 if it reaches the program iterations
  pub fn increment_iterator_index(&mut self) {
    self.iterator_index += 1;
    if self.program_iterations <= self.iterator_index {
      self.iterator_index = 0;
    }
  }

  pub fn get_all_dynamic_data(&mut self) -> Result<DynamicData> {
    Ok(DynamicData {
      cpu: self.get_cpu()?,
      ram: self.get_ram()?,
      swap: self.get_swap()?,
      gpu: self.get_gpu().ok(),
      process_count: self.get_total_process_count()? as i32,
      disks: self.get_disks()?,
      temps: self.get_temps().ok(),
      network: self.get_network()?,
      host_uptime: self.get_uptime()?,
      reporter_uptime: self.get_reporter_uptime()?,
    })
  }

  /// Gets the hostname of the system
  pub fn get_hostname() -> Result<String> {
    let fetcher = System::new_all();

    fetcher.host_name().ok_or(anyhow!(
      "Could not get hostname. Are you running this on a supported platform?"
    ))
  }

  /// Gets the total amount of processes running
  pub fn get_total_process_count(&mut self) -> Result<usize> {
    self
      .fetcher
      .refresh_processes_specifics(ProcessRefreshKind::new());
    return Ok(self.fetcher.processes().len());
  }

  /// Gets all the static information about the system
  /// that can't change in runtime
  pub async fn get_statics(&self) -> Result<StaticData> {
    let processor_info = self.fetcher.global_processor_info();

    return Ok(StaticData {
      cpu_model: processor_info.brand().trim().to_string(),
      public_ip: DataCollector::get_current_ip().await?,
      hostname: self.fetcher.host_name(),
      os_version: self.fetcher.os_version(),
      os_name: self.fetcher.name(),
      cpu_cores: self.fetcher.physical_core_count(),
      cpu_threads: self.fetcher.processors().len(),
      total_mem: self.fetcher.total_memory(),
      reporter_version: env!("CARGO_PKG_VERSION").to_string(),
    });
  }
}
