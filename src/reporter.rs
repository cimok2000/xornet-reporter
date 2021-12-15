use crate::data_collector::DataCollector;
use anyhow::Result;
use serde_json::json;
use std::net::TcpStream;
use websocket::sync::Client;
use websocket::{ClientBuilder, Message};

pub struct Reporter {
    pub data_collector: DataCollector,
    pub version: String,
    pub websocket: Client<TcpStream>,
}

impl Reporter {
    pub fn new() -> Result<Self> {
        let data_collector: DataCollector = DataCollector::new()?;
        let version: String = env!("CARGO_PKG_VERSION").to_string();
        let statics = data_collector.get_statics()?;

        let mut websocket = ClientBuilder::new("ws://localhost:8000")?.connect_insecure()?;
        websocket.send_message(&Message::text(
            &json!({
                "version": &version,
                "name": "Xornet Reporter",
                "statics": statics,
            })
            .to_string(),
        ))?;

        return Ok(Self {
            data_collector,
            version,
            websocket,
        });
    }
}
