use crate::models::custom_error::RepositoryError;
use crate::models::users::UserLoginRequest;
use crate::repositories::users_repository::UserRepository;
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use crate::models::users::{CreateUserRequest, UserResponse};
use dotenv::dotenv;
use reqwest::{Client};
use serde_json::json;
use std::env::var;

// pub async fn signup(client: &Client, email: &str, password: &str) -> Result<(), Error> {
//     dotenv().ok();

//     let _ = client.post(format!("https://{}.supabase.co/auth/v1/signup", var("SUPABASE_PROJECT_ID").unwrap()))
//         .header("apikey", var("SUPABASE_ANON_KEY").unwrap())
//         .header("Content-Type", "application/json")
//         .json(&json!({
//             "email": email,
//             "password": password,
//         }))
//         .send()
//         .await?;

//     Ok(())
// }

pub async fn signin(client: Data<Client>, body: Json<UserLoginRequest>) -> HttpResponse {
    dotenv().ok();

    let res = client
        .post(format!(
            "https://{}.supabase.co/auth/v1/token?grant_type=password",
            var("SUPABASE_PROJECT_ID").unwrap()
        ))
        .header("apikey", var("SUPABASE_ANON_KEY").unwrap())
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": body.email,
            "password": body.password
        }))
        .send()
        .await;

    match res {
        Ok(res) => HttpResponse::Ok().body(res.text().await.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub async fn signup(
    repository: Data<UserRepository>,
    request: Json<CreateUserRequest>,
) -> HttpResponse {
    let client = Client::new();

    match repository.create_user(&request).await {
        Ok(user) => {
            match client
                .post(format!(
                    "https://{}.supabase.co/auth/v1/signup",
                    var("SUPABASE_PROJECT_ID").unwrap()
                ))
                .header("apikey", var("SUPABASE_ANON_KEY").unwrap())
                .header("Content-Type", "application/json")
                .json(&json!({
                    "email": &request.email,
                    "password": &request.password,
                }))
                .send()
                .await {
                    Ok(_) => {}
                    Err(err) => {
                        return HttpResponse::InternalServerError().json(format!("Signup error: {:?}", err))
                    }  
                }

            HttpResponse::Created().json(UserResponse {
                user_id: user.user_id,
                first_name: user.first_name,
                middle_name: user.middle_name,
                last_name: user.last_name,
                email: user.email,
            })
        }
        Err(RepositoryError::AlreadyExists) => HttpResponse::Conflict().json("User already exists"),
        Err(RepositoryError::DoesNotExist) => HttpResponse::NotFound().json("User not found"),
        Err(RepositoryError::Database(err)) => {
            HttpResponse::InternalServerError().json(format!("Database error: {}", err.to_string()))
        }
    }
}
