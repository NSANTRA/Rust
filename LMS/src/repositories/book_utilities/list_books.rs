use crate::models::books::{BookResponse, SearchBookRequest};
use crate::models::custom_error::RepositoryError;
use crate::repositories::books_repository::BooksRepository;
use sqlx::query_as;

pub async fn list_books(
    book_repo: &BooksRepository,
    request: &SearchBookRequest,
) -> Result<Vec<BookResponse>, RepositoryError> {
    if request.title.is_none()
        && request.publisher_name.is_none()
        && request.author_name.is_none()
        && request.genre.is_none()
    {
        return Ok(Vec::new());
    }

    if let Some(title) = &request.title {
        let rows = query_as::<_, BookResponse>("SELECT * FROM search_books_by_title($1)")
            .bind(title)
            .fetch_all(&book_repo.database_client)
            .await?;

        Ok(rows)
    } else if let Some(publisher_name) = &request.publisher_name {
        let rows = query_as::<_, BookResponse>("SELECT * FROM search_books_by_publisher($1)")
            .bind(publisher_name)
            .fetch_all(&book_repo.database_client)
            .await?;

        Ok(rows)
    } else {
        Ok(Vec::new())
    }
}
