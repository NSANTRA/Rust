use uuid::Uuid;
use sqlx::{Row, postgres::PgRow, query, Transaction, Postgres};
use crate::models::custom_error::RepositoryError;
use crate::models::users::{CreateUserRequest, UserResponse};
use crate::repositories::users_repository::UserRepository;

pub async fn create_user(user_repo: &UserRepository, request: &CreateUserRequest) -> Result<UserResponse, RepositoryError> {
    let mut transaction: Transaction<Postgres> = user_repo.database_client.begin().await?;

    let middle_name: &str = match &request.middle_name {
        Some(m) => m,
        None => "",
    };

    let row: Option<PgRow> = query("INSERT INTO users (first_name, middle_name, last_name, email, password) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (email) DO NOTHING RETURNING user_id")
        .bind(&request.first_name)
        .bind(&middle_name)
        .bind(&request.last_name)
        .bind(&request.email)
        .bind(&request.password)
        .fetch_optional(&mut *transaction)
        .await?;

    let user_id: Uuid = match row {
        Some(row) => row.get("user_id"),
        None => return Err(RepositoryError::AlreadyExists)
    };

    transaction.commit().await?;

    Ok(UserResponse {
        user_id,
        first_name: request.first_name.clone(),
        middle_name: middle_name.to_string(),
        last_name: request.last_name.clone(),
        email: request.email.clone(),
    })
}