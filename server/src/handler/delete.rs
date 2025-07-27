use rskvs_core::KvsEngine;
use std::sync::{Arc, Mutex};

use crate::handler::error::ServerError;

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String) -> Result<String, ServerError> {
    let mut db_lock = db.lock().map_err(|_| ServerError::LockError)?;
    db_lock.delete(key)?;

    Ok("OK\n".to_string())
}
