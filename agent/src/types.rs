use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub config: AgentConfig,
    pub state: AgentState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub system_prompt: String,
    pub max_iterations: usize,
    pub llm_provider: String,
    pub model: String,
    pub temperature: f32,
    pub llm_endpoint: Option<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: "You are a helpful assistant.".to_string(),
            max_iterations: 10,
            llm_provider: "huggingface".to_string(),
            model: "meta-llama/Llama-3-8B-Instruct".to_string(),
            temperature: 0.7,
            llm_endpoint: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Thinking,
    Acting {
        tool_name: String,
        args: serde_json::Value,
    },
    Observing {
        output: String,
    },
    Error(String),
    Done {
        output: String,
    },
}

impl Agent {
    pub fn new(name: String, config: AgentConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            config,
            state: AgentState::Idle,
        }
    }
}
