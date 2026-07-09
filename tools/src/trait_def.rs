use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToolOutput {
    pub content: String,
    pub is_error: bool,
    pub metadata: HashMap<String, String>,
}

impl ToolOutput {
    pub fn success(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_error: false,
            metadata: HashMap::new(),
        }
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_error: true,
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolContext {
    pub session_id: Option<uuid::Uuid>,
    pub agent_id: Option<uuid::Uuid>,
    pub tenant_id: Option<String>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value, ctx: &ToolContext) -> Result<ToolOutput, crate::error::ToolError>;
}
