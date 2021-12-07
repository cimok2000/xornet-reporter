use anyhow::Result;
use colored::Colorize;
use parking_lot::Mutex;
use serde_json::Value;
use std::{io::Write, sync::Arc};
use thiserror::Error;

use crate::{
    reporter::Reporter,
    util::{self, bytes_to_gb, bytes_to_kb, bytes_to_mb, trim_one_character},
};

#[derive(Error, Debug)]
pub enum UiError {
    #[error("CPU usage unavailable")]
    NoCPU,
    #[error("GPU usage unavailable")]
    NoGPU,
}

pub struct Ui {}

impl Ui {
    pub fn get_cpu(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let result = reporter.lock().data_collector.get_cpu();

        match result {
            Ok(result) => {
                return Ok(format!(
                    " {} {}       {:.5}{} ",
                    prefix.red(),
                    "CPU".bright_black(),
                    format!("{}", result[0].get("cpu_usage").ok_or(|| UiError::NoCPU)?).red(),
                    "%".bright_black()
                ));
            }
            Err(error) => return Ok(format!("{}", UiError::NoCPU)),
        };
    }

    pub fn get_gpu(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let gpu = reporter.lock().data_collector.get_gpu();

        let gpu_power_usage = format!("{}", gpu.get("power_usage").unwrap());

        let gpu_vram = gpu.get("vram").unwrap();
        let gpu_vram_used = format!("{}", bytes_to_mb(gpu_vram.get("used").unwrap()));
        let gpu_vram_total = format!("{}", bytes_to_mb(gpu_vram.get("total").unwrap()));

        return Ok(format!(
            " {} {}       {:.5}{} {} {} {} {}",
            prefix.cyan(),
            "GPU".bright_black(),
            gpu_power_usage.cyan(),
            "mW".bright_black(),
            gpu_vram_used.cyan(),
            "/".bright_black(),
            gpu_vram_total.cyan(),
            "MB".bright_black(),
        ));
    }

    pub fn get_process_count(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let proc_count = format!(
            "{}",
            reporter
                .lock()
                .data_collector
                .get_total_process_count()
                .to_string()
        );
        return Ok(format!(
            " {} {} {} ",
            prefix.green(),
            "Processes".bright_black(),
            proc_count.green()
        ));
    }

    pub fn get_memory(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let used_memory = format!(
            "{}",
            bytes_to_kb(
                reporter
                    .lock()
                    .data_collector
                    .get_ram()
                    .get("used_memory")
                    .expect("Error in getting memory")
            )
        );
        let total_memory = format!(
            "{}",
            bytes_to_kb(
                reporter
                    .lock()
                    .data_collector
                    .get_ram()
                    .get("total_memory")
                    .expect("Error in getting memory")
            )
        );

        return Ok(format!(
            " {} {}    {} {} {} {} ",
            prefix.yellow(),
            "Memory".bright_black(),
            used_memory.yellow(),
            "/".bright_black(),
            total_memory.yellow(),
            "MB".bright_black()
        ));
    }

    pub fn get_nics(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let nics_header = format!(" {} {} \n", prefix.cyan(), "NICs".bright_black());
        let nics = reporter.lock().data_collector.get_network();

        let mut nics_info = String::new();
        nics_info.push_str(&nics_header);
        for i in 0..nics.len() {
            let nic = &nics[i];

            // Network
            let rx = format!("{}", nic.get("rx").expect("Error in getting network"));
            let tx = format!("{}", nic.get("tx").expect("Error in getting network"));
            let name = trim_one_character(
                &nic.get("name")
                    .unwrap_or(&Value::String("NIC".to_string()))
                    .to_string(),
            );

            nics_info.push_str(&format!(
                "     {}  {} {} {} {}\n",
                name.bright_black(),
                rx.blue(),
                "rx".bright_black(),
                tx.blue(),
                "tx".bright_black(),
            ));
        }

        return Ok(nics_info.trim_end().to_string());
    }

    pub fn get_disks(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let disks_header = format!(" {} {} \n", prefix.magenta(), "Disks".bright_black());
        let disks = reporter.lock().data_collector.get_disks();

        let mut disks_list = String::new();
        disks_list.push_str(&disks_header);
        for disk in disks {
            let disk_name = trim_one_character(
                &disk
                    .get("mount")
                    .unwrap_or(disk.get("name").expect("Couldn't get disk mount/name"))
                    .to_string()
                    .as_str()
                    .replace("\\", ""),
            );
            // Disk
            let used_disk = format!(
                "{}",
                bytes_to_gb(disk.get("used").expect("Error in getting disk"))
            );
            let total_disk = format!(
                "{}",
                bytes_to_gb(disk.get("total").expect("Error in getting total disk"))
            );
            let disk_info = format!(
                "     {}   {} {} {} {}\n",
                disk_name.bright_black(),
                used_disk.magenta(),
                "/".bright_black(),
                total_disk.magenta(),
                "GB".bright_black()
            );
            disks_list.push_str(&disk_info);
        }

        return Ok(disks_list.trim_end().to_string());
    }

    pub fn get_connection(prefix: &str, _reporter: Arc<Mutex<Reporter>>) -> Result<String> {
        let connection_status = format!("{}", "Disconnected");
        let con_info = format!(
            " {} {}    {} ",
            prefix.bright_black(),
            "Status".bright_black(),
            connection_status.red()
        );

        return Ok(con_info);
    }

    pub fn header() -> Result<String> {
        return Ok(format!(" Xornet Reporter v{} ", env!("CARGO_PKG_VERSION"))
            .bright_black()
            .to_string());
    }

    pub fn new(prefix: &str, no_clear: bool, reporter: Arc<Mutex<Reporter>>) -> Self {
        let attempts = [
            Ui::header(),
            Ui::get_cpu(prefix, reporter.clone()),
            Ui::get_memory(prefix, reporter.clone()),
            Ui::get_process_count(prefix, reporter.clone()),
            Ui::get_gpu(prefix, reporter.clone()),
            Ui::get_nics(prefix, reporter.clone()),
            Ui::get_disks(prefix, reporter.clone()),
            Ok("".to_string()),
            Ui::get_connection(prefix, reporter.clone()),
        ];

        let mut string = "".to_string();

        for attempt in attempts {
            match attempt {
                Ok(data) => string.push_str(&(data + "\n")),
                Err(err) => println!("{:?}", err),
            }
        }

        println!("{}", string);

        std::io::stdout().flush().expect("Couldn't flush stdout");

        // Reset the cursor back to 0, 0
        if !no_clear {
            util::reset_cursor();
        };

        return Self {};
    }
}
