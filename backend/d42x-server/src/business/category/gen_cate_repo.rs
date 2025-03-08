use crate::{business::cache::Cache, db::DbConnHelper};
use db_entity::categories;
use migration::async_trait;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use super::{CategoryItem, CategoryRepository};

const TOP_CATEGORIES_CACHE_KEY: &str = "TOP_CATEGORIES_CACHE_KEY";

pub struct GenCategoryRepo<TCache, TDb>
where
    TCache: Cache<&'static str, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
{
    cache: Option<TCache>,
    db: TDb,
}

impl<TCache, TDb> GenCategoryRepo<TCache, TDb>
where
    TCache: Cache<&'static str, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
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
    TCache: Cache<&'static str, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
{
    async fn get_categories(&self) -> Vec<super::CategoryItem> {
        if let Some(cache) = &self.cache {
            if let Some(value) = cache.get(&TOP_CATEGORIES_CACHE_KEY) {
                if let Ok(value) = serde_json::from_str::<Vec<CategoryItem>>(value.as_str()) {
                    return value;
                } else {
                    cache.remove(&TOP_CATEGORIES_CACHE_KEY);
                }
            }
        }

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

        category_list
    }
}
