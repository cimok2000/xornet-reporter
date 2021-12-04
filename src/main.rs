use colored::Colorize;
use core::time;
use serde_json::Value;
use std::{
    str::FromStr,
    thread::{self, spawn},
};
use util::{arcmutex, LaunchParams};

mod data_collector;
mod info_box;
mod reporter;
mod util;
use crate::{
    reporter::Reporter,
    util::bytes_to_kb,
    util::{bytes_to_gb, trim_one_character},
};

fn main() {
    // Get arguments from launch
    let args = LaunchParams::new();

    // Setup the terminal
    util::setup_terminal();

    // Cosmetic display configuration
    let prefix = args.prefix;
    let show_border = !args.borderless;

    // Start the reporter
    let reporter = arcmutex(Reporter::new());

    // Get all static shit
    // println!(
    //     "{} Info: {}",
    //     prefix.white(),
    //     reporter.lock().data_collector.get_statics()
    // );

    // Todo: make these run on a loop with unique intervals for each
    // that the user can set in a config
    let reporter = reporter.clone();
    let data_collection_handle = spawn(move || loop {
        let mut reporter = reporter.lock();

        let mut info = info_box::InfoBox {
            pushed_lines: Vec::new(),
            pushed_len: Vec::new(),
            longest_line: 0,
            border: show_border,
        };

        // Header
        let header = format!(" Xornet Reporter v{} ", env!("CARGO_PKG_VERSION"));
        if show_border {
            info.push(&header.bright_black().to_string(), header.len());
            info.push(&" ".to_owned(), " ".len());
        };

        // CPU
        let cpu = format!(
            "{}",
            reporter.data_collector.get_cpu()[0]
                .get("cpu_usage")
                .expect("Error in getting cpu")
        );

        let cpu_info = format!(" {} CPU       {:.2}% ", prefix.clone(), cpu);
        let cpu_info_colored = format!(
            " {} {}       {:.2}{} ",
            prefix.red(),
            "CPU".bright_black(),
            cpu.red(),
            "%".bright_black()
        );

        info.push(&cpu_info_colored, cpu_info.chars().count());

        // Process Count
        let proc_count = format!(
            "{}",
            reporter
                .data_collector
                .get_total_process_count()
                .to_string()
        );
        let process_count = format!(" {} Processes {} ", prefix, proc_count);
        let process_count_colored = format!(
            " {} {} {} ",
            prefix.yellow(),
            "Processes".bright_black(),
            proc_count.yellow()
        );

        info.push(&process_count_colored, process_count.chars().count());

        // Memory
        let used_memory = format!(
            "{}",
            bytes_to_kb(
                reporter
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
                    .data_collector
                    .get_ram()
                    .get("total_memory")
                    .expect("Error in getting memory")
            )
        );

        let mem_info = format!(
            " {} {}    {} / {} MB ",
            prefix, "Memory", used_memory, total_memory
        );
        let mem_info_colored = format!(
            " {} {}    {} {} {} {} ",
            prefix.green(),
            "Memory".bright_black(),
            used_memory.green(),
            "/".bright_black(),
            total_memory.green(),
            "MB".bright_black()
        );

        info.push(&mem_info_colored, mem_info.chars().count());

        // Print ● NICs
        let net_info_header = format!(" {} {}", prefix, "NICs");
        let net_info_header_colored = format!(" {} {}", prefix.blue(), "NICs".bright_black());
        info.push(&net_info_header_colored, net_info_header.chars().count());

        let nics = reporter.data_collector.get_network();
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

            let net_info = format!("     {}  {} {} {} {}", name, rx, "rx", tx, "tx");
            let net_info_colored = format!(
                "     {}  {} {} {} {}",
                name.bright_black(),
                rx.blue(),
                "rx".bright_black(),
                tx.blue(),
                "tx".bright_black(),
            );

            info.push(&net_info_colored, net_info.chars().count());
        }

        // Print ● Disks
        let disk_info_header = format!(" {} {}", prefix, "Disks");
        let disk_info_header_colored = format!(" {} {}", prefix.magenta(), "Disks".bright_black());
        info.push(&disk_info_header_colored, disk_info_header.chars().count());

        let disks = reporter.data_collector.get_disks();
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
            let disk_info = format!("     {}   {} / {} GB ", disk_name, used_disk, total_disk);
            let disk_info_colored = format!(
                "     {}   {} {} {} {} ",
                disk_name.bright_black(),
                used_disk.magenta(),
                "/".bright_black(),
                total_disk.magenta(),
                "GB".bright_black()
            );

            info.push(&disk_info_colored, disk_info.chars().count());
        }

        // ● Disks
        //     C: 378 / 465 GB
        //     D: 378 / 465 GB

        info.push(&" ".to_owned(), " ".len());

        // if (connected) {
        // con = "coneted".gren()
        // } else {
        // con = "not cenod".black()
        // }
        // Status Ratted 💀
        let connection_status = format!("{}", "Disconnected");
        let con_info = format!(" {} Status    {} ", prefix, connection_status);
        let con_info_colored = format!(
            " {} {}    {} ",
            prefix.bright_black(),
            "Status".bright_black(),
            connection_status.red()
        );
        info.push(&con_info_colored, con_info.chars().count());

        println!("{}", info.to_string().trim_end());

        // Reset the cursor back to 0, 0
        if !args.no_clear {
            util::reset_cursor()
        };

        // Wait for interval
        thread::sleep(time::Duration::from_secs_f64(args.interval));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
