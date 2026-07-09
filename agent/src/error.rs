use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Invalid state transition: {0}")]
    InvalidTransition(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Max iterations exceeded")]
    MaxIterationsExceeded,

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("LLM error: {0}")]
    LlmError(String),
}

impl From<String> for AgentError {
    fn from(s: String) -> Self {
        AgentError::InvalidTransition(s)
    }
}
