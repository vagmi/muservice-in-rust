use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaoError {
    #[error("error caused by a SQLX call")]
    SqlxError {#[from]err: sqlx::Error}
}
