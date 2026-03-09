use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Book {
    pub book_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub publisher_id: Option<Uuid>,
}

#[derive(Debug)]
#[allow(dead_code)]
// Admin book creation
pub struct CreateBookRequest {
    pub title: String,
    pub description: Option<String>,
    pub publisher_name: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub copies: i32
}

#[derive(Debug)]
#[allow(dead_code)]
// Admin book creation
pub struct SearchBookRequest {
    pub title: Option<String>,
    pub publisher_name: Option<String>,
    pub author_name: Option<String>,
    pub genre: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BookResponse {
    pub book_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub publisher_name: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub available_copies: i32,
}