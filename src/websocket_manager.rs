use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::TcpStream;
use std::sync::Arc;
use websocket::sync::stream::TlsStream;
use websocket::sync::Client;
use websocket::{ClientBuilder, Message};

use crate::types::{CPUStats, DiskStats, GPUStats, NetworkInterfaceStats, RAMStats, TempStats};
use crate::util::arcmutex;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketEvent {
  Login {
    auth_token: String,
  },
  DynamicData {
    cpu: CPUStats,
    ram: RAMStats,
    gpu: Option<GPUStats>,
    processes: String,
    disks: Vec<DiskStats>,
    temps: Option<Vec<TempStats>>,
    network: Vec<NetworkInterfaceStats>,
  },
  StaticData {
    hostname: Option<String>,
    public_ip: String,
    cpu_model: String,
    os_version: Option<String>,
    os_name: Option<String>,
    cpu_cores: Option<usize>,
    cpu_threads: usize,
    total_mem: u64,
  },
}

pub fn get_event_id(ev: &WebsocketEvent) -> &str {
  match ev {
    WebsocketEvent::Login { .. } => "login",
    WebsocketEvent::StaticData { .. } => "staticData",
    WebsocketEvent::DynamicData { .. } => "dynamicData",
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketMessage {
  e: WebsocketEvent,
  data: Value,
}

pub struct WebsocketManager {
  pub websocket_url: String,
  pub websocket: Arc<Mutex<Client<TlsStream<TcpStream>>>>,
}

impl WebsocketManager {
  pub fn new(websocket_url: &str) -> Result<Self> {
    let websocket = arcmutex(ClientBuilder::new(websocket_url)?.connect_secure(None)?);

    return Ok(Self {
      websocket_url: websocket_url.to_string(),
      websocket,
    });
  }

  pub fn send(&mut self, data: WebsocketEvent) -> Result<()> {
    let message = Message::text(
      json!({
          "e": get_event_id(&data),
          "d": &data,
      })
      .to_string(),
    );

    Ok(self.websocket.lock().send_message(&message)?)
  }
}
