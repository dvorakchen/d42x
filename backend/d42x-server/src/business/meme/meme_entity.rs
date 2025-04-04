use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::db::DbConnHelper;

use super::MemeError;

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

    pub async fn increase_like(&self) -> Result<(), MemeError> {
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

    pub async fn increase_unlike(&self) -> Result<(), MemeError> {
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
}
