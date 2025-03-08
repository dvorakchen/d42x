use migration::async_trait;
use sea_orm::{Database, DatabaseConnection, DbErr};

use super::DbConnHelper;

#[derive(Clone, Debug)]
pub struct SharedDbHelper {
    db_url: String,
}

impl SharedDbHelper {
    pub fn new(db_url: String) -> Self {
        Self { db_url }
    }
}

#[async_trait::async_trait]
impl DbConnHelper for SharedDbHelper {
    async fn get_connection(&self) -> Result<DatabaseConnection, DbErr> {
        let db: DatabaseConnection = Database::connect(self.db_url.as_str()).await?;
        db.ping().await?;

        Ok(db)
    }
}
