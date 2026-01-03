mod rest;
mod rest_model;

use crate::service::ServiceModule;
use actix_web::web::scope;
use actix_web::Scope;

#[derive(Clone)]
pub struct WebModule;

impl WebModule {
    pub fn app(services: ServiceModule) -> Scope {
        scope("/api")
            .app_data(services.user)
            .configure(rest::routes)
    }
}
