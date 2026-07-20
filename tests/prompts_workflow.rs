//! Protocol-level `prompts/list`/`prompts/get` tests, kept out of
//! `src/core/mcp_server.rs`'s tool-focused `#[cfg(test)] mod tests` --
//! mirrors that module's `mcp_protocol_routes_search_get_and_call_requests`
//! duplex-transport pattern. See `docs/mcp-prompts-workflow-plan.md`.

use std::sync::Arc;

use jira_dc_mcp::auth::auth_manager::AuthManager;
use jira_dc_mcp::core::config_schema::{AuthMethod, Config};
use jira_dc_mcp::core::mcp_server::McpifyServer;
use rmcp::ServiceExt;
use rmcp::model::GetPromptRequestParams;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "basic"
    }))
    .unwrap();
    McpifyServer::new(
        "11.3".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::Basic))),
    )
}

fn prompt_text(result: &rmcp::model::GetPromptResult) -> &str {
    &result.messages[0]
        .content
        .as_text()
        .expect("prompt message should be text content")
        .text
}

#[tokio::test]
async fn server_info_advertises_the_prompts_capability() {
    use rmcp::ServerHandler;
    let info = server().get_info();
    assert!(info.capabilities.prompts.is_some());
}

#[tokio::test]
async fn prompts_list_and_get_round_trip_over_the_mcp_protocol() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let prompts = client.list_all_prompts().await.unwrap();
    let names: Vec<&str> = prompts.iter().map(|p| p.name.as_ref()).collect();
    assert_eq!(
        names.len(),
        11,
        "expected exactly 11 prompts, got {names:?}"
    );
    for expected in [
        "jira_workflow",
        "jira_workflow_issues",
        "jira_workflow_issue_collaboration",
        "jira_workflow_search",
        "jira_workflow_projects",
        "jira_workflow_agile",
        "jira_workflow_workflows_statuses",
        "jira_workflow_issue_types_fields",
        "jira_workflow_permissions_security",
        "jira_workflow_users_groups",
        "jira_workflow_admin_monitoring",
    ] {
        assert!(names.contains(&expected), "missing prompt {expected}");
    }
    assert!(names.iter().all(|name| name.starts_with("jira_workflow")));

    let issues_prompt = prompts
        .iter()
        .find(|p| p.name == "jira_workflow_issues")
        .expect("jira_workflow_issues should be advertised");
    let arg_names: Vec<&str> = issues_prompt
        .arguments
        .as_ref()
        .expect("jira_workflow_issues should advertise arguments")
        .iter()
        .map(|a| a.name.as_str())
        .collect();
    for expected in ["project_key", "issue_type", "summary", "issue_key"] {
        assert!(arg_names.contains(&expected), "missing argument {expected}");
    }
    assert!(
        issues_prompt
            .arguments
            .as_ref()
            .unwrap()
            .iter()
            .all(|a| a.required == Some(false)),
        "every jira_workflow_issues argument should be optional"
    );

    // `jira_workflow` with no arguments should link to `jira_workflow_issues`.
    let master = client
        .get_prompt(GetPromptRequestParams::new("jira_workflow"))
        .await
        .unwrap();
    assert!(prompt_text(&master).contains("jira_workflow_issues"));

    // `jira_workflow_issues` with partial arguments should echo the
    // supplied values and list the still-missing ones.
    let mut partial_args = serde_json::Map::new();
    partial_args.insert("project_key".to_string(), serde_json::json!("PROJ"));
    partial_args.insert("issue_type".to_string(), serde_json::json!("Bug"));
    let issues = client
        .get_prompt(
            GetPromptRequestParams::new("jira_workflow_issues").with_arguments(partial_args),
        )
        .await
        .unwrap();
    let issues_text = prompt_text(&issues);
    assert!(issues_text.contains("project_key: PROJ"));
    assert!(issues_text.contains("issue_type: Bug"));
    assert!(issues_text.contains("- summary"));
    assert!(issues_text.contains("- issue_key"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}
