//! tts-web-server - Static file server for TTS web UI.

use anyhow::Result;

mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port = parse_port(&args);
    let static_dir = parse_static_dir(&args);

    println!("Starting TTS web server on http://localhost:{port}");
    println!("Serving static files from: {static_dir}");

    server::run(port, &static_dir).await
}

fn parse_port(args: &[String]) -> u16 {
    args.iter()
        .position(|a| a == "--port" || a == "-p")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

fn parse_static_dir(args: &[String]) -> String {
    args.iter()
        .position(|a| a == "--static-dir" || a == "-d")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "./dist".to_string())
}
