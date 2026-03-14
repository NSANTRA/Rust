use sqlx::{query, Error};
use crate::models::books::BookResponse;
use crate::repositories::books_repository::BooksRepository;

pub async fn delete_book(
    book_repo: &BooksRepository,
    books: Vec<BookResponse>,
) -> Result<BookResponse, sqlx::Error> {
    if !books.is_empty() {
        let book_id = &books[0].book_id;

        query("DELETE FROM book_authors WHERE book_id = $1")
            .bind(book_id)
            .execute(&book_repo.database_client)
            .await?;

        query("DELETE FROM book_genres WHERE book_id = $1")
            .bind(book_id)
            .execute(&book_repo.database_client)
            .await?;

        query("DELETE FROM book_inventory WHERE book_id = $1")
            .bind(book_id)
            .execute(&book_repo.database_client)
            .await?;

        query("DELETE FROM books WHERE book_id = $1")
            .bind(book_id)
            .execute(&book_repo.database_client)
            .await?;

        Ok(BookResponse {
            book_id: books[0].book_id.clone(),
            title: books[0].title.clone(),
            description: books[0].description.clone(),
            publisher_name: books[0].publisher_name.clone(),
            authors: books[0].authors.clone(),
            genres: books[0].genres.clone(),
            available_copies: books[0].available_copies,
        })
    } else {
        Err(Error::RowNotFound)
    }
}
