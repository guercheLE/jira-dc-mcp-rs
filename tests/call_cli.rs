use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::time::Duration;

#[test]
fn call_command_dispatches_a_catalog_operation_and_prints_the_response() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();

        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let read = stream.read(&mut buffer).unwrap();
            assert_ne!(
                read, 0,
                "CLI closed the connection before sending a request"
            );
            request.extend_from_slice(&buffer[..read]);

            let Some(headers_end) = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .map(|index| index + 4)
            else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            if request.len() >= headers_end + content_length {
                break;
            }
        }

        let body = r#"{"accepted":true}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        String::from_utf8(request).unwrap()
    });

    let home = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp"))
        .args([
            "call",
            "moveIssuesToBacklog",
            "--args",
            r#"{"body":{"issues":["COVERAGE-1"]}}"#,
        ])
        .env("JIRA_DC_MCP_URL", format!("http://{address}"))
        .env("JIRA_DC_MCP_AUTH_METHOD", "basic")
        .env("JIRA_DC_MCP_USERNAME", "coverage-user")
        .env("JIRA_DC_MCP_PASSWORD", "coverage-password")
        .env("JIRA_DC_MCP_API_VERSION", "11.3")
        .env("JIRA_DC_MCP_TRANSPORT", "stdio")
        .env("JIRA_DC_MCP_RETRY_ATTEMPTS", "0")
        .env("JIRA_DC_MCP_TIMEOUT_MS", "2000")
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
        serde_json::json!({"accepted": true})
    );

    let request = server.join().unwrap();
    assert!(request.starts_with("POST /agile/1.0/backlog/issue HTTP/1.1\r\n"));
    let authorization = request.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("authorization")
            .then_some(value.trim())
    });
    assert_eq!(
        authorization,
        Some("Basic Y292ZXJhZ2UtdXNlcjpjb3ZlcmFnZS1wYXNzd29yZA==")
    );
    assert!(request.ends_with(r#"{"issues":["COVERAGE-1"]}"#));
}
