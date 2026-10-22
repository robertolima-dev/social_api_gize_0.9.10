use sqlx::PgPool;

use super::dto::{CreatePost, UpdatePost};
use super::model::Post;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Post, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreatePost) -> Result<Post, sqlx::Error> {
    sqlx::query_as::<_, Post>(
        "INSERT INTO posts (body, published, author_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(input.body.clone())
    .bind(input.published)
    .bind(input.author_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdatePost,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as::<_, Post>(
        "UPDATE posts SET body = $1, published = $2, author_id = $3, updated_at = now() WHERE id = $4 RETURNING *",
    )
        .bind(input.body.clone())
        .bind(input.published)
        .bind(input.author_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
