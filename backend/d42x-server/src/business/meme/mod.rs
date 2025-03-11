pub mod gen_meme_repo;

use chrono::{DateTime, FixedOffset};
use migration::async_trait;
use sea_orm::{DbErr, prelude::Uuid};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::config::AllowMemeFormats;

use super::Pagination;

#[async_trait::async_trait]
pub trait MemeRepository {
    async fn get_paginated_memes(&self, _page: u64, _category: Option<String>) -> Pagination<Meme> {
        unimplemented!()
    }

    async fn get_paginated_all_memes(&self, _filter: GetFilter) -> Pagination<Meme> {
        unimplemented!()
    }

    async fn post_memes(&self, _memes: Vec<PostMeme>) -> Result<(), MemeError> {
        unimplemented!()
    }
}

pub struct PanicMemeRepository;

impl MemeRepository for PanicMemeRepository {}

#[derive(Serialize)]
pub struct GetFilter {
    pub page: u64,
    pub size: u64,
    pub status: Option<db_entity::memes::Status>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meme {
    pub id: Uuid,
    pub likes: usize,
    pub unlikes: usize,
    pub categories: Vec<String>,
    pub nickname: String,
    pub show_date_time: DateTime<FixedOffset>,
    pub list: Vec<MemeUrl>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemeUrl {
    pub id: Uuid,
    pub url: String,
    pub cover: String,
    pub format: AllowMemeFormats,
    pub sort: i32,
}

#[derive(Serialize, Debug, Validate)]
pub struct PostMeme {
    pub username: String,
    /// category_1;category_2
    pub categories: Vec<String>,
    pub message: String,
    #[validate(length(min = 1))]
    pub memes: Vec<PostMemeUrl>,
}

#[derive(Serialize, Debug, Validate)]
pub struct PostMemeUrl {
    #[validate(length(min = 1))]
    pub url: String,
    pub cover: String,
    pub format: AllowMemeFormats,
    pub hash: String,
    pub bed_id: String,
}

#[derive(Error, Debug)]
pub enum MemeError {
    #[error("has not any meme")]
    HasNotAnyMeme,
    #[error("Database error ocurrs: {0}")]
    DatabaseErr(#[from] DbErr),
}
