use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Genre {
    pub genre_id: Uuid,
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CreateGenreRequest {
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchGenreRequest {
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchGenreResponse {
    pub genre_id: Uuid,
    pub name: String,
}