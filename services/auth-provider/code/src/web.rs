mod rest;

use crate::service::ServiceModule;
use actix_web::web::{scope, Data};
use actix_web::Scope;

#[derive(Clone)]
pub struct WebModule;

impl WebModule {
    pub fn app(services: ServiceModule) -> Scope {
        scope("/api")
            .app_data(Data::from(services.user))
            .configure(rest::routes)
    }
}
