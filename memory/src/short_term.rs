use crate::types::MemoryEntry;
use chrono::Utc;

pub struct ShortTermMemory {
    entries: Vec<MemoryEntry>,
    max_tokens: usize,
    current_tokens: usize,
}

impl ShortTermMemory {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_tokens,
            current_tokens: 0,
        }
    }

    pub fn add(&mut self, content: &str, source: &str) {
        let estimated_tokens = content.len() / 4;
        let entry = MemoryEntry::new(content, source);

        self.current_tokens += estimated_tokens;
        self.entries.push(entry);

        while self.current_tokens > self.max_tokens && !self.entries.is_empty() {
            let removed = self.entries.remove(0);
            self.current_tokens -= removed.content.len() / 4;
        }
    }

    pub fn get_context(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.content.clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_tokens = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve() {
        let mut memory = ShortTermMemory::new(1000);
        memory.add("Hello, world!", "user");
        assert_eq!(memory.len(), 1);
        assert_eq!(memory.get_context()[0], "Hello, world!");
    }

    #[test]
    fn test_eviction() {
        let mut memory = ShortTermMemory::new(20);
        memory.add("This is a longer message that should be evicted", "user");
        memory.add("Short", "user");
        assert_eq!(memory.len(), 1);
        assert_eq!(memory.get_context()[0], "Short");
    }
}
