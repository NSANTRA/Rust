use sqlx::{query, Error, Row};
use sqlx::postgres::{PgQueryResult, PgRow};
use uuid::Uuid;
use crate::models::books::{BookResponse, CreateBookRequest};
use crate::repositories::books_repository::BooksRepository;

pub async fn create_book(
    book_repo: &BooksRepository,
    request: &CreateBookRequest,
) -> Result<BookResponse, Error> {
    let mut transaction = book_repo.database_client.begin().await?;

    let row: PgRow = query("INSERT INTO publishers (name) VALUES ($1) ON CONFLICT (name) DO UPDATE SET NAME = publishers.name RETURNING publisher_id")
        .bind(&request.publisher_name)
        .fetch_one(&mut *transaction)
        .await?;

    let publisher_id: Uuid = row.get("publisher_id");

    let mut author_ids: Vec<Uuid> = Vec::with_capacity(request.authors.len());

    let mut genre_ids: Vec<Uuid> = Vec::with_capacity(request.genres.len());

    for author in &request.authors {
        let parts: Vec<&str> = author.split_whitespace().collect();

        let (first_name, middle_name, last_name) = match parts.len() {
            0 => continue,
            1 => (parts[0], String::new(), ""),
            2 => (parts[0], String::new(), parts[1]),
            _ => (
                parts[0],
                parts[1..parts.len() - 1].join(" "),
                parts[parts.len() - 1],
            ),
        };

        let row: PgRow = query(
            "INSERT INTO authors (first_name, middle_name, last_name) VALUES ($1, $2, $3) ON CONFLICT (first_name, middle_name, last_name) DO UPDATE SET first_name = authors.first_name RETURNING author_id",
        )
            .bind(&first_name)
            .bind(&middle_name)
            .bind(&last_name)
            .fetch_one(&mut *transaction)
            .await?;

        author_ids.push(row.get("author_id"));
    }

    for genre in &request.genres {
        let row: PgRow = query("INSERT INTO genres (name) VALUES ($1) ON CONFLICT (name) DO UPDATE SET name = genres.name RETURNING genre_id")
            .bind(&genre)
            .fetch_one(&mut *transaction)
            .await?;

        genre_ids.push(row.get("genre_id"));
    }

    let row: PgRow = query("INSERT INTO books (title, description, publisher_id) VALUES ($1, $2, $3) RETURNING book_id")
        .bind(&request.title)
        .bind(&request.description)
        .bind(&publisher_id)
        .fetch_one(&mut *transaction)
        .await?;

    let book_id: Uuid = row.get("book_id");

    for author_id in &author_ids {
        let _: PgQueryResult =
            query("INSERT INTO book_authors (book_id, author_id) VALUES ($1, $2)")
                .bind(&book_id)
                .bind(&author_id)
                .execute(&mut *transaction)
                .await?;
    }

    for genre_id in &genre_ids {
        let _: PgQueryResult =
            query("INSERT INTO book_genres (book_id, genre_id) VALUES ($1, $2)")
                .bind(&book_id)
                .bind(&genre_id)
                .execute(&mut *transaction)
                .await?;
    }

    for _ in 0..request.copies {
        let _: PgQueryResult =
            query("INSERT INTO book_inventory (book_id, status) VALUES ($1, $2::book_status)")
                .bind(&book_id)
                .bind("available")
                .execute(&mut *transaction)
                .await?;
    }

    transaction.commit().await?;

    Ok(BookResponse {
        book_id,
        title: request.title.clone(),
        description: request.description.clone(),
        publisher_name: request.publisher_name.clone(),
        authors: request.authors.clone(),
        genres: request.genres.clone(),
        available_copies: request.copies.clone(),
    })
}