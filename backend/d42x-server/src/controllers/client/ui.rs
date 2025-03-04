use axum::{Json, extract::Query};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::{categories, memes};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, prelude::Uuid};
use serde::Deserialize;

use crate::{config::AllowMemeFormats, db::DbHelper};

use super::models::{CategoryItem, PaginatedMemeList};

const SIZE_PER_PAGE: u64 = 20;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub category: Option<String>,
}

pub async fn get_meme_list(Query(pagination): Query<Pagination>) -> Json<PaginatedMemeList> {
    let paginated_memes = get_meme_list_pagination(pagination.page, pagination.category).await;

    Json(PaginatedMemeList {
        page: paginated_memes.page,
        total: paginated_memes.total_page,
        list: paginated_memes.list,
    })
}

struct PaginatedMemes {
    pub page: u64,
    pub total_page: u64,
    pub list: Vec<super::models::Meme>,
}

async fn get_meme_list_pagination(page: u64, category: Option<String>) -> PaginatedMemes {
    let fetch_page = if page > 0 { page - 1 } else { page };

    let db = DbHelper::get_connection().await.unwrap();

    let now: DateTime<FixedOffset> = Utc::now().into();

    let mut paged_memes = memes::Entity::find();

    match category {
        Some(value) if !value.is_empty() => {
            paged_memes =
                paged_memes.filter(memes::Column::Categories.contains(format!(";{};", value)))
        }
        _ => {}
    }

    let paged_memes = paged_memes
        .filter(memes::Column::Status.eq(memes::Status::Published))
        .filter(memes::Column::ShowDateTime.lt(now))
        .order_by_desc(memes::Column::ShowDateTime)
        .paginate(&db, SIZE_PER_PAGE);

    let list: Vec<_> = paged_memes
        .fetch_page(fetch_page)
        .await
        .unwrap()
        .into_iter()
        .map(|item| super::models::Meme {
            id: item.id,
            url: item.url,
            cover: item.cover,
            format: AllowMemeFormats::try_from(item.format.as_str()).unwrap(),
            likes: item.likes as usize,
            unlikes: item.unlikes as usize,
            categories: item
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
            nickname: item.nickname,
            show_date_time: item.show_date_time,
        })
        .collect();

    let total_page = paged_memes.num_pages().await.unwrap();

    PaginatedMemes {
        page,
        total_page,
        list,
    }
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
