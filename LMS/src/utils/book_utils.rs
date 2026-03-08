use crate::models::books::Books;
use postgres::{Client, Error};
use uuid::Uuid;

pub fn delete(book_id: &Uuid, database_client: &mut Client) -> Result<bool, Error> {
    let rows = database_client.execute(
        "DELETE FROM books WHERE book_id = $1",
        &[&book_id]
    )?;

    Ok(rows == 1)
}

pub fn insert(new_book: &Books, database_client: &mut Client) -> Result<bool, Error> {
    let rows = database_client.execute(
        "INSERT INTO users VALUES ($1, $2, $3, $4, $5, $6)",
        &[&new_book.book_id,
            &new_book.title,
            &new_book.description,
            &new_book.genre,
            &new_book.publisher]
    )?;

    Ok(rows == 1)
}