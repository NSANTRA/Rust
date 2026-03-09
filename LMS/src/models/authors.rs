use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Author {
    pub author_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CreateAuthorRequest {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchAuthorRequest {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchAuthorResponse {
    pub author_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}