use sqlx::PgPool;

use super::dto::{CreateProfile, UpdateProfile};
use super::model::Profile;

/// One page of rows, newest first. `id` breaks ties so the order is stable across pages.
pub async fn list(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Profile>, sqlx::Error> {
    sqlx::query_as::<_, Profile>(
        "SELECT * FROM profiles ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM profiles")
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, id: uuid::Uuid) -> Result<Profile, sqlx::Error> {
    sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &PgPool, input: &CreateProfile) -> Result<Profile, sqlx::Error> {
    sqlx::query_as::<_, Profile>(
        "INSERT INTO profiles (bio, avatar_url, user_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(input.bio.clone())
    .bind(input.avatar_url.clone())
    .bind(input.user_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    input: &UpdateProfile,
) -> Result<Profile, sqlx::Error> {
    sqlx::query_as::<_, Profile>(
        "UPDATE profiles SET bio = $1, avatar_url = $2, user_id = $3, updated_at = now() WHERE id = $4 RETURNING *",
    )
        .bind(input.bio.clone())
        .bind(input.avatar_url.clone())
        .bind(input.user_id)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM profiles WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
