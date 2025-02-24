use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub(crate) struct LogInReq {
    #[validate(length(min = 1, code = "username_empty"))]
    pub username: String,
    #[validate(length(min = 1, code = "password_empty"))]
    pub hashed_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LogInRes {
    pub username: String,
    pub email: String,
    pub jwt_token: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub(crate) struct ChangePwdReq {
    #[validate(length(min = 6, code = "hashed_password_current empty"))]
    hashed_password_current: String,
    #[validate(length(min = 6, code = "hashed_password_new empty"))]
    hashed_password_new: String,
}
