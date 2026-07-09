use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub content: String,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl MemoryEntry {
    pub fn new(content: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.into(),
            source: source.into(),
            created_at: Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryQuery {
    pub text: String,
    pub top_k: usize,
}

impl MemoryQuery {
    pub fn new(text: impl Into<String>, top_k: usize) -> Self {
        Self {
            text: text.into(),
            top_k,
        }
    }
}
