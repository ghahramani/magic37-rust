use crate::domain::user::User;
use crate::repository::user::UserRepository;
use crate::service::user::UserService;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        self.repo.find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        self.repo.find_by_email(email).await
    }
}
