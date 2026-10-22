use sqlx::PgPool;

use super::dto::{CreateMessage, UpdateMessage};
use super::model::Message;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "SELECT * FROM messages ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// One page of the messages `user` sent or received, newest first.
pub async fn list_for(
    pool: &PgPool,
    user: uuid::Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE sender_id = $1 OR recipient_id = $1 \
         ORDER BY created_at DESC, id DESC LIMIT $2 OFFSET $3",
    )
    .bind(user)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_for(pool: &PgPool, user: uuid::Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE sender_id = $1 OR recipient_id = $1")
        .bind(user)
        .fetch_one(pool)
        .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Message, sqlx::Error> {
    sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateMessage) -> Result<Message, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "INSERT INTO messages (body, read, sender_id, recipient_id) VALUES ($1, $2, $3, $4) RETURNING *",
    )
        .bind(input.body.clone())
        .bind(input.read)
        .bind(input.sender_id)
        .bind(input.recipient_id)
        .fetch_one(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateMessage,
) -> Result<Message, sqlx::Error> {
    sqlx::query_as::<_, Message>(
        "UPDATE messages SET body = $1, read = $2, sender_id = $3, recipient_id = $4, updated_at = now() WHERE id = $5 RETURNING *",
    )
        .bind(input.body.clone())
        .bind(input.read)
        .bind(input.sender_id)
        .bind(input.recipient_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM messages WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
