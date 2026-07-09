use actix_web::HttpResponse;
use serde_json::json;

pub async fn health_live() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "alive"}))
}

pub async fn health_ready() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "ready"}))
}

pub async fn metrics() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body("# AEGIS Agent Platform Metrics\n# Phase 0: Scaffold\n")
}
