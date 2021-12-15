use crate::types::{CPUStats, DiskStats, GPUStats, NetworkInterfaceStats, RAMStats, StaticData};
use anyhow::Result;
use nvml::NVML;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, SystemExt};
use thiserror::Error;

const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";

#[derive(Error, Debug)]
pub enum DataCollectorError {
    #[error("GPU usage unavailable")]
    NoGPU,
}

#[derive(Debug)]
pub struct DataCollector {
    pub gpu_fetcher: Option<NVML>,
    pub fetcher: System,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentIP {
    pub ip: String,
}

impl DataCollector {
    /// Creates a new data collector
    pub fn new() -> Result<Self> {
        let fetcher = System::new_all();

        // This guy panics on systems without nvidia
        let gpu_fetcher = NVML::init().ok();

        return Ok(Self {
            gpu_fetcher,
            fetcher,
        });
    }

    /// Gets the total amount of processes running
    pub fn get_total_process_count(&mut self) -> Result<usize> {
        self.fetcher.refresh_processes();
        return Ok(self.fetcher.processes().len());
    }

    /// Gets the current public IP address
    pub async fn get_current_ip() -> Result<String, reqwest::Error> {
        let response = reqwest::get(IP_ADDRESS_URL).await?;
        let cur_ip: CurrentIP = response.json().await?;
        Ok(cur_ip.ip)
    }

    /**
    Gets all the static information about the system
    that can't change in runtime
    */
    pub async fn get_statics(&self) -> Result<StaticData> {
        let processor_info = self.fetcher.global_processor_info();

        return Ok(StaticData {
            cpu_model: processor_info.brand().trim().to_string(),
            public_ip: DataCollector::get_current_ip().await?,
            hostname: self.fetcher.host_name(),
            os_version: self.fetcher.os_version(),
            cpu_cores: self.fetcher.physical_core_count(),
            cpu_threads: self.fetcher.processors().len(),
            // kernel_version: .,
            // os_name: self.fetcher.,
            // os_arch: todo!(),
            // cpu_base_frequency: todo!(),
            total_mem: self.fetcher.total_memory(),
        });
    }

    /// Gets the current network stats
    pub fn get_network(&mut self) -> Result<Vec<NetworkInterfaceStats>> {
        let mut nics = Vec::new();
        self.fetcher.refresh_networks();

        for (interface_name, data) in self.fetcher.networks() {
            // Ignore bullshit loopback interfaces, no one cares
            if interface_name.contains("NPCAP")
                || interface_name.starts_with("lo")
                || interface_name.starts_with("loopback")
            {
                continue;
            };

            let nic = NetworkInterfaceStats {
                name: interface_name.to_string(),
                tx: data.transmitted(),
                rx: data.received(),
            };

            nics.push(nic);
        }

        return Ok(nics);
    }

    /// Gets the current CPU stats
    /// wait what the fuck this is an array of cores?
    pub fn get_cpu(&mut self) -> Result<Vec<CPUStats>> {
        let mut processors = Vec::<CPUStats>::new();
        self.fetcher.refresh_cpu();

        for processor in self.fetcher.processors() {
            let processor = CPUStats {
                usage: processor.cpu_usage() as usize,
                freq: processor.frequency(),
            };

            processors.push(processor);
        }

        return Ok(processors);
    }

    /// Gets the current RAM stats
    pub fn get_ram(&mut self) -> Result<RAMStats> {
        self.fetcher.refresh_memory();

        return Ok(RAMStats {
            used: self.fetcher.used_memory(),
            total: self.fetcher.total_memory(),
        });
    }

    pub fn get_gpu(&mut self) -> Result<GPUStats> {
        let gpu_fetcher = self.gpu_fetcher.as_ref().ok_or(DataCollectorError::NoGPU)?;

        // Get the first `Device` (GPU) in the system
        let device = gpu_fetcher.device_by_index(0)?;

        let brand = format!("{:?}", device.brand()?); // GeForce on my system
        let util = device.encoder_utilization()?; // Currently 0 on my system; Not encoding anything
        let memory_info = device.memory_info()?; // Currently 1.63/6.37 GB used on my system

        return Ok(GPUStats {
            brand,
            gpu_usage: util.utilization,
            power_usage: device.power_usage()?,
            mem_used: memory_info.used,
            mem_total: memory_info.total,
        });
    }

    /// Gets the current DISKS stats
    pub fn get_disks(&self) -> Result<Vec<DiskStats>> {
        let mut disks = Vec::<DiskStats>::new();

        for disk in self.fetcher.disks() {
            let name = disk.name().to_string_lossy();
            let mount = disk.mount_point().to_string_lossy();

            // Ignore docker disks because they are the same as their host's disk
            if name.contains("docker") || mount.contains("docker") {
                continue;
            }

            let fs_type = disk.file_system();
		    let mut str = String::from("");
            
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
}
