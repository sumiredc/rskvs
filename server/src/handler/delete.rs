use core::KvsEngine;
use std::sync::{Arc, Mutex};

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String) -> String {
    // データを削除
    db.lock().unwrap().delete(key);
    "OK\n".to_string()
}
