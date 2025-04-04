pub mod gen_cate_repo;

use migration::async_trait;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait CategoryRepository {
    async fn get_categories(&self) -> Vec<CategoryItem> {
        unimplemented!()
    }

    async fn append_categories(&self, _list: Vec<String>) {
        unimplemented!()
    }

    async fn update_catgories(&self, _meme_id: Uuid, _new_list: Vec<String>) {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryItem {
    pub id: Uuid,
    pub name: String,
    pub meme_count: i64,
}

pub struct PanicCategoryRepo;

#[async_trait::async_trait]
impl CategoryRepository for PanicCategoryRepo {}
