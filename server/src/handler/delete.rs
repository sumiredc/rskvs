use core::KvsEngine;
use std::sync::{Arc, Mutex};

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String) -> String {
    match db.lock().unwrap().delete(key) {
        Ok(_) => "OK\n".to_string(),
        Err(_) => "Error: Could not write to log file.\n".to_string(),
    }
}
