use axum::routing::get;
use axum::{Router, middleware};

use super::handler;
use crate::auth::require_auth;
use crate::state::AppState;

/// Routes for the `messages` resource. Messages are private: every route requires a valid
/// bearer token, and callers see only the messages they sent or received.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/messages", get(handler::list).post(handler::create))
        .route(
            "/messages/:id",
            get(handler::show)
                .put(handler::update)
                .delete(handler::delete),
        )
        .route_layer(middleware::from_fn(require_auth))
}
