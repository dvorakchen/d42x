use db_entity::categories;
use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{ActiveModelBehavior, ActiveModelTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(uuid(Categories::Id).primary_key())
                    .col(uuid(Categories::Parent))
                    .col(string(Categories::Name))
                    .col(timestamp_with_time_zone(Categories::CreatedDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        categories::ActiveModel {
            name: Set(String::from("meme")),
            ..categories::ActiveModel::new()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    #[sea_orm(iden = "categories")]
    Table,
    Id,
    #[sea_orm(iden = "parent")]
    Parent,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
}
