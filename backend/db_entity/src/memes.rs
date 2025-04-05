use chrono::{FixedOffset, Utc};
use nanoid::nanoid;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "memes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub short_id: String,
    pub message: String,
    pub nickname: String,
    pub email: String,
    pub id_addr: String,
    pub likes: i32,
    pub unlikes: i32,
    /// ;categories_1;categories_2;
    pub categories: String,
    pub status: Status,
    pub user_id: Uuid,
    pub show_date_time: chrono::DateTime<FixedOffset>,
    pub created_date_time: chrono::DateTime<FixedOffset>,
    pub last_actiity_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum Status {
    #[sea_orm(string_value = "uncensored")]
    Uncensored,
    #[sea_orm(string_value = "published")]
    Published,
    #[sea_orm(string_value = "deleted")]
    Deleted,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Status::Deleted => "Deleted",
            Status::Published => "Published",
            Status::Uncensored => "Uncensored",
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "deleted" => Ok(Status::Deleted),
            "published" => Ok(Status::Published),
            "uncensored" => Ok(Status::Uncensored),
            _ => Err(format!("incorrect value: {}", value)),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::meme_urls::Entity")]
    MemeUrls,
    #[sea_orm(has_many = "super::suggests::Entity")]
    Suggests,
}

impl Related<super::meme_urls::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MemeUrls.def()
    }
}

impl Related<super::suggests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Suggests.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now().into();
        Self {
            id: Set(Uuid::now_v7()),
            short_id: Set(nanoid!(10)),
            nickname: Set(String::new()),
            email: Set(String::new()),
            message: Set(String::new()),
            id_addr: Set(String::new()),
            likes: Set(0),
            unlikes: Set(0),
            categories: Set(format!(";{};", crate::DEFAULT_CATEGORY)),
            status: Set(Status::Uncensored),
            user_id: Set(Uuid::nil()),
            show_date_time: Set(now),
            created_date_time: Set(now),
            last_actiity_date_time: Set(now),
        }
    }
}
