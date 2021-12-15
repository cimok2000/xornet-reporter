use serde::Serialize;

#[derive(Serialize)]
pub struct StaticData {
    pub hostname: Option<String>,
    pub public_ip: String,
    // pub kernel_version: String,
    // pub os_name: String,
    // pub os_arch: String,
    // pub os_version: String,
    pub cpu_model: String,
    pub os_version: Option<String>,
    // pub cpu_base_frequency: String,
    pub cpu_cores: Option<usize>,
    pub cpu_threads: usize,
    pub total_mem: u64,
}

#[derive(Serialize)]
pub struct NetworkInterfaceStats {
    pub name: String,
    pub tx: u64,
    pub rx: u64,
}

#[derive(Serialize)]
pub struct CPUStats {
    pub usage: usize,
    pub freq: u64,
}

#[derive(Serialize)]
pub struct RAMStats {
    pub used: u64,
    pub total: u64,
}

#[derive(Serialize)]
pub struct GPUStats {
    pub brand: String,
    pub gpu_usage: u32,
    pub power_usage: u32,
    pub mem_used: u64,
    pub mem_total: u64,
}

#[derive(Serialize)]
pub struct DiskStats {
    pub name: String,
    pub mount: String,
    pub fs: String,
    pub r#type: String,
    pub total: u64,
    pub used: u64,
}
