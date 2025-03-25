use chrono::{FixedOffset, Utc};
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "suggests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub meme_id: Uuid,
    pub before: String,
    pub after: String,
    pub status: Status,
    pub account_id: Uuid,
    pub operator_id: Uuid,
    pub created_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum Status {
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "refused")]
    Refused,
    #[sea_orm(string_value = "wait")]
    Wait,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::memes::Entity",
        from = "Column::MemeId",
        to = "super::memes::Column::Id"
    )]
    Meme,
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id"
    )]
    Operator,
}

impl Related<super::memes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Meme.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now().into();
        Self {
            id: Set(Uuid::now_v7()),
            meme_id: Set(Uuid::nil()),
            before: Set(String::new()),
            after: Set(String::new()),
            status: Set(Status::Wait),
            account_id: Set(Uuid::nil()),
            operator_id: Set(Uuid::nil()),
            created_date_time: Set(now),
        }
    }
}
