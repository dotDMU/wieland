#[derive(Debug, Error)]
pub enum WieError {
    #[error("validation failed: {0}")]
    ValidationError(String),
    #[error(transparent)]
    Repo(#[from] DbError),
    #[error("unexpected: {0}")]
    Unexpected(String),
}

