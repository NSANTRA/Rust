use actix_web::{web::{Json, Data}, HttpResponse};
use crate::repositories::users_repository::UserRepository;
use crate::models::users::{CreateUserRequest, UserResponse};
use crate::models::custom_error::RepositoryError;
use reqwest::{Client};
use crate::handlers::auth_handler::{signup as auth_signup};

pub async fn signup(repository: Data<UserRepository>, request: Json<CreateUserRequest>) -> HttpResponse {  
    let client = Client::new();
    
    match repository.create_user(&request).await {
        Ok(user) => {
            auth_signup(&client, &user.email, &request.password).await.ok();
            
            HttpResponse::Created().json(UserResponse {
                user_id: user.user_id,
                first_name: user.first_name,
                middle_name: user.middle_name,
                last_name: user.last_name,
                email: user.email
            })
        }
        Err(RepositoryError::AlreadyExists) => {
            HttpResponse::Conflict().json("User already exists")
        }
        Err(RepositoryError::Database(err)) => {
            HttpResponse::InternalServerError().json(format!("Database error: {}", err.to_string()))
        }
    }
}