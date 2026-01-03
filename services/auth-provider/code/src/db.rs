use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
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
