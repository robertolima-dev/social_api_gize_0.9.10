use sqlx::PgPool;

use super::dto::{CreateHashtag, UpdateHashtag};
use super::model::Hashtag;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Hashtag>, sqlx::Error> {
    sqlx::query_as::<_, Hashtag>(
        "SELECT * FROM hashtags ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM hashtags")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Hashtag, sqlx::Error> {
    sqlx::query_as::<_, Hashtag>("SELECT * FROM hashtags WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateHashtag) -> Result<Hashtag, sqlx::Error> {
    sqlx::query_as::<_, Hashtag>("INSERT INTO hashtags (name) VALUES ($1) RETURNING *")
        .bind(input.name.clone())
        .fetch_one(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateHashtag,
) -> Result<Hashtag, sqlx::Error> {
    sqlx::query_as::<_, Hashtag>(
        "UPDATE hashtags SET name = $1, updated_at = now() WHERE id = $2 RETURNING *",
    )
    .bind(input.name.clone())
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM hashtags WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
