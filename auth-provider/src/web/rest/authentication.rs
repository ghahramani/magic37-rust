use crate::config::web::error::{OptionExt, ServiceError};
use crate::service::user::UserService;
use crate::web::rest_model::current_user_response::CurrentUserResponse;
use actix_web::web::{scope, Data};
use actix_web::{get, post, web, HttpResponse, Responder};

use uuid::Uuid;

#[get("/current")]
async fn user_current(service: Data<dyn UserService>) -> Result<CurrentUserResponse, ServiceError> {
    let user = service
        .find_by_id(Uuid::new_v4())
        .await
        .map_err(|e| ServiceError::Internal(e.to_string()))?
        .ok_or_not_found()?;
    Ok(CurrentUserResponse::from(user))
}

#[post("/login")]
async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/auth").service(user_login).service(user_current));
}
