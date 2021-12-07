use crate::data_collector::DataCollector;
use anyhow::Result;

#[derive(Debug)]
pub struct Reporter {
    pub data_collector: DataCollector,
    pub version: String,
}

impl Reporter {
    pub fn new() -> Result<Self> {
        let data_collector: DataCollector = DataCollector::new()?;
        let version: String = env!("CARGO_PKG_VERSION").to_string();

        return Ok(Self {
            data_collector,
            version,
        });
    }
}
