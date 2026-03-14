use actix_web::{web::{Json, Data, Query}, HttpResponse};
use crate::repositories::books_repository::BooksRepository;
use crate::models::books::{CreateBookRequest, BookResponse, SearchBookRequest};
use crate::models::custom_error::RepositoryError;

pub async fn create_book(repository: Data<BooksRepository>, request: Json<CreateBookRequest>) -> HttpResponse {
    match repository.create_book(&request).await {
        Ok(book) => {
            HttpResponse::Created().json(BookResponse {
                book_id: book.book_id,
                title: book.title,
                description: book.description,
                genres: book.genres,
                authors: book.authors,
                publisher_name: book.publisher_name,
                available_copies: book.available_copies,
            })
        }
        Err(RepositoryError::AlreadyExists) => {
            HttpResponse::Conflict().json("Book already exists")
        }
        Err(RepositoryError::DoesNotExist) => {
            HttpResponse::NotFound().json("Book not found")
        }        
        Err(RepositoryError::Database(err)) => {
            HttpResponse::InternalServerError().json(format!("Database error: {:?}", err.to_string()))
        }
    }
}

pub async fn list_books(repository: Data<BooksRepository>, request: Query<SearchBookRequest>) -> HttpResponse {
    match repository.list_books(&request.into_inner()).await {
        Ok(books) => {
            HttpResponse::Ok().json(books)
        }
        Err(RepositoryError::AlreadyExists) => {
            HttpResponse::Conflict().json("Book already exists")
        }
        Err(RepositoryError::DoesNotExist) => {
            HttpResponse::NotFound().json("Book not found")
        }
        Err(RepositoryError::Database(err)) => {
            HttpResponse::InternalServerError()
                .json(format!("Database error: {}", err))
        }
    }
}

pub async fn delete_book(repository: Data<BooksRepository>, request: Query<SearchBookRequest>) -> HttpResponse {
    match repository.delete_book(&request.into_inner()).await {
        Ok(book) => {
            HttpResponse::Ok().json(book)
        }
        Err(RepositoryError::AlreadyExists) => {
            HttpResponse::Conflict().json("Book already exists")
        }
        Err(RepositoryError::DoesNotExist) => {
            HttpResponse::NotFound().json("Book not found")
        }
        Err(RepositoryError::Database(err)) => {
            HttpResponse::InternalServerError()
                .json(format!("Database error: {}", err))
        }
    }
}