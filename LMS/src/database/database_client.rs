use sqlx::{PgPool, Error};
use std::env::var;
use dotenv::dotenv;

pub async fn initialize() -> Result<PgPool, Error> {
    dotenv().ok();

    let host = var("SUPABASE_HOST").unwrap();
    let password = var("SUPABASE_PASSWORD").unwrap();
    let project_ref = var("SUPABASE_PROJECT_ID").unwrap();
    let port = var("SUPABASE_PORT").unwrap();

    let conn = format!(
        "postgresql://postgres.{}:{}@{}:{}/postgres",
        project_ref, password, host, port
    );

    let client = PgPool::connect(&conn).await?;

    println!("Supabase Connected!");

    Ok(client)
}