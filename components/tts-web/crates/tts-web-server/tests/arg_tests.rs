//! Argument parsing tests.

#[test]
fn test_default_port() {
    // Default port should be 8080 when not specified
    let args: Vec<String> = vec!["tts-web-server".into()];
    let port = parse_port(&args);
    assert_eq!(port, 8080);
}

#[test]
fn test_custom_port() {
    let args: Vec<String> = vec!["tts-web-server".into(), "--port".into(), "3000".into()];
    let port = parse_port(&args);
    assert_eq!(port, 3000);
}

#[test]
fn test_default_static_dir() {
    let args: Vec<String> = vec!["tts-web-server".into()];
    let dir = parse_static_dir(&args);
    assert_eq!(dir, "./dist");
}

#[test]
fn test_custom_static_dir() {
    let args: Vec<String> = vec![
        "tts-web-server".into(),
        "--static-dir".into(),
        "/var/www".into(),
    ];
    let dir = parse_static_dir(&args);
    assert_eq!(dir, "/var/www");
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
