use crate::long_term::LongTermMemory;
use crate::short_term::ShortTermMemory;
use crate::types::MemoryEntry;

pub struct MemoryConsolidator<'a> {
    short_term: &'a mut ShortTermMemory,
    long_term: &'a mut LongTermMemory,
    consolidation_threshold: usize,
}

impl<'a> MemoryConsolidator<'a> {
    pub fn new(
        short_term: &'a mut ShortTermMemory,
        long_term: &'a mut LongTermMemory,
        consolidation_threshold: usize,
    ) -> Self {
        Self {
            short_term,
            long_term,
            consolidation_threshold,
        }
    }

    pub fn maybe_consolidate(&mut self) {
        if self.short_term.len() >= self.consolidation_threshold {
            self.consolidate();
        }
    }

    pub fn consolidate(&mut self) {
        let context = self.short_term.get_context();
        if context.is_empty() {
            return;
        }

        let summary = self.summarize(&context);
        let entry = MemoryEntry::new(&summary, "consolidation");
        self.long_term.store(entry);
        self.short_term.clear();
    }

    fn summarize(&self, messages: &[String]) -> String {
        if messages.len() <= 2 {
            return messages.join(" ");
        }

        let mut summary = String::from("Conversation summary: ");
        for msg in messages.iter().take(3) {
            summary.push_str(msg);
            summary.push_str("; ");
        }
        if messages.len() > 3 {
            summary.push_str(&format!("... ({} more messages)", messages.len() - 3));
        }
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidation() {
        let mut short = ShortTermMemory::new(1000);
        let mut long = LongTermMemory::new();

        short.add("Message 1", "user");
        short.add("Message 2", "user");
        short.add("Message 3", "user");

        let mut consolidator = MemoryConsolidator::new(&mut short, &mut long, 3);
        consolidator.consolidate();

        assert_eq!(short.len(), 0);
        assert_eq!(long.len(), 1);
    }

    #[test]
    fn test_maybe_consolidate() {
        let mut short = ShortTermMemory::new(1000);
        let mut long = LongTermMemory::new();

        let mut consolidator = MemoryConsolidator::new(&mut short, &mut long, 3);
        consolidator.maybe_consolidate();
        assert_eq!(long.len(), 0);

        short.add("Msg 1", "user");
        short.add("Msg 2", "user");
        consolidator.maybe_consolidate();
        assert_eq!(long.len(), 0);

        short.add("Msg 3", "user");
        consolidator.maybe_consolidate();
        assert_eq!(long.len(), 1);
    }
}
