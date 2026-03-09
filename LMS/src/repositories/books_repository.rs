use crate::models::books::{BookResponse, CreateBookRequest};
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{query, Error, PgPool, Row};
use uuid::Uuid;

pub struct BooksRepository {
    pub database_client: PgPool,
}

impl BooksRepository {
    pub fn new(database_client: PgPool) -> BooksRepository {
        BooksRepository { database_client }
    }

    pub async fn create_book(
        self: &BooksRepository,
        request: &CreateBookRequest,
    ) -> Result<BookResponse, Error> {
        let mut transaction = self.database_client.begin().await?;

        let row: PgRow = query("INSERT INTO publishers (name) VALUES ($1) ON CONFLICT (name) DO UPDATE SET NAME = publishers.name RETURNING publisher_id")
            .bind(&request.publisher_name)
            .fetch_one(&mut *transaction)
            .await?;

        let publisher_id: Uuid = row.get("publisher_id");

        let mut author_ids: Vec<Uuid> = Vec::with_capacity(request.authors.len());

        let mut genre_ids: Vec<Uuid> = Vec::with_capacity(request.genres.len());

        for author in &request.authors {
            let parts: Vec<&str> = author.split_whitespace().collect();

            let mut first_name: &str = "";
            let mut middle_name: Option<String> = None;
            let mut last_name: &str = "";

            match parts.len() {
                0 => continue,
                1 => {
                    first_name = parts[0];
                    middle_name = None;
                    last_name = "";
                }
                2 => {
                    first_name = parts[0];
                    middle_name = None;
                    last_name = parts[1];
                }
                _ => {
                    first_name = parts[0];
                    middle_name = Some(parts[1..parts.len() - 1].join(" "));
                    last_name = parts[parts.len() - 1];
                }
            }

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
                query("INSERT INTO book_inventory (book_id, status) VALUES ($1, $2)")
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
}
