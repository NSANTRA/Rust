use actix_web::{web::{Json, Data}, HttpResponse};
use crate::repositories::users_repository::UserRepository;
use crate::models::users::{CreateUserRequest, UserResponse};
use crate::models::custom_error::RepositoryError;
use reqwest::Client;
use dotenv::dotenv;
use std::env::var;
use serde_json::json;

pub async fn signup(repository: Data<UserRepository>, request: Json<CreateUserRequest>) -> HttpResponse {
    dotenv().ok();
    
    match repository.create_user(&request).await {
        Ok(user) => {
            let client = Client::new();
            
            let _ = client.post(format!("https://{}.supabase.co/auth/v1/signup", var("SUPABASE_PROJECT_ID").unwrap()))
                .header("apikey", var("SUPABASE_ANON_KEY").unwrap())
                .header("Content-Type", "application/json")
                .json(&json!({
                    "email": request.email,
                    "password": request.password,
                }))
                .send()
                .await;
            
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