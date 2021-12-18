use anyhow::Result;
use arg_parser::ArgParser;
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
  // The bad boys
  // Asking for trouble and making it double
  let args = ArgParser::new().await?;
  let reporter = arcmutex(Reporter::new().await?);
  reporter.lock().login()?;
  reporter.lock().send_static_data().await?;

  // Setup the terminal
  util::setup_terminal();

  if args.silent {
    println!("Xornet Reporter Started");
  }

  let data_collection_handle = spawn(move || loop {
    if !args.silent {
      let _ui = Ui::new(&args.prefix, args.no_clear, reporter.clone());
    }

    if !args.offline {
      reporter.lock().send_dynamic_data();
    }

    thread::sleep(time::Duration::from_secs_f64(args.interval));
  });

  data_collection_handle.join().expect("main panicked");
  Ok(())
}
