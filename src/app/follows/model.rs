use serde::{Deserialize, Serialize};

/// The `Follow` domain model, mapped to the `follows` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Follow {
    pub id: uuid::Uuid,
    pub follower_id: uuid::Uuid,
    pub followee_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
