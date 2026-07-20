# Guided workflow: workflows & statuses

This sub-workflow is designed to be run as an isolated sub-task where
possible -- everything you need is in this prompt's own text; report back
only a short summary when done rather than the full step-by-step trace.

Covers workflows, workflow schemes (including their project/issue-type
associations), statuses and status categories, priorities and priority
schemes, and resolutions. As always, describe every operation as a
capability to search for -- e.g. "search for how to get the workflow scheme
for a project" -- never a specific `operationId`, and read the schema `get`
returns before relying on any field name in it.

## Step 0 -- gather required parameters

Ask for the project key (and, for direct workflow-scheme edits, the scheme
id) if not already known. Most operations here are scoped to one project's
workflow scheme.

## Step 1 -- read before you write

Before changing anything, search for and call the operation that gets the
current workflow scheme for the project, and note which workflow is mapped
to which issue type. Changes described below are relative to this current
state, not assumed defaults.

## Step 2 -- the draft/publish gotcha (gated)

**Editing a live workflow scheme's mappings does not take effect
immediately.** Search for how workflow scheme edits work for this version:
typically, the first edit to an active scheme creates a *draft* copy: all
further mapping changes (issue-type-to-workflow associations, adding or
removing a workflow) apply to that draft, not to the live scheme, until the
draft is explicitly published. Concretely:

1. Make the mapping changes the user asked for (search for and call the
   scheme/draft update operations).
2. Confirm you're editing the draft, not assuming the live scheme changed --
   search for and call the operation that reads the draft back, and check
   it reflects what you just set.
3. Only if the user wants the change to take effect for real issues now:
   search for and call the publish operation, and gate on that call
   succeeding -- publishing can fail if there are unresolved status
   mappings for issues already using a workflow being replaced, in which
   case Jira needs the user to choose a status mapping first.
4. If the user only wanted to stage the change, stop after step 2 and tell
   them the draft exists but hasn't been published.

Never report a workflow scheme change as "done" without being explicit
about whether it's published or still a draft -- the two states look
similar in a raw response but mean very different things operationally.

## Statuses, priorities, resolutions

These are simpler, mostly-global CRUD resources (not always project-scoped)
-- search for and call the matching create/list/update/delete operation
directly, without the draft/publish gating above, which is specific to
workflow schemes.

## Composing with other workflows

Assigning issue security or permission schemes to a project is a separate
concern -- see `jira_workflow_permissions_security`. Issue type scheme
associations (which issue types exist for a project) live in
`jira_workflow_issue_types_fields`.
