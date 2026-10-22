//! WebSocket routes, merged into the app router like any module.

use axum::Router;
use axum::routing::get;

use super::handler;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/ws", get(handler::upgrade))
}
