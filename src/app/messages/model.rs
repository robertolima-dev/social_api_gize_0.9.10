use serde::{Deserialize, Serialize};

/// The `Message` domain model, mapped to the `messages` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: uuid::Uuid,
    pub body: String,
    pub read: bool,
    pub sender_id: uuid::Uuid,
    pub recipient_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
