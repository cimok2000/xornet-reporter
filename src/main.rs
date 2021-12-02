use serde_json::{json, Value};
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

#[derive(Debug)]
struct Reporter {
    pub data_collector: DataCollector,
    pub version: String,
}

impl Reporter {
    fn new() -> Self {
        let data_collector: DataCollector = DataCollector::new();
        let version: String = env!("CARGO_PKG_VERSION").to_string();

        return Self {
            data_collector,
            version,
        };
    }
}

#[derive(Debug)]
struct DataCollector {
    pub fetcher: System,
}

impl DataCollector {
    fn new() -> Self {
        let fetcher = System::new_all();
        return Self { fetcher };
    }

    /// Gets all the static information about the system
    /// that can't change in runtime
    pub fn get_statics(&self) {
        todo!();
    }

    /// Gets the current CPU stats
    pub fn get_cpu(&mut self) -> Vec<Value> {
        let mut serialized_processors = Vec::new();
        self.fetcher.refresh_cpu();

        for processor in self.fetcher.processors() {
            let json = json!({
              // "name": processor.name(),
              "cpu_usage": processor.cpu_usage().to_string(),
              // "vendor_id": processor.vendor_id(),
              // "brand": processor.brand(),
              "frequency": processor.frequency().to_string(),
            });

            serialized_processors.push(json);
        }

        return serialized_processors;
    }

    pub fn get_disks(&self) -> Vec<Value> {
        let mut serialized_disks = Vec::new();

        for disk in self.fetcher.disks() {
            let json = json!({
              "name": format!("{:?}", disk.name()),
              "filesystem": format!("{:?}", disk.file_system()),
              "type": format!("{:?}", disk.type_()),
              "mount":format!("{:?}", disk.mount_point()),
              "total": disk.total_space().to_string(),
              "free": disk.available_space().to_string(),
            });

            serialized_disks.push(json);
        }

        return serialized_disks;
    }
}

fn main() {
    let mut reporter: Reporter = Reporter::new();

    println!("{:?}", reporter.data_collector.get_disks());
    println!("{:?}", reporter.data_collector.get_cpu());
}
