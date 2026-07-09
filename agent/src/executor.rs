use crate::error::AgentError;
use crate::state_machine::{StateTransition, ToolCallRequest};
use crate::types::{Agent, AgentState};
use aegis_tools::{LlmTool, ToolRegistry};
use tracing::{info, warn};

pub struct AgentExecutor;

impl AgentExecutor {
    pub async fn run(
        agent: &mut Agent,
        user_message: &str,
        tool_registry: &ToolRegistry,
        llm: &LlmTool,
    ) -> Result<String, AgentError> {
        let mut conversation: Vec<ConversationMessage> = vec![ConversationMessage {
            role: "user".into(),
            content: user_message.to_string(),
        }];

        agent.transition(StateTransition::Start)?;

        for _iteration in 0..agent.config.max_iterations {
            match &agent.state {
                AgentState::Thinking => {
                    let response = Self::call_llm(agent, &conversation, llm).await?;
                    conversation.push(ConversationMessage {
                        role: "assistant".into(),
                        content: response.content.clone(),
                    });

                    if let Some(tool_calls) = response.tool_calls {
                        agent.transition(StateTransition::ThinkComplete {
                            content: response.content,
                            tool_calls: Some(
                                tool_calls
                                    .into_iter()
                                    .map(|tc| ToolCallRequest {
                                        tool_name: tc.0,
                                        args: tc.1,
                                    })
                                    .collect(),
                            ),
                        })?;
                    } else {
                        agent.transition(StateTransition::ThinkComplete {
                            content: response.content,
                            tool_calls: None,
                        })?;
                    }
                }
                AgentState::Acting { tool_name, args } => {
                    let tool_name = tool_name.clone();
                    let args = args.clone();

                    info!(tool = %tool_name, "Executing tool");

                    let output = match tool_registry.execute(&tool_name, args).await {
                        Ok(output) => output,
                        Err(e) => {
                            warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            format!("Error: {}", e)
                        }
                    };

                    conversation.push(ConversationMessage {
                        role: "tool".into(),
                        content: output.clone(),
                    });

                    agent.transition(StateTransition::ActComplete { output })?;
                }
                AgentState::Observing { output } => {
                    let _output = output.clone();
                    agent.transition(StateTransition::Start)?;
                }
                AgentState::Done { output } => {
                    return Ok(output.clone());
                }
                AgentState::Error(msg) => {
                    return Err(AgentError::ExecutionFailed(msg.clone()));
                }
                AgentState::Idle => {
                    return Err(AgentError::ExecutionFailed(
                        "Agent is idle".to_string(),
                    ));
                }
            }
        }

        Err(AgentError::MaxIterationsExceeded)
    }

    async fn call_llm(
        agent: &Agent,
        conversation: &[ConversationMessage],
        llm: &LlmTool,
    ) -> Result<ThinkResponse, AgentError> {
        let mut messages = Vec::new();

        messages.push(aegis_tools::llm::ChatMessage {
            role: "system".into(),
            content: agent.config.system_prompt.clone(),
        });

        for msg in conversation {
            messages.push(aegis_tools::llm::ChatMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
            });
        }

        let response = llm
            .chat(messages)
            .await
            .map_err(|e| AgentError::LlmError(e.to_string()))?;

        let tool_calls = Self::parse_tool_calls(&response);

        Ok(ThinkResponse {
            content: response,
            tool_calls,
        })
    }

    fn parse_tool_calls(response: &str) -> Option<Vec<(String, serde_json::Value)>> {
        let response = response.trim();

        if let Ok(value) = serde_json::from_str::<serde_json::Value>(response) {
            if let Some(calls) = value.get("tool_calls").and_then(|v| v.as_array()) {
                let parsed: Vec<(String, serde_json::Value)> = calls
                    .iter()
                    .filter_map(|call| {
                        let name = call.get("name")?.as_str()?.to_string();
                        let args = call.get("args").cloned().unwrap_or(serde_json::json!({}));
                        Some((name, args))
                    })
                    .collect();
                if !parsed.is_empty() {
                    return Some(parsed);
                }
            }
        }

        None
    }
}

struct ConversationMessage {
    role: String,
    content: String,
}

struct ThinkResponse {
    content: String,
    tool_calls: Option<Vec<(String, serde_json::Value)>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AgentConfig;

    #[test]
    fn test_parse_tool_calls_valid() {
        let response = r#"{"tool_calls": [{"name": "search", "args": {"query": "hello"}}]}"#;
        let calls = AgentExecutor::parse_tool_calls(response).unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "search");
    }

    #[test]
    fn test_parse_tool_calls_none() {
        let response = "Hello, how can I help?";
        assert!(AgentExecutor::parse_tool_calls(response).is_none());
    }

    #[test]
    fn test_parse_tool_calls_empty() {
        let response = r#"{"tool_calls": []}"#;
        assert!(AgentExecutor::parse_tool_calls(response).is_none());
    }

    fn mock_llm() -> LlmTool {
        LlmTool::new(
            "http://localhost:11434".into(),
            None,
            "llama3".into(),
        )
    }

    #[tokio::test]
    async fn test_run_with_mock_llm() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        let registry = ToolRegistry::new();
        let llm = mock_llm();
        // This will fail because no LLM is running, but it tests the code path
        let result = AgentExecutor::run(&mut agent, "hello", &registry, &llm).await;
        assert!(result.is_err());
    }
}
