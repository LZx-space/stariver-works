use std::fmt::Error;

use sea_orm::DatabaseConnection;
use sea_orm::prelude::async_trait::async_trait;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::security::authentication::util::hash_password;

pub struct UserRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, user: User) -> Result<bool, Error> {
        todo!()
    }

    async fn update(&self, user: User) -> Result<bool, Error> {
        todo!()
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        Ok(match hash_password("password", "ABCDEFGH") {
            Ok(hash_string) => Some(User {
                username: username.to_string(),
                password: hash_string.to_string(),
                phone: "".to_string(),
                email: "".to_string(),
            }),
            Err(_) => None,
        })
    }
}