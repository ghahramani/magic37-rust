use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CurrentUserResponseRestModel {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
