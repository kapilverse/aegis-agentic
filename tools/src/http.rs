use crate::error::ToolError;
use crate::trait_def::{Tool, ToolContext, ToolOutput};
use async_trait::async_trait;

pub struct HttpTool {
    client: reqwest::Client,
}

impl HttpTool {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for HttpTool {
    fn name(&self) -> &str {
        "http_request"
    }

    fn description(&self) -> &str {
        "Makes an HTTP request to a specified URL"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"]
                },
                "url": {"type": "string"},
                "headers": {"type": "object"},
                "body": {"type": "string"}
            },
            "required": ["method", "url"]
        })
    }

    async fn execute(&self, args: serde_json::Value, _ctx: &ToolContext) -> Result<ToolOutput, ToolError> {
        let method = args
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidArgs("missing 'method'".into()))?;

        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidArgs("missing 'url'".into()))?;

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => return Err(ToolError::InvalidArgs(format!("unsupported method: {}", method))),
        };

        if let Some(headers) = args.get("headers") {
            if let Some(obj) = headers.as_object() {
                for (k, v) in obj {
                    if let Some(val) = v.as_str() {
                        request = request.header(k.as_str(), val);
                    }
                }
            }
        }

        if let Some(body) = args.get("body").and_then(|v| v.as_str()) {
            request = request.body(body.to_string());
        }

        let response = request
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(ToolOutput::success(format!(
            "Status: {}\nBody: {}",
            status, body
        )))
    }
}
