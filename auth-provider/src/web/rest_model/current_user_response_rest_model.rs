use actix_web::{HttpRequest, HttpResponse, Responder};
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

impl Responder for CurrentUserResponseRestModel {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
