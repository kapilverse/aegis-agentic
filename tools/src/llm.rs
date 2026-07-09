use crate::error::ToolError;
use crate::trait_def::{Tool, ToolContext, ToolOutput};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct LlmTool {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
    model: String,
}

#[derive(Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize)]
struct HuggingFaceResponse {
    generated_text: Option<String>,
}

impl LlmTool {
    pub fn new(endpoint: String, api_key: Option<String>, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
            api_key,
            model,
        }
    }

    async fn call_huggingface(&self, prompt: &str) -> Result<String, ToolError> {
        let url = format!("{}/models/{}", self.endpoint, self.model);

        let mut request = self.client.post(&url).json(&serde_json::json!({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": 512,
                "temperature": 0.7,
                "return_full_text": false
            }
        }));

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        if let Some(text) = body.get("generated_text").and_then(|v| v.as_str()) {
            return Ok(text.to_string());
        }

        if let Some(arr) = body.as_array() {
            if let Some(first) = arr.first() {
                if let Some(text) = first.get("generated_text").and_then(|v| v.as_str()) {
                    return Ok(text.to_string());
                }
            }
        }

        Ok(body.to_string())
    }

    async fn call_openai_compatible(&self, messages: &[ChatMessage]) -> Result<String, ToolError> {
        let url = format!("{}/v1/chat/completions", self.endpoint);

        let mut request = self.client.post(&url).json(&ChatRequest {
            messages: messages.to_vec(),
            max_tokens: Some(512),
            temperature: Some(0.7),
        });

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let body: ChatResponse = response
            .json()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        body.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| ToolError::ExecutionFailed("No response content".into()))
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, ToolError> {
        if self.endpoint.contains("/models/") || self.endpoint.contains("api-inference") {
            if let Some(last) = messages.last() {
                return self.call_huggingface(&last.content).await;
            }
        }
        self.call_openai_compatible(&messages).await
    }
}

#[async_trait]
impl Tool for LlmTool {
    fn name(&self) -> &str {
        "llm_chat"
    }

    fn description(&self) -> &str {
        "Calls an LLM for text generation. Supports HuggingFace API and OpenAI-compatible endpoints (Ollama, vLLM, llama.cpp)."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "The user message to send to the LLM"
                },
                "system_prompt": {
                    "type": "string",
                    "description": "Optional system prompt"
                }
            },
            "required": ["prompt"]
        })
    }

    async fn execute(&self, args: serde_json::Value, _ctx: &ToolContext) -> Result<ToolOutput, ToolError> {
        let prompt = args
            .get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidArgs("missing 'prompt'".into()))?;

        let mut messages = Vec::new();

        if let Some(system) = args.get("system_prompt").and_then(|v| v.as_str()) {
            messages.push(ChatMessage {
                role: "system".into(),
                content: system.to_string(),
            });
        }

        messages.push(ChatMessage {
            role: "user".into(),
            content: prompt.to_string(),
        });

        let response = self.chat(messages).await?;
        Ok(ToolOutput::success(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_tool_schema() {
        let tool = LlmTool::new("http://localhost:8000".into(), None, "test".into());
        let schema = tool.parameters_schema();
        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_llm_tool_metadata() {
        let tool = LlmTool::new("http://localhost:8000".into(), None, "test".into());
        assert_eq!(tool.name(), "llm_chat");
        assert!(!tool.description().is_empty());
    }
}
