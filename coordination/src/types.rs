use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: Uuid,
    pub from: Uuid,
    pub to: Option<Uuid>,
    pub content: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    TaskRequest {
        description: String,
        context: serde_json::Value,
    },
    TaskResult {
        result: String,
        success: bool,
    },
    StatusUpdate {
        status: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: Uuid,
    pub name: String,
    pub capabilities: Vec<String>,
}
