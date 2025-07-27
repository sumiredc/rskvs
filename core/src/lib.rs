use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

pub struct KvsEngine {
    store: HashMap<String, String>,
    log_file: File,
}

impl KvsEngine {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let log_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)?;

        let mut store = HashMap::new();
        let reader = BufReader::new(&log_file);

        // ログファイルを読み込んで、メモリ上のストアを復元
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if let ["set", key, value] = parts.as_slice() {
                store.insert(key.to_string(), value.to_string());
            } else if let ["delete", key] = parts.as_slice() {
                store.remove(&key.to_string());
            }
        }

        Ok(KvsEngine { store, log_file })
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        let command = format!("set {} {}\n", key, value);
        self.log_file.write_all(command.as_bytes())?;
        self.store.insert(key, value);
        Ok({})
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn delete(&mut self, key: String) -> io::Result<()> {
        let command = format!("delete {}\n", key);
        self.log_file.write_all(command.as_bytes())?;
        self.store.remove(&key);
        Ok(())
    }
}
