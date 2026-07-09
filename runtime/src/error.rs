use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Agent error: {0}")]
    AgentError(String),

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
