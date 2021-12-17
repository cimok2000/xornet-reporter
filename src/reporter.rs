use crate::config_manager::ConfigManager;
use crate::data_collector::DataCollector;
use crate::util::arcmutex;
use anyhow::Result;
use parking_lot::Mutex;
use serde_json::json;
use std::net::TcpStream;
use std::sync::Arc;
use websocket::sync::Client;
use websocket::{ClientBuilder, Message};
extern crate machine_uid;

pub struct Reporter {
    pub data_collector: DataCollector,
    pub version: String,
    pub websocket: Client<TcpStream>,
    pub is_connected: Arc<Mutex<bool>>,
    pub config_manager: ConfigManager,
}

impl Reporter {
    pub async fn new() -> Result<Self> {
        let config_manager: ConfigManager = ConfigManager::new()?;
        let data_collector: DataCollector = DataCollector::new()?;
        let version: String = env!("CARGO_PKG_VERSION").to_string();
        let statics = data_collector.get_statics().await?;
        let is_connected = arcmutex(false);

        let mut websocket = ClientBuilder::new("ws://localhost:8000")?.connect_insecure()?;
        *is_connected.lock() = true;

        if !config_manager.config.access_token.is_empty() {
            websocket.send_message(&Message::text(
                &json!({
                    "e": 0x01,
                    "access_token": &config_manager.config.access_token,
                })
                .to_string(),
            ))?;
        }

        // websocket.send_message(&Message::text(
        //     &json!({
        //         "e": 0x03,
        //         "version": &version,
        //         "name": "Xornet Reporter",
        //         "statics": statics,
        //     })
        //     .to_string(),
        // ))?;

        return Ok(Self {
            data_collector,
            version,
            websocket,
            is_connected,
            config_manager,
        });
    }

    pub fn send_stats(&mut self) -> Result<()> {
        if *self.is_connected.lock() {
            self.websocket.send_message(&Message::text(
                &json!({
                    "e": 0x04,
                    "cpu": self.data_collector.get_cpu()?,
                    "ram": self.data_collector.get_ram()?,
                    "gpu": self.data_collector.get_gpu()?,
                    "processes": self.data_collector.get_total_process_count()?.to_string(),
                    "disks": self.data_collector.get_disks()?,
                })
                .to_string(),
            ))?;
        }

        return Ok(());
    }
}
