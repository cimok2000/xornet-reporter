use colored::Colorize;
use core::time;
use crossterm::{cursor, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use parking_lot::Mutex;
use std::{
    io::stdout,
    sync::Arc,
    thread::{self, spawn},
};

mod data_collector;
mod info_box;
mod reporter;
mod util;
use crate::{reporter::Reporter, util::bytes_to_gb, util::bytes_to_kb};

fn main() {
    
    // "Clear" the terminal
    execute!(stdout(), EnterAlternateScreen).unwrap();
    
    // Hide the cursor
    execute!(stdout(), cursor::Hide).unwrap();
    // Create the CTRL + C handler
    ctrlc::set_handler(move || {
        // Go back to normal terminal
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        // Show the cursor
        execute!(stdout(), cursor::Show).unwrap();
        // Exit the program
        std::process::exit(0);
    })
    .expect("Ctrl + C handler failed to be set");

    // Prefix of the display thing (can also be in config)
    let prefix = "●";

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    // // Get all static shit
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
        let mut stdout = stdout();

        let mut info = info_box::InfoBox {
            pushed_lines: Vec::new(),
            pushed_len: Vec::new(),
            longest_line: 0,
        };

        execute!(stdout, cursor::SavePosition).ok();

        // who wrote this code lol - azur

        // Header
        let header = format!(" Xornet Reporter v{} ", env!("CARGO_PKG_VERSION"));
        info.push(&header.bright_black().to_string(), header.len());

        // CPU
        let cpu = format!(
            "{}",
            reporter.data_collector.get_cpu()[0]
                .get("cpu_usage")
                .expect("Error in getting cpu")
        );

        let cpu_info = format!(" {} CPU {:.2}% ", prefix, cpu);
        let cpu_info_colored = format!(
            " {} {} {:.2}{} ",
            prefix.red(),
            "CPU".bright_black(),
            cpu.red(),
            "%".bright_black()
        );

        info.push(&cpu_info_colored, cpu_info.chars().count());

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
            " {} {} {} / {} MB ",
            prefix, "Memory", used_memory, total_memory
        );
        let mem_info_colored = format!(
            " {} {} {} {} {} {} ",
            prefix.green(),
            "Memory".bright_black(),
            used_memory.green(),
            "/".bright_black(),
            total_memory.green(),
            "MB".bright_black()
        );

        info.push(&mem_info_colored, mem_info.chars().count());

        // Network
        let rx = format!(
            "{}",
            reporter.data_collector.get_network()[0]
                .get("rx")
                .expect("Error in getting network")
        );
        let tx = format!(
            "{}",
            reporter.data_collector.get_network()[0]
                .get("tx")
                .expect("Error in getting network")
        );

        let net_info = format!(" {} {} {} {} {} {}", prefix, "Network", rx, "rx", tx, "tx");
        let net_info_colored = format!(
            " {} {} {} {} {} {}",
            prefix.blue(),
            "Network".bright_black(),
            rx.blue(),
            "rx".bright_black(),
            tx.blue(),
            "tx".bright_black(),
        );

        info.push(&net_info_colored, net_info.chars().count());
        // Disk
        let free_disk = format!(
            "{}",
            bytes_to_gb(
                reporter.data_collector.get_disks()[0]
                    .get("free")
                    .expect("Error in getting disk")
            )
        );
        let total_disk = format!(
            "{}",
            bytes_to_gb(
                reporter.data_collector.get_statics().get("disks").unwrap()[0]
                    .get("total")
                    .expect("Error in getting total disk")
            )
        );

        let disk_info = format!(" {} {} {} / {} GB ", prefix, "Disk", free_disk, total_disk);
        let disk_info_colored = format!(
            " {} {} {} {} {} {} ",
            prefix.magenta(),
            "Disk".bright_black(),
            free_disk.magenta(),
            "/".bright_black(),
            total_disk.magenta(),
            "GB".bright_black()
        );
        info.push(&disk_info_colored, disk_info.chars().count());

        println!("{}", info.to_string().trim_end());

        execute!(stdout, cursor::RestorePosition).ok();

        thread::sleep(time::Duration::from_millis(1000));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
