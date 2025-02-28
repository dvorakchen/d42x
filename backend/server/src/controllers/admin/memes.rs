use std::collections::HashMap;

use axum::{
    Extension, Json,
    extract::Query,
    http::StatusCode,
    response::{self, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::memes::{self, ActiveModel, Status};
use sea_orm::{
    ActiveModelBehavior, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::json;
use validator::Validate;

use crate::{
    authentication::AdminUser,
    controllers::admin::models::{MemeItemRes, Pagination},
    db::DbHelper,
    need_administrator,
};

use super::models::{Meme, PostMemesReq};

pub async fn post_memes(
    Extension(admin_user): Extension<AdminUser>,
    Json(data): Json<PostMemesReq>,
) -> Response {
    need_administrator!(admin_user.id);

    if let Err(e) = data.validate() {
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }

    let db = DbHelper::get_connection()
        .await
        .expect("post_memes get DatabaseConnection failed");

    let memes: Vec<_> = data.memes.iter().map(post_meme).collect();

    if let Err(e) = db_entity::memes::Entity::insert_many(memes).exec(&db).await {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    } else {
        StatusCode::OK.into_response()
    }
}

fn post_meme(meme: &Meme) -> ActiveModel {
    let mut new_meme = memes::ActiveModel::new();
    new_meme.url = Set(meme.url.clone());
    new_meme.hash = Set(meme.hash.clone());
    new_meme.bed_id = Set(meme.bed_id.clone());
    new_meme.status = Set(Status::Published);

    new_meme
}

pub async fn list_memes(
    Query(params): Query<HashMap<String, String>>,
    Extension(admin_user): Extension<AdminUser>,
) -> Response {
    need_administrator!(admin_user.id);

    let page = params
        .get("page")
        .map(String::to_string)
        .unwrap_or(String::from("0"))
        .parse()
        .unwrap_or(0)
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

    let list = paged_memes
        .fetch_page(page)
        .await
        .unwrap()
        .into_iter()
        .map(|item| MemeItemRes {
            id: item.id,
            url: item.url,
            format: item.format,
            status: item.status.to_string(),
            show_at: item.show_date_time.to_rfc3339(),
            created_at: item.created_date_time.to_rfc3339(),
        })
        .collect();

    let total = paged_memes.num_pages().await.unwrap();

    let res = Pagination {
        page: page as usize,
        size: 50,
        total: total as usize,
        list,
    };

    response::Json(json!(res)).into_response()
}
