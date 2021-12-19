use anyhow::Result;
use core::time;
use std::thread::{self, spawn};
use ui::Ui;
use util::arcmutex;

extern crate nvml_wrapper as nvml;

mod arg_parser;
mod auth_manager;
mod config_manager;
mod data_collector;
mod reporter;
mod types;
mod ui;
mod util;
mod websocket_manager;
use crate::reporter::Reporter;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new instance of the reporter
  let reporter = arcmutex(Reporter::new().await?);
  let args = reporter.lock().args.clone();

  // Setup the terminal
  util::setup_terminal();

  let data_collection_handle = spawn(move || loop {
    if !reporter.lock().args.silent {
      Ui::new(&args.prefix, args.no_clear, reporter.clone());
    }

    if !reporter.lock().args.offline {
      reporter.lock().send_dynamic_data();
    }

    thread::sleep(time::Duration::from_secs_f64(reporter.lock().args.interval));
  });
  data_collection_handle.join().expect("main panicked");
  Ok(())
}
