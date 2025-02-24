use crate::{
    authentication::{validate_claims, AdminUser, CLAIM_UID, CLAIM_USERNAME},
    config,
};
use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jwt::VerifyWithKey;
use sea_orm::prelude::Uuid;
use tracing::debug;

static WHITE_LIST: &[&str] = &["/api/admin/login"];
static WHITE_METHODS: &[Method] = &[Method::OPTIONS, Method::HEAD];

const BEARER_PREFIX: &str = "Bearer ";

pub async fn jwt_auth_middleware(mut request: Request, next: Next) -> Response {
    let path = request.uri().path();
    if WHITE_LIST.contains(&path) || WHITE_METHODS.contains(request.method()) {
        return next.run(request).await;
    }

    let mut response = (StatusCode::UNAUTHORIZED).into_response();

    // athentication, bearer
    if let Some(bearer) = request.headers().get(AUTHORIZATION) {
        if let Some(token) = bearer.to_str().unwrap().strip_prefix(BEARER_PREFIX) {
            debug!("token: {}", token);
            if let Ok(claims) = token.verify_with_key(&*config::JWT_KEY) {
                if validate_claims(&claims) {
                    match (
                        claims.private.get(CLAIM_UID),
                        claims.private.get(CLAIM_USERNAME),
                    ) {
                        (Some(id), Some(username)) => {
                            let id: Uuid = serde_json::from_value(id.clone()).unwrap();
                            let username: String =
                                serde_json::from_value(username.clone()).unwrap();
                            let au = AdminUser { id, username };

                            request.extensions_mut().insert(au);

                            response = next.run(request).await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    response
}
