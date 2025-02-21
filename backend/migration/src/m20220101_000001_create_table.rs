use db_entity::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelBehavior, ActiveModelTrait};
use sea_orm_migration::{prelude::*, schema::*, sea_orm::Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(uuid(Accounts::Id).primary_key())
                    .col(string(Accounts::Username))
                    .col(string(Accounts::HashedPassword))
                    .col(string(Accounts::Email))
                    .col(string(Accounts::UsualAddress))
                    .col(boolean(Accounts::IsAdmin))
                    .col(timestamp_with_time_zone(Accounts::CreatedDateTime))
                    .col(timestamp_with_time_zone(Accounts::LastActiityDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        accounts::ActiveModel {
            username: Set("dvorak".to_owned()),
            hashed_password: Set(
                "342b7765af5a847aa47e2f92098d323f3264d5a9bfae142cf31dd4ecb32f87b6".to_owned(),
            ),
            is_admin: Set(true),
            ..accounts::ActiveModel::new()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Accounts {
    #[sea_orm(iden = "accounts")]
    Table,
    Id,
    Username,
    #[sea_orm(iden = "hashed_password")]
    HashedPassword,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "usual_address")]
    UsualAddress,
    #[sea_orm(iden = "is_admin")]
    IsAdmin,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
    #[sea_orm(iden = "last_actiity_date_time")]
    LastActiityDateTime,
}
