use postgres::{Client, Error, NoTls};
use std::env::var;
use dotenv::dotenv;

pub fn initialize() -> Result<Client, Error> {
    dotenv().ok();

    let host = var("SUPABASE_HOST").unwrap();
    let password = var("SUPABASE_PASSWORD").unwrap();
    let project_ref = var("SUPABASE_PROJECT_ID").unwrap();

    let conn = format!(
        "host = {} user = postgres.{} password = {} port = 6543 dbname = postgres",
        host, project_ref, password
    );

    let client = Client::connect(&conn, NoTls)?;

    println!("Supabase Connected!");

    Ok(client)
}