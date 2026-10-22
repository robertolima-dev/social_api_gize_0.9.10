use axum::Json;
use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use validator::Validate;

use super::dto::{CreateHashtag, ListParams, Page, UpdateHashtag};
use super::error::Error;
use super::model::Hashtag;
use super::service;
use crate::state::AppState;

/// `GET /hashtags?page=&per_page=`. Invalid paging parameters are a 422, like other input.
pub async fn list(
    State(state): State<AppState>,
    params: Result<Query<ListParams>, QueryRejection>,
) -> Result<Json<Page<Hashtag>>, Error> {
    let Query(params) = params.map_err(|rejection| Error::Validation(rejection.body_text()))?;
    let (page, per_page) = params.resolve().map_err(Error::Validation)?;
    Ok(Json(service::list(&state.db, page, per_page).await?))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Hashtag>, Error> {
    let item = service::find(&state.db, id).await?;
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateHashtag>,
) -> Result<(StatusCode, Json<Hashtag>), Error> {
    input.validate()?;
    let item = service::create(&state.db, &input).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(input): Json<UpdateHashtag>,
) -> Result<Json<Hashtag>, Error> {
    input.validate()?;
    let item = service::update(&state.db, id, &input).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, Error> {
    service::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
