pub mod models;
pub mod database;
pub mod utils;

use postgres::{Client};
use std::{io::{stdout, stdin, Write}, str::FromStr};
use uuid::Uuid;
use models::{books::Books, users::Users};
use database::database_client::initialize;
use utils::user_utils;

fn main() {
    let mut database_client: Client = initialize().expect("Error connecting to database");

    let user: Users = Users {
        user_id: uuid::Uuid::new_v4(),
        first_name: String::from("Neelotpal"),
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
        publisher: String::from("Penguin House"),
    };

    // match user_utils::insert(&user, &mut database_client) {
    //     Ok(true) => {
    //         println!("{:#?} inserted to Supabase", user);
    //     }
    //     Ok(false) => {
    //         println!("{:#?} not inserted to Supabase", user);
    //     }
    //     Err(err) => {
    //         println!("{:#?}", err);
    //     }
    // }

    let user_id: Uuid = uuid::Uuid::parse_str("54933544-b6ed-4661-95e0-8a59dd45e800").unwrap();
    match user_utils::delete(&user_id, &mut database_client) {
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