use crate::long_term::LongTermMemory;
use crate::types::MemoryQuery;

pub struct MemoryRetriever<'a> {
    long_term: &'a LongTermMemory,
}

impl<'a> MemoryRetriever<'a> {
    pub fn new(long_term: &'a LongTermMemory) -> Self {
        Self { long_term }
    }

    pub fn retrieve_context(&self, query: &str, top_k: usize) -> String {
        let memory_query = MemoryQuery::new(query, top_k);
        let memories = self.long_term.retrieve(&memory_query);

        if memories.is_empty() {
            return String::new();
        }

        let mut context = "Relevant memories:\n".to_string();
        for (i, entry) in memories.iter().enumerate() {
            context.push_str(&format!("{}. {}\n", i + 1, entry.content));
        }
        context
    }
}
