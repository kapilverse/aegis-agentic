use std::env;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
    pub log_level: String,
    pub rate_limit_rps: u32,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            request_timeout_secs: env::var("API_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30),
            log_level: env::var("API_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            rate_limit_rps: env::var("RATE_LIMIT_RPS")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(100),
        }
    }
}
