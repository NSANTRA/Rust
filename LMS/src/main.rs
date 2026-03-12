use sqlx::PgPool;

pub mod database;
pub mod repositories;

pub mod handlers;
pub mod models;

use crate::repositories::{books_repository::BooksRepository, users_repository::UserRepository};
use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    web::{Data, get, post},
};
use handlers::{
    books_handler::{create_book, list_books},
    user_handler::signup,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let database_client: PgPool = database::database_client::initialize().await.unwrap();

    let book_repository: BooksRepository = BooksRepository::new(database_client.clone());
    let user_repository: UserRepository = UserRepository::new(database_client.clone());

    println!("Repository initialized");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(Data::new(user_repository.clone()))
            .app_data(Data::new(book_repository.clone()))
            .route("/create-user", post().to(signup))
            .route("/create-book", post().to(create_book))
            .route("/list-books", get().to(list_books))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
