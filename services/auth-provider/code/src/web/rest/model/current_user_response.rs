use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Serialize)]
pub struct CurrentUserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Responder for CurrentUserResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl From<User> for CurrentUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            modified_at: user.modified_at,
            created_at: user.created_at,
        }
    }
}
