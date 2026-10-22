use sqlx::PgPool;

use super::dto::{CreateComment, UpdateComment};
use super::model::Comment;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Comment>, sqlx::Error> {
    sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM comments")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Comment, sqlx::Error> {
    sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateComment) -> Result<Comment, sqlx::Error> {
    sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (body, post_id, author_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(input.body.clone())
    .bind(input.post_id)
    .bind(input.author_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateComment,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as::<_, Comment>(
        "UPDATE comments SET body = $1, post_id = $2, author_id = $3, updated_at = now() WHERE id = $4 RETURNING *",
    )
        .bind(input.body.clone())
        .bind(input.post_id)
        .bind(input.author_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
