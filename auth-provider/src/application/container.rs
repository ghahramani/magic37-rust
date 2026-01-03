use crate::db;
use crate::repository::RepositoryModule;
use crate::service::ServiceModule;
use crate::web::WebModule;
use actix_web::{App, HttpServer};

pub struct Application;

impl Application {
    pub async fn build() -> std::io::Result<()> {
        let db = db::connect().await;

        let repositories = RepositoryModule::new(db);
        let services = ServiceModule::new(repositories);

        println!("Starting server at http://0.0.0.0:8080");
        HttpServer::new(move || {
            let service_module = services.clone();
            let scope = WebModule::app(service_module);
            App::new().service(scope)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
}
