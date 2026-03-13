use reqwest::{Client, Error};
use dotenv::dotenv;
use std::env::var;
use serde_json::json;
use actix_web::{HttpResponse, web::{Data, Json}};
use crate::models::users::UserLoginRequest;

pub async fn signup(client: &Client, email: &str, password: &str) -> Result<(), Error> {
    dotenv().ok();
        
    let _ = client.post(format!("https://{}.supabase.co/auth/v1/signup", var("SUPABASE_PROJECT_ID").unwrap()))
        .header("apikey", var("SUPABASE_ANON_KEY").unwrap())
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await?;
    
    Ok(())
}

pub async fn signin(client: Data<Client>, body: Json<UserLoginRequest>) -> HttpResponse {
    dotenv().ok();
    
    let res = client
            .post(format!("https://{}.supabase.co/auth/v1/token?grant_type=password", var("SUPABASE_PROJECT_ID").unwrap()))
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