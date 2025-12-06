//! Web server and API handlers.

use crate::config::{Config, backend_url, to_engines_response};
use anyhow::{Result, bail};
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use std::sync::Arc;

/// Shared state for the web server.
#[derive(Clone)]
struct AppState {
    backend_url: String,
    config: Arc<Config>,
    client: reqwest::Client,
}

/// Serve the web UI with API proxy.
pub async fn serve_web_ui(bind: &str, config: &Config) -> Result<()> {
    use tower_http::services::ServeDir;

    let dist_dir = find_dist_dir()?;
    let default_url = config
        .backends
        .get("alltalk")
        .map(backend_url)
        .unwrap_or_else(|| "http://localhost:5157".into());

    let state = Arc::new(AppState {
        backend_url: default_url.trim_end_matches('/').to_string(),
        config: Arc::new(config.clone()),
        client: reqwest::Client::new(),
    });

    println!("Serving web UI from: {}", dist_dir.display());
    println!("Backend URL: {}", state.backend_url);
    println!("Open http://{bind} in your browser");

    let app = Router::new()
        .route("/api/engines", get(engines_handler))
        .route("/api/{*path}", any(proxy_handler))
        .with_state(state)
        .fallback_service(ServeDir::new(&dist_dir));

    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Handler for /api/engines - returns engine metadata from config.
async fn engines_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = to_engines_response(&state.config);
    Json(response)
}

/// Proxy handler for /api/* requests to the backend.
async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> impl IntoResponse {
    let path = req.uri().path();
    let query = req
        .uri()
        .query()
        .map(|q| format!("?{}", q))
        .unwrap_or_default();
    let url = format!("{}{}{}", state.backend_url, path, query);

    let method = req.method().clone();
    let headers = req.headers().clone();

    let body_bytes = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Body error: {}", e)).into_response(),
    };

    let mut builder = state.client.request(method, &url);
    for (name, value) in headers.iter() {
        if name != "host" {
            builder = builder.header(name, value);
        }
    }
    builder = builder.body(body_bytes.to_vec());

    match builder.send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::OK);
            let mut response = Response::builder().status(status);
            for (name, value) in resp.headers() {
                response = response.header(name, value);
            }
            match resp.bytes().await {
                Ok(body) => response.body(Body::from(body)).unwrap().into_response(),
                Err(e) => (StatusCode::BAD_GATEWAY, format!("Read error: {}", e)).into_response(),
            }
        }
        Err(e) => (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)).into_response(),
    }
}

fn find_dist_dir() -> Result<std::path::PathBuf> {
    let candidates = [
        std::env::current_exe()?
            .parent()
            .unwrap()
            .join("../../../components/tts-web/crates/tts-web-ui/dist"),
        std::path::PathBuf::from("components/tts-web/crates/tts-web-ui/dist"),
        std::path::PathBuf::from("../tts-web/crates/tts-web-ui/dist"),
    ];
    for p in &candidates {
        if p.exists() {
            return Ok(p.canonicalize()?);
        }
    }
    bail!("Web UI dist not found. Run: ./scripts/build-all.sh")
}
