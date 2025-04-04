pub mod shared_db_helper;

use migration::async_trait;
use sea_orm::{DatabaseConnection, DbErr};

#[async_trait::async_trait]
pub trait DbConnHelper {
    async fn get_connection(&self) -> Result<DatabaseConnection, DbErr>;
}

#[cfg(test)]
pub mod test {
    use migration::{Migrator, MigratorTrait, async_trait};

    use sea_orm::{Database, DatabaseConnection, DbErr};

    use super::DbConnHelper;

    #[derive(Clone)]
    pub struct TestDB(DatabaseConnection);

    #[async_trait::async_trait]
    impl DbConnHelper for TestDB {
        async fn get_connection(&self) -> Result<DatabaseConnection, DbErr> {
            Ok(self.0.clone())
        }
    }

    impl TestDB {
        pub async fn new() -> Self {
            let conn = Database::connect("sqlite::memory:").await.unwrap();
            conn.ping().await.unwrap();

            Migrator::up(&conn, None).await.unwrap();
            TestDB(conn)
        }
    }
}
