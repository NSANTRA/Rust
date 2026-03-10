use sqlx::Error;

#[derive(Debug)]
pub enum RepositoryError {
    AlreadyExists,
    Database(Error),
}

impl From<Error> for RepositoryError {
    fn from(error: Error) -> Self {
        RepositoryError::Database(error)
    }
}