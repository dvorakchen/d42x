use std::fmt::Display;

use chrono::{FixedOffset, Utc};
use sea_orm::{Set, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "memes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub url: String,
    pub source: String,
    pub format: String,
    pub hash: String,
    pub nickname: String,
    pub email: String,
    pub id_addr: String,
    pub likes: i32,
    pub unlikes: i32,
    pub targets: String,
    pub status: Status,
    pub bed: Bed,
    pub bed_id: String,
    pub show_date_time: chrono::DateTime<FixedOffset>,
    pub created_date_time: chrono::DateTime<FixedOffset>,
    pub last_actiity_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum Bed {
    /// 聚合图床
    #[sea_orm(string_value = "superbad")]
    SuperBed,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
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
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now().into();
        Self {
            id: Set(Uuid::now_v7()),
            url: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::new()),
            hash: Set(String::new()),
            nickname: Set(String::new()),
            email: Set(String::new()),
            id_addr: Set(String::new()),
            likes: Set(0),
            unlikes: Set(0),
            targets: Set(String::new()),
            status: Set(Status::Uncensored),
            bed: Set(Bed::SuperBed),
            bed_id: Set(String::new()),
            show_date_time: Set(now),
            created_date_time: Set(now),
            last_actiity_date_time: Set(now),
        }
    }
}
