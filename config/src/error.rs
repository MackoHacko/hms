use thiserror::Error;

#[derive(Error, Debug)]
pub enum HmsConfigError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Failed to process user input: {0}")]
    InputError(#[from] dialoguer::Error),
}
