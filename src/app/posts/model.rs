use serde::{Deserialize, Serialize};

/// The `Post` domain model, mapped to the `posts` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: uuid::Uuid,
    pub body: String,
    pub published: bool,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
