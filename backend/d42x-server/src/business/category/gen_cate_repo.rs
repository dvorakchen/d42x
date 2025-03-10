use crate::{business::cache::Cache, db::DbConnHelper};
use db_entity::categories;
use migration::async_trait;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde_json::json;
use tracing::debug;

use super::{CategoryItem, CategoryRepository};

lazy_static::lazy_static! {
    static ref TOP_CATEGORIES_CACHE_KEY: String = String::from("TOP_CATEGORIES_CACHE_KEY");
}

pub struct GenCategoryRepo<TCache, TDb>
where
    TCache: Cache<String, String>,
    TDb: DbConnHelper,
{
    cache: Option<TCache>,
    db: TDb,
}

impl<TCache, TDb> GenCategoryRepo<TCache, TDb>
where
    TCache: Cache<String, String>,
    TDb: DbConnHelper,
{
    pub fn new(db: TDb) -> Self {
        Self { cache: None, db }
    }

    pub fn with_cache(db: TDb, cache: Option<TCache>) -> Self {
        Self { cache, db }
    }
}

#[async_trait::async_trait]
impl<TCache, TDb> CategoryRepository for GenCategoryRepo<TCache, TDb>
where
    TCache: Cache<String, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
{
    async fn get_categories(&self) -> Vec<super::CategoryItem> {
        debug!("get categories");

        if let Some(cache) = &self.cache {
            if let Some(value) = cache.get(&TOP_CATEGORIES_CACHE_KEY) {
                if let Ok(value) = serde_json::from_str::<Vec<CategoryItem>>(value.as_str()) {
                    debug!("get in cache: {:?}", value);
                    return value;
                } else {
                    debug!("incorrect data in cache, remove");
                    cache.remove(&TOP_CATEGORIES_CACHE_KEY);
                }
            }
        }

        debug!("has not cache data");

        let db = self.db.get_connection().await.unwrap();

        let category_list: Vec<_> = categories::Entity::find()
            .filter(categories::Column::Parent.eq(Uuid::nil()))
            .order_by_asc(categories::Column::Name)
            .all(&db)
            .await
            .unwrap()
            .into_iter()
            .map(|category| CategoryItem {
                id: category.id,
                name: category.name,
            })
            .collect();

        if let Some(cache) = &self.cache {
            let cache_value = json!(category_list).to_string();
            debug!("set cache data: {:?}", cache_value);

            cache.insert(TOP_CATEGORIES_CACHE_KEY.clone(), cache_value);
        }

        category_list
    }
}
