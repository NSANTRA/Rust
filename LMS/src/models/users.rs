use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
// User Signup
pub struct CreateUserRequest {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
// User Signup
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
// Admin User Search
pub struct SearchUserRequest {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
// Admin User Search Response
pub struct UserResponse {
    pub user_id: Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String
}