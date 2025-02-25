mod models;

use axum::{
    http::{header::ORIGIN, HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    Extension,
};
use models::{ChangePwdReq, LogInReq, LogInRes};
use tracing::{debug, error, warn};
use validator::Validate;

use crate::{
    authentication::{gen_jwt_token, AdminUser},
    business::auth::{Administrator, AdministratorError},
};

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

    let admin = Administrator::new_from_id(admin_user.id).await;

    debug!("change_pwd_req: {:?}", change_pwd_req);
    debug!("admin_user.id: {}", admin_user.id);

    (StatusCode::OK).into_response()
}
