use actix_web::HttpResponse;
use serde_json::json;

pub async fn list_tools() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "tools": [
            {
                "name": "http_request",
                "description": "Makes an HTTP request to a specified URL",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE"]},
                        "url": {"type": "string"}
                    },
                    "required": ["method", "url"]
                }
            }
        ],
        "total": 1
    }))
}
