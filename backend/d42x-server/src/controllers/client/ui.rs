use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::memes;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

use crate::{app::shared_data::CategoryRepoSSType, business::category::CategoryItem, db::DbHelper};

use super::models::{MemeUrl, PaginatedMemeList};

const SIZE_PER_PAGE: u64 = 10;

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

    let list: Vec<_> = paged_memes.fetch_page(fetch_page).await.unwrap();

    let mut url_list = vec![];

    for item in list {
        url_list.push(super::models::Meme {
            id: item.id,
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
            nickname: item.nickname.clone(),
            show_date_time: item.show_date_time,
            list: item
                .find_related(db_entity::meme_urls::Entity)
                .all(&db)
                .await
                .unwrap()
                .into_iter()
                .map(|e| MemeUrl {
                    id: e.id,
                    url: e.url,
                    cover: e.cover,
                    format: e.format.as_str().try_into().unwrap(),
                    sort: e.sort,
                })
                .collect(),
        });
    }
    let total_page = paged_memes.num_pages().await.unwrap();

    PaginatedMemes {
        page,
        total_page,
        list: url_list,
    }
}

/// get all top categories
pub async fn get_categories(
    State(category_repo): State<CategoryRepoSSType>,
) -> Json<Vec<CategoryItem>> {
    let list = category_repo.repo.get_categories().await;
    Json(list)
}
