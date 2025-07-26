use std::collections::HashMap;

#[derive(Default)]
pub struct KvsEngine {
    store: HashMap<String, String>,
}

impl KvsEngine {
    pub fn new() -> Self {
        KvsEngine {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
}
