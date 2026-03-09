use uuid::Uuid;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Publisher {
    pub publisher_id: Uuid,
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CreatePublisherRequest {
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchPublisherRequest {
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchPublisherResponse {
    pub publisher_id: Uuid,
    pub name: String,
}