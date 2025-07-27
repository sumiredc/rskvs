use rskvs_core::KvsEngine;
use std::sync::{Arc, Mutex};

use crate::handler::error::ServerError;

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String) -> Result<String, ServerError> {
    let db_lock = db.lock().map_err(|_| ServerError::LockError)?;

    match db_lock.get(key.to_string()) {
        Some(value) => Ok(format!("{}\n", value)),
        None => Ok("Key not found\n".to_string()),
    }
}
