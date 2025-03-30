pub mod shared_db_helper;

use migration::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use sea_orm::{DatabaseConnection, DbErr};

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait DbConnHelper {
    async fn get_connection(&self) -> Result<DatabaseConnection, DbErr>;
}

#[cfg(test)]
pub mod test {
    use migration::{Migrator, MigratorTrait};

    use sea_orm::{Database, DatabaseConnection};

    pub async fn setup_db() -> DatabaseConnection {
        println!("setup_db");
        let conn = Database::connect("sqlite::memory:").await.unwrap();
        conn.ping().await.unwrap();

        Migrator::fresh(&conn).await.unwrap();

        conn
    }
}
