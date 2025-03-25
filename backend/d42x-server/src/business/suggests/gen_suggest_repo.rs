use db_entity::suggests;
use migration::async_trait;
use sea_orm::{Condition, QueryOrder, Set, prelude::*};

use crate::{
    business::{Pagination, meme::MemeUrl, suggests::SuggestError},
    db::DbConnHelper,
};

use super::{GetFilter, SuggestRepository, SuggestResult, Suggestion};

// const PAGINATED_SUGGEST_CACHE_KEY: &str = "PAGINATED_SUGGEST_CACHE_KEY";
const DEFAULT_PAGE_SIZE: u64 = 10;

pub struct GenSuggestRepo<TDb>
where
    TDb: DbConnHelper,
{
    db: TDb,
    page_size: u64,
}

impl<TDb> GenSuggestRepo<TDb>
where
    TDb: DbConnHelper,
{
    pub fn new(db: TDb) -> Self {
        Self {
            db,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

#[async_trait::async_trait]
impl<TDb> SuggestRepository for GenSuggestRepo<TDb>
where
    TDb: DbConnHelper + Sync + Send,
{
    async fn create(
        &self,
        meme_id: Uuid,
        list: Vec<String>,
        apply_user_id: Uuid,
    ) -> SuggestResult<()> {
        let db = self.db.get_connection().await?;

        let meme = db_entity::memes::Entity::find_by_id(meme_id)
            .one(&db)
            .await?
            .ok_or(SuggestError::CreateFail("cannot find related meme"))?;

        let account = db_entity::accounts::Entity::find_by_id(apply_user_id)
            .one(&db)
            .await?
            .ok_or(SuggestError::CreateFail("cannot find related meme"))?;

        db_entity::suggests::ActiveModel {
            meme_id: Set(meme_id),
            before: Set(meme.categories),
            after: Set(format!(";{};", list.join(";"))),
            status: Set(suggests::Status::Wait),
            account_id: Set(account.id),
            operator_id: Set(Uuid::nil()),
            ..suggests::ActiveModel::new()
        }
        .insert(&db)
        .await
        .expect("create suggests failed");

        Ok(())
    }

    async fn get_paginated_suggests(&self, filter: GetFilter) -> Pagination<Suggestion> {
        let db = self.db.get_connection().await.unwrap();

        let mut condition = Condition::all();

        if let Some(status) = filter.status {
            condition = condition.add(suggests::Column::Status.eq(status));
        }

        let fetch_page = if filter.page > 0 { filter.page - 1 } else { 0 };

        let page_query = suggests::Entity::find()
            .filter(condition)
            .order_by_asc(suggests::Column::CreatedDateTime)
            .paginate(&db, self.page_size);

        let suggest_list = page_query
            .fetch_page(fetch_page)
            .await
            .expect("get_paginated_suggest failed");

        let total = page_query.num_pages().await.unwrap();

        let memes = db_entity::memes::Entity::find()
            .find_with_related(db_entity::meme_urls::Entity)
            .filter(db_entity::memes::Column::Id.is_in(suggest_list.iter().map(|t| t.meme_id)))
            .all(&db)
            .await
            .expect("query memes failed");

        let apply_users = db_entity::accounts::Entity::find()
            .filter(
                db_entity::accounts::Column::Id.is_in(suggest_list.iter().map(|t| t.account_id)),
            )
            .all(&db)
            .await
            .expect("query apply_users failed");

        let operate_users = db_entity::accounts::Entity::find()
            .filter(
                db_entity::accounts::Column::Id.is_in(suggest_list.iter().map(|t| t.operator_id)),
            )
            .all(&db)
            .await
            .expect("query operate_users failed");

        let suggest_model_2_suggestion = |suggest: suggests::Model| -> Suggestion {
            let apply_username = apply_users
                .iter()
                .find_map(|item| {
                    if item.id == suggest.account_id {
                        Some(item.username.clone())
                    } else {
                        None
                    }
                })
                .or(Some(String::new()))
                .unwrap();

            let operator_username = operate_users
                .iter()
                .find_map(|item| {
                    if item.id == suggest.operator_id {
                        Some(item.username.clone())
                    } else {
                        None
                    }
                })
                .or(Some(String::new()))
                .unwrap();

            let cur_category = memes
                .iter()
                .find_map(|item| {
                    if item.0.id == suggest.meme_id {
                        Some(
                            item.0
                                .categories
                                .split(';')
                                .map(|t| t.to_string())
                                .collect(),
                        )
                    } else {
                        None
                    }
                })
                .or(Some(vec![]))
                .unwrap();

            let meme_urls = memes
                .iter()
                .find_map(|item| {
                    if item.0.id == suggest.meme_id {
                        let meme_urls: Vec<_> = item
                            .1
                            .iter()
                            .map(|url| MemeUrl {
                                id: url.id,
                                url: url.url.clone(),
                                cover: url.cover.clone(),
                                format: url.format.as_str().try_into().unwrap(),
                                sort: url.sort,
                            })
                            .collect();
                        Some(meme_urls)
                    } else {
                        None
                    }
                })
                .unwrap();

            Suggestion {
                id: suggest.id,
                meme_id: suggest.meme_id,
                before_category: suggest.before.split(';').map(|t| t.to_string()).collect(),
                after_category: suggest.after.split(';').map(|t| t.to_string()).collect(),
                apply_user_id: suggest.account_id,
                operator: suggest.operator_id,
                created_date_time: suggest.created_date_time,
                apply_username,
                operator_username,
                meme_urls,
                cur_category,
            }
        };

        let list: Vec<_> = suggest_list
            .into_iter()
            .map(suggest_model_2_suggestion)
            .collect();

        Pagination {
            page: filter.page,
            total,
            size: self.page_size,
            list,
        }
    }

    async fn set_suggest_status(
        &self,
        id: Uuid,
        status: db_entity::suggests::Status,
        operator_id: Uuid,
    ) {
        let db = self.db.get_connection().await.unwrap();

        let model = suggests::Entity::find_by_id(id).one(&db).await.unwrap();
        if model.is_none() {
            return;
        }

        let mut model: suggests::ActiveModel = model.unwrap().into();

        model.status = Set(status);
        model.operator_id = Set(operator_id);

        model.update(&db).await.unwrap();
    }
}
