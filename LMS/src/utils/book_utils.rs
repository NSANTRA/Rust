use crate::models::books::Books;
use uuid::Uuid;
use sqlx::{PgPool, Error, query};

pub async fn delete(book_id: &Uuid, database_client: &PgPool) -> Result<bool, Error> {
    let rows = query("DELETE FROM books WHERE id = $1")
        .bind(&book_id)
        .execute(database_client)
        .await?;

    Ok(rows.rows_affected() == 1)
}

pub async fn insert(new_book: &Books, database_client: &PgPool) -> Result<bool, Error> {

    let rows = query(
        "INSERT INTO books (book_id, title, description, genre, author, publisher)
         VALUES ($1,$2,$3,$4,$5,$6)"
    )
        .bind(&new_book.book_id)
        .bind(&new_book.title)
        .bind(&new_book.description)
        .bind(&new_book.genre)
        .bind(&new_book.author)
        .bind(&new_book.publisher)
        .execute(database_client)
        .await?;

    Ok(rows.rows_affected() == 1)
}