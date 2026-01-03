mod r#impl;
pub mod user;

use crate::repository::r#impl::user_impl::SeaOrmUserRepository;
use crate::repository::user::UserRepository;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct RepositoryModule {
    pub user_repository: Arc<dyn UserRepository>,
}

impl RepositoryModule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            user_repository: Arc::new(SeaOrmUserRepository::new(db)),
        }
    }
}
