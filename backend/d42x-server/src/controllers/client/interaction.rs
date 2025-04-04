use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::prelude::Uuid;

use crate::{app::shared_data::MemeRepoSSType, business::meme::Interaction};

pub async fn like_increase(
    Path(id): Path<Uuid>,
    State(meme_repo): State<MemeRepoSSType>,
) -> Response {
    if let Ok(Some(meme)) = meme_repo.repo.get_meme(id).await {
        if let Ok(_) = meme.increase_like().await {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    .into_response()
}

pub async fn unlike_increase(
    Path(id): Path<Uuid>,
    State(meme_repo): State<MemeRepoSSType>,
) -> Response {
    if let Ok(Some(meme)) = meme_repo.repo.get_meme(id).await {
        if let Ok(_) = meme.increase_unlike().await {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    .into_response()
}

pub async fn get_interactions(
    State(meme_repo): State<MemeRepoSSType>,
    Json(ids): Json<Vec<Uuid>>,
) -> Json<Vec<Interaction>> {
    let list = meme_repo.repo.get_interactions(ids).await;
    Json(list)
}
