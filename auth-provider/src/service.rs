mod r#impl;
pub mod user;

use crate::repository::RepositoryModule;
use r#impl::user_impl::UserServiceImpl;
use std::sync::Arc;
use user::UserService;

#[derive(Clone)]
pub struct ServiceModule {
    pub user: Arc<dyn UserService>,
}

impl ServiceModule {
    pub fn new(repos: RepositoryModule) -> Self {
        Self {
            user: Arc::new(UserServiceImpl::new(repos.user_repository)),
        }
    }
}
