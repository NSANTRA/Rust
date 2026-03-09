use sqlx::PgPool;
use crate::models::books::BookResponse;

pub mod database;
pub mod repositories;

pub mod models;

#[tokio::main]
async fn main() {
    let database_client: PgPool = database::database_client::initialize().await.unwrap();

    let book_repository = repositories::books_repository::BooksRepository::new(database_client.clone());

    println!("Repository initialized");
    
    let request: models::books::CreateBookRequest = models::books::CreateBookRequest{
        title: String::from("Percy Jackson"),
        description: None,
        authors: Vec::from([String::from("Rick Riordan"), String::from("J.K Rowling")]),
        genres: Vec::from([String::from("Mythology"), String::from("Greek")]),
        publisher_name: String::from("Penguin"),
        copies: 10
    };
    
    let res: BookResponse = book_repository.create_book(&request).await.unwrap();
    
    println!("Created book: {:?}", res.clone());
}