use axum::routing::{get, post};
use axum::{Router, middleware};

use super::handler;
use crate::auth::require_auth;
use crate::state::AppState;

/// Routes for the `follows` resource. Mutating routes require a valid bearer token.
pub fn routes() -> Router<AppState> {
    let public = Router::new()
        .route("/follows", get(handler::list))
        .route("/follows/:id", get(handler::show));

    let protected = Router::new()
        .route("/follows", post(handler::create))
        .route(
            "/follows/:id",
            axum::routing::put(handler::update).delete(handler::delete),
        )
        .route_layer(middleware::from_fn(require_auth));

    public.merge(protected)
}
