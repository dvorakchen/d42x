use db_entity::suggests;
use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(uuid(Post::Id).primary_key())
                    .col(uuid(Post::MemeId))
                    .col(string(Post::Before))
                    .col(string(Post::After))
                    .col(string(Post::Status))
                    .col(uuid(Post::AccountId))
                    .col(uuid(Post::OperatorId))
                    .col(timestamp_with_time_zone(Post::CreatedDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        let meme = db_entity::memes::Entity::find()
            .one(db)
            .await
            .unwrap()
            .unwrap();

        let account = db_entity::accounts::Entity::find()
            .one(db)
            .await
            .unwrap()
            .unwrap();

        suggests::ActiveModel {
            meme_id: Set(meme.id),
            before: Set(String::new()),
            after: Set(String::from(";meme;")),
            status: Set(suggests::Status::Wait),
            account_id: Set(account.id),
            operator_id: Set(account.id),
            ..suggests::ActiveModel::new()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    #[sea_orm(iden = "suggests")]
    Table,
    Id,
    #[sea_orm(iden = "meme_id")]
    MemeId,
    #[sea_orm(iden = "before")]
    Before,
    #[sea_orm(iden = "after")]
    After,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "account_id")]
    AccountId,
    #[sea_orm(iden = "operator_id")]
    OperatorId,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
}
