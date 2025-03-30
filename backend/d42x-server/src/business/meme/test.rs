#[cfg(test)]
mod test {
    use sea_orm::{EntityTrait, PaginatorTrait};

    use crate::db::test::setup_db;

    #[tokio::test]
    async fn db_conn() {
        let db = setup_db().await;

        assert!(db.ping().await.is_ok());

        let count = db_entity::accounts::Entity::find()
            .count(&db)
            .await
            .unwrap();

        assert!(count > 0);
    }
}
