use migration::async_trait;
use sea_orm::prelude::Uuid;

#[cfg(test)]
mod test;

pub mod admin;
pub mod gen_account_repo;

use admin::Administrator;
#[async_trait::async_trait]
pub trait AccountRepository {
    async fn get_administractor_by_id(&self, _id: Uuid) -> Option<Administrator> {
        unimplemented!()
    }

    async fn get_administractor_by_username(&self, _username: String) -> Option<Administrator> {
        unimplemented!()
    }
}

pub struct PanicAccountRepo;

#[async_trait::async_trait]
impl AccountRepository for PanicAccountRepo {}
