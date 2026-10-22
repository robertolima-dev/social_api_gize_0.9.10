use sqlx::PgPool;

use super::dto::{CreateFollow, UpdateFollow};
use super::model::Follow;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Follow>, sqlx::Error> {
    sqlx::query_as::<_, Follow>(
        "SELECT * FROM follows ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM follows")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Follow, sqlx::Error> {
    sqlx::query_as::<_, Follow>("SELECT * FROM follows WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateFollow) -> Result<Follow, sqlx::Error> {
    sqlx::query_as::<_, Follow>(
        "INSERT INTO follows (follower_id, followee_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(input.follower_id)
    .bind(input.followee_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateFollow,
) -> Result<Follow, sqlx::Error> {
    sqlx::query_as::<_, Follow>(
        "UPDATE follows SET follower_id = $1, followee_id = $2, updated_at = now() WHERE id = $3 RETURNING *",
    )
        .bind(input.follower_id)
        .bind(input.followee_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM follows WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
