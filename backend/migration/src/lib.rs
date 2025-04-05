pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250225_082047_create_memes;
mod m20250303_085702_create_categories;
mod m20250324_110708_create_suggests;
mod m20250405_031951_create_meme_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250225_082047_create_memes::Migration),
            Box::new(m20250303_085702_create_categories::Migration),
            Box::new(m20250324_110708_create_suggests::Migration),
            Box::new(m20250405_031951_create_meme_index::Migration),
        ]
    }
}
