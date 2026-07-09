use crate::error::AgentError;
use crate::state_machine::{StateTransition, ToolCallRequest};
use crate::types::{Agent, AgentState};
use aegis_tools::ToolRegistry;
use tracing::{info, warn};

pub struct AgentExecutor;

impl AgentExecutor {
    pub async fn run(
        agent: &mut Agent,
        user_message: &str,
        tool_registry: &ToolRegistry,
    ) -> Result<String, AgentError> {
        let mut conversation: Vec<ConversationMessage> = vec![ConversationMessage {
            role: "user".into(),
            content: user_message.to_string(),
        }];

        agent.transition(StateTransition::Start)?;

        for _iteration in 0..agent.config.max_iterations {
            match &agent.state {
                AgentState::Thinking => {
                    // Simulate LLM thinking (Phase 1 will implement real LLM call)
                    let response = Self::simulate_think(agent, &conversation);
                    conversation.push(ConversationMessage {
                        role: "assistant".into(),
                        content: response.content.clone(),
                    });

                    if let Some(tool_calls) = response.tool_calls {
                        agent.transition(StateTransition::ThinkComplete {
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
                        agent.transition(StateTransition::ThinkComplete { tool_calls: None })?;
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

    fn simulate_think(
        _agent: &Agent,
        conversation: &[ConversationMessage],
    ) -> ThinkResponse {
        let last_message = conversation.last().map(|m| m.content.as_str()).unwrap_or("");

        if last_message.contains("hello") || last_message.contains("hi") {
            ThinkResponse {
                content: "Hello! How can I help you today?".to_string(),
                tool_calls: None,
            }
        } else if last_message.contains("time") {
            ThinkResponse {
                content: "Let me check the time for you.".to_string(),
                tool_calls: Some(vec![(
                    "get_time".to_string(),
                    serde_json::json!({}),
                )]),
            }
        } else {
            ThinkResponse {
                content: format!("I received your message: '{}'. I don't have a specific tool for this, but I'm here to help!", last_message),
                tool_calls: None,
            }
        }
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

    #[tokio::test]
    async fn test_simple_conversation() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        let registry = ToolRegistry::new();
        let result = AgentExecutor::run(&mut agent, "hello", &registry)
            .await
            .unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_max_iterations() {
        let mut config = AgentConfig::default();
        config.max_iterations = 1;
        let mut agent = Agent::new("test".into(), config);
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(crate::executor::tests::EchoTool));
        let _ = AgentExecutor::run(&mut agent, "use echo", &registry).await;
    }
}
