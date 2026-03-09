use crate::models::users::Users;
use sqlx::{PgPool, Error, query, postgres::PgQueryResult};
use uuid::Uuid;

pub async fn delete(user_id: &Uuid, database_client: &PgPool) -> Result<bool, Error> {
    let rows: PgQueryResult = query("DELETE FROM users WHERE user_id = $1")
        .bind(&user_id)
        .execute(database_client)
        .await?;

    Ok(rows.rows_affected() == 1)
}

pub async fn insert(new_user: &Users, database_client: &PgPool) -> Result<bool, Error> {
    let rows: PgQueryResult = query("INSERT INTO users VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&new_user.user_id)
        .bind(&new_user.first_name)
        .bind(&new_user.middle_name)
        .bind(&new_user.last_name)
        .bind(&new_user.age)
        .bind(&new_user.email)
        .execute(database_client)
        .await?;

    Ok(rows.rows_affected() == 1)
}