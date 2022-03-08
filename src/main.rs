use anyhow::Result;
use core::time;
use std::thread;
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

  loop {
    if !reporter.lock().args.silent {
      Ui::new(&args.prefix, args.no_clear, reporter.clone());
    }

    match reporter.lock().update_dynamic_data().await {
      Ok(_) => {}
      Err(e) => {
        println!("{}", e);
        thread::sleep(time::Duration::from_secs(1));
      }
    }

    match reporter.lock().send_dynamic_data().await {
      Ok(_) => {
        println!("{}", "Xornet Reporter Sending Data...")
      }
      Err(e) => {
        eprintln!("Error while sending dynamic data: {}", e);
      }
    }

    thread::sleep(time::Duration::from_secs_f64(reporter.lock().args.interval));
  }
}
