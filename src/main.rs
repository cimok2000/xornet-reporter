use arg_parser::ArgParser;
use core::time;
use std::thread::{self, spawn};
use ui::Ui;
use util::arcmutex;

extern crate nvml_wrapper as nvml;

mod arg_parser;
mod data_collector;
mod reporter;
mod types;
mod ui;
mod util;
use crate::reporter::Reporter;

fn main() {
    // Get arguments from launch
    let args = ArgParser::new();

    // Setup the terminal
    util::setup_terminal();

    // Start the reporter
    let reporter = arcmutex(Reporter::new().unwrap());

    if args.silent {
        println!("Xornet Reporter Started");
    }

    let data_collection_handle = spawn(move || loop {
        if args.silent {
            return;
        }

        let _ui = Ui::new(&args.prefix, args.no_clear, reporter.clone());

        thread::sleep(time::Duration::from_secs_f64(args.interval));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
