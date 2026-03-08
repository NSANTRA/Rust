use crate::models::users::Users;
use postgres::{Client, Error};
use uuid::Uuid;

pub fn delete(user_id: &Uuid, database_client: &mut Client) -> Result<bool, Error> {
    let rows = database_client.execute(
        "DELETE FROM users WHERE user_id = $1",
        &[&user_id]
    )?;

    Ok(rows == 1)
}

pub fn insert(new_user: &Users, database_client: &mut Client) -> Result<bool, Error> {
    let rows = database_client.execute(
        "INSERT INTO users VALUES ($1, $2, $3, $4, $5)",
        &[&new_user.user_id,
            &new_user.first_name,
            &new_user.last_name,
            &new_user.age,
            &new_user.email]
    )?;

    Ok(rows == 1)
}