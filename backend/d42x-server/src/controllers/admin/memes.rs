use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::{self, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::{
    meme_urls,
    memes::{self},
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;
use tracing::error;

use crate::{
    app::shared_data::MemeRepoSSType,
    authentication::AdminUser,
    controllers::admin::models::{MemeItemRes, MemeUrlsItem, Pagination},
    db::DbHelper,
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

pub async fn list_memes(
    Query(params): Query<HashMap<String, String>>,
    Extension(admin_user): Extension<AdminUser>,
) -> Response {
    need_administrator!(admin_user.id);

    let page = params
        .get("page")
        .map(String::to_string)
        .unwrap_or(String::from("1"))
        .parse()
        .unwrap_or(1)
        - 1;

    let status = params.get("status");

    let now: DateTime<FixedOffset> = Utc::now().into();

    let db = DbHelper::get_connection().await.unwrap();

    let mut paged_memes =
        db_entity::memes::Entity::find().filter(memes::Column::ShowDateTime.lt(now));

    if let Some(status) = status {
        if let Ok(status) = memes::Status::try_from(status.as_str()) {
            paged_memes = paged_memes.filter(memes::Column::Status.eq(status));
        }
    }
    let paged_memes = paged_memes
        .order_by_desc(memes::Column::ShowDateTime)
        .paginate(&db, 50);

    let list = paged_memes.fetch_page(page).await.unwrap();

    let mut result = vec![];
    for item in list {
        result.push(MemeItemRes {
            id: item.id,
            status: item.status.to_string(),
            show_at: item.show_date_time.to_rfc3339(),
            created_at: item.created_date_time.to_rfc3339(),
            list: item
                .find_related(meme_urls::Entity)
                .all(&db)
                .await
                .unwrap()
                .into_iter()
                .map(|e| MemeUrlsItem {
                    id: e.id,
                    url: e.url,
                    cover: e.cover,
                    format: crate::config::AllowMemeFormats::try_from(e.format.as_str()).unwrap(),
                })
                .collect(),
        });
    }

    let total = paged_memes.num_pages().await.unwrap();

    let res = Pagination {
        page: (page + 1) as usize,
        size: 50,
        total: total as usize,
        list: result,
    };

    response::Json(res).into_response()
}
