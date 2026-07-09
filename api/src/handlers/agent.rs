use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub system_prompt: Option<String>,
    pub model: Option<String>,
    pub llm_provider: Option<String>,
    pub max_iterations: Option<usize>,
}

#[derive(Serialize)]
pub struct AgentResponse {
    pub id: Uuid,
    pub name: String,
    pub system_prompt: String,
    pub model: String,
    pub llm_provider: String,
}

pub async fn create_agent(body: web::Json<CreateAgentRequest>) -> HttpResponse {
    let agent_id = Uuid::new_v4();
    let response = AgentResponse {
        id: agent_id,
        name: body.name.clone(),
        system_prompt: body
            .system_prompt
            .clone()
            .unwrap_or_else(|| "You are a helpful assistant.".to_string()),
        model: body
            .model
            .clone()
            .unwrap_or_else(|| "meta-llama/Llama-3-8B-Instruct".to_string()),
        llm_provider: body
            .llm_provider
            .clone()
            .unwrap_or_else(|| "huggingface".to_string()),
    };
    HttpResponse::Created().json(response)
}

pub async fn list_agents() -> HttpResponse {
    HttpResponse::Ok().json(json!({"agents": [], "total": 0}))
}

pub async fn get_agent(path: web::Path<Uuid>) -> HttpResponse {
    let agent_id = path.into_inner();
    HttpResponse::Ok().json(json!({"id": agent_id, "name": "placeholder"}))
}

pub async fn delete_agent(path: web::Path<Uuid>) -> HttpResponse {
    let _agent_id = path.into_inner();
    HttpResponse::Ok().json(json!({"deleted": true}))
}
