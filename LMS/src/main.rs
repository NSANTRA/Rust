use sqlx::PgPool;

pub mod database;
pub mod repositories;

pub mod models;

use models::{
    custom_error::RepositoryError,
    books::CreateBookRequest
};
use crate::models::users::CreateUserRequest;
use crate::repositories::{
    books_repository::BooksRepository,
    users_repository::UserRepository
};

#[tokio::main]
async fn main() {
    let database_client: PgPool = database::database_client::initialize().await.unwrap();

    let book_repository: BooksRepository = BooksRepository::new(database_client.clone());
    let user_repository: UserRepository = UserRepository::new(database_client.clone());

    println!("Repository initialized");
    
    let request: CreateBookRequest = CreateBookRequest{
        title: String::from("Percy Jackson"),
        description: None,
        authors: Vec::from([String::from("Rick Riordan"), String::from("J.K Rowling")]),
        genres: Vec::from([String::from("Mythology"), String::from("Greek")]),
        publisher_name: String::from("Penguin"),
        copies: 10
    };
    
    match book_repository.create_book(&request).await {
        Ok(book) => {
            println!("Created book: {:#?}", book);
        }
        Err(RepositoryError::AlreadyExists) => {
            println!("Row already exists");
        }
        Err(RepositoryError::Database(err)) => {
            println!("Database error: {:#?}", err);
        }
    }

    let request: CreateUserRequest = CreateUserRequest {
        first_name: String::from("Neelotpal"),
        middle_name: None,
        last_name: String::from("Santra"),
        email: String::from("ns@test.com")
    };

    match user_repository.create_user(&request).await {
        Ok(user) => {
            println!("Created user: {:#?}", user);
        }
        Err(RepositoryError::AlreadyExists) => {
            println!("Row already exists");
        }
        Err(RepositoryError::Database(err)) => {
            println!("Database error: {:#?}", err);
        }
    }
}