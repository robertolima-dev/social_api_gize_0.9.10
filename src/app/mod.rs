//! Application modules. `gize make app <name>` registers modules here.

use axum::Router;

use crate::state::AppState;

// gize:modules (do not remove this marker)
mod comments;
mod follows;
mod hashtags;
mod likes;
mod messages;
mod openapi;
mod posts;
mod profiles;
mod selfies;
mod users;
mod ws;

/// Merge every module's routes. `gize make app` extends this function.
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(users::routes())
        .merge(openapi::routes())
        .merge(ws::routes())
        .merge(profiles::routes())
        .merge(posts::routes())
        .merge(comments::routes())
        .merge(likes::routes())
        .merge(follows::routes())
        .merge(selfies::routes())
        .merge(messages::routes())
        .merge(hashtags::routes())
    // gize:module-routes (do not remove this marker)
}
