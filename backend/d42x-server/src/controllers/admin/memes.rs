use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
};
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use tracing::error;

use crate::{
    app::shared_data::{AccountRepoSSType, CategoryRepoSSType, MemeRepoSSType},
    authentication::AuthInformation,
    business::meme::{GetFilter, Meme},
    need_administrator,
};

use super::models::PostMemesReq;

pub async fn post_memes(
    Extension(admin_user): Extension<AuthInformation>,
    State(account_repo): State<AccountRepoSSType>,
    State(category_repo): State<CategoryRepoSSType>,
    State(meme_repo): State<MemeRepoSSType>,
    Json(post_memes): Json<Vec<PostMemesReq>>,
) -> Response {
    need_administrator!(account_repo, admin_user.id);

    let new_catepories: Vec<_> = post_memes
        .iter()
        .map(|item| item.categories.clone())
        .flatten()
        .collect();

    {
        let cate = category_repo.read().await;
        cate.repo.append_categories(new_catepories).await;
    }

    let new_memes = post_memes
        .into_iter()
        .map(crate::business::meme::PostMeme::from)
        .collect();

    if let Err(e) = meme_repo.repo.post_memes(new_memes).await {
        error!("post memes error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    } else {
        StatusCode::OK.into_response()
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    /// page number, base 1
    pub page: u64,
    pub size: u64,
    pub status: Option<String>,
}

pub async fn list_memes(
    Query(params): Query<QueryParams>,
    State(account_repo): State<AccountRepoSSType>,
    State(meme_repo): State<MemeRepoSSType>,
    Extension(admin_user): Extension<AuthInformation>,
) -> Response {
    need_administrator!(account_repo, admin_user.id);

    let page = if params.page > 0 { params.page } else { 0 };
    let mut status = params.status;
    if let Some(s) = &status {
        if db_entity::memes::Status::try_from(s.as_str()).is_err() {
            status = None;
        }
    }

    let list: crate::business::Pagination<Meme> = meme_repo
        .repo
        .get_paginated_all_memes(GetFilter {
            page,
            size: params.size,
            status: status.map(|s| db_entity::memes::Status::try_from(s.as_str()).unwrap()),
        })
        .await;

    Json(list).into_response()
}

pub async fn delete_meme(
    Path(id): Path<Uuid>,
    State(account_repo): State<AccountRepoSSType>,
    State(meme_repo): State<MemeRepoSSType>,
    Extension(admin_user): Extension<AuthInformation>,
) -> Response {
    need_administrator!(account_repo, admin_user.id);

    if let Ok(Some(meme)) = meme_repo.repo.get_meme(id).await {
        if let Ok(_) = meme.delete().await {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    .into_response()
}
