# Guided workflow: project setup

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if you were delegated here from `jira_workflow`'s routing, or
your environment otherwise supports running this as its own sub-task,
everything you need is in this prompt's own text plus the parameters
already listed above; report back only a short summary when done rather
than the full step-by-step trace.

Every operation below is described as a capability to search for, never as
a specific `operationId` -- the exact operation and its response schema can
differ across the supported Jira Data Center API versions. Call `search`
with the described capability, `get` the operationId it resolves to, and
read that operation's *current* schema before relying on any field name in
it.

## Step 0 -- gather required parameters

Check the "Context already provided" header above first. Only ask the user
for what's still listed as missing:

- Creating a new project needs `project_key`, `project_name`, and
  `project_type`.
- Configuring an existing project needs only `project_key`.

## Step 1 -- fork: new project or existing project?

Ask (if not already obvious from context): "Are you setting up a brand new
project, or configuring the scheme associations of one that already
exists?"

**(A) Creating a new project.** Search for and call the operation that
lists the available project types, and confirm the type with the user
before creating -- a project's type shapes which features (e.g. boards,
service-desk queues) are available to it and isn't something to change
casually afterward. Then search for and call the operation to create the
project with the confirmed key, name, and type. Gate: don't proceed to Step
2 until a follow-up read confirms the project exists.

**(B) Configuring an existing project.** Skip straight to Step 2 with the
known `project_key`.

## Step 2 -- read current scheme associations before changing anything

Search for and call the operations that read the project's *current*
workflow scheme, permission scheme, notification scheme, priority scheme,
issue security scheme, and issue type scheme association. Don't assume
defaults -- a newly created project's default associations vary by project
type and version, and an existing project may already have deliberate,
non-default associations that shouldn't be silently overwritten.

## Step 3 -- set the scheme associations the user wants (parallelizable, delegable)

Once the project exists, assigning each scheme is independent of the
others -- workflow scheme, permission scheme, notification scheme, priority
scheme, issue security scheme, and issue type scheme project association
don't depend on each other. Call this out to the user as safe to do
concurrently, and: if your environment provides a way to run a sub-task in
its own context, delegate each independent assignment and have it return
only a short confirmation -- don't pull the full request/response bodies
into this conversation. If no such sub-task mechanism is available, just
make the calls directly, one after another.

For the full detail on any one scheme -- what it controls, and any gotchas
specific to editing it (e.g. the workflow-scheme draft/publish model) --
fetch the prompt that owns that domain rather than duplicating it here:
`jira_workflow_workflows_statuses` (workflow scheme),
`jira_workflow_permissions_security` (permission/notification/security
schemes), `jira_workflow_issue_types_fields` (issue type scheme
association).

## Step 4 -- verify

For each scheme assignment made in Step 3, confirm it took effect with a
follow-up read (the same operations used in Step 2) rather than trusting a
non-error response alone.

## Step 5 -- summarize

Report what was created and/or associated, in plain terms: the project key
and type (if newly created), and which schemes were confirmed assigned.

## Composing with other workflows

- Adding issue types or custom fields beyond the scheme association itself
  overlaps with `jira_workflow_issue_types_fields`.
- Assigning project roles and their members overlaps with
  `jira_workflow_permissions_security`.
- Creating the project's first issues, versions, or components overlaps
  with `jira_workflow_issues` and `jira_workflow_projects`.

Fetch those prompts by name for more detail rather than assuming their
content from this one.
