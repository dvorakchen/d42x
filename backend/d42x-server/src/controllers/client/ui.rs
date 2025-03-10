use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{
    app::shared_data::{CategoryRepoSSType, MemeRepoSSType},
    business::category::CategoryItem,
};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub category: Option<String>,
}

pub async fn get_paginated_memes(
    Query(pagination): Query<Pagination>,
    State(meme_repo): State<MemeRepoSSType>,
) -> Json<crate::business::Pagination<crate::business::meme::Meme>> {
    let list: crate::business::Pagination<crate::business::meme::Meme> = meme_repo
        .repo
        .get_paginated_memes(pagination.page, pagination.category)
        .await;

    Json(list)
}

/// get all top categories
pub async fn get_categories(
    State(category_repo): State<CategoryRepoSSType>,
) -> Json<Vec<CategoryItem>> {
    let list = category_repo.repo.get_categories().await;
    Json(list)
}
