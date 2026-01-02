use crate::domain::user::User;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthenticationService {}

impl AuthenticationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn user_current(&self, id: Uuid) -> User {
        User {
            id,
            email: "AASS".to_string(),
            first_name: "Navid".to_string(),
            last_name: "Ghahremani".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }
}
