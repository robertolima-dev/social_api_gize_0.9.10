use sqlx::PgPool;

use super::dto::{CreateLike, UpdateLike};
use super::model::Like;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Like>, sqlx::Error> {
    sqlx::query_as::<_, Like>(
        "SELECT * FROM likes ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM likes")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Like, sqlx::Error> {
    sqlx::query_as::<_, Like>("SELECT * FROM likes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateLike) -> Result<Like, sqlx::Error> {
    sqlx::query_as::<_, Like>("INSERT INTO likes (post_id, user_id) VALUES ($1, $2) RETURNING *")
        .bind(input.post_id)
        .bind(input.user_id)
        .fetch_one(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateLike,
) -> Result<Like, sqlx::Error> {
    sqlx::query_as::<_, Like>(
        "UPDATE likes SET post_id = $1, user_id = $2, updated_at = now() WHERE id = $3 RETURNING *",
    )
    .bind(input.post_id)
    .bind(input.user_id)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM likes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
