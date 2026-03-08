#[derive(Debug)]
#[allow(dead_code)]
pub struct Users {
    pub user_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub email: String
}
