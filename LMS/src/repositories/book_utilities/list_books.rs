use crate::models::books::{BookResponse, SearchBookRequest};
use crate::models::custom_error::RepositoryError;
use crate::repositories::books_repository::BooksRepository;

pub async fn list_books(
    book_repo: &BooksRepository,
    request: &SearchBookRequest,
) -> Result<Vec<BookResponse>, RepositoryError> {

    if request.title.is_none() && request.publisher_name.is_none() &&
       request.author_name.is_none() && request.genre.is_none() {
        return Ok(Vec::new());
    }

    if let Some(title) = &request.title {
        let rows = sqlx::query_as::<_, BookResponse>(
            r#"
            SELECT 
                b.book_id,
                b.title,
                b.description,
                p.name AS publisher_name,
                ARRAY(
                    SELECT a.first_name || ' ' || a.last_name
                    FROM authors a
                    JOIN book_authors ba ON ba.author_id = a.author_id
                    WHERE ba.book_id = b.book_id
                ) AS authors,
                ARRAY(
                    SELECT g.name
                    FROM genres g
                    JOIN book_genres bg ON bg.genre_id = g.genre_id
                    WHERE bg.book_id = b.book_id
                ) AS genres,
                (
                    SELECT COUNT(*)::INT 
                    FROM book_inventory bi 
                    WHERE bi.book_id = b.book_id AND bi.status = 'available'
                ) AS available_copies
            FROM books b
            JOIN publishers p ON p.publisher_id = b.publisher_id
            WHERE b.title ILIKE '%' || $1 || '%'
            "#
        )
        .bind(title)
        .fetch_all(&book_repo.database_client)
        .await?;

        Ok(rows)
    } else {
        Ok(Vec::new())
    }
}