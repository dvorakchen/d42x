use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::Mutex;
use tracing::trace;

use crate::config;

async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    trace!("database_url: {}", *config::DATABASE_URL);

    let db: DatabaseConnection = Database::connect(config::DATABASE_URL.to_string()).await?;
    db.ping().await?;

    Ok(db)
}

pub type ExtensionDb = Arc<Mutex<DbHelper>>;
pub struct DbHelper;

impl DbHelper {
    pub async fn get_connection() -> Result<DatabaseConnection, DbErr> {
        crate::db::get_connection().await
    }
}
