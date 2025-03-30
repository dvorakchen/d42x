use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use sea_orm::prelude::Uuid;

use crate::{
    app::shared_data::{AccountRepoSSType, CategoryRepoSSType},
    authentication::AdminUser,
    need_administrator,
};

pub async fn update_categories(
    Path(meme_id): Path<Uuid>,
    Extension(admin_user): Extension<AdminUser>,
    State(category_repo): State<CategoryRepoSSType>,
    State(account_repo): State<AccountRepoSSType>,
    Json(list): Json<Vec<String>>,
) -> Response {
    need_administrator!(account_repo, admin_user.id);

    let cate = category_repo.write().await;
    cate.repo.update_catgories(meme_id, list).await;

    StatusCode::OK.into_response()
}
