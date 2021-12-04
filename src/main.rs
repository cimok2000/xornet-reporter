use core::time;
use parking_lot::Mutex;
use std::{
    sync::Arc,
    thread::{self, spawn},
};
use colored::Colorize;

mod data_collector;
mod reporter;
use crate::reporter::Reporter;

fn main() {

    // Prefix of the display thing (can also be in config)
    let prefix = "●";

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    // Get all static shit
    println!("{} Info: {}", prefix.white(), reporter.lock().data_collector.get_statics());

    // Todo: make these run on a loop with unique intervals for each
    // that the user can set in a config
    let reporter = reporter.clone();
    let data_collection_handle = spawn(move || loop {
        let mut reporter = reporter.lock();

        println!("{} Disks: {}", prefix.red(),
                reporter.data_collector.get_disks()[0].get("free").expect("Error in CPU displaying."));
        println!("{} Ram: {} / {}", prefix.green(),
                reporter.data_collector.get_ram().get("available_memory").expect("Erorr in RAM displaying."),
                reporter.data_collector.get_ram().get("total_memory").expect("Error in RAM displaying."));
        println!("{} Network: {}", prefix.blue(),
                reporter.data_collector.get_network()[0]);
        println!("{} CPU: {:.2}%", prefix.magenta(),
                reporter.data_collector.get_cpu()[0].get("cpu_usage").expect("Error In CPU displaying."));

        thread::sleep(time::Duration::from_millis(1000));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
