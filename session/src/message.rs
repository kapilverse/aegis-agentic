use crate::types::{Message, Role, ToolCall};
use uuid::Uuid;

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::User,
            content: content.into(),
            tool_calls: None,
            token_count: 0,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::Assistant,
            content: content.into(),
            tool_calls: None,
            token_count: 0,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::System,
            content: content.into(),
            tool_calls: None,
            token_count: 0,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn tool(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::Tool,
            content: content.into(),
            tool_calls: Some(tool_calls),
            token_count: 0,
            timestamp: chrono::Utc::now(),
        }
    }
}
