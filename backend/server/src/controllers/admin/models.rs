use sea_orm::prelude::Uuid;
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
pub struct ChangePwdReq {
    #[validate(length(min = 6, code = "hashed_password_current empty"))]
    pub hashed_password_current: String,
    #[validate(length(min = 6, code = "hashed_password_new empty"))]
    pub hashed_password_new: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PostMemesReq {
    #[validate(length(min = 1))]
    pub memes: Vec<Meme>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Meme {
    #[validate(length(min = 1))]
    pub url: String,
    pub hash: String,
    pub bed_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemeItemRes {
    pub id: Uuid,
    pub url: String,
    pub format: String,
    pub status: String,
    pub show_at: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination<T>
where
    T: Sized,
{
    /// current page number
    pub page: usize,
    /// list size per page
    pub size: usize,
    /// total page count
    pub total: usize,
    /// list data
    pub list: Vec<T>,
}
