use sqlx::{PgPool};
use crate::models::users::{UserResponse, CreateUserRequest};
use crate::models::custom_error::RepositoryError;
use crate::repositories::user_utilities::create_user::create_user;

#[derive(Clone)]
pub struct UserRepository {
    pub database_client: PgPool,
}

impl UserRepository {
    pub fn new(database_client: PgPool) -> UserRepository {
        UserRepository { database_client }
    }
    
    pub async fn create_user(self: &UserRepository, create_user_request: &CreateUserRequest) -> Result<UserResponse, RepositoryError> {
        create_user(self, create_user_request).await
    }
}