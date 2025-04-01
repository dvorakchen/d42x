use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod cache;
pub mod category;
pub mod meme;
pub mod suggests;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination<T: std::fmt::Debug> {
    pub page: u64,
    pub total: u64,
    pub size: u64,
    pub list: Vec<T>,
}
