# Guided workflow: projects

Covers project lifecycle (create/read/update/delete), project categories,
components, and versions (releases). Describe every operation as a
capability to search for -- e.g. "search for how to create a component in a
project" -- never a specific `operationId`, and read the schema `get`
returns before relying on any field name in it.

Ask for the project key up front if the user hasn't given one; most
component/version operations are scoped to a single project and need it.

**Real gotcha:** deleting a version or component that issues still reference
usually requires deciding what happens to those issues (leave the field
empty, or move them to a replacement version/component) -- search for the
delete operation's parameters and surface that choice to the user rather
than deleting silently.

For creating and managing issue types, issue type schemes, and custom
fields available to a project, see `jira_workflow_issue_types_fields`. For
permission/security scheme assignment, see
`jira_workflow_permissions_security`.
