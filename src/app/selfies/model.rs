use serde::{Deserialize, Serialize};

/// The `Selfie` domain model, mapped to the `selfies` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Selfie {
    pub id: uuid::Uuid,
    pub image_url: String,
    pub caption: String,
    pub author_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
