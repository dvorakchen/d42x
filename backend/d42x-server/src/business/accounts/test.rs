#[cfg(test)]
mod tests {
    use migration::async_trait;
    use mockall::mock;
    use sea_orm::{DatabaseConnection, EntityTrait};

    use crate::{
        business::accounts::{AccountRepository, gen_account_repo::GenAccountRepo},
        db::{DbConnHelper, test::setup_db},
    };
    use pretty_assertions::assert_eq;

    mock! {
        MyDbConn {}
        impl Clone for MyDbConn {
            fn clone(&self) -> Self;
        }

        #[async_trait::async_trait]
        impl DbConnHelper for MyDbConn {
            async fn get_connection(&self) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr>;
        }
    }

    fn setup_mock(db: DatabaseConnection) -> MockMyDbConn {
        let mut mock = MockMyDbConn::new();
        let mut clone_mock = MockMyDbConn::new();
        clone_mock
            .expect_get_connection()
            .return_once(move || Ok(db));
        mock.expect_clone().return_once(move || clone_mock);

        mock
    }

    #[tokio::test]
    async fn get_administractor_by_username_success() {
        let db = setup_db().await;

        let mock = setup_mock(db);

        let acc_repo = GenAccountRepo::new(mock);
        let admin = acc_repo
            .get_administractor_by_username("dvorak".to_owned())
            .await;

        assert!(admin.is_some());
        let admin = admin.unwrap();

        assert_eq!(admin.model.username, "dvorak".to_owned());
    }

    #[tokio::test]
    async fn get_administractor_by_id() {
        let db = setup_db().await;

        let model = db_entity::accounts::Entity::find().one(&db).await.unwrap();
        assert!(model.is_some());
        let id = model.unwrap().id;

        let mock = setup_mock(db);

        let acc_repo = GenAccountRepo::new(mock);
        let admin = acc_repo.get_administractor_by_id(id).await;

        let admin = admin.unwrap();

        assert_eq!(admin.model.id, id);
        assert_eq!(admin.model.username, "dvorak".to_owned());
    }
}
