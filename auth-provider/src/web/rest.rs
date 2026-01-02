use crate::web::rest::authentication_resource::auth_scope;
use actix_web::web::scope;
use actix_web::Scope;

pub mod authentication_resource;

pub fn all_rest() -> Scope {
    scope("").service(auth_scope())
}
