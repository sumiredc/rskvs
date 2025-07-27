use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
use thiserror::Error;

// カスタムエラー型を定義
#[derive(Error, Debug)]
pub enum KvsError {
    // I/O エラーのラップ
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // ログパースエラー
    #[error("Parse error: invalid log entry")]
    ParseError,
}

// カスタムエラーを実装した Result 型を定義
pub type Result<T> = std::result::Result<T, KvsError>;

pub struct KvsEngine {
    store: HashMap<String, String>,
    log_file: File,
}

impl KvsEngine {
    pub fn new(path: PathBuf) -> Result<Self> {
        // ログファイルの読み込み
        let log_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)?;

        let mut store = HashMap::new();
        let reader = BufReader::new(&log_file);

        // ログファイルを読み込んで、メモリ上のストアを復元
        for l in reader.lines() {
            let line = l?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            match parts.as_slice() {
                ["set", key, value] => {
                    store.insert(key.to_string(), value.to_string());
                }
                ["delete", key] => {
                    store.remove(&key.to_string());
                }
                // 空行は無視
                [] => (),
                // 不正な形式のログエントリはエラー
                _ => return Err(KvsError::ParseError),
            }
        }

        Ok(KvsEngine { store, log_file })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = format!("set {} {}\n", key, value);
        self.log_file.write_all(command.as_bytes())?; // I/O エラーがラップされて返却される
        self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn delete(&mut self, key: String) -> Result<()> {
        let command = format!("delete {}\n", key);
        self.log_file.write_all(command.as_bytes())?; // I/O エラーがラップされて返却される
        self.store.remove(&key);
        Ok(())
    }
}
