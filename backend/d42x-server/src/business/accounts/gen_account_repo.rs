use migration::async_trait;
use sea_orm::prelude::Uuid;

use crate::{business::auth::Administrator, db::DbConnHelper};

use super::AccountRepository;

pub struct GenAccountRepo<T: DbConnHelper + Clone> {
    db: T,
}

impl<T: DbConnHelper + Clone> GenAccountRepo<T> {
    pub fn new(db: T) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl<T> AccountRepository for GenAccountRepo<T>
where
    T: DbConnHelper + 'static + Sync + Send + Clone,
{
    async fn get_administractor_by_id(&self, id: Uuid) -> Option<Administrator> {
        Administrator::new_from_id(id, self.db.clone()).await.ok()
    }

    async fn get_administractor_by_username(&self, username: String) -> Option<Administrator> {
        Administrator::new(username, self.db.clone()).await.ok()
    }
}
