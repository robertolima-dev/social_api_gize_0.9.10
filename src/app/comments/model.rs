use serde::{Deserialize, Serialize};

/// The `Comment` domain model, mapped to the `comments` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub body: String,
    pub post_id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
