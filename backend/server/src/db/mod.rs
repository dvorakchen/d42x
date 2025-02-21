use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::Mutex;
use tracing::trace;

async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = dotenv::var("DATABASE_URL").expect("connat get env: DATABASE_URL");
    trace!("database_url: {}", database_url);

    let db: DatabaseConnection = Database::connect(database_url).await?;
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
