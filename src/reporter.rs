use crate::arg_parser::ArgParser;
use crate::config_manager::ConfigManager;
use crate::data_collector::DataCollector;
use crate::types::DynamicData;
use crate::websocket_manager::{WebsocketEvent, WebsocketManager};
use anyhow::Result;

pub struct Reporter {
  pub data_collector: DataCollector,
  pub version: String,
  pub config_manager: ConfigManager,
  pub websocket_manager: Option<WebsocketManager>,
  pub args: ArgParser,
  pub dynamic_data: DynamicData,
}

impl Reporter {
  pub async fn new() -> Result<Self> {
    let args = ArgParser::new().await?;
    let websocket_manager: Option<WebsocketManager> = None;

    let config_manager: ConfigManager = ConfigManager::new()?;
    let mut data_collector: DataCollector = DataCollector::new()?;
    let version: String = env!("CARGO_PKG_VERSION").to_string();
    let dynamic_data: DynamicData = DynamicData {
      cpu: data_collector.get_cpu()?,
      ram: data_collector.get_ram()?,
      swap: data_collector.get_swap()?,
      gpu: data_collector.get_gpu().ok(),
      process_count: data_collector.get_total_process_count()? as i32,
      disks: data_collector.get_disks()?,
      temps: data_collector.get_temps().ok(),
      network: data_collector.get_network()?,
      host_uptime: data_collector.get_uptime()?,
      reporter_uptime: data_collector.get_reporter_uptime()?,
    };

    let mut this = Self {
      data_collector,
      version,
      websocket_manager,
      config_manager,
      args,
      dynamic_data,
    };

    if !this.args.offline {
      this.init_connection()?;
      this.send_static_data().await?;
    }

    return Ok(this);
  }

  pub fn init_connection(&mut self) -> Result<()> {
    let websocket_url: String = format!(
      "wss://{}/reporter",
      if self.args.use_local_backend {
        String::from("localhost:7000")
      } else {
        self.config_manager.config.backend_hostname.to_owned()
      }
    );
    self.websocket_manager = Some(WebsocketManager::new(&websocket_url)?);
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
    if let Some(websocket_manager) = self.websocket_manager.as_mut() {
      let static_data = self.data_collector.get_statics().await?;
      websocket_manager.send(WebsocketEvent::StaticData {
        hostname: static_data.hostname,
        public_ip: static_data.public_ip,
        cpu_model: static_data.cpu_model,
        os_version: static_data.os_version,
        os_name: static_data.os_name,
        cpu_cores: static_data.cpu_cores,
        cpu_threads: static_data.cpu_threads,
        total_mem: static_data.total_mem,
        reporter_version: self.version.clone(),
      })?;
    }

    Ok(())
  }

  pub async fn update_dynamic_data(&mut self) -> Result<()> {
    self.dynamic_data = DynamicData {
      cpu: self.data_collector.get_cpu()?,
      ram: self.data_collector.get_ram()?,
      swap: self.data_collector.get_swap()?,
      gpu: self.data_collector.get_gpu().ok(),
      process_count: self.data_collector.get_total_process_count()? as i32,
      disks: self.data_collector.get_disks()?,
      temps: self.data_collector.get_temps().ok(),
      network: self.data_collector.get_network()?,
      host_uptime: self.data_collector.get_uptime()?,
      reporter_uptime: self.data_collector.get_reporter_uptime()?,
    };
    return Ok(());
  }

  pub async fn send_dynamic_data(&mut self) -> Result<()> {
    if let Some(websocket_manager) = self.websocket_manager.as_mut() {
      let dd = self.dynamic_data.clone();
      if let Err(e) = websocket_manager.send(WebsocketEvent::DynamicData {
        cpu: dd.cpu,
        ram: dd.ram,
        swap: dd.swap,
        gpu: dd.gpu,
        process_count: dd.process_count,
        disks: dd.disks,
        temps: dd.temps,
        network: dd.network,
        host_uptime: dd.host_uptime,
        reporter_uptime: dd.reporter_uptime,
      }) {
        eprintln!("Websocket error: {}", e);
        self.init_connection()?;
        self.send_static_data().await?;
      }
      self.data_collector.increment_iterator_index();
    }

    Ok(())
  }
}
