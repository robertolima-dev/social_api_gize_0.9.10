use serde::{Deserialize, Serialize};

/// The `Profile` domain model, mapped to the `profiles` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub id: uuid::Uuid,
    pub bio: String,
    pub avatar_url: String,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
