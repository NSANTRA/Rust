#[derive(Debug)]
#[allow(dead_code)]
pub struct Books {
    pub book_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub genre: Vec<String>,
    pub author: String,
    pub publisher: Option<String>,
}
