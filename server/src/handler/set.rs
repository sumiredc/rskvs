use core::KvsEngine;
use std::sync::{Arc, Mutex};

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String, value: String) -> String {
    db.lock().unwrap().set(key, value);
    "OK\n".to_string()
}
