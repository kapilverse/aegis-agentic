/// AEGIS API - Agent Orchestration Runtime
/// HTTP API for managing agents, sessions, and tool execution

use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tracing::info;

mod config;
mod jwt_auth;
mod security_middleware;
mod telemetry;
mod metrics;
mod database;
mod db_migrations;
mod handlers;

use config::ApiConfig;
use metrics::PrometheusMetrics;
use jwt_auth::{ApiKeyValidator, JwtAuthMiddleware};
use security_middleware::{RateLimitMiddleware, SecurityHeadersMiddleware, RequestIdMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init_tracing("aegis-api").expect("Failed to initialize tracing");

    let config = ApiConfig::from_env();
    info!("AEGIS API configuration loaded");
    info!("Host: {}, Port: {}", config.host, config.port);

    let prometheus_metrics = web::Data::new(
        PrometheusMetrics::new().expect("Failed to initialize Prometheus metrics"),
    );

    let db_pool = match database::create_pool().await {
        Ok(pool) => {
            info!("PostgreSQL database initialized");
            web::Data::new(pool)
        }
        Err(e) => {
            info!("PostgreSQL not available: {}. Running in-memory mode.", e);
            panic!("Database initialization failed: {}", e);
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me-in-production".to_string());
    let fallback_api_keys = std::env::var("API_KEYS")
        .unwrap_or_else(|_| "sk-demo123".to_string())
        .split(',')
        .map(|k| k.trim().to_string())
        .collect();
    let api_key_validator = web::Data::new(ApiKeyValidator::new(jwt_secret, fallback_api_keys));

    let tool_registry = web::Data::new(aegis_tools::ToolRegistry::new());

    info!("Starting AEGIS API on http://{}:{}", config.host, config.port);
    info!("Available endpoints:");
    info!("  POST   /agents              - Create agent");
    info!("  GET    /agents              - List agents");
    info!("  GET    /agents/:id          - Get agent");
    info!("  POST   /sessions            - Create session");
    info!("  GET    /sessions/:id        - Get session");
    info!("  POST   /sessions/:id/messages - Send message");
    info!("  GET    /tools               - List tools");
    info!("  GET    /health/live         - Liveness probe");
    info!("  GET    /health/ready        - Readiness probe");
    info!("  GET    /metrics             - Prometheus metrics");

    HttpServer::new(move || {
        App::new()
            .app_data(prometheus_metrics.clone())
            .app_data(api_key_validator.clone())
            .app_data(tool_registry.clone())
            .app_data(db_pool.clone())
            .wrap(RequestIdMiddleware)
            .wrap(middleware::Logger::default())
            .wrap(SecurityHeadersMiddleware)
            .wrap(RateLimitMiddleware::new(
                config.rate_limit_rps as u32 * 60 / 1000,
            ))
            .wrap(JwtAuthMiddleware::new(
                api_key_validator.get_ref().clone(),
            ))
            .wrap(middleware::NormalizePath::trim())
            // Agent endpoints
            .route("/agents", web::post().to(handlers::create_agent))
            .route("/agents", web::get().to(handlers::list_agents))
            .route("/agents/{id}", web::get().to(handlers::get_agent))
            .route("/agents/{id}", web::delete().to(handlers::delete_agent))
            // Session endpoints
            .route("/sessions", web::post().to(handlers::create_session))
            .route("/sessions/{id}", web::get().to(handlers::get_session))
            .route(
                "/sessions/{id}/messages",
                web::post().to(handlers::send_message),
            )
            .route(
                "/sessions/{id}/messages",
                web::get().to(handlers::get_messages),
            )
            // Tool endpoints
            .route("/tools", web::get().to(handlers::list_tools))
            // Health & metrics
            .route("/health/live", web::get().to(handlers::health_live))
            .route("/health/ready", web::get().to(handlers::health_ready))
            .route("/metrics", web::get().to(handlers::metrics))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
