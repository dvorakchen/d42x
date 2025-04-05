use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set, TransactionTrait,
};

use crate::db::DbConnHelper;

use super::{Meme, MemeError, MemeResult, MemeUrl};

pub struct MemeEntity {
    model: db_entity::memes::Model,
    db: Box<dyn DbConnHelper + 'static + Sync + Send>,
}

impl MemeEntity {
    pub fn new(
        model: db_entity::memes::Model,
        db: impl DbConnHelper + 'static + Sync + Send,
    ) -> Self {
        Self {
            model,
            db: Box::new(db),
        }
    }

    pub async fn get_detail(&self) -> MemeResult<Meme> {
        let db = self.db.get_connection().await?;

        let urls: Vec<_> = db_entity::meme_urls::Entity::find()
            .filter(db_entity::meme_urls::Column::MemeId.eq(self.model.id))
            .all(&db)
            .await
            .unwrap()
            .into_iter()
            .map(|url| MemeUrl {
                id: url.id,
                url: url.url,
                cover: url.cover,
                format: crate::config::AllowMemeFormats::try_from(url.format.as_str()).unwrap(),
                sort: url.sort,
            })
            .collect();

        let detail = Meme {
            id: self.model.id,
            short_id: self.model.short_id.to_owned(),
            categories: self
                .model
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
            nickname: self.model.nickname.clone(),
            show_date_time: self.model.show_date_time,
            create_date_time: self.model.created_date_time,
            status: self.model.status,
            list: urls,
        };

        Ok(detail)
    }

    pub async fn increase_like(&self) -> MemeResult<()> {
        let db = self.db.get_connection().await?;

        let meme = db_entity::memes::Entity::find_by_id(self.model.id)
            .one(&db)
            .await?
            .ok_or(MemeError::HasNotAnyMeme)?;
        let likes = meme.likes + 1;
        let mut meme: db_entity::memes::ActiveModel = meme.into();
        meme.likes = Set(likes);

        meme.update(&db).await?;

        return Ok(());
    }

    pub async fn increase_unlike(&self) -> MemeResult<()> {
        let db = self.db.get_connection().await?;

        let meme = db_entity::memes::Entity::find_by_id(self.model.id)
            .one(&db)
            .await?
            .ok_or(MemeError::HasNotAnyMeme)?;
        let unlikes = meme.unlikes + 1;
        let mut meme: db_entity::memes::ActiveModel = meme.into();
        meme.unlikes = Set(unlikes);

        meme.update(&db).await?;

        return Ok(());
    }

    pub async fn delete(self) -> MemeResult<db_entity::memes::Model> {
        let db = self.db.get_connection().await?;
        let txn = db.begin().await?;

        let model = db_entity::memes::Entity::find_by_id(self.model.id)
            .one(&db)
            .await?;

        let backup_model;
        if let Some(meme) = model {
            backup_model = meme.clone();
            db_entity::meme_urls::Entity::delete_many()
                .filter(db_entity::meme_urls::Column::MemeId.eq(meme.id))
                .exec(&txn)
                .await?;
            meme.delete(&txn).await?;
        } else {
            txn.rollback().await.unwrap();
            return Err(MemeError::HasNotAnyMeme);
        }

        txn.commit().await?;

        Ok(backup_model)
    }
}
