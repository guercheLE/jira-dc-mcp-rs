//! `#[prompt_router]`-decorated `impl McpifyServer` block. Kept separate
//! from `core::mcp_server`'s `#[tool_router]` block -- see
//! `docs/mcp-prompts-workflow-plan.md`.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{PromptMessage, Role};
use rmcp::{prompt, prompt_router};

use crate::core::mcp_server::McpifyServer;
use crate::prompts::{IssuesWorkflowArgs, MasterWorkflowArgs, render_context_header};

#[prompt_router(vis = "pub")]
impl McpifyServer {
    #[prompt(
        name = "jira_workflow",
        description = "Start here. Presents the available Jira Data Center management \
                        workflows, routes to the right guided sub-workflow based on the \
                        user's goal, and -- where the environment supports it -- delegates \
                        that whole sub-workflow to an isolated sub-task to spare this \
                        conversation's context window."
    )]
    async fn jira_workflow_prompt(
        &self,
        Parameters(args): Parameters<MasterWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("goal", args.goal.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n{}", include_str!("content/master.md")),
        )]
    }

    #[prompt(
        name = "jira_workflow_issues",
        description = "Guided issue lifecycle: discover the fields required to create an \
                        issue, look up and apply a valid transition on an existing issue, \
                        and verify the resulting state -- rather than guessing at required \
                        fields or transition ids."
    )]
    async fn jira_workflow_issues_prompt(
        &self,
        Parameters(args): Parameters<IssuesWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("project_key", args.project_key.as_deref()),
            ("issue_type", args.issue_type.as_deref()),
            ("summary", args.summary.as_deref()),
            ("issue_key", args.issue_key.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n{}", include_str!("content/issues.md")),
        )]
    }

    #[prompt(
        name = "jira_workflow_issue_collaboration",
        description = "Comments, worklogs, attachments, issue links/remote links, watchers, \
                        and votes on an existing issue."
    )]
    async fn jira_workflow_issue_collaboration_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/issue_collaboration.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_search",
        description = "JQL search over issues, saved filters, and search-result-size limits."
    )]
    async fn jira_workflow_search_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/search.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_projects",
        description = "Project lifecycle, project categories, components, and versions."
    )]
    async fn jira_workflow_projects_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/projects.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_agile",
        description = "Jira Software boards, sprints, epics, and backlog management."
    )]
    async fn jira_workflow_agile_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/agile.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_workflows_statuses",
        description = "Workflows and workflow schemes -- including the draft-then-publish \
                        two-phase edit model -- statuses/status categories, \
                        priorities/priority schemes, and resolutions."
    )]
    async fn jira_workflow_workflows_statuses_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/workflows_statuses.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_issue_types_fields",
        description = "Issue types, issue type schemes and their project/issue-type \
                        associations, fields, custom fields, and screens/screen tabs."
    )]
    async fn jira_workflow_issue_types_fields_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/issue_types_fields.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_permissions_security",
        description = "Permission schemes, issue security schemes/levels, project roles and \
                        role actors, and notification schemes."
    )]
    async fn jira_workflow_permissions_security_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/permissions_security.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_users_groups",
        description = "User and group lifecycle, application roles, `myself`/preferences, \
                        session/websudo auth, and password policy."
    )]
    async fn jira_workflow_users_groups_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/users_groups.md"),
        )]
    }

    #[prompt(
        name = "jira_workflow_admin_monitoring",
        description = "Thin pointer to the right read-only or admin signal: cluster status, \
                        monitoring, reindex/index snapshots, server info, application \
                        properties, and dashboards."
    )]
    async fn jira_workflow_admin_monitoring_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/admin_monitoring.md"),
        )]
    }
}
