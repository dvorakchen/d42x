use db_entity::{meme_urls, memes};
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
                    .col(string(Memes::Nickname))
                    .col(string(Memes::Message))
                    .col(string(Memes::Email))
                    .col(string(Memes::IdAddr))
                    .col(integer(Memes::Likes))
                    .col(integer(Memes::Unlikes))
                    .col(string(Memes::Categories))
                    .col(string(Memes::Status))
                    .col(uuid(Memes::UserId))
                    .col(timestamp_with_time_zone(Memes::ShowDateTime))
                    .col(timestamp_with_time_zone(Memes::CreatedDateTime))
                    .col(timestamp_with_time_zone(Memes::LastActiityDateTime))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MemeUrls::Table)
                    .if_not_exists()
                    .col(uuid(MemeUrls::Id).primary_key())
                    .col(uuid(MemeUrls::MemeId))
                    .col(string(MemeUrls::Url))
                    .col(string(MemeUrls::Cover))
                    .col(string(MemeUrls::Source))
                    .col(string(MemeUrls::Format))
                    .col(string(MemeUrls::Hash))
                    .col(string(MemeUrls::Bed))
                    .col(string(MemeUrls::BedId))
                    .col(integer(MemeUrls::Sort))
                    .col(timestamp_with_time_zone(MemeUrls::CreatedDateTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        let model = memes::ActiveModel {
            status: Set(memes::Status::Published),
            nickname: Set(String::from("dvorak")),
            ..memes::ActiveModel::new()
        }
        .insert(db)
        .await?;

        meme_urls::ActiveModel {
            meme_id: Set(model.id),
            url: Set(String::from(
                "https://pic1.imgdb.cn/item/67c5b905d0e0a243d40ae56d.png",
            )),
            cover: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::from("JPG")),
            sort: Set(0),
            ..meme_urls::ActiveModel::new()
        }
        .insert(db)
        .await?;

        meme_urls::ActiveModel {
            meme_id: Set(model.id),
            url: Set(String::from(
                "https://pic1.imgdb.cn/item/67c5b228d0e0a243d40ae1ae.jpg",
            )),
            cover: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::from("JPG")),
            sort: Set(1),
            ..meme_urls::ActiveModel::new()
        }
        .insert(db)
        .await?;

        let model = memes::ActiveModel {
            status: Set(memes::Status::Published),
            nickname: Set(String::from("dvorak")),
            ..memes::ActiveModel::new()
        }
        .insert(db)
        .await?;

        meme_urls::ActiveModel {
            meme_id: Set(model.id),
            url: Set(String::from(
                "https://pic1.imgdb.cn/item/67c5b83cd0e0a243d40ae473.png",
            )),
            cover: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::from("PNG")),
            sort: Set(0),
            ..meme_urls::ActiveModel::new()
        }
        .insert(db)
        .await?;

        meme_urls::ActiveModel {
            meme_id: Set(model.id),
            url: Set(String::from(
                "https://pic1.imgdb.cn/item/67c573ddd0e0a243d40abd09.webp",
            )),
            cover: Set(String::new()),
            source: Set(String::new()),
            format: Set(String::from("WEBP")),
            sort: Set(1),
            ..meme_urls::ActiveModel::new()
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
    #[sea_orm(iden = "nickname")]
    Nickname,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "message")]
    Message,
    #[sea_orm(iden = "id_addr")]
    IdAddr,
    #[sea_orm(iden = "likes")]
    Likes,
    #[sea_orm(iden = "unlikes")]
    Unlikes,
    #[sea_orm(iden = "categories")]
    Categories,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "show_date_time")]
    ShowDateTime,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
    #[sea_orm(iden = "last_actiity_date_time")]
    LastActiityDateTime,
}

#[derive(DeriveIden)]
enum MemeUrls {
    #[sea_orm(iden = "meme_urls")]
    Table,
    Id,
    #[sea_orm(iden = "meme_id")]
    MemeId,
    #[sea_orm(iden = "url")]
    Url,
    #[sea_orm(iden = "cover")]
    Cover,
    #[sea_orm(iden = "source")]
    Source,
    #[sea_orm(iden = "format")]
    Format,
    #[sea_orm(iden = "hash")]
    Hash,
    #[sea_orm(iden = "bed")]
    Bed,
    #[sea_orm(iden = "bed_id")]
    BedId,
    #[sea_orm(iden = "sort")]
    Sort,
    #[sea_orm(iden = "created_date_time")]
    CreatedDateTime,
}
