use hms_config::error::HmsConfigError;
use hms_db::error::HmsDbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HmsError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] HmsConfigError),

    #[error("Db error: {0}")]
    DbError(#[from] HmsDbError),

    #[error("Not initialized")]
    NotInitialized,
}
