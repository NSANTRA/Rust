use dotenv::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Error, PgPool};
use std::env::var;

pub async fn initialize() -> Result<PgPool, Error> {
    dotenv().ok();

    let host = var("SUPABASE_HOST").unwrap();
    let password = var("SUPABASE_PASSWORD").unwrap();
    let project_ref = var("SUPABASE_PROJECT_ID").unwrap();
    let port = var("SUPABASE_PORT").unwrap();

    let conn = format!(
        "postgresql://postgres.{}:{}@{}:{}/postgres?pgbouncer=true",
        project_ref, password, host, port
    );

    let options: PgConnectOptions = conn
        .parse::<PgConnectOptions>()?
        .statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    println!("Supabase Connected!");

    Ok(pool)
}