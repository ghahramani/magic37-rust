use auth_provider::application::container::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::build().await
}
