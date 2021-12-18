use crate::config_manager::ConfigManager;
use crate::data_collector::DataCollector;
use crate::websocket_manager::{WebsocketEvent, WebsocketManager};
use anyhow::Result;
extern crate machine_uid;

pub struct Reporter {
    pub data_collector: DataCollector,
    pub version: String,
    pub config_manager: ConfigManager,
    pub websocket_manager: WebsocketManager,
}

impl Reporter {
    pub async fn new() -> Result<Self> {
        let websocket_manager: WebsocketManager = WebsocketManager::new("ws://localhost:8000")?;
        let config_manager: ConfigManager = ConfigManager::new()?;
        let data_collector: DataCollector = DataCollector::new()?;
        let version: String = env!("CARGO_PKG_VERSION").to_string();
        let statics = data_collector.get_statics().await?;

        return Ok(Self {
            data_collector,
            version,
            websocket_manager,
            config_manager,
        });
    }

    pub fn login(&mut self) -> Result<()> {
        self.websocket_manager.send(WebsocketEvent::Login {
            access_token: self.config_manager.config.access_token.to_string(),
        })?;

        return Ok(());
    }

    pub async fn send_static_data(&mut self) -> Result<()> {
        let static_data = self.data_collector.get_statics().await?;

        // This is is kinda troll
        self.websocket_manager.send(WebsocketEvent::StaticData {
            hostname: static_data.hostname,
            public_ip: static_data.public_ip,
            cpu_model: static_data.cpu_model,
            os_version: static_data.os_version,
            cpu_cores: static_data.cpu_cores,
            cpu_threads: static_data.cpu_threads,
            total_mem: static_data.total_mem,
        })?;
        return Ok(());
    }

    pub fn send_dynamic_data(&mut self) -> Result<()> {
        self.websocket_manager.send(WebsocketEvent::DynamicData {
            cpu: self.data_collector.get_cpu()?,
            ram: self.data_collector.get_ram()?,
            gpu: self.data_collector.get_gpu()?,
            processes: self.data_collector.get_total_process_count()?.to_string(),
            disks: self.data_collector.get_disks()?,
        })?;
        return Ok(());
    }
}
