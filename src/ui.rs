use anyhow::Result;
use colored::Colorize;
use parking_lot::Mutex;
use std::{io::Write, sync::Arc};

use crate::{
  data_collector::DataCollectorError,
  reporter::Reporter,
  util::{self, bytes_to_gb, bytes_to_kb, clear_screen, parse_time},
};

pub struct Ui {
  prefix: String,
  reporter: Arc<Mutex<Reporter>>,
}

impl Ui {
  pub fn new(prefix: &str, no_clear: bool, reporter: Arc<Mutex<Reporter>>) -> Self {
    let mut this: Self = Self {
      prefix: prefix.to_string(),
      reporter,
    };
    let attempts = [
      this.get_uptimes(),
      this.get_cpu(),
      this.get_memory(),
      this.get_process_count(),
      this.get_gpu(),
      this.get_nics(),
      this.get_disks(),
      this.get_temps(),
      this.get_version(),
    ];

    let mut string = "".to_string();
    let mut errors = "".to_string();

    // Handle errors from the data collector here
    for attempt in attempts {
      match attempt {
        Ok(data) => string.push_str(&(data + "\n")),
        Err(err) => errors.push_str(&format!("\n {} {}", prefix, &err).to_string()),
      }
    }
    clear_screen();
    println!("{} {}", string, errors.bright_black());

    std::io::stdout().flush().expect("Couldn't flush stdout");

    // Reset the cursor back to 0, 0
    if !no_clear {
      util::reset_cursor();
    };

    return this;
  }

  pub fn get_uptimes(&mut self) -> Result<String> {
    let reporter_uptime = self.reporter.lock().dynamic_data.reporter_uptime;
    let uptime = self.reporter.lock().dynamic_data.host_uptime;
    Ok(format!(
      " {} {} {} {} {}",
      self.prefix.green(),
      "Uptime:".bright_black(),
      parse_time(uptime).bright_black(),
      "Reporter Uptime:".bright_black(),
      parse_time(reporter_uptime).bright_black()
    ))
  }

  pub fn get_cpu(&mut self) -> Result<String> {
    let cpus = self.reporter.lock().dynamic_data.cpu.clone();
    let mut cpu_usage: u16 = 0;
    for i in 1..cpus.usage.len() {
      cpu_usage = cpu_usage + cpus.usage[i] as u16;
    }
    cpu_usage = cpu_usage / cpus.usage.len() as u16;

    Ok(format!(
      " {} {}       {:.5}{} ",
      self.prefix.red(),
      "CPU".bright_black(),
      format!("{}", cpu_usage).red(),
      "%".bright_black()
    ))
  }

  pub fn get_gpu(&mut self) -> Result<String> {
    let gpu = self
      .reporter
      .lock()
      .dynamic_data
      .gpu
      .clone()
      .ok_or(DataCollectorError::NoGPU)?;

    let gpu_power_usage = format!("{}", gpu.power_usage);
    let gpu_usage = format!("{}", gpu.gpu_usage);

    return Ok(format!(
      " {} {}       {}{} {:.5}{}",
      self.prefix.cyan(),
      "GPU".bright_black(),
      gpu_usage.cyan(),
      "%".bright_black(),
      gpu_power_usage.cyan(),
      "mW".bright_black(),
    ));
  }

  pub fn get_process_count(&mut self) -> Result<String> {
    let proc_count = format!(
      "{}",
      self.reporter.lock().dynamic_data.process_count.to_string()
    );
    return Ok(format!(
      " {} {} {} ",
      self.prefix.green(),
      "Processes".bright_black(),
      proc_count.green()
    ));
  }

  pub fn get_memory(&mut self) -> Result<String> {
    let used_swap = format!(
      "{}",
      bytes_to_kb(self.reporter.lock().dynamic_data.swap.used)
    );
    let total_swap = format!(
      "{}",
      bytes_to_kb(self.reporter.lock().dynamic_data.swap.total)
    );
    let used_memory = format!(
      "{}",
      bytes_to_kb(self.reporter.lock().dynamic_data.ram.used)
    );
    let total_memory = format!(
      "{}",
      bytes_to_kb(self.reporter.lock().dynamic_data.ram.total)
    );

    return Ok(format!(
      " {} {}    {} {} {} {}\n {} {}      {} {} {} {} ",
      self.prefix.yellow(),
      "Memory".bright_black(),
      used_memory.yellow(),
      "/".bright_black(),
      total_memory.yellow(),
      "MB".bright_black(),
      self.prefix.yellow(),
      "Swap".bright_black(),
      used_swap.yellow(),
      "/".bright_black(),
      total_swap.yellow(),
      "MB".bright_black()
    ));
  }

  pub fn get_nics(&mut self) -> Result<String> {
    let nics_header = format!(" {} {} \n", self.prefix.blue(), "NICs".bright_black());
    let nics = self.reporter.lock().dynamic_data.network.clone();

    let mut nics_info = String::new();
    nics_info.push_str(&nics_header);
    for i in 0..nics.len() {
      let nic = &nics[i];

      // Network
      let rx = format!("{}", nic.rx);
      let tx = format!("{}", nic.tx);
      let speed = format!("{}", nic.s);
      let name = &nic.n;

      nics_info.push_str(&format!(
        "     {}  {} {} {} {} {} {}\n",
        name.bright_black(),
        rx.blue(),
        "rx".bright_black(),
        tx.blue(),
        "tx".bright_black(),
        speed.blue(),
        "Mbps".bright_black()
      ));
    }

    return Ok(nics_info.trim_end().to_string());
  }

  pub fn get_disks(&mut self) -> Result<String> {
    let disks_header = format!(" {} {} \n", self.prefix.magenta(), "Disks".bright_black());
    let disks = self.reporter.lock().dynamic_data.disks.clone();

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

  pub fn get_temps(&mut self) -> Result<String> {
    let mut temp_list = String::new();
    temp_list.push_str(&format!(
      " {} {} \n",
      self.prefix.bright_purple(),
      "Temperatures".bright_black(),
    ));
    let temps = self
      .reporter
      .lock()
      .dynamic_data
      .temps
      .clone()
      .ok_or(DataCollectorError::NoTemp)?;
    for i in 0..temps.len() {
      temp_list.push_str(&format!(
        "     {} \t{}{}\n",
        temps[i].label.bright_black(),
        temps[i].value.to_string().purple(),
        "°C".bright_black()
      ));
    }
    return Ok(temp_list.trim_end().to_string());
  }

  pub fn get_version(&mut self) -> Result<String> {
    return Ok(
      format!(
        "\n {} Xornet Reporter v{} ",
        self.prefix,
        env!("CARGO_PKG_VERSION")
      )
      .bright_black()
      .to_string(),
    );
  }
}
