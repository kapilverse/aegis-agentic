use std::collections::HashMap;

pub struct SharedMemory {
    data: HashMap<String, serde_json::Value>,
}

impl SharedMemory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
}
