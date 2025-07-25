use std::collections::HashMap;

struct KvsEngine {
    store: HashMap<String, String>,
}

impl KvsEngine {
    fn new() -> Self {
        KvsEngine {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
}

fn main() {
    let mut engine = KvsEngine::new();

    engine.set("key1".to_string(), "value1".to_string());
    println!("key1: {:?}", engine.get("key1".to_string()));
    println!("key2: {:?}", engine.get("key2".to_string()));
}
