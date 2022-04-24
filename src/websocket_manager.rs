use anyhow::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream};

use crate::types::{
  CPUStats, DiskStats, GPUStats, NetworkInterfaceStats, RAMStats, SwapStats, TempStats,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketEvent {
  Login {
    auth_token: String,
  },
  DynamicData {
    cpu: CPUStats,
    ram: RAMStats,
    swap: SwapStats,
    gpu: Option<GPUStats>,
    process_count: i32,
    disks: Vec<DiskStats>,
    temps: Option<Vec<TempStats>>,
    network: Vec<NetworkInterfaceStats>,
    host_uptime: u64,
    reporter_uptime: u64,
  },
  StaticData {
    hostname: Option<String>,
    public_ip: Option<String>,
    country: Option<String>,
    city: Option<String>,
    isp: Option<String>,
    timezone: Option<i32>,
    cpu_model: String,
    os_version: Option<String>,
    os_name: Option<String>,
    cpu_cores: Option<usize>,
    cpu_threads: usize,
    total_mem: u64,
    reporter_version: String,
  },
}

pub fn get_event_id(ev: &WebsocketEvent) -> &str {
  match ev {
    WebsocketEvent::Login { .. } => "login",
    WebsocketEvent::StaticData { .. } => "static-data",
    WebsocketEvent::DynamicData { .. } => "dynamic-data",
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketMessage {
  e: WebsocketEvent,
  data: Value,
}

pub struct WebsocketManager {
  pub websocket_url: String,
  pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
  pub write:
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>,
}

impl WebsocketManager {
  pub async fn new(websocket_url: &str) -> Result<Self> {
    let (stream, _) = tokio_tungstenite::connect_async(websocket_url).await?;
    let (write, read) = stream.split();

    Ok(Self {
      websocket_url: websocket_url.to_string(),
      read,
      write,
    })
  }

  pub async fn send(&mut self, data: WebsocketEvent) -> Result<()> {
    let message = json!({
      "e": get_event_id(&data),
      "d": &data,
    })
    .to_string();

    self.write.send(Message::Text(message)).await?;
    // send the string through the websocket

    Ok(())
  }
}
