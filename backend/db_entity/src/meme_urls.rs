use chrono::{FixedOffset, Utc};
use sea_orm::{Set, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "meme_urls")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub meme_id: Uuid,
    pub url: String,
    pub cover: String,
    pub source: String,
    pub format: String,
    pub hash: String,
    pub bed: Bed,
    pub bed_id: String,
    pub created_date_time: chrono::DateTime<FixedOffset>,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum Bed {
    /// 聚合图床
    #[sea_orm(string_value = "superbad")]
    SuperBed,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::memes::Entity",
        from = "Column::MemeId",
        to = "super::memes::Column::Id"
    )]
    Meme,
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
            meme_id: Set(Uuid::now_v7()),
            url: Set(String::new()),
            cover: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::new()),
            hash: Set(String::new()),
            bed: Set(Bed::SuperBed),
            bed_id: Set(String::new()),
            created_date_time: Set(now),
        }
    }
}
