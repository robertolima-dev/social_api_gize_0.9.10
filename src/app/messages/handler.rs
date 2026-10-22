use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use validator::Validate;

use super::dto::{CreateMessage, ListParams, Page, UpdateMessage};
use super::error::Error;
use super::model::Message;
use super::service;
use crate::auth::Claims;
use crate::shared;
use crate::state::AppState;

/// `GET /messages?page=&per_page=`. Invalid paging parameters are a 422, like other input.
/// `GET /messages`: the caller's conversations (every message, for an admin).
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    params: Result<Query<ListParams>, QueryRejection>,
) -> Result<Json<Page<Message>>, Error> {
    let Query(params) = params.map_err(|rejection| Error::Validation(rejection.body_text()))?;
    let (page, per_page) = params.resolve().map_err(Error::Validation)?;
    if claims.is_admin {
        return Ok(Json(service::list(&state.db, page, per_page).await?));
    }
    let me = shared::caller(&claims)?;
    Ok(Json(
        service::list_for(&state.db, me, page, per_page).await?,
    ))
}

/// Someone else's message answers 404, as if it did not exist.
pub async fn show(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message>, Error> {
    let item = service::find(&state.db, id).await?;
    let me = shared::caller(&claims)?;
    if !claims.is_admin && me != item.sender_id && me != item.recipient_id {
        return Err(Error::NotFound);
    }
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(mut input): Json<CreateMessage>,
) -> Result<(StatusCode, Json<Message>), Error> {
    input.validate()?;
    input.sender_id = shared::caller(&claims)?;
    let item = service::create(&state.db, &input).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
    Json(mut input): Json<UpdateMessage>,
) -> Result<Json<Message>, Error> {
    input.validate()?;
    let current = service::find(&state.db, id).await?;
    if shared::caller(&claims)? == current.recipient_id && !claims.is_admin {
        // The recipient can only mark it read or unread.
        input.body = current.body.clone();
        input.recipient_id = current.recipient_id;
    } else {
        shared::ensure_owner(&claims, current.sender_id)?;
    }
    // A row keeps its owner.
    input.sender_id = current.sender_id;
    let item = service::update(&state.db, id, &input).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, Error> {
    let current = service::find(&state.db, id).await?;
    shared::ensure_owner(&claims, current.sender_id)?;
    service::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
