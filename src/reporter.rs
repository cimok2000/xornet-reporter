use crate::arg_parser::ArgParser;
use crate::config_manager::ConfigManager;
use crate::data_collector::DataCollector;
use crate::websocket_manager::{WebsocketEvent, WebsocketManager};
use anyhow::Result;

pub struct Reporter {
  pub data_collector: DataCollector,
  pub version: String,
  pub config_manager: ConfigManager,
  pub websocket_manager: Option<WebsocketManager>,
  pub args: ArgParser,
}

impl Reporter {
  pub async fn new() -> Result<Self> {
    let args = ArgParser::new().await?;
    let websocket_manager: Option<WebsocketManager>;

    let config_manager: ConfigManager = ConfigManager::new()?;
    let data_collector: DataCollector = DataCollector::new()?;
    let version: String = env!("CARGO_PKG_VERSION").to_string();

    websocket_manager = None;

    let mut this: Self = Self {
      data_collector,
      version,
      websocket_manager,
      config_manager,
      args: args,
    };

    if !this.args.offline {
      this.init_connection()?;
      this.send_static_data().await?;
    }

    return Ok(this);
  }

  pub fn init_connection(&mut self) -> Result<()> {
    self.websocket_manager = Some(WebsocketManager::new(&format!(
      "wss://{}/reporter",
      self.config_manager.config.backend_hostname
    ))?);
    self.login()?;
    Ok(())
  }

  pub fn login(&mut self) -> Result<()> {
    match self.websocket_manager.as_mut() {
      Some(websocket_manager) => {
        websocket_manager.send(WebsocketEvent::Login {
          auth_token: self.config_manager.config.access_token.to_string(),
        })?;
      }
      None => {}
    }

    return Ok(());
  }

  pub async fn send_static_data(&mut self) -> Result<()> {
    let static_data = self.data_collector.get_statics().await?;

    // This is is kinda troll
    match self.websocket_manager.as_mut() {
      Some(websocket_manager) => {
        websocket_manager.send(WebsocketEvent::StaticData {
          hostname: static_data.hostname,
          public_ip: static_data.public_ip,
          cpu_model: static_data.cpu_model,
          os_version: static_data.os_version,
          os_name: static_data.os_name,
          cpu_cores: static_data.cpu_cores,
          cpu_threads: static_data.cpu_threads,
          total_mem: static_data.total_mem,
        })?;
      }
      None => {}
    }

    return Ok(());
  }

  pub fn send_dynamic_data(&mut self) -> Result<()> {
    match self.websocket_manager.as_mut() {
      Some(websocket_manager) => {
        let status = websocket_manager.send(WebsocketEvent::DynamicData {
          cpu: self.data_collector.get_cpu()?,
          ram: self.data_collector.get_ram()?,
          gpu: self.data_collector.get_gpu().ok(),
          processes: self.data_collector.get_total_process_count()?.to_string(),
          disks: self.data_collector.get_disks()?,
          temps: self.data_collector.get_temps().ok(),
          network: self.data_collector.get_network()?,
        });
        match status {
          Ok(_) => {}
          Err(e) => {
            eprintln!("Websocket error: {}", e);
            self.init_connection()?;
            self.send_static_data();
          }
        }
      }
      None => {}
    }

    return Ok(());
  }
}
