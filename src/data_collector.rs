use anyhow::Result;
use nvml::NVML;
use serde::Serialize;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};

#[derive(Debug)]
pub struct DataCollector {
    pub gpu_fetcher: NVML,
    pub fetcher: System,
}

#[derive(Serialize)]
pub struct StaticData {
    pub cpu: StaticCPUData,
}

#[derive(Serialize)]
pub struct NetworkInterfaceStats {
    pub name: String,
    pub tx: u64,
    pub rx: u64,
}

#[derive(Serialize)]
pub struct CPUStats {
    pub cpu_usage: f32,
    pub frequency: u64,
}

#[derive(Serialize)]
pub struct RAMStats {
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_memory: u64,
}

#[derive(Serialize)]
pub struct GPUStats {
    pub brand: String,
    pub gpu_usage: u32,
    pub power_usage: u32,
    pub memory_free: u64,
    pub memory_used: u64,
    pub memory_total: u64,
}

#[derive(Serialize)]
pub struct DiskStats {
    pub name: String,
    pub mount: String,
    pub filesystem: String,
    pub disk_type: String,
    pub free: u64,
    pub total: u64,
    pub used: u64,
}

#[derive(Serialize)]
pub struct StaticCPUData {
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
}

impl DataCollector {
    /// Creates a new data collector
    pub fn new() -> Result<Self> {
        let fetcher = System::new_all();

        // How to fix @Bluskript
        // This guy panics :whysphere:
        let gpu_fetcher = NVML::init()?;

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

    /// Gets all the static information about the system
    /// that can't change in runtime
    pub fn _get_statics(&self) -> Result<StaticData> {
        let processor_info = self.fetcher.global_processor_info();

        return Ok(StaticData {
            cpu: StaticCPUData {
                name: processor_info.name().to_string(),
                vendor_id: processor_info.vendor_id().to_string(),
                brand: processor_info.brand().to_string(),
            },
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

            let json = NetworkInterfaceStats {
                name: interface_name.to_string(),
                tx: data.transmitted(),
                rx: data.received(),
            };

            nics.push(json);
        }

        return Ok(nics);
    }

    /// Gets the current CPU stats
    /// wait what the fuck this is an array of cores?
    pub fn get_cpu(&mut self) -> Result<Vec<CPUStats>> {
        let mut processors = Vec::<CPUStats>::new();
        self.fetcher.refresh_cpu();

        for processor in self.fetcher.processors() {
            let json = CPUStats {
                cpu_usage: processor.cpu_usage(),
                frequency: processor.frequency(),
            };

            processors.push(json);
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
        // Get the first `Device` (GPU) in the system
        let device = self.gpu_fetcher.device_by_index(0)?;

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
