use crate::types::CPUStats;
use anyhow::Result;
use sysinfo::{ProcessorExt, SystemExt};

use super::DataCollector;

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

    return Ok(CPUStats { usage, freq });
  }
}
