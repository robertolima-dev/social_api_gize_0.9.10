//! Typed WebSocket messages. Extend these enums to define your own protocol; the
//! `type` tag selects the variant (e.g. `{"type":"echo","text":"hi"}`).

use serde::{Deserialize, Serialize};

/// A message received from the client.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Echo { text: String },
}

/// A message sent back to the client.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Echo { text: String },
    Error { message: String },
}
