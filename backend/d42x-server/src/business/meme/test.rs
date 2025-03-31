#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use sea_orm::EntityTrait;

    use crate::db::DbConnHelper;
    use crate::{
        business::{
            cache::MockCache,
            meme::{GetFilter, MemeRepository, gen_meme_repo::GenMemeRepo},
        },
        db::{MockDbConnHelper, test::setup_db},
    };

    async fn mock_db_helper() -> MockDbConnHelper {
        let db_conn = setup_db().await;
        let mut db = MockDbConnHelper::new();
        db.expect_get_connection()
            .returning(move || Ok(db_conn.clone()));
        db
    }

    #[tokio::test]
    async fn get_paginated_memes_success() {
        const EXPECTED_PAGE: u64 = 1;
        const EXPECTED_TOTAL: u64 = 1;
        const EXPECTED_LIST_LENGTH: usize = 2;
        const EXPECTED_MEME_URLS_LENGTH: usize = 4;

        let db = mock_db_helper().await;

        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let list = repo.get_paginated_memes(1, None).await;
        assert_eq!(list.page, EXPECTED_PAGE);
        assert_eq!(list.total, EXPECTED_TOTAL);
        assert_eq!(list.list.len(), EXPECTED_LIST_LENGTH);
        assert_eq!(
            list.list.into_iter().flat_map(|item| item.list).count(),
            EXPECTED_MEME_URLS_LENGTH
        );
    }

    #[tokio::test]
    async fn get_all_memes_success() {
        const EXPECTED_PAGE: u64 = 1;
        const EXPECTED_TOTAL: u64 = 1;
        const EXPECTED_SIZE: u64 = 10;
        const EXPECTED_LIST_LENGTH: usize = 2;
        const EXPECTED_STATUS: Option<db_entity::memes::Status> = None;

        let db = mock_db_helper().await;
        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let list = repo
            .get_paginated_all_memes(GetFilter {
                page: EXPECTED_PAGE,
                size: EXPECTED_SIZE,
                status: EXPECTED_STATUS,
            })
            .await;

        assert_eq!(list.page, EXPECTED_PAGE);
        assert_eq!(list.total, EXPECTED_TOTAL);
        assert_eq!(list.size, EXPECTED_SIZE);
        assert_eq!(list.list.len(), EXPECTED_LIST_LENGTH);
    }

    #[tokio::test]
    async fn get_memes_published_success() {
        const EXPECTED_PAGE: u64 = 1;
        const EXPECTED_TOTAL: u64 = 1;
        const EXPECTED_SIZE: u64 = 10;
        const EXPECTED_LIST_LENGTH: usize = 2;
        const EXPECTED_STATUS: Option<db_entity::memes::Status> =
            Some(db_entity::memes::Status::Published);

        let db = mock_db_helper().await;
        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let list = repo
            .get_paginated_all_memes(GetFilter {
                page: EXPECTED_PAGE,
                size: EXPECTED_SIZE,
                status: EXPECTED_STATUS,
            })
            .await;

        assert_eq!(list.page, EXPECTED_PAGE);
        assert_eq!(list.total, EXPECTED_TOTAL);
        assert_eq!(list.size, EXPECTED_SIZE);
        assert_eq!(list.list.len(), EXPECTED_LIST_LENGTH);
    }

    #[tokio::test]
    async fn get_memes_uncensored_success() {
        const EXPECTED_PAGE: u64 = 1;
        const EXPECTED_TOTAL: u64 = 0;
        const EXPECTED_SIZE: u64 = 10;
        const EXPECTED_LIST_LENGTH: usize = 0;
        const EXPECTED_STATUS: Option<db_entity::memes::Status> =
            Some(db_entity::memes::Status::Uncensored);

        let db = mock_db_helper().await;
        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let list = repo
            .get_paginated_all_memes(GetFilter {
                page: EXPECTED_PAGE,
                size: EXPECTED_SIZE,
                status: EXPECTED_STATUS,
            })
            .await;

        assert_eq!(list.page, EXPECTED_PAGE);
        assert_eq!(list.total, EXPECTED_TOTAL);
        assert_eq!(list.size, EXPECTED_SIZE);
        assert_eq!(list.list.len(), EXPECTED_LIST_LENGTH);
    }

    #[tokio::test]
    async fn increase_like_success() {
        const EXPECTED_LIKES: i32 = 1;

        let db = mock_db_helper().await;
        let db_conn = db.get_connection().await.unwrap();
        let id = {
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let res = repo.like_increase(id).await;
        assert!(res.is_ok());

        let model = db_entity::memes::Entity::find_by_id(id)
            .one(&db_conn)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(model.likes, EXPECTED_LIKES);
    }

    #[tokio::test]
    async fn increase_like_mutiple_success() {
        const EXPECTED_LIKES: i32 = 2;

        let db = mock_db_helper().await;
        let db_conn = db.get_connection().await.unwrap();
        let id = {
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        // action
        repo.like_increase(id).await.unwrap();
        repo.like_increase(id).await.unwrap();

        let model = db_entity::memes::Entity::find_by_id(id)
            .one(&db_conn)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(model.likes, EXPECTED_LIKES);
    }

    #[tokio::test]
    async fn get_interactions_has_likes_success() {
        let db = mock_db_helper().await;
        let db_conn = db.get_connection().await.unwrap();
        let id = {
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, MockDbConnHelper> = GenMemeRepo::new(db);

        let res = repo.get_interactions(vec![id]).await;
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(0).unwrap().likes, 0);

        repo.like_increase(id).await.unwrap();

        let res = repo.get_interactions(vec![id]).await;
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(0).unwrap().likes, 1);
    }
}
