use crate::service::authentication_service::AuthenticationService;
use crate::web::rest_model::current_user_response_rest_model::CurrentUserResponseRestModel;
use spring_web::axum::response::IntoResponse;
use spring_web::extractor::Component;
use spring_web::{axum, get, post};
use uuid::Uuid;

#[get("/api/auth/current")]
async fn user_current(Component(service): Component<AuthenticationService>) -> impl IntoResponse {
    let user = service.user_current(Uuid::new_v4());

    axum::Json(CurrentUserResponseRestModel {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        created_at: user.created_at,
        modified_at: user.modified_at,
    })
}

#[post("/api/auth/login")]
async fn user_login() -> impl IntoResponse {
    // You can call service.login, etc.
    axum::Json("Test")
}
