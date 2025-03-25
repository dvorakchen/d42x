use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    app::shared_data::{CategoryRepoSSType, MemeRepoSSType, SuggestRepoSSType},
    business::category::CategoryItem,
};

use super::models::CreateSuggestReq;

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
    let list = {
        let cate = category_repo.read().await;
        cate.repo.get_categories().await
    };

    Json(list)
}

pub async fn create_suggest(
    State(suggest_repo): State<SuggestRepoSSType>,
    Json(req): Json<CreateSuggestReq>,
) -> Response {
    if let Err(_) = req.validate() {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    if let Ok(()) = suggest_repo
        .repo
        .create(req.meme_id, req.list, req.apply_user_id)
        .await
    {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    .into_response()
}
