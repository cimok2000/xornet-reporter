use std::{io::Write, sync::Arc};

use colored::Colorize;
use parking_lot::Mutex;
use serde_json::Value;

use crate::{
    reporter::Reporter,
    util::{self, bytes_to_gb, bytes_to_kb, trim_one_character},
};

pub struct Ui {}

impl Ui {
    pub fn get_cpu(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
        let cpu = format!(
            "{}",
            reporter.lock().data_collector.get_cpu()[0]
                .get("cpu_usage")
                .expect("Error in getting cpu")
        );
        return format!(
            " {} {}       {:.5}{} ",
            prefix.red(),
            "CPU".bright_black(),
            cpu.red(),
            "%".bright_black()
        );
    }

    pub fn get_process(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
        let proc_count = format!(
            "{}",
            reporter
                .lock()
                .data_collector
                .get_total_process_count()
                .to_string()
        );
        return format!(
            " {} {} {} ",
            prefix.yellow(),
            "Processes".bright_black(),
            proc_count.yellow()
        );
    }

    pub fn get_memory(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
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

        return format!(
            " {} {}    {} {} {} {} ",
            prefix.green(),
            "Memory".bright_black(),
            used_memory.green(),
            "/".bright_black(),
            total_memory.green(),
            "MB".bright_black()
        );
    }

    pub fn get_nics(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
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

        return nics_info.trim_end().to_string();
    }

    pub fn get_disk(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
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

        return disks_list.trim_end().to_string();
    }

    pub fn get_connection(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> String {
        let connection_status = format!("{}", "Disconnected");
        let con_info = format!(
            " {} {}    {} ",
            prefix.bright_black(),
            "Status".bright_black(),
            connection_status.red()
        );

        return con_info;
    }

    pub fn header() -> String {
        return format!(" Xornet Reporter v{} ", env!("CARGO_PKG_VERSION"))
            .bright_black()
            .to_string();
    }

    pub fn new(prefix: &str, no_clear: bool, reporter: Arc<Mutex<Reporter>>) -> Self {
        let info = [
            Ui::header(),
            Ui::get_cpu(prefix, reporter.clone()),
            Ui::get_memory(prefix, reporter.clone()),
            Ui::get_process(prefix, reporter.clone()),
            Ui::get_nics(prefix, reporter.clone()),
            Ui::get_disk(prefix, reporter.clone()),
            "".to_string(),
            Ui::get_connection(prefix, reporter.clone()),
        ];

        println!("{}", info.join("\n"));

        std::io::stdout().flush().expect("Couldn't flush stdout");

        // Reset the cursor back to 0, 0
        if !no_clear {
            util::reset_cursor();
        };

        return Self {};
    }
}
