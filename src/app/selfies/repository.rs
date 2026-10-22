use sqlx::PgPool;

use super::dto::{CreateSelfie, UpdateSelfie};
use super::model::Selfie;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Selfie>, sqlx::Error> {
    sqlx::query_as::<_, Selfie>(
        "SELECT * FROM selfies ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM selfies")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Selfie, sqlx::Error> {
    sqlx::query_as::<_, Selfie>("SELECT * FROM selfies WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateSelfie) -> Result<Selfie, sqlx::Error> {
    sqlx::query_as::<_, Selfie>(
        "INSERT INTO selfies (image_url, caption, author_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(input.image_url.clone())
    .bind(input.caption.clone())
    .bind(input.author_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateSelfie,
) -> Result<Selfie, sqlx::Error> {
    sqlx::query_as::<_, Selfie>(
        "UPDATE selfies SET image_url = $1, caption = $2, author_id = $3, updated_at = now() WHERE id = $4 RETURNING *",
    )
        .bind(input.image_url.clone())
        .bind(input.caption.clone())
        .bind(input.author_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM selfies WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
