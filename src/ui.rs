use anyhow::Result;
use colored::Colorize;
use parking_lot::Mutex;
use std::{io::Write, sync::Arc};

use crate::{
  data_collector::DataCollector,
  reporter::Reporter,
  util::{self, bytes_to_gb, bytes_to_kb, bytes_to_mb},
};

pub struct Ui {}

impl Ui {
  pub fn get_cpu(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let cpus = reporter.lock().data_collector.get_cpu()?;
    let mut cpu_usage: u16 = 0;
    for i in 1..cpus.usage.len() {
      cpu_usage = cpu_usage + cpus.usage[i] as u16;
    }
    cpu_usage = cpu_usage / cpus.usage.len() as u16;

    Ok(format!(
      " {} {}       {:.5}{} ",
      prefix.red(),
      "CPU".bright_black(),
      format!("{}", cpu_usage).red(),
      "%".bright_black()
    ))
  }

  pub fn get_gpu(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let gpu = reporter.lock().data_collector.get_gpu()?;
    let gpu_power_usage = format!("{}", gpu.power_usage);
    let gpu_vram_used = format!("{}", bytes_to_mb(gpu.mem_used));
    let gpu_vram_total = format!("{}", bytes_to_mb(gpu.mem_total));

    return Ok(format!(
      " {} {}       {:.5}{} {} {} {} {}",
      prefix.cyan(),
      "GPU".bright_black(),
      gpu_power_usage.cyan(),
      "mW".bright_black(),
      gpu_vram_used.cyan(),
      "/".bright_black(),
      gpu_vram_total.cyan(),
      "MB".bright_black(),
    ));
  }

  pub fn get_process_count(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let proc_count = format!(
      "{}",
      reporter
        .lock()
        .data_collector
        .get_total_process_count()?
        .to_string()
    );
    return Ok(format!(
      " {} {} {} ",
      prefix.green(),
      "Processes".bright_black(),
      proc_count.green()
    ));
  }

  pub fn get_memory(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let used_memory = format!(
      "{}",
      bytes_to_kb(reporter.lock().data_collector.get_ram()?.used)
    );
    let total_memory = format!(
      "{}",
      bytes_to_kb(reporter.lock().data_collector.get_ram()?.total),
    );

    return Ok(format!(
      " {} {}    {} {} {} {} ",
      prefix.yellow(),
      "Memory".bright_black(),
      used_memory.yellow(),
      "/".bright_black(),
      total_memory.yellow(),
      "MB".bright_black()
    ));
  }

  pub fn get_nics(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let nics_header = format!(" {} {} \n", prefix.cyan(), "NICs".bright_black());
    let nics = reporter.lock().data_collector.get_network()?;

    let mut nics_info = String::new();
    nics_info.push_str(&nics_header);
    for i in 0..nics.len() {
      let nic = &nics[i];

      // Network
      let rx = format!("{}", nic.rx);
      let tx = format!("{}", nic.tx);
      let name = &nic.name;

      nics_info.push_str(&format!(
        "     {}  {} {} {} {}\n",
        name.bright_black(),
        rx.blue(),
        "rx".bright_black(),
        tx.blue(),
        "tx".bright_black(),
      ));
    }

    return Ok(nics_info.trim_end().to_string());
  }

  pub fn get_disks(prefix: &str, reporter: Arc<Mutex<Reporter>>) -> Result<String> {
    let disks_header = format!(" {} {} \n", prefix.magenta(), "Disks".bright_black());
    let disks = reporter.lock().data_collector.get_disks()?;

    let mut disks_list = String::new();
    disks_list.push_str(&disks_header);
    for disk in disks {
      let disk_name = &disk.mount.replace("\\", "");
      // Disk
      let used_disk = format!("{}", bytes_to_gb(disk.used));
      let total_disk = format!("{}", bytes_to_gb(disk.total));
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

    return Ok(disks_list.trim_end().to_string());
  }

  pub fn get_uuids(prefix: &str) -> Result<String> {
    return Ok(format!(
      " {} {} {} ",
      prefix.bright_black(),
      "Hardware UUID".bright_black(),
      DataCollector::get_hardware_uuid()?.bright_black()
    ));
  }

  pub fn header() -> Result<String> {
    return Ok(
      format!(" Xornet Reporter v{} ", env!("CARGO_PKG_VERSION"))
        .bright_black()
        .to_string(),
    );
  }

  pub fn new(prefix: &str, no_clear: bool, reporter: Arc<Mutex<Reporter>>) -> Self {
    let attempts = [
      Ui::header(),
      Ui::get_cpu(prefix, reporter.clone()),
      Ui::get_memory(prefix, reporter.clone()),
      Ui::get_process_count(prefix, reporter.clone()),
      Ui::get_gpu(prefix, reporter.clone()),
      Ui::get_nics(prefix, reporter.clone()),
      Ui::get_disks(prefix, reporter.clone()),
      Ok("".to_string()),
      Ui::get_uuids(prefix),
    ];

    let mut string = "".to_string();

    // Handle errors from the data collector here
    for attempt in attempts {
      match attempt {
        Ok(data) => string.push_str(&(data + "\n")),
        Err(err) => println!("{:?}", err),
      }
    }

    println!("{}", string);

    std::io::stdout().flush().expect("Couldn't flush stdout");

    // Reset the cursor back to 0, 0
    if !no_clear {
      util::reset_cursor();
    };

    return Self {};
  }
}
