# Jira Data Center workflow menu

Match the user's goal to one of the guided sub-workflows below, then fetch
that prompt by name (`prompts/get`) for step-by-step instructions.

**Delegate whole sub-workflows when you can.** If your environment provides a
way to run a sub-task/agent in an isolated context, delegate the entire
matched sub-workflow to it: hand the sub-task the sub-workflow's prompt name
and whatever parameters you already know, let it fetch that prompt itself and
carry out every step -- including its own `search`/`get`/`call` traffic --
entirely in its own context, and have it report back only a short summary
(what was accomplished/confirmed, and anything still needed from the user).
Only run a sub-workflow's steps directly in this conversation if no such
delegation mechanism is available.

## Sub-workflows

- **`jira-issues`** -- create an issue, transition an existing issue,
  or both.
- **`jira-issue-collaboration`** -- comments, worklogs, attachments,
  issue links, watchers, and votes on an existing issue.
- **`jira-search`** -- find issues with JQL, or work with saved
  filters.
- **`jira-project-setup`** -- bootstrap a new project (or configure
  an existing one), wiring up its workflow, permission, notification,
  priority, and issue security scheme associations.
- **`jira-projects`** -- project lifecycle, categories, components,
  and versions.
- **`jira-agile`** -- boards, sprints, epics, and backlog
  management.
- **`jira-workflows-statuses`** -- workflows, workflow schemes,
  statuses, priorities, and resolutions.
- **`jira-issue-types-fields`** -- issue types, issue type schemes,
  fields, custom fields, and screens.
- **`jira-permissions-security`** -- permission schemes, issue
  security, project roles, and notification schemes.
- **`jira-users-groups`** -- users, groups, application roles, and
  account/session settings.
- **`jira-admin-monitoring`** -- cluster status, monitoring,
  reindexing, and other read-only/admin signals.

If the user's goal doesn't clearly match a sub-workflow above, ask a short
clarifying question rather than guessing which one they mean.
