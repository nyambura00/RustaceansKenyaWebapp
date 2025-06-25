use bincode::error::{EncodeError, DecodeError};
use redb::{CommitError, StorageError, TableError, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Redb(#[from] redb::Error),

    #[error("Bincode encode error: {0}")]
    Encode(#[from] EncodeError),

    #[error("Bincode decode error: {0}")]
    Decode(#[from] DecodeError),

    #[error("Transcation Error: {0}")]
    Transcation(#[from] TransactionError),

    #[error("Table Error: {0}")]
    Table(#[from] TableError),

    #[error("Storage Error: {0}")]
    Storage(#[from] StorageError),

    #[error("Commit Error: {0}")]
    Commit(#[from] CommitError),

    #[error("EntityNotFound Error")]
    EntityNotFound,
}
