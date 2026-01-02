mod domain;
mod service;
mod web;

use spring::{auto_config, App};
use spring_web::{WebConfigurator, WebPlugin};

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new().add_plugin(WebPlugin).run().await
}
