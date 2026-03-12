use sqlx::{PgPool};
use crate::models::books::{BookResponse, CreateBookRequest, SearchBookRequest};
use crate::models::custom_error::RepositoryError;
use crate::repositories::book_utilities::{create_book::create_book, list_books::list_books};

#[derive(Clone)]
pub struct BooksRepository {
    pub database_client: PgPool,
}

impl BooksRepository {
    pub fn new(database_client: PgPool) -> BooksRepository {
        BooksRepository { database_client }
    }

    pub async fn create_book(self: &BooksRepository, create_book_request: &CreateBookRequest) -> Result<BookResponse, RepositoryError> {
        create_book(self, create_book_request).await
    }
    
    pub async fn list_books(self: &BooksRepository, request: &SearchBookRequest) -> Result<Vec<BookResponse>, RepositoryError> {
        list_books(self, request).await
    }
}
