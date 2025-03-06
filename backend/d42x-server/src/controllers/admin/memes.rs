use std::collections::HashMap;

use axum::{
    Extension, Json,
    extract::Query,
    http::StatusCode,
    response::{self, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::{
    meme_urls,
    memes::{self},
};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

use crate::{
    authentication::AdminUser,
    controllers::admin::models::{MemeItemRes, MemeUrlsItem, Pagination},
    db::DbHelper,
    need_administrator,
};

use super::models::PostMemesReq;

pub async fn post_memes(
    Extension(admin_user): Extension<AdminUser>,
    Json(post_memes): Json<Vec<PostMemesReq>>,
) -> Response {
    need_administrator!(admin_user.id);

    if post_memes.len() == 0 {
        return (StatusCode::BAD_REQUEST, "Not Post").into_response();
    }

    let db = DbHelper::get_connection()
        .await
        .expect("post_memes get DatabaseConnection failed");

    let txn = db.begin().await.unwrap();

    let mut result = vec![];

    for item in post_memes {
        result.push(post_meme(item, admin_user.username.clone(), &db).await);
    }

    if result.iter().any(|e| e.is_err()) {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    } else {
        txn.commit().await.unwrap();
        StatusCode::OK.into_response()
    }
}

async fn post_meme<'a, C: ConnectionTrait>(
    data: PostMemesReq,
    username: String,
    db: &'a C,
) -> Result<(), DbErr> {
    // insert memes and meme_urls
    let model = memes::ActiveModel {
        status: Set(memes::Status::Published),
        nickname: Set(username),
        message: Set(data.message.clone()),
        categories: Set(if data.categories.is_empty() {
            format!(";{};", db_entity::DEFAULT_CATEGORY)
        } else {
            format!(";{};", data.categories)
        }),
        ..memes::ActiveModel::new()
    }
    .insert(db)
    .await
    .unwrap();

    let memes: Vec<_> = data
        .memes
        .iter()
        .map(|item| meme_urls::ActiveModel {
            meme_id: Set(model.id),
            url: Set(item.url.clone()),
            cover: Set(item.cover.clone()),
            format: Set(item.format.to_string()),
            hash: Set(item.hash.clone()),
            bed_id: Set(item.bed_id.clone()),
            ..meme_urls::ActiveModel::new()
        })
        .collect();

    meme_urls::Entity::insert_many(memes).exec(db).await?;

    Ok(())
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
