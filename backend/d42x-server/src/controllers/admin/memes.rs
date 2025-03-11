use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use serde::Deserialize;
use tracing::error;

use crate::{
    app::shared_data::MemeRepoSSType,
    authentication::AdminUser,
    business::meme::{GetFilter, Meme},
    need_administrator,
};

use super::models::PostMemesReq;

pub async fn post_memes(
    Extension(admin_user): Extension<AdminUser>,
    State(meme_repo): State<MemeRepoSSType>,
    Json(post_memes): Json<Vec<PostMemesReq>>,
) -> Response {
    need_administrator!(admin_user.id);

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
    pub page: u64,
    pub size: u64,
    pub status: Option<String>,
}

pub async fn list_memes(
    Query(params): Query<QueryParams>,
    State(meme_repo): State<MemeRepoSSType>,
    Extension(admin_user): Extension<AdminUser>,
) -> Response {
    need_administrator!(admin_user.id);

    let page = if params.page > 0 { params.page - 1 } else { 0 };
    let status = params.status;

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
