use crate::web::rest_model::current_user_response_rest_model::CurrentUserResponseRestModel;
use spring_web::axum::response::{IntoResponse, Response};
use spring_web::axum::Json;

pub mod current_user_response_rest_model;

impl IntoResponse for CurrentUserResponseRestModel {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
