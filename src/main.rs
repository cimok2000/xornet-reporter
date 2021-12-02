use core::time;
use parking_lot::Mutex;
use std::{
    sync::Arc,
    thread::{self, spawn},
};

mod data_collector;
mod reporter;
use crate::reporter::Reporter;

fn main() {
    let reporter = Arc::new(Mutex::new(Reporter::new()));

    // Get all static shit
    println!("{}", reporter.lock().data_collector.get_statics());

    // Todo: make these run on a loop with unique intervals for each
    // that the user can set in a config
    let reporter = reporter.clone();
    let data_collection_handle = spawn(move || loop {
        let mut reporter = reporter.lock();
        println!("{}", reporter.data_collector.get_disks());
        println!("{}", reporter.data_collector.get_cpu());
        println!("{}", reporter.data_collector.get_ram());
        println!("{}", reporter.data_collector.get_network());
        thread::sleep(time::Duration::from_millis(1000));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
