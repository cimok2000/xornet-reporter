use core::time;
use std::{
    sync::Arc,
    thread::{self, spawn},
};

use parking_lot::Mutex;
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
    /// Creates a new data collector
    fn new() -> Self {
        let fetcher = System::new_all();
        return Self { fetcher };
    }

    /// Gets all the static information about the system
    /// that can't change in runtime
    pub fn get_statics(&self) -> Value {
        let processor_info = self.fetcher.global_processor_info();
        let mut disks = vec![];

        for disk in self.fetcher.disks() {
            let json = json!({
              "name": format!("{:?}", disk.name()),
              "filesystem": format!("{:?}", disk.file_system()),
              "type": format!("{:?}", disk.type_()),
              "mount":format!("{:?}", disk.mount_point()),
              "total": disk.total_space(),
            });

            disks.push(json);
        }

        return json!({
          "cpu": {
            "name": processor_info.name(),
            "vendor_id": processor_info.vendor_id(),
            "brand": processor_info.brand(),
          },
          "disks": Value::Array(disks)
        });
    }

    /// Gets the current CPU stats
    pub fn get_cpu(&mut self) -> Value {
        let mut serialized_processors = Vec::new();
        self.fetcher.refresh_cpu();

        for processor in self.fetcher.processors() {
            let json = json!({
              "cpu_usage": processor.cpu_usage(),
              "frequency": processor.frequency(),
            });

            serialized_processors.push(json);
        }

        return Value::Array(serialized_processors);
    }

    /// Gets the current RAM stats
    pub fn get_ram(&mut self) -> Value {
        self.fetcher.refresh_memory();

        return json!({
          "free_memory": self.fetcher.free_memory(),
          "available_memory": self.fetcher.available_memory(),
          "used_memory": self.fetcher.used_memory(),
          "total_memory": self.fetcher.total_memory(),
        });
    }

    /// Gets the current DISKS stats
    pub fn get_disks(&self) -> Value {
        let mut serialized_disks = Vec::new();

        for disk in self.fetcher.disks() {
            let json = json!({
              "free": disk.available_space(),
            });

            serialized_disks.push(json);
        }
        return Value::Array(serialized_disks);
    }
}

fn main() {
    let reporter = Arc::new(Mutex::new(Reporter::new()));

    // Get all static shit
    println!("{}", reporter.lock().data_collector.get_statics());

    // Todo: make these run on a loop with unique intervals for each
    // that the user can set in a config
    let reporter = reporter.clone();
    let data_collection_handle = spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", reporter.lock().data_collector.get_disks());
        println!("{}", reporter.lock().data_collector.get_cpu());
        println!("{}", reporter.lock().data_collector.get_ram());
    });

    data_collection_handle
        .join()
        .expect("data_collection panicked");
}
