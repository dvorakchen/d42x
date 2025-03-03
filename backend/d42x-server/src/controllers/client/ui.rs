use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{Json, extract::Query};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::{categories, memes};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, prelude::Uuid};
use serde::Deserialize;

use crate::db::DbHelper;

use super::models::CategoryItem;

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
    pub targets: Vec<String>,
}

#[derive(Deserialize)]
pub struct HomeQuery {
    pub category: Option<String>,
}

pub async fn home(Query(_query): Query<HomeQuery>) -> Response {
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
            targets: item
                .categories
                .split(';')
                .filter_map(|c| {
                    if c.is_empty() {
                        None
                    } else {
                        Some(c.to_string())
                    }
                })
                .collect(),
        })
        .collect();

    let total = paged_memes.num_pages().await.unwrap();

    HomeTemplate { total, list }.into_response()
}

/// get all top categories
pub async fn get_categories() -> Json<Vec<CategoryItem>> {
    let db = DbHelper::get_connection().await.unwrap();

    let category_list: Vec<_> = categories::Entity::find()
        .filter(categories::Column::Parent.eq(Uuid::nil()))
        .order_by_asc(categories::Column::Name)
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|category| CategoryItem {
            id: category.id,
            name: category.name,
        })
        .collect();

    Json(category_list)
}
