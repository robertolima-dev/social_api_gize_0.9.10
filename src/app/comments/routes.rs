use axum::routing::{get, post};
use axum::{Router, middleware};

use super::handler;
use crate::auth::require_auth;
use crate::state::AppState;

/// Routes for the `comments` resource. Mutating routes require a valid bearer token.
pub fn routes() -> Router<AppState> {
    let public = Router::new()
        .route("/comments", get(handler::list))
        .route("/comments/:id", get(handler::show));

    let protected = Router::new()
        .route("/comments", post(handler::create))
        .route(
            "/comments/:id",
            axum::routing::put(handler::update).delete(handler::delete),
        )
        .route_layer(middleware::from_fn(require_auth));

    public.merge(protected)
}
