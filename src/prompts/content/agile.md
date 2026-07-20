# Guided workflow: agile (boards, sprints, epics)

This sub-workflow is designed to be run as an isolated sub-task where
possible; report back only a short summary rather than the full
step-by-step trace.

Covers Jira Software boards, sprints, epics, and backlog management. As
always, describe every operation as a capability to search for -- e.g.
"search for how to move issues into a sprint" -- never a specific
`operationId`, and read the schema `get` returns before relying on any
field name in it.

## Typical flow

1. Resolve the board (and, if relevant, the project it's scoped to) before
   anything else -- most sprint/epic/backlog operations are addressed by
   board id or sprint id, not by project key.
2. For sprint work: confirm whether the sprint is in the state the user
   expects (future/active/closed) before acting -- e.g. you generally can't
   add issues to a closed sprint, and starting or closing a sprint are
   distinct, deliberate actions, not implied by moving issues.
3. For epics: an issue's epic link and a sprint's issue membership are
   independent facts about the issue -- don't assume setting one implies
   the other.
4. For estimation: an issue's estimate is set *per board*, and which field
   that actually means (story points vs. original time estimate) is that
   board's own configuration -- search for and read the board's
   configuration before setting an estimate, rather than assuming a field
   name.
5. Ranking an issue or epic (its position in the backlog/board order) is a
   distinct operation from moving it into a sprint or epic -- don't assume
   one implies the other.

## Bulk operations (delegable)

Moving many issues into a sprint or backlog in one call, or listing all
issues on a board, can produce a large response. If your environment
provides a way to run a sub-task in its own context, delegate the bulk
operation or listing and have it return only a short confirmation or
summary (counts, or the few issues that mattered) rather than the full
response. If no such mechanism is available, do it directly but summarize
the result rather than repeating it in full.

For creating or transitioning the issues themselves, use
`jira-issues`; for finding issues to move, use
`jira-search`.
