use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub agent_id: Uuid,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub tenant_id: String,
}

pub async fn create_session(body: web::Json<CreateSessionRequest>) -> HttpResponse {
    let session_id = Uuid::new_v4();
    let response = SessionResponse {
        id: session_id,
        agent_id: body.agent_id,
        tenant_id: "default".to_string(),
    };
    HttpResponse::Created().json(response)
}

pub async fn get_session(path: web::Path<Uuid>) -> HttpResponse {
    let session_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "id": session_id,
        "agent_id": Uuid::new_v4(),
        "messages": []
    }))
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

pub async fn send_message(
    path: web::Path<Uuid>,
    body: web::Json<SendMessageRequest>,
) -> HttpResponse {
    let session_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "session_id": session_id,
        "user_message": body.content,
        "assistant_message": "Agent response placeholder - will be implemented in Phase 1",
        "tool_calls": []
    }))
}

pub async fn get_messages(path: web::Path<Uuid>) -> HttpResponse {
    let _session_id = path.into_inner();
    HttpResponse::Ok().json(json!({"messages": []}))
}
