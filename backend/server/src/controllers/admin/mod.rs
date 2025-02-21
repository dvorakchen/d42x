mod models;

use axum::{
    http::{header::ORIGIN, HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
};
use models::{LogInReq, LogInRes};
use tracing::{error, warn};
use validator::Validate;

use crate::business::auth::{Administrator, AdministratorError};

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
                return StatusCode::UNAUTHORIZED.into_response();
            }

            let origin = header.get(ORIGIN).unwrap();
            admin
                .log_in_activity(origin.to_str().unwrap())
                .await
                .unwrap();

            Json(LogInRes {
                username: admin.model.username,
                email: admin.model.email,
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
