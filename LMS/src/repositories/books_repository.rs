use sqlx::{PgPool, Error};
use crate::models::books::{BookResponse, CreateBookRequest, SearchBookRequest};
use crate::models::custom_error::RepositoryError;
use crate::repositories::book_utilities::{create_book::create_book, list_books::list_books, delete_book::delete_book};

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
    
    pub async fn delete_book(self: &BooksRepository, request: &SearchBookRequest) -> Result<BookResponse, RepositoryError> {
        let books = list_books(self, request).await?;
        
        match delete_book(self, books).await {
            Ok(book) => {
                Ok(BookResponse {
                    book_id: book.book_id,
                    title: book.title,
                    description: book.description,
                    publisher_name: book.publisher_name,
                    authors: book.authors,
                    genres: book.genres,
                    available_copies: book.available_copies,
                })
            }
            Err(Error::RowNotFound) => {
                Err(RepositoryError::DoesNotExist)
            }
            Err(err) => {
                Err(RepositoryError::Database(err))
            }
        }
    }
}
