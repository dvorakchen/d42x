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
        db::test::TestDB,
    };

    #[tokio::test]
    async fn get_paginated_memes_success() {
        const EXPECTED_PAGE: u64 = 1;
        const EXPECTED_TOTAL: u64 = 1;
        const EXPECTED_LIST_LENGTH: usize = 2;
        const EXPECTED_MEME_URLS_LENGTH: usize = 4;

        let db = TestDB::new().await;

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

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

        let db = TestDB::new().await;
        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

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

        let db = TestDB::new().await;
        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

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

        let db = TestDB::new().await;
        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

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

        let db = TestDB::new().await;
        let db_conn = db.get_connection().await.unwrap();
        let id = {
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

        let meme_entity = repo.get_meme(id).await.unwrap().unwrap();

        let res = meme_entity.increase_like().await;
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

        let db = TestDB::new().await;
        let db_conn = db.get_connection().await.unwrap();
        let id = {
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db);

        let meme_entity = repo.get_meme(id).await.unwrap().unwrap();

        // action
        meme_entity.increase_like().await.unwrap();
        meme_entity.increase_like().await.unwrap();

        let model = db_entity::memes::Entity::find_by_id(id)
            .one(&db_conn)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(model.likes, EXPECTED_LIKES);
    }

    #[tokio::test]
    async fn get_interactions_has_likes_success() {
        let db = TestDB::new().await;
        let id = {
            let db_conn = db.get_connection().await.unwrap();
            let model = db_entity::memes::Entity::find()
                .one(&db_conn)
                .await
                .unwrap()
                .unwrap();
            model.id
        };

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db.clone());

        let res = repo.get_interactions(vec![id]).await;
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(0).unwrap().likes, 0);

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db.clone());
        let meme_entity = repo.get_meme(id).await.unwrap().unwrap();
        meme_entity.increase_like().await.unwrap();

        let repo: GenMemeRepo<MockCache<_, _>, TestDB> = GenMemeRepo::new(db.clone());
        let res = repo.get_interactions(vec![id]).await;
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(0).unwrap().likes, 1);
    }
}
