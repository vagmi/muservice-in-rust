use thiserror::Error;

#[derive(Error, Debug)]
pub enum MulibError {
    #[error("Error starting server")]
    ServerError,
    #[error("SQLX Errors")]
    SqlxError {#[from] err: sqlx::Error},
    #[error("Config Error")]
    ConfigError {#[from] err: std::env::VarError}
}

pub type Result<T> = std::result::Result<T, MulibError>;
