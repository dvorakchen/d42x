use serde::{Deserialize, Serialize};

pub mod auth;
pub mod cache;
pub mod category;

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub page: u64,
    pub total: u64,
    pub list: Vec<T>,
}
