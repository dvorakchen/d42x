mod category;
mod memes;
mod models;

use axum::{
    Extension,
    http::{HeaderMap, StatusCode, header::ORIGIN},
    response::{IntoResponse, Json, Response},
};
use models::{ChangePwdReq, LogInReq, LogInRes};
use tracing::{error, warn};
use validator::Validate;

use crate::{
    authentication::{AdminUser, gen_jwt_token},
    business::auth::{Administrator, AdministratorError},
};

pub use memes::*;
pub use category::*;

#[macro_export]
macro_rules! need_administrator {
    ($admin_id: expr) => {
        use axum::response::IntoResponse;
        if let Err(e) = crate::business::auth::Administrator::new_from_id($admin_id).await {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };
}

pub async fn check_logged_in() -> Response {
    StatusCode::OK.into_response()
}

pub(crate) async fn log_in(header: HeaderMap, Json(log_in_req): Json<LogInReq>) -> Response {
    if let Err(e) = log_in_req.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
    }

    match Administrator::new(
        log_in_req.username.clone(), // , log_in_req.hashed_password
    )
    .await
    {
        Ok(mut admin) => {
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

        Err(AdministratorError::NotFound(username)) => {
            warn!("not found: {}", username);

            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            error!("error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn change_password(
    Extension(admin_user): Extension<AdminUser>,
    Json(change_pwd_req): Json<ChangePwdReq>,
) -> impl IntoResponse {
    need_administrator!(admin_user.id);

    if let Err(e) = change_pwd_req.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
    }

    match Administrator::new_from_id(admin_user.id).await {
        Ok(mut admin) if admin.model.username == admin_user.username => {
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
