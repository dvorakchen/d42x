use chrono::FixedOffset;
use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    /// usual used ip address
    pub usual_address: String,
    pub is_admin: bool,
    pub created_date_time: chrono::DateTime<FixedOffset>,
    pub last_actiity_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            username: Set(Default::default()),
            hashed_password: Set(Default::default()),
            email: Set(Default::default()),
            usual_address: Set(Default::default()),
            is_admin: Set(false),
            created_date_time: Set(chrono::Utc::now().into()),
            last_actiity_date_time: Set(chrono::Utc::now().into()),
        }
    }
}
