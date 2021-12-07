use nvml::NVML;
use serde_json::{json, Value};
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};

#[derive(Debug)]
pub struct DataCollector {
    pub gpu_fetcher: NVML,
    pub fetcher: System,
}

impl DataCollector {
    /// Creates a new data collector
    pub fn new() -> Self {
        let gpu_fetcher = NVML::init().unwrap();
        let fetcher = System::new_all();
        return Self {
            gpu_fetcher,
            fetcher,
        };
    }

    /// Gets the total amount of processes running
    pub fn get_total_process_count(&mut self) -> usize {
        self.fetcher.refresh_processes();
        return self.fetcher.processes().len();
    }

    /// Gets all the static information about the system
    /// that can't change in runtime
    pub fn _get_statics(&self) -> Value {
        let processor_info = self.fetcher.global_processor_info();

        return json!({
          "cpu": {
            "name": processor_info.name(),
            "vendor_id": processor_info.vendor_id(),
            "brand": processor_info.brand(),
          },
        });
    }

    /// Gets the current network stats
    pub fn get_network(&mut self) -> Vec<Value> {
        let mut serialized_networks = Vec::new();
        self.fetcher.refresh_networks();

        for (interface_name, data) in self.fetcher.networks() {
            // Ignore bullshit loopback interfaces, no one cares
            if interface_name.contains("NPCAP")
                || interface_name.starts_with("lo")
                || interface_name.starts_with("loopback")
            {
                continue;
            };

            let json = json!({
                "name": interface_name,
                "tx": data.transmitted(),
                "rx": data.received(),
            });

            serialized_networks.push(json);
        }

        return serialized_networks;
    }

    /// Gets the current CPU stats
    pub fn get_cpu(&mut self) -> Value {
        let mut serialized_processors = Vec::new();
        self.fetcher.refresh_cpu();

        for processor in self.fetcher.processors() {
            let json = json!({
              "cpu_usage": processor.cpu_usage(),
              "frequency": processor.frequency(),
            });

            serialized_processors.push(json);
        }

        return Value::Array(serialized_processors);
    }

    /// Gets the current RAM stats
    pub fn get_ram(&mut self) -> Value {
        self.fetcher.refresh_memory();

        return json!({
          "free_memory": self.fetcher.free_memory(),
          "available_memory": self.fetcher.available_memory(),
          "used_memory": self.fetcher.used_memory(),
          "total_memory": self.fetcher.total_memory(),
        });
    }

    pub fn get_gpu(&mut self) -> Value {
        // Get the first `Device` (GPU) in the system
        let device = self.gpu_fetcher.device_by_index(0).unwrap();

        let brand = format!("{:?}", device.brand().unwrap()); // GeForce on my system
        let util = device.encoder_utilization().unwrap(); // Currently 0 on my system; Not encoding anything
        let memory_info = device.memory_info().unwrap(); // Currently 1.63/6.37 GB used on my system

        return json!({
            "brand": brand,
            "gpu_usage": util.utilization,
            "power_usage": device.power_usage().unwrap(),
            "vram": {
                "free": memory_info.free,
                "used": memory_info.used,
                "total": memory_info.total
            }
        });
    }

    /// Gets the current DISKS stats
    pub fn get_disks(&self) -> Vec<Value> {
        let mut serialized_disks = Vec::new();

        for disk in self.fetcher.disks() {
            let name = disk.name().to_string_lossy();
            let mount = disk.mount_point().to_string_lossy();

            // Ignore docker disks because they are the same as their host's disk
            if name.contains("docker") || mount.contains("docker") {
                continue;
            }

            let json = json!({
                "name": format!("{}", disk.name().to_string_lossy()),
                "mount":format!("{}", disk.mount_point().to_string_lossy()),
                "filesystem": format!("{:?}", disk.file_system()),
                "type": format!("{:?}", disk.type_()),
                "free": disk.available_space(),
                "total": disk.total_space(),
                "used": disk.total_space() - disk.available_space(),
            });

            serialized_disks.push(json);
        }
        return serialized_disks;
    }
}
