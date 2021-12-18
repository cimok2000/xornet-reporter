use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::TcpStream;
use std::sync::Arc;
use websocket::sync::Client;
use websocket::{ClientBuilder, Message};

use crate::types::{CPUStats, DiskStats, GPUStats, RAMStats};
use crate::util::arcmutex;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketEvent {
  Login {
    access_token: String,
  },
  DynamicData {
    cpu: CPUStats,
    ram: RAMStats,
    gpu: GPUStats,
    processes: String,
    disks: Vec<DiskStats>,
  },
  StaticData {
    hostname: Option<String>,
    public_ip: String,
    cpu_model: String,
    os_version: Option<String>,
    cpu_cores: Option<usize>,
    cpu_threads: usize,
    total_mem: u64,
  },
}

pub fn get_event_id(ev: &WebsocketEvent) -> u16 {
  match ev {
    WebsocketEvent::Login { .. } => 0x01,
    WebsocketEvent::StaticData { .. } => 0x04,
    WebsocketEvent::DynamicData { .. } => 0x05,
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketMessage {
  e: WebsocketEvent,
  data: Value,
}

pub struct WebsocketManager {
  pub websocket_url: String,
  pub websocket: Arc<Mutex<Client<TcpStream>>>,
}

impl WebsocketManager {
  pub fn new(websocket_url: &str) -> Result<Self> {
    let websocket = arcmutex(ClientBuilder::new(websocket_url)?.connect_insecure()?);

    return Ok(Self {
      websocket_url: websocket_url.to_string(),
      websocket,
    });
  }

  pub fn send(&mut self, data: WebsocketEvent) -> Result<()> {
    let message = Message::text(
      json!({
          "e": get_event_id(&data),
          "data": &data,
      })
      .to_string(),
    );

    Ok(self.websocket.lock().send_message(&message)?)
  }
}
