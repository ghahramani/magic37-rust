use crate::web::rest_model::current_user_response_rest_model::CurrentUserResponseRestModel;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod current_user_response_rest_model;

impl Responder for CurrentUserResponseRestModel {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
