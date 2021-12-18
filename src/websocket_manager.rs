use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::TcpStream;
use std::sync::Arc;
use websocket::sync::Client;
use websocket::{ClientBuilder, Message};

use crate::util::arcmutex;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketEvent {
    Login { access_token: String },
}

pub fn get_event_id(ev: &WebsocketEvent) -> u16 {
    match ev {
        WebsocketEvent::Login { .. } => 0x01,
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
        let wtf = Message::text(
            json!({
                "e": get_event_id(&data),
                "data": &data,
            })
            .to_string(),
        );

        Ok(self.websocket.lock().send_message(&wtf)?)
    }

    // pub fn login(&mut self, token: &str) -> Result<()> {
    //     if let Some(websocket) = &self.websocket {
    //         websocket.lock().send_message(&Message::text(
    //             &json!({
    //                 "e": WebsocketEvent::Login as u16,
    //                 "access_token": token,
    //             })
    //             .to_string(),
    //         ))?;
    //     }

    //     return Ok(());
    // }

    // pub fn send_statics(&mut self, statics: &str) -> Result<()> {
    //     if let Some(websocket) = &self.websocket {
    //         websocket.lock().send_message(&Message::text(statics))?;
    //     }

    //     return Ok(());
    // }
    // pub fn authenticate(&mut self) -> Result<()> {}
    // pub fn authenticate(&mut self) -> Result<()> {}
    // pub fn authenticate(&mut self) -> Result<()> {}
    // pub fn authenticate(&mut self) -> Result<()> {}
    // pub fn authenticate(&mut self) -> Result<()> {}
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
