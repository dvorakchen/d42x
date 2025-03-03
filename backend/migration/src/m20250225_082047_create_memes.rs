use db_entity::memes;
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
                    .table(Memes::Table)
                    .if_not_exists()
                    .col(uuid(Memes::Id).primary_key())
                    .col(string(Memes::Url))
                    .col(string(Memes::Source))
                    .col(string(Memes::Format))
                    .col(string(Memes::Hash))
                    .col(string(Memes::Nickname))
                    .col(string(Memes::Email))
                    .col(string(Memes::IdAddr))
                    .col(integer(Memes::Likes))
                    .col(integer(Memes::Unlikes))
                    .col(string(Memes::Targets))
                    .col(string(Memes::Status))
                    .col(string(Memes::Bed))
                    .col(string(Memes::BedId))
                    .col(timestamp_with_time_zone(Memes::ShowDateTime))
                    .col(timestamp_with_time_zone(Memes::CreatedDateTime))
                    .col(timestamp_with_time_zone(Memes::LastActiityDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        memes::ActiveModel {
            url: Set(String::from(
                "https://pic1.imgdb.cn/item/67c573ddd0e0a243d40abd09.webp",
            )),
            status: Set(memes::Status::Published),
            ..memes::ActiveModel::new()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Memes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Memes {
    #[sea_orm(iden = "memes")]
    Table,
    Id,
    #[sea_orm(iden = "url")]
    Url,
    #[sea_orm(iden = "source")]
    Source,
    #[sea_orm(iden = "format")]
    Format,
    #[sea_orm(iden = "hash")]
    Hash,
    #[sea_orm(iden = "nickname")]
    Nickname,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "id_addr")]
    IdAddr,
    #[sea_orm(iden = "likes")]
    Likes,
    #[sea_orm(iden = "unlikes")]
    Unlikes,
    #[sea_orm(iden = "targets")]
    Targets,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "bed")]
    Bed,
    #[sea_orm(iden = "bed_id")]
    BedId,
    #[sea_orm(iden = "show_date_time")]
    ShowDateTime,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
    #[sea_orm(iden = "last_actiity_date_time")]
    LastActiityDateTime,
}
