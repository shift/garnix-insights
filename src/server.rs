//! HTTP server implementation for Garnix Fetcher

use crate::client::GarnixClient;
use crate::error::{GarnixError, GarnixResult};
use crate::types::BuildStatusRequest;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Result as ActixResult,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, warn};

/// HTTP server for Garnix Fetcher
pub struct GarnixHttpServer {
    client: Arc<GarnixClient>,
    bind_address: String,
    port: u16,
}

impl Default for GarnixHttpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl GarnixHttpServer {
    /// Create a new HTTP server instance
    pub fn new() -> Self {
        Self {
            client: Arc::new(GarnixClient::new()),
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
        }
    }

    /// Create a new HTTP server with custom client
    pub fn with_client(client: GarnixClient) -> Self {
        Self {
            client: Arc::new(client),
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
        }
    }

    /// Set the bind address for the server
    pub fn bind_address<T: Into<String>>(mut self, address: T) -> Self {
        self.bind_address = address.into();
        self
    }

    /// Set the port for the server
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Run the HTTP server
    pub async fn run(self) -> GarnixResult<()> {
        let bind_addr = format!("{}:{}", self.bind_address, self.port);
        info!("Starting Garnix HTTP server on {}", bind_addr);

        let client = self.client.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(client.clone()))
                .wrap(Logger::default())
                .service(
                    web::scope("/api/v1")
                        .route("/health", web::get().to(health_check))
                        .route("/build-status", web::post().to(get_build_status))
                        .route(
                            "/build-status/{commit_id}",
                            web::get().to(get_build_status_by_path),
                        ),
                )
                .route("/", web::get().to(index))
                .default_service(web::route().to(not_found))
        })
        .bind(&bind_addr)
        .map_err(|e| GarnixError::IoError(format!("Failed to bind to {}: {}", bind_addr, e)))?
        .run()
        .await
        .map_err(|e| GarnixError::ApiError(format!("Server error: {}", e)))
    }
}

/// Health check endpoint
async fn health_check() -> ActixResult<HttpResponse> {
    info!("Health check requested");
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "garnix-fetcher",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get build status via POST request
async fn get_build_status(
    client: Data<Arc<GarnixClient>>,
    request: web::Json<BuildStatusRequest>,
) -> ActixResult<HttpResponse> {
    info!("Build status requested for commit: {}", request.commit_id);

    // Validate input
    if request.jwt_token.is_empty() {
        warn!("Missing JWT token in request");
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "JWT token is required",
            "code": "MISSING_TOKEN"
        })));
    }

    if request.commit_id.is_empty() {
        warn!("Missing commit ID in request");
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Commit ID is required",
            "code": "MISSING_COMMIT_ID"
        })));
    }

    // Validate commit ID format (basic check for hex string)
    if !request.commit_id.chars().all(|c| c.is_ascii_hexdigit()) || request.commit_id.len() < 7 {
        warn!("Invalid commit ID format: {}", request.commit_id);
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid commit ID format",
            "code": "INVALID_COMMIT_ID"
        })));
    }

    match client
        .fetch_build_status(&request.jwt_token, &request.commit_id)
        .await
    {
        Ok(response) => {
            info!(
                "Successfully fetched build status for commit: {}",
                request.commit_id
            );
            Ok(HttpResponse::Ok().json(response))
        }
        Err(GarnixError::AuthenticationError(msg)) => {
            warn!("Authentication failed: {}", msg);
            Ok(HttpResponse::Unauthorized().json(json!({
                "error": msg,
                "code": "AUTHENTICATION_FAILED"
            })))
        }
        Err(GarnixError::NotFound(msg)) => {
            warn!("Resource not found: {}", msg);
            Ok(HttpResponse::NotFound().json(json!({
                "error": msg,
                "code": "NOT_FOUND"
            })))
        }
        Err(GarnixError::RateLimit(msg)) => {
            warn!("Rate limited: {}", msg);
            Ok(HttpResponse::TooManyRequests().json(json!({
                "error": msg,
                "code": "RATE_LIMITED"
            })))
        }
        Err(GarnixError::NetworkError(msg)) => {
            error!("Network error: {}", msg);
            Ok(HttpResponse::BadGateway().json(json!({
                "error": "Failed to connect to Garnix API",
                "code": "NETWORK_ERROR",
                "details": msg
            })))
        }
        Err(e) => {
            error!("Unexpected error: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            })))
        }
    }
}

/// Get build status via GET request with commit ID in path
async fn get_build_status_by_path(
    client: Data<Arc<GarnixClient>>,
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let commit_id = path.into_inner();
    info!("Build status requested for commit: {} (via GET)", commit_id);

    // Get JWT token from query parameters
    let jwt_token = match query.get("token") {
        Some(token) if !token.is_empty() => token,
        _ => {
            warn!("Missing JWT token in query parameters");
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "JWT token is required as 'token' query parameter",
                "code": "MISSING_TOKEN"
            })));
        }
    };

    // Validate commit ID format
    if !commit_id.chars().all(|c| c.is_ascii_hexdigit()) || commit_id.len() < 7 {
        warn!("Invalid commit ID format: {}", commit_id);
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid commit ID format",
            "code": "INVALID_COMMIT_ID"
        })));
    }

    // Create request object and reuse the POST handler logic
    let request = BuildStatusRequest {
        jwt_token: jwt_token.clone(),
        commit_id,
    };

    get_build_status(client, web::Json(request)).await
}

/// Root endpoint with API documentation
async fn index() -> ActixResult<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Garnix Fetcher API</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
            h1 { color: #333; }
            h2 { color: #666; border-bottom: 1px solid #ddd; padding-bottom: 10px; }
            code { background: #f4f4f4; padding: 2px 4px; border-radius: 3px; }
            pre { background: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }
            .endpoint { margin: 20px 0; }
        </style>
    </head>
    <body>
        <h1>Garnix Fetcher API</h1>
        <p>HTTP API for fetching Garnix.io CI build status</p>
        
        <h2>Endpoints</h2>
        
        <div class="endpoint">
            <h3>GET /api/v1/health</h3>
            <p>Health check endpoint</p>
            <pre>curl http://localhost:8080/api/v1/health</pre>
        </div>
        
        <div class="endpoint">
            <h3>POST /api/v1/build-status</h3>
            <p>Get build status for a commit</p>
            <pre>curl -X POST http://localhost:8080/api/v1/build-status \
  -H "Content-Type: application/json" \
  -d '{"jwt_token": "your-jwt-token", "commit_id": "abc123..."}'</pre>
        </div>
        
        <div class="endpoint">
            <h3>GET /api/v1/build-status/{commit_id}?token=jwt_token</h3>
            <p>Get build status for a commit via GET request</p>
            <pre>curl "http://localhost:8080/api/v1/build-status/abc123...?token=your-jwt-token"</pre>
        </div>
        
        <h2>Response Format</h2>
        <p>Successful responses return JSON with build summary and individual build details:</p>
        <pre>{
  "summary": {
    "repo_owner": "owner",
    "repo_name": "repo",
    "git_commit": "abc123...",
    "branch": "main",
    "succeeded": 5,
    "failed": 1,
    "pending": 0,
    "cancelled": 0
  },
  "builds": [
    {
      "id": "build-id",
      "package": "package-name",
      "status": "Success",
      "system": "x86_64-linux",
      ...
    }
  ]
}</pre>
        
        <h2>Error Responses</h2>
        <p>Errors return JSON with error message and code:</p>
        <pre>{
  "error": "Error description",
  "code": "ERROR_CODE"
}</pre>
        
        <p><strong>Version:</strong> "#.to_string() + env!("CARGO_PKG_VERSION") + r#"</p>
    </body>
    </html>
    "#;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

/// 404 handler
async fn not_found() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::NotFound().json(json!({
        "error": "Endpoint not found",
        "code": "NOT_FOUND",
        "available_endpoints": [
            "GET /",
            "GET /api/v1/health",
            "POST /api/v1/build-status",
            "GET /api/v1/build-status/{commit_id}"
        ]
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    async fn create_test_app() {
        // Simple test app creation helper
        let client = Arc::new(GarnixClient::new());
        let _app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .service(
                    web::scope("/api/v1")
                        .route("/health", web::get().to(health_check))
                        .route("/build-status", web::post().to(get_build_status))
                        .route(
                            "/build-status/{commit_id}",
                            web::get().to(get_build_status_by_path),
                        ),
                )
                .route("/", web::get().to(index))
                .default_service(web::route().to(not_found)),
        )
        .await;
    }

    #[actix_web::test]
    async fn test_health_check() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .route("/health", web::get().to(health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "garnix-fetcher");
    }

    #[actix_web::test]
    async fn test_index_endpoint() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .route("/", web::get().to(index)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_not_found() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .default_service(web::route().to(not_found)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nonexistent").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], "NOT_FOUND");
    }

    #[actix_web::test]
    async fn test_build_status_missing_token() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .route("/build-status", web::post().to(get_build_status)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/build-status")
            .set_json(json!({
                "jwt_token": "",
                "commit_id": "abc123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], "MISSING_TOKEN");
    }

    #[actix_web::test]
    async fn test_build_status_missing_commit_id() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .route("/build-status", web::post().to(get_build_status)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/build-status")
            .set_json(json!({
                "jwt_token": "test-token",
                "commit_id": ""
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], "MISSING_COMMIT_ID");
    }

    #[actix_web::test]
    async fn test_build_status_invalid_commit_id() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(
            App::new()
                .app_data(Data::new(client))
                .route("/build-status", web::post().to(get_build_status)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/build-status")
            .set_json(json!({
                "jwt_token": "test-token",
                "commit_id": "invalid-commit"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], "INVALID_COMMIT_ID");
    }

    #[actix_web::test]
    async fn test_build_status_by_path_missing_token() {
        let client = Arc::new(GarnixClient::new());
        let app = test::init_service(App::new().app_data(Data::new(client)).route(
            "/build-status/{commit_id}",
            web::get().to(get_build_status_by_path),
        ))
        .await;

        let req = test::TestRequest::get()
            .uri("/build-status/abc123456789")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["code"], "MISSING_TOKEN");
    }

    #[tokio::test]
    async fn test_server_creation() {
        let server = GarnixHttpServer::new();
        assert_eq!(server.bind_address, "127.0.0.1");
        assert_eq!(server.port, 8080);

        let server = GarnixHttpServer::new().bind_address("0.0.0.0").port(3000);
        assert_eq!(server.bind_address, "0.0.0.0");
        assert_eq!(server.port, 3000);
    }

    #[tokio::test]
    async fn test_server_with_client() {
        let client = GarnixClient::new();
        let server = GarnixHttpServer::with_client(client);
        assert_eq!(server.bind_address, "127.0.0.1");
        assert_eq!(server.port, 8080);
    }
}
