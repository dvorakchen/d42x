use sea_orm::prelude::Uuid;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CategoryItem {
    pub id: Uuid,
    pub name: String,
}
