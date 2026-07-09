use crate::error::ToolError;
use crate::trait_def::{Tool, ToolContext, ToolOutput};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, Arc::from(tool));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn list(&self) -> Vec<ToolInfo> {
        self.tools
            .values()
            .map(|t| ToolInfo {
                name: t.name().to_string(),
                description: t.description().to_string(),
                parameters_schema: t.parameters_schema(),
            })
            .collect()
    }

    pub async fn execute(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<String, ToolError> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| ToolError::NotFound(tool_name.to_string()))?;

        let ctx = ToolContext {
            session_id: None,
            agent_id: None,
            tenant_id: None,
        };

        let output = tool.execute(args, &ctx).await?;
        Ok(output.content)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct EchoTool;

    #[async_trait]
    impl Tool for EchoTool {
        fn name(&self) -> &str {
            "echo"
        }

        fn description(&self) -> &str {
            "Echoes the input"
        }

        fn parameters_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                },
                "required": ["message"]
            })
        }

        async fn execute(&self, args: serde_json::Value, _ctx: &ToolContext) -> Result<ToolOutput, ToolError> {
            let msg = args
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("no message");
            Ok(ToolOutput::success(msg))
        }
    }

    #[tokio::test]
    async fn test_registry_register_and_execute() {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(EchoTool));

        let result = registry
            .execute("echo", serde_json::json!({"message": "hello"}))
            .await
            .unwrap();
        assert_eq!(result, "hello");
    }

    #[tokio::test]
    async fn test_registry_not_found() {
        let registry = ToolRegistry::new();
        let result = registry.execute("nonexistent", serde_json::json!({})).await;
        assert!(result.is_err());
    }
}
