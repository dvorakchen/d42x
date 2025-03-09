use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Uuid;
use serde::Serialize;

use crate::config::AllowMemeFormats;

#[derive(Serialize)]
pub struct PaginatedMemeList {
    pub page: u64,
    pub total: u64,
    pub list: Vec<Meme>,
}

#[derive(Serialize)]
pub struct Meme {
    pub id: Uuid,
    pub likes: usize,
    pub unlikes: usize,
    pub categories: Vec<String>,
    pub nickname: String,
    pub show_date_time: DateTime<FixedOffset>,
    pub list: Vec<MemeUrl>,
}

#[derive(Serialize)]
pub struct MemeUrl {
    pub id: Uuid,
    pub url: String,
    pub cover: String,
    pub format: AllowMemeFormats,
    pub sort: i32,
}
