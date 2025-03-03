use db_entity::targets;
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
                    .table(Targets::Table)
                    .if_not_exists()
                    .col(uuid(Targets::Id).primary_key())
                    .col(string(Targets::Name))
                    .col(timestamp_with_time_zone(Targets::CreatedDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        targets::ActiveModel {
            name: Set(String::from("meme")),
            ..targets::ActiveModel::new()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Targets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Targets {
    #[sea_orm(iden = "targets")]
    Table,
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
}
