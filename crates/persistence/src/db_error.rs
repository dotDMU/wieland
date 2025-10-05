#[derive(Debug, Error)]
pub enum DbError {
    #[error("not found")]
    NotFound,
    #[error("db failed: {0}")]
    Db(String),
    // Bibliotheksfehler einkapseln
    #[error(transparent)]
    Idb(#[from] idb::Error),
}