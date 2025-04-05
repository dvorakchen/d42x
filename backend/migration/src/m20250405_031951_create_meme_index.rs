use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const IDX_SHORT_ID_NAME: &str = "idx_meme_short_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !manager
            .has_column(Post::Table.to_string(), Post::ShortId.to_string())
            .await?
        {
            manager
                .alter_table(
                    sea_query::Table::alter()
                        .table(Post::Table)
                        .add_column_if_not_exists(
                            ColumnDef::new(Post::ShortId)
                                .string()
                                .not_null()
                                .default(String::new()),
                        )
                        .to_owned(),
                )
                .await
                .unwrap();

            let db = manager.get_connection();
            let txn = db.begin().await.unwrap();

            let list = db_entity::memes::Entity::find()
                .filter(db_entity::memes::Column::ShortId.eq(String::new()))
                .all(&txn)
                .await
                .unwrap();

            for model in list {
                let mut meme: db_entity::memes::ActiveModel = model.into();
                meme.short_id = Set(nanoid::nanoid!(10));
                meme.update(&txn).await.unwrap();
            }
            txn.commit().await.unwrap();

            manager
                .create_index(
                    sea_query::Index::create()
                        .name(IDX_SHORT_ID_NAME)
                        .table(Post::Table)
                        .col(Post::ShortId)
                        .to_owned(),
                )
                .await
                .unwrap();
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Post::Table)
                    .drop_column(Post::ShortId)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .drop_index(
                sea_query::Index::drop()
                    .if_exists()
                    .name(IDX_SHORT_ID_NAME)
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    #[sea_orm(iden = "memes")]
    Table,
    #[sea_orm(iden = "short_id")]
    ShortId,
}
