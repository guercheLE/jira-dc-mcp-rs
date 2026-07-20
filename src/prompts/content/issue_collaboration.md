# Guided workflow: issue collaboration

This sub-workflow is designed to be run as an isolated sub-task where
possible -- everything you need is in this prompt's own text plus an
`issue_key`; report back only a short summary when done rather than the full
step-by-step trace.

Covers the actions people take on an issue that already exists: comments,
worklogs, attachments, issue links and remote links, watchers, and votes.
For creating or transitioning the issue itself, use `jira_workflow_issues`
instead.

As always, describe every operation as a capability to search for -- e.g.
"search for how to add a comment to an issue" -- never a specific
`operationId`, and read the schema `get` returns before relying on any field
name in it (the exact operation and shape can differ across the supported
API versions).

## Pattern for each sub-resource

1. Confirm you have an `issue_key`. If not, ask the user, or point them at
   `jira_workflow_search` to find one.
2. Search for and call the operation matching the requested action
   (add/list/update/delete a comment, worklog, attachment, link, watcher, or
   vote).
3. Verify with a follow-up read (e.g. list comments again) rather than
   trusting a non-error response alone -- this matters most for anything
   that isn't immediately visible in the call's own response, such as a
   remote link or an attachment upload.

## Real gotchas

- **Issue links** connect two issues via a named link type (e.g. "blocks",
  "relates to") -- search for the available link types before creating one
  if the user hasn't specified which relationship they mean; don't guess a
  link type name.
- **Remote links** (links to external URLs/systems) are a distinct resource
  from issue links -- don't conflate the two operations.
- **Worklogs** commonly interact with the issue's remaining-estimate field;
  ask the user whether they want the remaining estimate auto-adjusted,
  left alone, or set to a specific value before logging work, since the
  default behavior can silently change the issue's estimate.
- **Manually notifying people** is a separate operation from watchers --
  search for how to send a notification about an issue to chosen
  recipients if the user wants to alert someone who isn't (and shouldn't
  necessarily become) a watcher.

## Independent actions (parallelizable, delegable)

Adding a comment, setting a watcher, and adding a worklog entry on the same
issue are independent of each other. If your environment provides a way to
run a sub-task in its own context, delegate each independent action and have
it return only a short confirmation; otherwise just make the calls directly,
one after another.
