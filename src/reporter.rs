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

    // pub fn login() -> Result<()> {
    //     login
    // }

    // pub fn send_stats(&mut self) -> Result<()> {
    //     if *self.is_connected.lock() {
    //         self.websocket.send_message(&Message::text(
    //             &json!({
    //                 "e": 0x04,
    //                 "cpu": self.data_collector.get_cpu()?,
    //                 "ram": self.data_collector.get_ram()?,
    //                 "gpu": self.data_collector.get_gpu()?,
    //                 "processes": self.data_collector.get_total_process_count()?.to_string(),
    //                 "disks": self.data_collector.get_disks()?,
    //             })
    //             .to_string(),
    //         ))?;
    //     }

    //     return Ok(());
    // }
}
