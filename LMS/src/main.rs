use sqlx::PgPool;

mod database;

#[tokio::main]
async fn main() {
    let _: PgPool = database::database_client::initialize().await.unwrap();
}