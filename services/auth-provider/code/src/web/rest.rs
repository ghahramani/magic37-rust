pub mod authentication;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(authentication::routes);
}
