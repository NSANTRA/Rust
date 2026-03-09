pub mod models;
pub mod database;
pub mod utils;

use std::{io::{stdout, stdin, Write}, str::FromStr};
use sqlx::PgPool;
use uuid::Uuid;
use models::{books::Books, users::Users};
use database::database_client::initialize;
use utils::{user_utils, book_utils};

#[tokio::main]
async fn main() {
    let database_client: PgPool = initialize().await.unwrap();

    let user: Users = Users {
        user_id: uuid::Uuid::new_v4(),
        first_name: String::from("Neelotpal"),
        middle_name: None,
        last_name: String::from("Santra"),
        age: 22,
        email: String::from("ns@test.com"),
    };

    let book: Books = Books {
        book_id: uuid::Uuid::new_v4(),
        title: String::from("Percy Jackson"),
        description: Some(String::from("Demigods")),
        genre: Vec::from([String::from("Sci-fi"), String::from("Fantasy")]),
        author: String::from("Rick Riordan"),
        publisher: Some(String::from("Penguin House")),
    };

    println!("Inserting User");

    match user_utils::insert(&user, &database_client).await{
        Ok(true) => {
            println!("{:#?} inserted to Supabase", user);
        }
        Ok(false) => {
            println!("{:#?} not inserted to Supabase", user);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    println!("User Inserted");

    println!("Inserting Book");

    match book_utils::insert(&book, &database_client).await {
        Ok(true) => {
            println!("{:#?} inserted to Supabase", book);
        }
        Ok(false) => {
            println!("{:#?} not inserted to Supabase", book);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    println!("Book inserted");

    let user_id: Uuid = uuid::Uuid::parse_str("65ef990a-6c72-4fce-ad79-678d2b10671b").unwrap();
    match user_utils::delete(&user_id, &database_client).await {
        Ok(true) => {
            println!("Deleted user {}", user_id);
        }
        Ok(false) => {
            println!("User not found");
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

}