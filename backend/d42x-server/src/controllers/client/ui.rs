use askama::Template;
use askama_axum::{IntoResponse, Response};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::memes;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

use crate::db::DbHelper;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub total: u64,
    pub list: Vec<Meme>,
}

pub struct Meme {
    pub url: String,
    pub likes: usize,
    pub unlikes: usize,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
}

pub async fn home() -> Response {
    const SIZE_PER_PAGE: u64 = 20;
    let db = DbHelper::get_connection().await.unwrap();

    let now: DateTime<FixedOffset> = Utc::now().into();

    let paged_memes = memes::Entity::find()
        .filter(memes::Column::Status.eq(memes::Status::Published))
        .filter(memes::Column::ShowDateTime.lt(now))
        .order_by_desc(memes::Column::ShowDateTime)
        .paginate(&db, SIZE_PER_PAGE);

    let list = paged_memes
        .fetch_page(0)
        .await
        .unwrap()
        .into_iter()
        .map(|item| Meme {
            url: item.url,
            likes: item.likes as usize,
            unlikes: item.unlikes as usize,
        })
        .collect();

    let total = paged_memes.num_pages().await.unwrap();

    HomeTemplate { total, list }.into_response()
}
