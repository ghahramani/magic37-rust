use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
