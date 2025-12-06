//! Command handlers for CLI operations.

use crate::cli::Cli;
use crate::config::load_config;
use anyhow::{Result, bail};
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
};
use std::path::Path;
use std::sync::Arc;

/// Shared state for the web server.
#[derive(Clone)]
struct AppState {
    backend_url: String,
    client: reqwest::Client,
}

/// Main entry point for CLI execution.
pub async fn run(cli: Cli) -> Result<()> {
    let config = load_config(cli.config_file.as_deref())?;

    if cli.web_ui {
        let default_url = config
            .backends
            .get("alltalk")
            .map(|b| b.url())
            .unwrap_or_else(|| "http://localhost:5157".into());
        let backend_url = cli.backend_url.unwrap_or(default_url);
        return serve_web_ui(&cli.bind, &backend_url).await;
    }

    if let Some(script_path) = &cli.script_file {
        return render_script(&config, script_path, cli.output_file.as_deref()).await;
    }

    println!("No action specified. Use --help for usage information.");
    Ok(())
}

async fn serve_web_ui(bind: &str, backend_url: &str) -> Result<()> {
    use tower_http::services::ServeDir;

    let dist_dir = find_dist_dir()?;
    let state = Arc::new(AppState {
        backend_url: backend_url.trim_end_matches('/').to_string(),
        client: reqwest::Client::new(),
    });

    println!("Serving web UI from: {}", dist_dir.display());
    println!("Backend URL: {}", state.backend_url);
    println!("Open http://{bind} in your browser");

    let app = Router::new()
        .route("/api/{*path}", any(proxy_handler))
        .with_state(state)
        .fallback_service(ServeDir::new(&dist_dir));

    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
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

async fn render_script(
    config: &crate::config::Config,
    script_path: &Path,
    output: Option<&Path>,
) -> Result<()> {
    let output_path = output.unwrap_or_else(|| Path::new("output.wav"));
    println!("Loading script: {}", script_path.display());
    let script = load_script(script_path)?;
    println!("Validating script...");
    tts_spec_script::validate::validate(&script)?;
    println!(
        "Rendering {} segments to {}",
        script.segments.len(),
        output_path.display()
    );
    let engines = config.all_engines();
    println!("Available engines: {:?}", engines);
    if let Some(url) = config.url_for_engine(&config.default_engine) {
        println!("Default engine '{}' at: {}", config.default_engine, url);
    }
    println!("Rendering not yet implemented - waiting for tts-engine component");
    Ok(())
}

fn load_script(path: &Path) -> Result<tts_spec_model::Script> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let file = std::fs::File::open(path)?;
    match ext {
        "yaml" | "yml" => Ok(tts_spec_script::parse::from_yaml_reader(file)?),
        "json" => Ok(tts_spec_script::parse::from_json_reader(file)?),
        _ => bail!("unsupported file extension: {ext} (use .yaml or .json)"),
    }
}
