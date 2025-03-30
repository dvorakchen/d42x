//! Administrator
//!
//! ## About hashed_password
//! the hashed_password in database table accounts column hashed_password is getting bloke3 hashed
//! the client passing the password that is the hashed_password getting blake3 and bcrypt
//! so verify the password need bcrypt the password that getting blake3 from client and compares with hashed_password from database
//! use function verify_password()

use bcrypt::verify;
use db_entity::accounts;
use sea_orm::ActiveModelTrait;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, DbErr, Set};
use sea_orm::{EntityTrait, QueryFilter};
use thiserror::Error;

use crate::db::DbConnHelper;

pub struct Administrator {
    db: Box<dyn DbConnHelper + 'static + Sync + Send>,
    pub model: accounts::Model,
}

impl Administrator {
    pub async fn new(
        username: String,
        db: impl DbConnHelper + 'static + Sync + Send,
    ) -> AdminResult<Self> {
        let db_conn = db.get_connection().await?;

        let admin = {
            accounts::Entity::find()
                .filter(accounts::Column::Username.eq(username.clone()))
                // .filter(accounts::Column::HashedPassword.eq(hashed_password))
                .filter(accounts::Column::IsAdmin.eq(true))
                .one(&db_conn)
                .await
                .map_err(AdministratorError::from)?
        };

        if let Some(model) = admin {
            Ok(Self {
                db: Box::new(db),
                model,
            })
        } else {
            Err(AdministratorError::NotFound(username))
        }
    }

    pub async fn new_from_id(
        id: Uuid,
        db: impl DbConnHelper + 'static + Sync + Send,
    ) -> AdminResult<Self> {
        let db_conn = db.get_connection().await?;

        let admin = {
            accounts::Entity::find_by_id(id)
                .filter(accounts::Column::IsAdmin.eq(true))
                .one(&db_conn)
                .await
                .map_err(AdministratorError::from)?
        };

        if let Some(model) = admin {
            Ok(Self {
                db: Box::new(db),
                model,
            })
        } else {
            Err(AdministratorError::NotFound(id.to_string()))
        }
    }

    pub async fn change_password(&mut self, cur_pwd: &str, new_pwd: &str) -> AdminResult<()> {
        if !self.verify_password(cur_pwd) {
            return Err(AdministratorError::IncorrectPassword);
        }

        let now = chrono::Utc::now().into();

        let mut model: accounts::ActiveModel = self.model.clone().into();
        model.hashed_password = Set(new_pwd.to_string());
        model.last_actiity_date_time = Set(now);
        model.update(&self.db.get_connection().await?).await?;

        self.model.hashed_password = new_pwd.to_string();
        self.model.last_actiity_date_time = now;

        Ok(())
    }

    /// verify password
    ///
    /// # Arguments
    /// hashed_password: the password getting blake3 and bcrypt from client
    pub fn verify_password(&self, hashed_password: &str) -> bool {
        verify(&self.model.hashed_password, hashed_password).unwrap()
    }

    pub async fn log_in_activity(&mut self, ip_addr: &str) -> AdminResult<()> {
        let now = chrono::Utc::now().into();

        let mut model: accounts::ActiveModel = self.model.clone().into();
        model.usual_address = Set(ip_addr.to_string());
        model.last_actiity_date_time = Set(now);
        model.update(&self.db.get_connection().await?).await?;

        self.model.usual_address = ip_addr.to_string();
        self.model.last_actiity_date_time = now;
        Ok(())
    }
}

pub type AdminResult<T> = Result<T, AdministratorError>;

#[derive(Error, Debug)]
pub enum AdministratorError {
    #[error("Database error ocurrs")]
    DatabaseErr(#[from] DbErr),
    #[error("Account not found: {0}")]
    NotFound(String),
    #[error("Incorrect password")]
    IncorrectPassword,
}
