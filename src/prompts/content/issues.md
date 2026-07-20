# Guided workflow: issues

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if you were delegated here from `jira_workflow`'s routing, or
your environment otherwise supports running this as its own sub-task,
everything you need is in this prompt's own text plus the parameters already
listed above; report back only a short summary when done rather than the
full step-by-step trace.

Every operation below is described as a capability to search for, never as a
specific `operationId` -- the exact operation and its response schema can
differ across the supported Jira Data Center API versions. Call `search`
with the described capability, `get` the operationId it resolves to, and
read that operation's *current* schema before relying on any field name in
it.

## Step 0 -- gather required parameters

Check the "Context already provided" header above first. Only ask the user
for what's still listed as missing:

- Creating a new issue needs `project_key`, `issue_type`, and `summary`.
- Acting on an existing issue needs `issue_key`.

## Step 1 -- fork: new issue or existing issue?

Ask (if not already obvious from context): "Are you creating a new issue, or
acting on an existing one?"

**(A) Creating a new issue.** Don't assume `summary` is the only required
field -- search for how to discover the fields required to create an issue
of a given type in a given project, call it, and check the result for any
additional mandatory fields beyond `summary` (custom fields and issue-type
configuration vary by project and version). Fill in what the user already
gave you, ask for anything else that's genuinely required, then search for
and call the operation to create an issue. Gate: don't report the issue as
created until a follow-up read confirms it exists and echoes back the values
you set.

**(B) Acting on an existing issue.** Skip straight to Step 2.

## Step 2 -- the transition gotcha (gated)

Never guess or hardcode a transition id or name. Transition ids are
workflow-scheme-specific per project and issue type, and can differ across
the supported API versions -- a transition id that worked for one issue is
not guaranteed to work for another. Always:

1. Search for and call the operation that lists the transitions available
   *for this specific issue*.
2. Present the real options it returns to the user if more than one is
   plausible, rather than picking one silently.
3. Search for and call the transition operation using the id actually
   returned in step 1 -- never a value from a previous run or from memory.

Gate: don't report the transition as done until a follow-up read confirms
the issue's status actually changed. A non-error response from the
transition call alone is not sufficient confirmation.

## Step 3 -- independent follow-on actions (parallelizable, delegable)

Once the issue exists or its transition is confirmed, actions like adding a
comment, setting watchers, and linking to another issue are independent of
each other. Call this out to the user as safe to do concurrently, and: if
your environment provides a way to run a sub-task in its own context,
delegate each independent follow-on action and have it return only a short
confirmation -- don't pull the full request/response bodies into this
conversation. If no such sub-task mechanism is available, just make the
calls directly, one after another.

For the full detail on any one of comments, worklogs, attachments, links,
watchers, or votes, fetch the `jira_workflow_issue_collaboration` prompt
rather than duplicating it here.

## Step 4 -- summarize

Report what was created or changed and confirmed, in plain terms: the issue
key, its final status, and any follow-on actions taken.

## Composing with other workflows

- Picking (or creating) the right project overlaps with
  `jira_workflow_projects`.
- Placing the issue on a board or into a sprint overlaps with
  `jira_workflow_agile`.
- Finding existing issues to act on overlaps with `jira_workflow_search`.

Fetch those prompts by name for more detail rather than assuming their
content from this one.
