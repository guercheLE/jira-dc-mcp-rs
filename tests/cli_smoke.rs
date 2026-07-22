use std::process::{Command, Output};

fn generated_server(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp"))
        .args(args)
        .env("JIRA_DC_MCP_URL", "http://127.0.0.1")
        .env("JIRA_DC_MCP_AUTH_METHOD", "basic")
        .env_remove("JIRA_DC_MCP_API_VERSION")
        .env_remove("JIRA_DC_MCP_TRANSPORT")
        // Isolates the config cascade's global-config layer
        // (`~/.jira-dc-mcp/config.yml`, resolved via
        // `core::credential_storage::resolve_home_dir`'s `HOME`/`USERPROFILE`
        // lookup) from whatever this machine's real user has configured —
        // without this, a developer who has actually run `setup` for real
        // use gets spurious failures here from their own settings (e.g. a
        // stale `api_version` predating a version-labeling change),
        // depending on the machine `cargo test` happens to run on rather
        // than on this crate's own code.
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .output()
        .expect("generated server should run")
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).expect("stdout should be UTF-8")
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).expect("stderr should be UTF-8")
}

#[test]
fn version_help_versions_and_config_are_available() {
    let version = generated_server(&["version"]);
    assert!(version.status.success(), "{}", stderr(&version));
    assert_eq!(stdout(&version), concat!(env!("CARGO_PKG_VERSION"), "\n"));

    let help = generated_server(&["--help"]);
    assert!(help.status.success(), "{}", stderr(&help));
    assert!(stdout(&help).contains("Semantic search for operations"));

    let versions = generated_server(&["versions"]);
    assert!(versions.status.success(), "{}", stderr(&versions));
    assert!(stdout(&versions).contains("11.3"));

    let config = generated_server(&["config"]);
    assert!(config.status.success(), "{}", stderr(&config));
    let config: serde_json::Value = serde_json::from_slice(&config.stdout).unwrap();
    assert_eq!(config["api_version"], "11.3");
    assert_eq!(config["transport"], "stdio");
}

#[test]
fn search_and_get_exercise_the_embedded_operation_catalog() {
    let search = generated_server(&["search", "find an operation", "--limit", "2"]);
    assert!(search.status.success(), "{}", stderr(&search));
    let results: serde_json::Value = serde_json::from_slice(&search.stdout).unwrap();
    assert!(!results.as_array().unwrap().is_empty());

    let get = generated_server(&["get", "moveIssuesToBacklog"]);
    assert!(get.status.success(), "{}", stderr(&get));
    let operation: serde_json::Value = serde_json::from_slice(&get.stdout).unwrap();
    assert_eq!(operation["operation_id"], "moveIssuesToBacklog");
}

#[test]
fn invalid_commands_operations_and_arguments_fail_without_network_calls() {
    let unknown_command = generated_server(&["definitely-unknown-command"]);
    assert_eq!(unknown_command.status.code(), Some(2));

    let unknown_get = generated_server(&["get", "definitely-unknown-operation"]);
    assert!(!unknown_get.status.success());
    assert!(stderr(&unknown_get).contains("unknown operationId"));

    let invalid_json = generated_server(&["call", "moveIssuesToBacklog", "--args", "not-json"]);
    assert!(!invalid_json.status.success());

    let unknown_call = generated_server(&["call", "definitely-unknown-operation", "--args", "{}"]);
    assert!(!unknown_call.status.success());
    assert!(stderr(&unknown_call).contains("unknown operationId"));
}

#[test]
fn profiling_workload_controls_are_hidden_and_reject_zero_iterations() {
    let help = generated_server(&["search", "--help"]);
    assert!(help.status.success(), "{}", stderr(&help));
    assert!(!stdout(&help).contains("profile-warmups"));
    assert!(!stdout(&help).contains("profile-iterations"));

    let invalid = generated_server(&["search", "test query", "--profile-iterations", "0"]);
    assert!(!invalid.status.success());
    assert!(stderr(&invalid).contains("--profile-iterations must be at least 1"));

    let profile_run = generated_server(&[
        "search",
        "test query",
        "--profile-warmups",
        "1",
        "--profile-iterations",
        "2",
    ]);
    assert!(profile_run.status.success(), "{}", stderr(&profile_run));
    assert!(stderr(&profile_run).contains("1 warmup(s), 2 measured iteration(s)"));
}

#[cfg(feature = "profiling")]
#[test]
fn profiling_feature_records_warmed_search_and_non_search_commands() {
    fn heap_profile() -> serde_json::Value {
        let contents = std::fs::read_to_string("dhat-heap.json")
            .expect("profiling command should write dhat-heap.json");
        serde_json::from_str(&contents).expect("dhat-heap.json should contain valid JSON")
    }

    let search = generated_server(&[
        "search",
        "test query",
        "--profile-warmups",
        "1",
        "--profile-iterations",
        "2",
    ]);
    assert!(search.status.success(), "{}", stderr(&search));
    assert!(stderr(&search).contains("1 warmup(s), 2 measured iteration(s)"));
    let search_profile = heap_profile();
    assert_eq!(search_profile["dhatFileVersion"], 2);
    assert_eq!(search_profile["mode"], "rust-heap");
    assert!(
        search_profile["cmd"]
            .as_str()
            .unwrap()
            .contains("search test query")
    );
    assert!(!search_profile["pps"].as_array().unwrap().is_empty());

    let version = generated_server(&["version"]);
    assert!(version.status.success(), "{}", stderr(&version));
    assert_eq!(stdout(&version), concat!(env!("CARGO_PKG_VERSION"), "\n"));
    let version_profile = heap_profile();
    assert_eq!(version_profile["dhatFileVersion"], 2);
    assert!(
        version_profile["cmd"]
            .as_str()
            .unwrap()
            .ends_with(" version")
    );
}

#[test]
fn stdio_and_helper_binaries_cover_their_bootstrap_and_error_paths() {
    // Skipped only on GitHub Actions' Windows runners: `Command::output()`
    // is documented to give the child an immediately-EOF-closed stdin, and
    // that reliably makes rmcp's stdio transport fail fast with
    // "connection closed: initialize request" everywhere this has been
    // observed to work. On those specific runners, tokio's stdin reader (a
    // background blocking thread — Windows has no IOCP-style readiness
    // notification for console/pipe handles the way Unix does) doesn't
    // observe that same immediate closure, so `generated_server(&["start"])`
    // blocks indefinitely instead of exiting — a multi-hour CI hang, not a
    // slow test. Left enabled on a real Windows machine (e.g. a developer
    // running the suite locally), where this hasn't been confirmed to
    // reproduce.
    let skip_on_windows_ci = cfg!(windows) && std::env::var_os("GITHUB_ACTIONS").is_some();
    if !skip_on_windows_ci {
        let stdio = generated_server(&["start"]);
        assert!(!stdio.status.success());
        #[cfg(unix)]
        assert!(stderr(&stdio).contains("connection closed: initialize request"));
    }

    let healthy = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp-healthcheck"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(healthy.status.success(), "{}", stderr(&healthy));

    let empty = tempfile::tempdir().unwrap();
    let unhealthy = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp-healthcheck"))
        .current_dir(empty.path())
        .output()
        .unwrap();
    assert!(!unhealthy.status.success());

    let populate = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp-populate-embeddings"))
        .arg("definitely-missing-store.db")
        .output()
        .unwrap();
    assert!(!populate.status.success());

    // Only the `.zst` sibling is committed at the project root (see
    // .gitignore) — copying that and pointing populate-embeddings at the
    // bare `mcp_store.db` path exercises the same `ensure_raw_db`
    // decompress-on-demand path a real `cargo install`/CI run relies on.
    let store_copy = empty.path().join("mcp_store.db");
    let store_copy_zst = empty.path().join("mcp_store.db.zst");
    std::fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("mcp_store.db.zst"),
        &store_copy_zst,
    )
    .unwrap();
    let populated = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp-populate-embeddings"))
        .arg(&store_copy)
        .output()
        .unwrap();
    assert!(populated.status.success(), "{}", stderr(&populated));

    // Same GitHub-Actions-Windows-only skip as the stdio check above, and
    // for a related reason: `setup` drives `inquire`'s interactive
    // Text/Select/Password prompts, and `inquire`'s Windows backend reads
    // raw console input rather than treating a closed/non-console stdin as
    // an immediate EOF the way Unix does -- so this call blocks
    // indefinitely there too, instead of failing fast once the first
    // prompt has nothing to read.
    if !skip_on_windows_ci {
        let setup = generated_server(&["setup"]);
        assert!(!setup.status.success());
    }

    let test_connection = generated_server(&["test-connection"]);
    assert!(!test_connection.status.success());

    let http = generated_server(&[
        "http",
        "--host",
        "not a valid socket address",
        "--port",
        "3000",
        "--cors-allow",
        "https://client.example",
    ]);
    assert!(!http.status.success());
}

#[cfg(unix)]
#[test]
fn http_command_serves_health_and_shuts_down_cleanly() {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let mut child = Command::new(env!("CARGO_BIN_EXE_jira-dc-mcp"))
        .args([
            "http",
            "--host",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            "--cors-allow",
            "https://client.example",
        ])
        .env("JIRA_DC_MCP_URL", "http://127.0.0.1")
        .env("JIRA_DC_MCP_AUTH_METHOD", "basic")
        // See generated_server()'s identical HOME override above.
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(10);
    let mut response = String::new();
    while Instant::now() < deadline {
        if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) {
            stream
                .set_read_timeout(Some(Duration::from_secs(1)))
                .unwrap();
            stream
                .write_all(b"GET /healthz HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
                .unwrap();
            if let Err(err) = stream.read_to_string(&mut response)
                && !matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                )
            {
                panic!("failed to read HTTP health response: {err}");
            }
            if response.contains(" 200 OK") {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    let signal = Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(signal.success());
    let status = child.wait().unwrap();
    let mut server_stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut server_stderr)
        .unwrap();

    assert!(status.success(), "{server_stderr}");
    assert!(response.contains(" 200 OK"), "{response}\n{server_stderr}");
    assert!(response.contains("access-control-allow-origin: https://client.example"));
}
