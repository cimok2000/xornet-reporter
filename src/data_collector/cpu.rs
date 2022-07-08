#[cfg(target_family = "unix")]
use sysinfo::{ProcessorExt, SystemExt};

use crate::types::CPUStats;

use anyhow::Result;



use super::DataCollector;

#[cfg(target_family = "unix")]
impl DataCollector {
  /// Gets the current CPU stats
  /// wait what the fuck this is an array of cores? 🥴👍
  pub fn get_cpu(&mut self) -> Result<CPUStats> {
    let (mut usage, mut freq) = (vec![], vec![]);

    for processor in self.fetcher.processors() {
      usage.push(processor.cpu_usage().floor() as u16);
      freq.push(processor.frequency() as u16);
    }

    self.fetcher.refresh_cpu();

    Ok(CPUStats { usage, freq })
  }
}

#[cfg(target_family = "windows")]
impl DataCollector {
  pub fn get_cpu(&mut self) -> Result<CPUStats> {
    let (mut usage, mut freq) = (vec![], vec![]);
    // TODO: Implement use of perfmon querying.
    //       We need to figure out where to store perfmon structures.
    Ok(CPUStats { usage, freq })
  }
}