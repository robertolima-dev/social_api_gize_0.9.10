use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use validator::Validate;

use super::dto::{CreateLike, ListParams, Page, UpdateLike};
use super::error::Error;
use super::model::Like;
use super::service;
use crate::auth::Claims;
use crate::shared;
use crate::state::AppState;

/// `GET /likes?page=&per_page=`. Invalid paging parameters are a 422, like other input.
pub async fn list(
    State(state): State<AppState>,
    params: Result<Query<ListParams>, QueryRejection>,
) -> Result<Json<Page<Like>>, Error> {
    let Query(params) = params.map_err(|rejection| Error::Validation(rejection.body_text()))?;
    let (page, per_page) = params.resolve().map_err(Error::Validation)?;
    Ok(Json(service::list(&state.db, page, per_page).await?))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Like>, Error> {
    let item = service::find(&state.db, id).await?;
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(mut input): Json<CreateLike>,
) -> Result<(StatusCode, Json<Like>), Error> {
    input.validate()?;
    input.user_id = shared::caller(&claims)?;
    let item = service::create(&state.db, &input).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
    Json(mut input): Json<UpdateLike>,
) -> Result<Json<Like>, Error> {
    input.validate()?;
    let current = service::find(&state.db, id).await?;
    shared::ensure_owner(&claims, current.user_id)?;
    // A row keeps its owner.
    input.user_id = current.user_id;
    let item = service::update(&state.db, id, &input).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, Error> {
    let current = service::find(&state.db, id).await?;
    shared::ensure_owner(&claims, current.user_id)?;
    service::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
