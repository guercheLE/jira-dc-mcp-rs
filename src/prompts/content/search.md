# Guided workflow: search

Covers finding issues with JQL, and working with saved filters and
search-result-size limits. Describe every operation as a capability to
search for (e.g. "search for how to search issues using JQL"), never a
specific `operationId` -- read the schema `get` returns before relying on
any field name in it.

Ask the user for their intent in plain terms and translate it into a JQL
query yourself rather than asking them to write JQL, unless they've already
given you one. If a saved filter already covers the need, prefer reusing or
referencing it over rebuilding the same query ad hoc.

**A JQL search can return far more issues than are useful to pull into this
conversation.** If your environment provides a way to run a sub-task in its
own context, delegate the search and have it return only the distilled
result the caller actually needs (a count, the matching issue keys, or the
few fields that matter) rather than the full result set. If no such
mechanism is available, apply `maxResults`/paging yourself and summarize
rather than dumping every returned issue.

To act on an issue once found (comment, transition, link, etc.), hand off to
`jira_workflow_issues` or `jira_workflow_issue_collaboration` with the
resolved `issue_key` rather than duplicating that logic here.
