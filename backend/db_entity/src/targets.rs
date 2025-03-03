use chrono::{FixedOffset, Utc};
use sea_orm::{Set, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "targets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub created_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now().into();
        Self {
            id: Set(Uuid::now_v7()),
            name: Set(String::new()),
            created_date_time: Set(now),
        }
    }
}
