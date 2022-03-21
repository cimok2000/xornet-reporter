use std::process::Command;

use crate::types::DockerStats;
use anyhow::Result;

use super::{DataCollector, DataCollectorError};

impl DataCollector {
  pub fn get_docker_stats(&mut self) -> Result<Vec<DockerStats>> {
    let command = Command::new("docker")
      .args([
        "stats",
        "--no-stream",
        "--format",
        "{\"container\":\"{{ .Container }}\",\"name\":\"{{ .Name }}\",\"memory\":{\"raw\":\"{{ .MemUsage }}\",\"percent\":\"{{ .MemPerc }}\"},\"cpu\":\"{{ .CPUPerc }}\"}",
      ])
      .output()?;

    // trim the whitespace from stdout
    let output_string = String::from_utf8(command.stdout)?.trim().to_string();
    let mut stats: Vec<DockerStats> = Vec::new();

    for line in output_string.split("\n") {
      let stat = serde_json::from_str(line);
      if let Ok(stat) = stat {
        stats.push(stat);
      }
    }

    // parse the output as DockerStats struct from the json
    return match command.status.code() {
      Some(0) => Ok(stats),
      Some(_code) => Err(DataCollectorError::NoDockerStats)?,
      None => Err(DataCollectorError::NoDockerStats)?,
    };
  }
}
