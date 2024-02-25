use thiserror::Error;

#[derive(Error, Debug)]
pub enum HmsDbError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Query error: {0}")]
    QueryError(#[from] diesel::result::Error),

    #[error("Connection error: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Duplicate alias exists")]
    AliasConstraintError,
}
