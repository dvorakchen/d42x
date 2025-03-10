pub mod gen_cate_repo;

use migration::async_trait;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait CategoryRepository {
    async fn get_categories(&self) -> Vec<CategoryItem> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryItem {
    pub id: Uuid,
    pub name: String,
}

pub struct PanicCategoryRepo;

#[async_trait::async_trait]
impl CategoryRepository for PanicCategoryRepo {}
