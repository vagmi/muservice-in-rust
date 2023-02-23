use thiserror::Error;

#[derive(Error, Debug)]
pub enum MulibError {
    #[error("Error starting server")]
    ServerError
}

pub type Result<T> = std::result::Result<T, MulibError>;
