mod category;
mod memes;
mod models;

use axum::{
    Extension,
    extract::State,
    http::{HeaderMap, StatusCode, header::ORIGIN},
    response::{IntoResponse, Json, Response},
};
use models::{ChangePwdReq, LogInReq, LogInRes};
use tracing::warn;
use validator::Validate;

use crate::{
    app::shared_data::AccountRepoSSType,
    authentication::{AuthInformation, gen_jwt_token},
    business::accounts::admin::AdministratorError,
};

pub use category::*;
pub use memes::*;

#[macro_export]
macro_rules! need_administrator {
    ($acc_repo: expr, $admin_id: expr) => {
        use axum::response::IntoResponse;
        if $acc_repo
            .repo
            .get_administractor_by_id($admin_id)
            .await
            .is_none()
        {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
}

pub async fn check_logged_in() -> Response {
    StatusCode::OK.into_response()
}

pub(crate) async fn log_in(
    header: HeaderMap,
    State(account_repo): State<AccountRepoSSType>,
    Json(log_in_req): Json<LogInReq>,
) -> Response {
    if let Err(e) = log_in_req.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
    }

    let admin = account_repo
        .repo
        .get_administractor_by_username(log_in_req.username.clone())
        .await;
    match admin {
        Some(mut admin) => {
            if !admin.verify_password(&log_in_req.hashed_password) {
                return StatusCode::BAD_REQUEST.into_response();
            }

            let origin = header.get(ORIGIN).unwrap();
            admin
                .log_in_activity(origin.to_str().unwrap())
                .await
                .unwrap();

            let jwt_token = gen_jwt_token(&admin.model.id, &admin.model.username);

            Json(LogInRes {
                username: admin.model.username,
                email: admin.model.email,
                jwt_token,
            })
            .into_response()
        }
        None => {
            warn!("not found: {}", log_in_req.username);

            StatusCode::NO_CONTENT.into_response()
        }
    }
}

pub async fn change_password(
    Extension(admin_user): Extension<AuthInformation>,
    State(account_repo): State<AccountRepoSSType>,
    Json(change_pwd_req): Json<ChangePwdReq>,
) -> impl IntoResponse {
    need_administrator!(account_repo, admin_user.id);

    if let Err(e) = change_pwd_req.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
    }

    let admin = account_repo
        .repo
        .get_administractor_by_id(admin_user.id)
        .await;
    match admin {
        Some(mut admin) if admin.model.username == admin_user.username => {
            match admin
                .change_password(
                    &change_pwd_req.hashed_password_current,
                    &change_pwd_req.hashed_password_new,
                )
                .await
            {
                Ok(_) => StatusCode::OK.into_response(),
                Err(AdministratorError::IncorrectPassword) => {
                    StatusCode::BAD_REQUEST.into_response()
                }
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        _ => (StatusCode::BAD_REQUEST, "Not the same admin user").into_response(),
    }
}
