pub mod shared_db_helper;

use migration::async_trait;
use sea_orm::{DatabaseConnection, DbErr};

#[async_trait::async_trait]
pub trait DbConnHelper {
    async fn get_connection(&self) -> Result<DatabaseConnection, DbErr>;
}

#[cfg(test)]
pub mod test {
    use migration::{Migrator, MigratorTrait};
    use tokio::sync::OnceCell;

    use sea_orm::{Database, DatabaseConnection};

    static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

    pub async fn setup_db() -> DatabaseConnection {
        let t = DB_CONNECTION
            .get_or_init(|| async {
                let conn = Database::connect("sqlite::memory:").await.unwrap();
                conn.ping().await.unwrap();

                Migrator::fresh(&conn).await.unwrap();

                conn
            })
            .await;

        t.clone()
    }
}
