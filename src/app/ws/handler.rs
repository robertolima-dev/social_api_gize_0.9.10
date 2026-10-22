//! The WebSocket upgrade handler and per-connection echo loop.

use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;

use super::message::{ClientMessage, ServerMessage};
use crate::state::AppState;

/// Upgrade the HTTP connection to a WebSocket. `AppState` is available here for auth, shared
/// channels, database access, etc. — this echo example does not use it yet.
pub async fn upgrade(ws: WebSocketUpgrade, State(_state): State<AppState>) -> Response {
    ws.on_upgrade(handle_socket)
}

/// Per-connection loop: decode each typed client message and echo it back. Extend the `match`
/// on `ClientMessage` to handle your own message types.
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                let reply = match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::Echo { text }) => ServerMessage::Echo { text },
                    Err(err) => ServerMessage::Error {
                        message: err.to_string(),
                    },
                };
                let payload = serde_json::to_string(&reply).unwrap_or_default();
                if socket.send(Message::Text(payload)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
