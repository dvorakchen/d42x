use chrono::{DateTime, FixedOffset};
use migration::async_trait;
use sea_orm::{DbErr, prelude::Uuid};
use serde::Serialize;
use thiserror::Error;

use super::{Pagination, meme::MemeUrl};

pub mod gen_suggest_repo;

pub type SuggestResult<T> = Result<T, SuggestError>;

#[async_trait::async_trait]
pub trait SuggestRepository {
    async fn create(
        &self,
        _meme_id: Uuid,
        _list: Vec<String>,
        _apply_user_id: Uuid,
    ) -> SuggestResult<()> {
        unimplemented!()
    }

    async fn get_paginated_suggests(&self, _filter: GetFilter) -> Pagination<Suggestion> {
        unimplemented!()
    }

    async fn set_suggest_status(
        &self,
        _id: Uuid,
        _status: db_entity::suggests::Status,
        _operator_id: Uuid,
    ) {
        unimplemented!()
    }
}

pub struct PanicSuggestRepository;

impl SuggestRepository for PanicSuggestRepository {}

#[derive(Serialize, Debug)]
pub struct GetFilter {
    page: u64,
    status: Option<db_entity::suggests::Status>,
}

#[derive(Serialize, Debug)]
pub struct Suggestion {
    pub id: Uuid,
    pub meme_id: Uuid,
    pub meme_urls: Vec<MemeUrl>,
    pub cur_category: Vec<String>,
    pub before_category: Vec<String>,
    pub after_category: Vec<String>,
    pub apply_user_id: Uuid,
    pub apply_username: String,
    pub operator: Uuid,
    pub operator_username: String,
    pub created_date_time: DateTime<FixedOffset>,
}

#[derive(Error, Debug)]
pub enum SuggestError {
    #[error("Database error ocurrs: {0}")]
    DatabaseErr(#[from] DbErr),
    #[error("create suggest failed: {0}")]
    CreateFail(&'static str),
}
