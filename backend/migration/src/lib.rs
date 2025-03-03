pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250225_082047_create_memes;
mod m20250303_085702_create_categories;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250225_082047_create_memes::Migration),
            Box::new(m20250303_085702_create_categories::Migration),
        ]
    }
}
