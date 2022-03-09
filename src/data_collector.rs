use std::process::Command;
use std::str::FromStr;
use std::time::SystemTime;
use std::{collections::HashMap, env};

use crate::types::{
  CPUStats, DiskStats, GPUStats, NetworkInterfaceStats, RAMStats, StaticData, SwapStats, TempStats,
};
use crate::util::parse_speed;
use anyhow::{anyhow, Result};
use nvml::NVML;
use serde::{Deserialize, Serialize};
use sysinfo::{
  ComponentExt, DiskExt, NetworkExt, ProcessRefreshKind, ProcessorExt, System, SystemExt,
};
use thiserror::Error;

const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsNetworkInterface {
  pub name: String,
  pub LinkSpeed: String,
}

#[derive(Debug)]
pub struct GPUFetcher {
  pub nvidia: Option<NVML>,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentIP {
  pub ip: String,
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

    return Ok(Self {
      gpu_fetcher,
      fetcher,
      iterator_index: 0,
      program_iterations: 60,
      network_interface_speeds: HashMap::new(),
      start_timestamp: SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis(),
    });
  }

  /// Increments the iterator index by one or resets it to 0 if it reaches the program iterations
  pub fn increment_iterator_index(&mut self) {
    self.iterator_index += 1;
    if self.program_iterations <= self.iterator_index {
      self.iterator_index = 0;
    }
  }

  /// Gets the hostname of the system
  pub fn get_hostname() -> Result<String> {
    let fetcher = System::new_all();

    fetcher.host_name().ok_or(anyhow!(
      "Could not get hostname. Are you running this on a supported platform?"
    ))
  }

  /// Get uptime of the system
  pub fn get_uptime(&mut self) -> Result<u64> {
    let boot_time = self.fetcher.boot_time() * 1000;
    let timeframe = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_millis() as u64
      - boot_time;
    return Ok(timeframe);
  }

  /// Get uptime of the reporter
  pub fn get_reporter_uptime(&mut self) -> Result<u64> {
    let timeframe = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_millis()
      - self.start_timestamp;
    return Ok(timeframe as u64);
  }

  /// Gets the total amount of processes running
  pub fn get_total_process_count(&mut self) -> Result<usize> {
    self
      .fetcher
      .refresh_processes_specifics(ProcessRefreshKind::new());
    return Ok(self.fetcher.processes().len());
  }

  /// Gets the current public IP address
  pub async fn get_current_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get(IP_ADDRESS_URL).await?;
    let cur_ip: CurrentIP = response.json().await?;
    Ok(cur_ip.ip)
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

  /// Gets the current network stats
  pub fn get_network(&mut self) -> Result<Vec<NetworkInterfaceStats>> {
    let mut nics = Vec::new();
    self.fetcher.refresh_networks();

    let nicspeeds = if self.iterator_index == 0 && env::consts::OS == "windows" {
      DataCollector::get_nic_linkspeeds()?
    } else {
      vec![]
    };

    for (interface_name, data) in self.fetcher.networks() {
      // Ignore bullshit loopback interfaces, no one cares
      if interface_name.contains("NPCAP")
        || interface_name.starts_with("lo")
        || interface_name.starts_with("loopback")
      {
        continue;
      };

      if self.iterator_index == 0 {
        // Get the speed of the interface on linux otherwise it's 0
        let speed = match env::consts::OS {
          "linux" => DataCollector::get_nic_linkspeed(&interface_name)?,
          "windows" => {
            let nic_index = nicspeeds
              .iter()
              .position(|(name, _)| name == interface_name)
              .ok_or(anyhow!(
                "Could not find interface {} in the list of nicspeeds",
                interface_name
              ))?;

            // Return the speed of the interface
            nicspeeds[nic_index].1
          }
          _ => 0.0,
        };
        self
          .network_interface_speeds
          .insert(interface_name.to_string(), speed);
      }

      let nic = NetworkInterfaceStats {
        n: interface_name.to_string(),
        tx: data.transmitted() * 8,
        rx: data.received() * 8,
        s: self
          .network_interface_speeds
          .get(&interface_name.to_string())
          .unwrap_or(&0.0)
          .to_owned(),
      };

      nics.push(nic);
    }

    return Ok(nics);
  }

  fn get_nic_linkspeed(interface_name: &str) -> Result<f32> {
    let interface_path = format!("/sys/class/net/{}/speed", interface_name);
    let interface_speed = Command::new("cat").arg(&interface_path).output()?;
    let interface_speed =
      f32::from_str(&String::from_utf8_lossy(&interface_speed.stdout).replace("\n", ""))
        .unwrap_or(0.0);
    return Ok(interface_speed);
  }

  fn get_nic_linkspeeds() -> Result<Vec<(String, f32)>> {
    let mut nics: Vec<(String, f32)> = Vec::new();
    let output_string = Command::new("powershell")
      .args([
        "-Command",
        "Get-NetAdapter | select name, linkSpeed | ConvertTo-Json",
      ])
      .output()?;

    // Convert the json output to a vector of WindowsNetworkInterface structs
    let output_json = serde_json::from_str::<Vec<WindowsNetworkInterface>>(
      &String::from_utf8_lossy(&output_string.stdout),
    )?;

    output_json.iter().for_each(|nic| {
      let split: Vec<&str> = nic.LinkSpeed.split_whitespace().collect();
      let speed = split[0];
      let mult = split[1];

      nics.push((
        nic.name.to_string(),
        parse_speed(f32::from_str(speed).unwrap_or(0.0), mult),
      ));
    });

    // Good job Windows!
    return Ok(nics);
  }

  /// Gets the current CPU stats
  /// wait what the fuck this is an array of cores? 🥴👍
  pub fn get_cpu(&mut self) -> Result<CPUStats> {
    let (mut usage, mut freq) = (vec![], vec![]);

    for processor in self.fetcher.processors() {
      usage.push(processor.cpu_usage().floor() as u16);
      freq.push(processor.frequency() as u16);
    }

    self.fetcher.refresh_cpu();

    return Ok(CPUStats { usage, freq });
  }

  /// Gets the current RAM stats
  pub fn get_ram(&mut self) -> Result<RAMStats> {
    self.fetcher.refresh_memory();

    return Ok(RAMStats {
      used: self.fetcher.used_memory(),
      total: self.fetcher.total_memory(),
    });
  }

  /// Gets the current swap states
  pub fn get_swap(&mut self) -> Result<SwapStats> {
    return Ok(SwapStats {
      used: self.fetcher.used_swap(),
      total: self.fetcher.total_swap(),
    });
  }

  /// Get the current GPU states
  pub fn get_gpu(&mut self) -> Result<GPUStats> {
    let gpu_fetcher = &self.gpu_fetcher;
    match gpu_fetcher.nvidia.as_ref() {
      Some(nvml) => {
        let device = nvml.device_by_index(0)?;
        let (brand, util) = (
          format!("{:?}", device.brand()?),
          device.utilization_rates()?,
        );

        return Ok(GPUStats {
          brand,
          gpu_usage: util.gpu,
          power_usage: device.power_usage()?,
        });
      }
      None => {
        return Err(DataCollectorError::NoGPU)?;
      }
    };
  }

  /// Gets the current disk(s) stats
  pub fn get_disks(&self) -> Result<Vec<DiskStats>> {
    let mut disks = Vec::<DiskStats>::new();

    for disk in self.fetcher.disks() {
      let (name, mount) = (
        disk.name().to_string_lossy(),
        disk.mount_point().to_string_lossy(),
      );

      // Ignore docker disks because they are the same as their host's disk
      if name.contains("docker") || mount.contains("docker") || mount.contains("boot") {
        continue;
      }

      let (fs_type, mut str) = (disk.file_system(), String::from(""));

      for unit in fs_type {
        str.push(*unit as char);
      }

      let disk = DiskStats {
        name: format!("{}", disk.name().to_string_lossy()),
        mount: format!("{}", disk.mount_point().to_string_lossy()),
        fs: str,
        r#type: format!("{:?}", disk.type_()),
        total: disk.total_space(),
        used: disk.total_space() - disk.available_space(),
      };

      disks.push(disk);
    }
    return Ok(disks);
  }

  /// Get the current temperature of the system
  pub fn get_temps(&mut self) -> Result<Vec<TempStats>> {
    self.fetcher.refresh_components();

    let components = self.fetcher.components();

    if components.len() == 0 {
      return Err(anyhow!(DataCollectorError::NoTemp));
    };

    let mut temps = Vec::<TempStats>::new();
    for component in components {
      let temp = component.temperature();
      temps.push(TempStats {
        label: component.label().to_string(),
        value: temp,
      });
    }
    return Ok(temps);
  }
}
