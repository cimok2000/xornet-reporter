use crate::types::{CPUStats, DiskStats, GPUStats, NetworkInterfaceStats, RAMStats, StaticData};
use anyhow::Result;
use nvml::NVML;
use sysinfo::System;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, SystemExt};
use thiserror::Error;

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

    /**
    Gets all the static information about the system
    that can't change in runtime
    */
    pub fn get_statics(&self) -> Result<StaticData> {
        let processor_info = self.fetcher.global_processor_info();

        return Ok(StaticData {
            cpu_model: processor_info.brand().trim().to_string(),
            // hostname: todo!(),
            // public_ip: todo!(),
            // kernel_version: todo!(),
            // os_name: todo!(),
            // os_arch: todo!(),
            // os_version: todo!(),
            // cpu_base_frequency: todo!(),
            // cpu_cores: todo!(),
            // cpu_threads: todo!(),
            // total_memory: todo!(),
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
                cpu_usage: processor.cpu_usage(),
                frequency: processor.frequency(),
            };

            processors.push(processor);
        }

        return Ok(processors);
    }

    /// Gets the current RAM stats
    pub fn get_ram(&mut self) -> Result<RAMStats> {
        self.fetcher.refresh_memory();

        return Ok(RAMStats {
            free_memory: self.fetcher.free_memory(),
            available_memory: self.fetcher.available_memory(),
            used_memory: self.fetcher.used_memory(),
            total_memory: self.fetcher.total_memory(),
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
            brand: brand,
            gpu_usage: util.utilization,
            power_usage: device.power_usage()?,
            memory_free: memory_info.free,
            memory_used: memory_info.used,
            memory_total: memory_info.total,
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

            let disk = DiskStats {
                name: format!("{}", disk.name().to_string_lossy()),
                mount: format!("{}", disk.mount_point().to_string_lossy()),
                filesystem: format!("{:?}", disk.file_system()),
                disk_type: format!("{:?}", disk.type_()),
                free: disk.available_space(),
                total: disk.total_space(),
                used: disk.total_space() - disk.available_space(),
            };

            disks.push(disk);
        }
        return Ok(disks);
    }
}
