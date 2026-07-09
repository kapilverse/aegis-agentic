use crate::types::{Agent, AgentState};

#[derive(Debug, Clone)]
pub enum StateTransition {
    Start,
    ThinkComplete {
        content: String,
        tool_calls: Option<Vec<ToolCallRequest>>,
    },
    ActComplete {
        output: String,
    },
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ToolCallRequest {
    pub tool_name: String,
    pub args: serde_json::Value,
}

impl Agent {
    pub fn transition(&mut self, event: StateTransition) -> Result<(), String> {
        let old_state = self.state.clone();

        match event {
            StateTransition::Start => match &self.state {
                AgentState::Idle => {
                    self.state = AgentState::Thinking;
                }
                _ => return Err(format!("Cannot start from state {:?}", old_state)),
            },
            StateTransition::ThinkComplete { content, tool_calls } => match &self.state {
                AgentState::Thinking => {
                    if let Some(calls) = tool_calls {
                        if let Some(first) = calls.into_iter().next() {
                            self.state = AgentState::Acting {
                                tool_name: first.tool_name,
                                args: first.args,
                            };
                        } else {
                            self.state = AgentState::Done { output: content };
                        }
                    } else {
                        self.state = AgentState::Done { output: content };
                    }
                }
                _ => return Err(format!("Cannot complete thinking from state {:?}", old_state)),
            },
            StateTransition::ActComplete { output } => match &self.state {
                AgentState::Acting { .. } => {
                    self.state = AgentState::Observing { output };
                }
                _ => return Err(format!("Cannot complete action from state {:?}", old_state)),
            },
            StateTransition::Error(msg) => {
                self.state = AgentState::Error(msg);
            }
        }

        tracing::debug!(
            agent_id = %self.id,
            from = ?old_state,
            to = ?self.state,
            "State transition"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AgentConfig;

    #[test]
    fn test_idle_to_thinking() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        assert!(agent.transition(StateTransition::Start).is_ok());
        assert!(matches!(agent.state, AgentState::Thinking));
    }

    #[test]
    fn test_thinking_to_acting() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        agent.transition(StateTransition::Start).unwrap();
        agent
            .transition(StateTransition::ThinkComplete {
                content: "test".into(),
                tool_calls: Some(vec![ToolCallRequest {
                    tool_name: "test_tool".into(),
                    args: serde_json::json!({}),
                }]),
            })
            .unwrap();
        assert!(matches!(agent.state, AgentState::Acting { .. }));
    }

    #[test]
    fn test_thinking_to_done() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        agent.transition(StateTransition::Start).unwrap();
        agent
            .transition(StateTransition::ThinkComplete {
                content: "done".into(),
                tool_calls: None,
            })
            .unwrap();
        assert!(matches!(agent.state, AgentState::Done { .. }));
    }

    #[test]
    fn test_invalid_transition() {
        let mut agent = Agent::new("test".into(), AgentConfig::default());
        agent.transition(StateTransition::Start).unwrap();
        assert!(agent.transition(StateTransition::Start).is_err());
    }
}
