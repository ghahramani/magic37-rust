use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::User;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
}
