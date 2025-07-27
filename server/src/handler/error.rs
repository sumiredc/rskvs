use rskvs_core::KvsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Internal server error (lock poisoned)")]
    LockError,

    #[error(transparent)]
    KvsError(#[from] KvsError),

    #[error("Invalid command")]
    InvalidCommand,
}
