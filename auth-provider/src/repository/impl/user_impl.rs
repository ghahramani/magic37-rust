use crate::domain::user::User;
use crate::entity::user::{Column, Entity as UserEntity};
use crate::repository::user::UserRepository;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub(crate) struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn convert_to_domain(model: crate::entity::user::Model) -> User {
        User {
            id: model.id,
            email: model.email,
            first_name: model.first_name,
            last_name: model.last_name,
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        Ok(UserEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .map(Self::convert_to_domain))
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        Ok(UserEntity::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await?
            .map(Self::convert_to_domain))
    }
}
