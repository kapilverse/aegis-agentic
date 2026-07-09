use crate::types::{MemoryEntry, MemoryQuery};
use std::collections::HashMap;

pub struct LongTermMemory {
    entries: Vec<MemoryEntry>,
    index: HashMap<String, Vec<usize>>,
}

impl LongTermMemory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn store(&mut self, entry: MemoryEntry) {
        let idx = self.entries.len();
        let tokens = Self::tokenize(&entry.content);
        for token in tokens {
            self.index
                .entry(token)
                .or_default()
                .push(idx);
        }
        self.entries.push(entry);
    }

    pub fn retrieve(&self, query: &MemoryQuery) -> Vec<MemoryEntry> {
        let query_tokens = Self::tokenize(&query.text);
        let mut scores: Vec<(usize, f64)> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let entry_tokens = Self::tokenize(&e.content);
                let score = Self::tfidf_score(&query_tokens, &entry_tokens, &self.index, self.entries.len());
                (i, score)
            })
            .filter(|(_, s)| *s > 0.0)
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
            .into_iter()
            .take(query.top_k)
            .map(|(i, _)| self.entries[i].clone())
            .collect()
    }

    pub fn delete(&mut self, id: uuid::Uuid) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty() && s.len() > 2)
            .map(|s| s.to_string())
            .collect()
    }

    fn tfidf_score(
        query_tokens: &[String],
        entry_tokens: &[String],
        index: &HashMap<String, Vec<usize>>,
        total_docs: usize,
    ) -> f64 {
        let mut score = 0.0;
        for qt in query_tokens {
            if let Some(df) = index.get(qt) {
                let tf = entry_tokens.iter().filter(|t| *t == qt).count() as f64;
                let idf = (total_docs as f64 / df.len() as f64).ln();
                score += tf * idf;
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let mut memory = LongTermMemory::new();
        memory.store(MemoryEntry::new("Rust is a systems programming language", "doc1"));
        memory.store(MemoryEntry::new("Python is used for machine learning", "doc2"));

        let query = MemoryQuery::new("programming language", 2);
        let results = memory.retrieve(&query);
        assert!(!results.is_empty());
        assert!(results[0].content.contains("Rust"));
    }
}
