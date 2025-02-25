//! Administrator
//!
//! ## About hashed_password
//! the hashed_password in database table accounts column hashed_password is getting bloke3 hashed
//! the client passing the password that is the hashed_password getting blake3 and bcrypt
//! so verify the password need bcrypt the password that getting blake3 from client and compares with hashed_password from database
//! use function verify_password()

use db_entity::accounts;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use sea_orm::{ColumnTrait, DbErr, Set};
use sea_orm::{EntityTrait, QueryFilter};
use thiserror::Error;

use crate::db::DbHelper;

#[derive(Debug)]
pub struct Administrator {
    db: DatabaseConnection,
    pub model: accounts::Model,
}

impl Administrator {
    pub async fn new(
        username: String,
        // hashed_password: String,
    ) -> Result<Self, AdministratorError> {
        let db = DbHelper::get_connection().await?;

        let admin = {
            accounts::Entity::find()
                .filter(accounts::Column::Username.eq(username.clone()))
                // .filter(accounts::Column::HashedPassword.eq(hashed_password))
                .filter(accounts::Column::IsAdmin.eq(true))
                .one(&db)
                .await
                .map_err(AdministratorError::from)?
        };

        if let Some(model) = admin {
            Ok(Self { db, model })
        } else {
            Err(AdministratorError::NotFound(username))
        }
    }

    pub async fn new_from_id(id: Uuid) -> Result<Self, AdministratorError> {
        let db = DbHelper::get_connection().await?;

        todo!()
    }

    /// verify password
    ///
    /// # Arguments
    /// hashed_password: the password getting blake3 and bcrypt from client
    pub fn verify_password(&self, hashed_password: &str) -> bool {
        use bcrypt::verify;

        verify(&self.model.hashed_password, hashed_password).unwrap()
    }

    pub async fn log_in_activity(&mut self, ip_addr: &str) -> Result<(), AdministratorError> {
        let now = chrono::Utc::now().into();

        let mut model: accounts::ActiveModel = self.model.clone().into();
        model.usual_address = Set(ip_addr.to_string());
        model.last_actiity_date_time = Set(now);
        model.update(&self.db).await?;

        self.model.usual_address = ip_addr.to_string();
        self.model.last_actiity_date_time = now;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum AdministratorError {
    #[error("Database error ocurrs")]
    DatabaseErr(#[from] DbErr),
    #[error("Account not found: {0}")]
    NotFound(String),
}
