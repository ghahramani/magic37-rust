use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> DatabaseConnection {
    let host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let port = std::env::var("DB_PORT").expect("DB_PORT not set");
    let user = std::env::var("DB_USER").expect("DB_USER not set");
    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let name = std::env::var("DB_NAME").expect("DB_NAME not set");
    
    let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, name);
    
    let mut retry_count = 0;
    loop {
        match Database::connect(&database_url).await {
            Ok(conn) => return conn,
            Err(e) => {
                retry_count += 1;
                if retry_count > 30 {
                    panic!("Failed to connect to database after 30 retries: {}", e);
                }
                println!("Database not ready... retrying ({}/30): {}", retry_count, e);
                actix_web::rt::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
}
