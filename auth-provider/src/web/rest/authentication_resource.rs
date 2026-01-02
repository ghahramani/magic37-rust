use crate::service::authentication_service::AuthenticationService;
use crate::web::rest_model::current_user_response_rest_model::CurrentUserResponseRestModel;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/current")]
async fn user_current(service: Data<AuthenticationService>) -> impl Responder {
    let user = service.user_current(Uuid::new_v4());
    CurrentUserResponseRestModel {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        created_at: user.created_at,
        modified_at: user.modified_at,
    }
}

#[post("/login")]
async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

pub fn auth_scope() -> actix_web::Scope {
    let service = AuthenticationService::new();
    web::scope("/auth")
        .app_data(Data::new(service))
        .service(user_login)
        .service(user_current)
}
