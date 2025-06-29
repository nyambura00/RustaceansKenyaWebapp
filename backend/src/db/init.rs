use redb::{Database, DatabaseError};
use thiserror::Error;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;

#[derive(Clone)]
pub struct DbState {
    pub db: Arc<Database>,
}

impl DbState {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[derive(Debug, Error)]
pub enum DbSetupError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Redb(#[from] DatabaseError),
}

pub async fn setup_db() -> Result<DbState, DbSetupError> {
    let data_dir = PathBuf::from("data");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).await?;
    }

    let db_path = data_dir.join("rk_site.redb");
    let db = Database::create(db_path)?;

    Ok(DbState::new(Arc::new(db)))
}