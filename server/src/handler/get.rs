use core::KvsEngine;
use std::sync::{Arc, Mutex};

pub fn handle(db: Arc<Mutex<KvsEngine>>, key: String) -> String {
    // 排他制御してデータストアにアクセス
    let db_lock = db.lock().unwrap();
    // データを取得
    match db_lock.get(key.to_string()) {
        Some(value) => format!("{}\n", value),
        None => "Key not found\n".to_string(),
    }
}
