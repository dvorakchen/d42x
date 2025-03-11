use crate::{
    business::{Pagination, cache::Cache, meme::MemeUrl},
    db::DbConnHelper,
};
use chrono::{DateTime, FixedOffset, Utc};
use db_entity::{meme_urls, memes};
use migration::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde_json::json;
use tracing::debug;

use super::{GetFilter, Meme, MemeError, MemeRepository, PostMeme};

const PAGINATED_MEMES_CACHE_KEY: &str = "PAGINATED_MEMES_CACHE_KEY";
const DEFAULT_PAGE_SIZE: u64 = 10;

pub struct GenMemeRepo<TCache, TDb>
where
    TCache: Cache<String, String>,
    TDb: DbConnHelper,
{
    cache: Option<TCache>,
    db: TDb,
    page_size: u64,
}

impl<TCache, TDb> GenMemeRepo<TCache, TDb>
where
    TCache: Cache<String, String>,
    TDb: DbConnHelper,
{
    pub fn new(db: TDb) -> Self {
        Self {
            cache: None,
            db,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }

    pub fn with_cache(db: TDb, cache: Option<TCache>) -> Self {
        Self {
            cache,
            db,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

#[async_trait::async_trait]
impl<TCache, TDb> MemeRepository for GenMemeRepo<TCache, TDb>
where
    TCache: Cache<String, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
{
    async fn get_paginated_memes(&self, page: u64, category: Option<String>) -> Pagination<Meme> {
        let key = get_paginated_meme_cache_key(page, &category);
        debug!("cache key: {}", key);
        if let Some(cache) = &self.cache {
            if let Some(value) = cache.get(&key) {
                if let Ok(value) = serde_json::from_str::<Pagination<Meme>>(value.as_str()) {
                    debug!("get meme from cache: {:?}", value);
                    return value;
                } else {
                    cache.remove(&key);
                }
            }
        }

        let fetch_page = if page > 0 { page - 1 } else { 0 };

        let db = self.db.get_connection().await.unwrap();

        let now: DateTime<FixedOffset> = Utc::now().into();

        let mut paged_memes = memes::Entity::find();

        match category {
            Some(value) if !value.is_empty() => {
                paged_memes =
                    paged_memes.filter(memes::Column::Categories.contains(format!(";{};", value)))
            }
            _ => {}
        }

        let paged_memes = paged_memes
            .filter(memes::Column::Status.eq(memes::Status::Published))
            .filter(memes::Column::ShowDateTime.lt(now))
            .order_by_desc(memes::Column::ShowDateTime)
            .paginate(&db, self.page_size);

        let list: Vec<_> = paged_memes.fetch_page(fetch_page).await.unwrap();

        let paginated_meme_list = models_2_meme_list(list, &db).await;

        let total = paged_memes.num_pages().await.unwrap();

        let result = Pagination {
            page,
            total,
            size: self.page_size,
            list: paginated_meme_list,
        };

        if let Some(cache) = &self.cache {
            let cache_value = json!(result).to_string();
            debug!("set cache data: {:?}", cache_value);

            cache.insert(key, cache_value);
        }

        result
    }

    async fn get_paginated_all_memes(&self, filter: GetFilter) -> Pagination<Meme> {
        let db = self.db.get_connection().await.unwrap();

        let mut paged_memes = db_entity::memes::Entity::find();
        if let Some(status) = filter.status {
            paged_memes = paged_memes.filter(memes::Column::Status.eq(status));
        }

        let paged_memes = paged_memes
            .order_by_desc(memes::Column::ShowDateTime)
            .paginate(&db, filter.size);

        let list = paged_memes.fetch_page(filter.page - 1).await.unwrap();

        let meme_list = models_2_meme_list(list, &db).await;

        let total = paged_memes.num_pages().await.unwrap();

        let result = Pagination {
            page: filter.page,
            total,
            size: filter.size,
            list: meme_list,
        };

        result
    }

    async fn post_memes(&self, memes: Vec<PostMeme>) -> Result<(), MemeError> {
        if let Some(cache) = &self.cache {
            cache.clear();
        }

        if memes.len() == 0 {
            return Err(MemeError::HasNotAnyMeme);
        }

        let db = self.db.get_connection().await.map_err(MemeError::from)?;

        let txn = db.begin().await.map_err(MemeError::from)?;

        for item in memes {
            if let Err(e) = self.post_meme(item, &db).await {
                return Err(e);
            }
        }

        txn.commit().await.map_err(MemeError::from)?;

        Ok(())
    }
}

impl<TCache, TDb> GenMemeRepo<TCache, TDb>
where
    TCache: Cache<String, String> + Sync + Send,
    TDb: DbConnHelper + Sync + Send,
{
    async fn post_meme<'a, C: ConnectionTrait>(
        &self,
        meme: PostMeme,
        db: &'a C,
    ) -> Result<(), MemeError> {
        // insert memes and meme_urls
        let model = memes::ActiveModel {
            status: Set(memes::Status::Published),
            nickname: Set(meme.username),
            message: Set(meme.message.clone()),
            categories: Set(if meme.categories.len() == 0 {
                format!(";{};", db_entity::DEFAULT_CATEGORY)
            } else {
                format!(";{};", meme.categories.join(";"))
            }),
            ..memes::ActiveModel::new()
        }
        .insert(db)
        .await
        .map_err(MemeError::from)?;

        let memes: Vec<_> = meme
            .memes
            .iter()
            .map(|item| meme_urls::ActiveModel {
                meme_id: Set(model.id),
                url: Set(item.url.clone()),
                cover: Set(item.cover.clone()),
                format: Set(item.format.to_string()),
                hash: Set(item.hash.clone()),
                bed_id: Set(item.bed_id.clone()),
                ..meme_urls::ActiveModel::new()
            })
            .collect();

        meme_urls::Entity::insert_many(memes)
            .exec(db)
            .await
            .map_err(MemeError::from)?;

        Ok(())
    }
}

fn get_paginated_meme_cache_key(page: u64, category: &Option<String>) -> String {
    let category = if let Some(value) = category {
        value
    } else {
        ""
    };

    format!("{}-{}-{}", PAGINATED_MEMES_CACHE_KEY, page, category)
}

async fn models_2_meme_list(
    models: Vec<db_entity::memes::Model>,
    db: &impl ConnectionTrait,
) -> Vec<Meme> {
    let mut meme_list = vec![];

    for item in models {
        meme_list.push(Meme {
            id: item.id,
            likes: item.likes as usize,
            unlikes: item.unlikes as usize,
            categories: item
                .categories
                .split(';')
                .filter_map(|c| {
                    if c.is_empty() {
                        None
                    } else {
                        Some(c.to_string())
                    }
                })
                .collect(),
            nickname: item.nickname.clone(),
            show_date_time: item.show_date_time,
            list: item
                .find_related(db_entity::meme_urls::Entity)
                .all(db)
                .await
                .unwrap()
                .into_iter()
                .map(|e| MemeUrl {
                    id: e.id,
                    url: e.url,
                    cover: e.cover,
                    format: e.format.as_str().try_into().unwrap(),
                    sort: e.sort,
                })
                .collect(),
        });
    }

    meme_list
}
