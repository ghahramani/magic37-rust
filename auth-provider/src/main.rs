mod domain;
mod service;
mod web;

use crate::web::rest::all_rest;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(scope("/api").service(all_rest()))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
