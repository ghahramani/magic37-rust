use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::DeriveEntityModel;
use sea_orm::{
    ActiveModelBehavior, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, RelationDef,
    RelationTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}
