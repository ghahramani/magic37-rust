use crate::config::web::error::{HttpError, OptionExt};
use crate::service::user::UserService;
use crate::web::rest::model::current_user_response::CurrentUserResponse;
use crate::web::rest::model::login_request::LoginRequest;
use actix_web::web::{scope, Data};
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;
use anyhow::Result;
use std::ops::Deref;
use uuid::Uuid;

#[get("/current")]
async fn user_current(service: Data<dyn UserService>) -> Result<CurrentUserResponse, HttpError> {
    let user = service
        .find_by_id(Uuid::new_v4())
        .await
        .map_err(|e| HttpError::Internal(e.to_string()))?
        .ok_or_not_found()?;
    Ok(CurrentUserResponse::from(user))
}

#[post("/login")]
async fn user_login(payload: Json<LoginRequest>) -> impl Responder {
    HttpResponse::Ok().json(payload.deref())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/auth").service(user_login).service(user_current));
}
