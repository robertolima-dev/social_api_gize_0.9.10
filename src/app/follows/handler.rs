use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use validator::Validate;

use super::dto::{CreateFollow, ListParams, Page, UpdateFollow};
use super::error::Error;
use super::model::Follow;
use super::service;
use crate::auth::Claims;
use crate::shared;
use crate::state::AppState;

/// `GET /follows?page=&per_page=`. Invalid paging parameters are a 422, like other input.
pub async fn list(
    State(state): State<AppState>,
    params: Result<Query<ListParams>, QueryRejection>,
) -> Result<Json<Page<Follow>>, Error> {
    let Query(params) = params.map_err(|rejection| Error::Validation(rejection.body_text()))?;
    let (page, per_page) = params.resolve().map_err(Error::Validation)?;
    Ok(Json(service::list(&state.db, page, per_page).await?))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Follow>, Error> {
    let item = service::find(&state.db, id).await?;
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(mut input): Json<CreateFollow>,
) -> Result<(StatusCode, Json<Follow>), Error> {
    input.validate()?;
    input.follower_id = shared::caller(&claims)?;
    if input.follower_id == input.followee_id {
        return Err(Error::Validation("you cannot follow yourself".to_string()));
    }
    let item = service::create(&state.db, &input).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
    Json(mut input): Json<UpdateFollow>,
) -> Result<Json<Follow>, Error> {
    input.validate()?;
    let current = service::find(&state.db, id).await?;
    shared::ensure_owner(&claims, current.follower_id)?;
    // A row keeps its owner.
    input.follower_id = current.follower_id;
    let item = service::update(&state.db, id, &input).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, Error> {
    let current = service::find(&state.db, id).await?;
    shared::ensure_owner(&claims, current.follower_id)?;
    service::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
