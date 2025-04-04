#[cfg(test)]
mod tests {
    use sea_orm::EntityTrait;

    use crate::{
        business::accounts::{AccountRepository, gen_account_repo::GenAccountRepo},
        db::{DbConnHelper, test::TestDB},
    };
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn get_administractor_by_username_success() {
        let db = TestDB::new().await;

        let acc_repo = GenAccountRepo::new(db);
        let admin = acc_repo
            .get_administractor_by_username("dvorak".to_owned())
            .await;

        assert!(admin.is_some());
        let admin = admin.unwrap();

        assert_eq!(admin.model.username, "dvorak".to_owned());
        assert!(admin.model.is_admin);
    }

    #[tokio::test]
    async fn get_administractor_by_id() {
        let db = TestDB::new().await;

        let model = db_entity::accounts::Entity::find()
            .one(&db.get_connection().await.unwrap())
            .await
            .unwrap();
        assert!(model.is_some());
        let id = model.unwrap().id;

        let acc_repo = GenAccountRepo::new(db);
        let admin = acc_repo.get_administractor_by_id(id).await;

        let admin = admin.unwrap();

        assert_eq!(admin.model.id, id);
        assert_eq!(admin.model.username, "dvorak".to_owned());
        assert!(admin.model.is_admin);
    }
}
