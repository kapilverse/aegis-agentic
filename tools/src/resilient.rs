use crate::error::ToolError;
use crate::trait_def::{Tool, ToolContext, ToolOutput};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::warn;

pub struct ResilientTool {
    inner: Arc<dyn Tool>,
    timeout_duration: Duration,
    max_retries: u32,
}

impl ResilientTool {
    pub fn new(tool: Arc<dyn Tool>, timeout_ms: u64, max_retries: u32) -> Self {
        Self {
            inner: tool,
            timeout_duration: Duration::from_millis(timeout_ms),
            max_retries,
        }
    }
}

#[async_trait]
impl Tool for ResilientTool {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn parameters_schema(&self) -> serde_json::Value {
        self.inner.parameters_schema()
    }

    async fn execute(&self, args: serde_json::Value, ctx: &ToolContext) -> Result<ToolOutput, ToolError> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match timeout(self.timeout_duration, self.inner.execute(args.clone(), ctx)).await {
                Ok(Ok(output)) => return Ok(output),
                Ok(Err(e)) => {
                    warn!(
                        tool = self.name(),
                        attempt = attempt + 1,
                        error = %e,
                        "Tool execution failed"
                    );
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        let backoff = Duration::from_millis(100 * 2u64.pow(attempt));
                        tokio::time::sleep(backoff).await;
                    }
                }
                Err(_) => {
                    warn!(
                        tool = self.name(),
                        attempt = attempt + 1,
                        "Tool execution timed out"
                    );
                    last_error = Some(ToolError::Timeout);
                    if attempt < self.max_retries {
                        let backoff = Duration::from_millis(100 * 2u64.pow(attempt));
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or(ToolError::ExecutionFailed(
            "All retries exhausted".into(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct AlwaysOkTool;

    #[async_trait]
    impl Tool for AlwaysOkTool {
        fn name(&self) -> &str { "always_ok" }
        fn description(&self) -> &str { "Always succeeds" }
        fn parameters_schema(&self) -> serde_json::Value { serde_json::json!({}) }
        async fn execute(&self, _args: serde_json::Value, _ctx: &ToolContext) -> Result<ToolOutput, ToolError> {
            Ok(ToolOutput::success("ok"))
        }
    }

    #[tokio::test]
    async fn test_resilient_tool_success() {
        let tool = ResilientTool::new(Arc::new(AlwaysOkTool), 1000, 2);
        let ctx = ToolContext { session_id: None, agent_id: None, tenant_id: None };
        let result = tool.execute(serde_json::json!({}), &ctx).await.unwrap();
        assert_eq!(result.content, "ok");
    }
}
