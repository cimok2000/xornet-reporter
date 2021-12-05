use core::time;
use crossterm::style::Print;
use std::thread::{self, spawn};
use ui::Ui;
use util::{arcmutex, LaunchParams};

mod data_collector;
mod info_box;
mod reporter;
mod ui;
mod util;
use crate::reporter::Reporter;

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

    if args.silent {
        println!("Xornet Reporter Started");
    }

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
        let reporter = reporter.lock();
        if !args.silent {
            let ui = Ui::new(&prefix, show_border, args.no_clear, reporter);
        }
        // Wait for interval
        thread::sleep(time::Duration::from_secs_f64(args.interval));
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
