# MCP prompts expansion: project setup + catalog gap fixes + README

## Context

`docs/mcp-prompts-workflow-plan.md` shipped an MCP prompts capability (v0.6.0):
a `jira_workflow` master prompt plus 11 domain sub-workflows covering issues,
issue collaboration, search/JQL, projects, agile, workflows & statuses,
issue types & fields, permissions & security, users & groups, and
admin/monitoring. That feature works well in practice. This plan is a
follow-up pass: re-review the full embedded operation catalog end to end for
(a) any genuinely valuable *cross-cutting* guided workflow not yet covered
by a single existing prompt, and (b) any real per-domain gotcha the existing
11 prompts missed, then document the whole prompts feature in `README.md`,
which currently has zero mentions of it.

### Catalog re-review method

Re-decompressed all 6 embedded version stores (`mcp_store.db.zst` = `11.3`
default plus `_v11.2`/`_v11.1`/`_v11.0`/`_v10.7`/`_v10.6`) and grouped every
one of the 437 (`11.3`) operations by its `/rest/{api|agile|auth}/{version}/
{resource}` prefix, cross-checked against the 11 existing prompts' actual
content. Every resource family maps to an existing prompt except a handful
of low-value cosmetic ones deliberately left uncovered (`api/avatar`,
`api/universal_avatar`, `api/settings` base-URL/issue-navigator-columns,
`api/terminology` epic/sprint renaming) -- these stay reachable through each
prompt's own "search for X" agnostic phrasing without dedicating prompt
content to them.

### Finding 1: project bootstrap is a real, currently-uncovered cross-cutting workflow

Creating a project and correctly wiring up its scheme associations spans
operations that live in four *different* existing prompts today
(`jira_workflow_projects` for `createProject`/`getAllProjectTypes`,
`jira_workflow_workflows_statuses` for the workflow scheme,
`jira_workflow_permissions_security` for the permission/notification/
security schemes, `jira_workflow_issue_types_fields` for the issue type
scheme association) -- no single prompt walks through doing this in order.
Verified every operation this new workflow needs exists, and is stable in
shape, identically across all 6 versions (`11.3` and `10.6` spot-checked
directly): `getAllProjectTypes`, `createProject`, `getWorkflowSchemeForProject`,
`getAssignedPermissionScheme`/`assignPermissionScheme`,
`getAssignedPriorityScheme`/`assignPriorityScheme`,
`getNotificationScheme_1`, `getIssueSecurityScheme_1`,
`getAssociatedProjects`/`addProjectAssociationsToScheme`. This is the same
shape of gap `jira_workflow_issues` filled for issue creation -- a genuinely
compound, order-dependent, multi-resource task -- so it earns a new,
dedicated 12th prompt: `jira_workflow_project_setup`.

### Finding 2: real per-domain gotchas the existing 11 prompts don't mention

All verified present and stable across every supported version:

- **Issue archiving.** `archiveIssue`/`archiveIssues`/`restoreIssue` exist
  in every version; archived issues become read-only and are excluded from
  default search results. `jira_workflow_issues` doesn't mention this at
  all today.
- **Bulk issue creation.** `createIssues` (`/issue/bulk`) exists alongside
  single-issue `createIssue`; bulk endpoints of this shape typically report
  per-item success/failure rather than failing the whole batch atomically --
  worth a one-line caution so a caller doesn't assume all-or-nothing.
- **Version release/deletion gotchas.** `getVersionUnresolvedIssueCount`/
  `getVersionRelatedIssueCounts` exist specifically to be checked before
  marking a version released; `merge` (merge two versions) and
  `removeAndSwap` (delete a version, reassigning its issues) are distinct
  operations from a plain delete -- `jira_workflow_projects` currently only
  gestures at "deciding what happens to referencing issues" in the abstract
  without naming this concrete mechanism.
- **Project archiving.** `archiveProject`/`restoreProject` exist,
  unmentioned in `jira_workflow_projects`.
- **Board-specific estimation field.** `estimateIssueForBoard` sets an
  issue's estimate *for a specific board*, and which field that actually
  means (story points vs. original time estimate) is board configuration,
  not universal -- `jira_workflow_agile` doesn't call this out, so a caller
  could set the wrong field.
- **Backlog/epic ranking.** `rankIssues`/`rankEpics` exist as their own
  operations, distinct from moving an issue into a sprint/epic --
  unmentioned today.
- **Filter sharing.** `getSharePermissions`/`addSharePermission`/
  `getDefaultShareScope` exist for saved filters; `jira_workflow_search`
  mentions saved filters but not their sharing/visibility model.
- **Manual notifications.** `notify` (`POST /issue/{key}/notify`) sends an
  ad hoc notification to chosen recipients, independent of watchers --
  unmentioned in `jira_workflow_issue_collaboration`.
- **Email template customization.** `downloadEmailTemplates`/
  `uploadEmailTemplates`/`applyEmailTemplates`/`revertEmailTemplatesToDefault`
  exist as a distinct admin capability, unmentioned anywhere.

These don't each warrant a new prompt -- they're targeted enhancements
(a sentence or short paragraph each) to the existing prompt that already
owns that domain, keeping the existing content-size bands intact.

## Approach

### New prompt: `jira_workflow_project_setup`

Same file-layout convention as every other prompt: a `ProjectSetupWorkflowArgs`
struct in `src/prompts/mod.rs` (`project_key`, `project_name`,
`project_type`, all `Option<String>`, doc-commented, no `required: true` --
same rationale as `IssuesWorkflowArgs`), a router method in
`src/prompts/router.rs`, and `src/prompts/content/project_setup.md`.

Content follows the same skeleton `content/issues.md` established (the
plan's established worked-example pattern, now demonstrated a second time
rather than re-explained in full):

- Opening note: self-contained/delegable, same as every other prompt.
- Step 0: gather `project_key`/`project_name`/`project_type` from the
  context header; ask for whichever the flow still needs.
- Step 1: fork -- "does the project already exist, or are we creating it?"
  (A) new: search for the available project types (`getAllProjectTypes`
  equivalent) and confirm the type with the user before creating, since it
  can't be changed as freely as other fields later; then create. (B)
  existing: skip straight to Step 2 with the known `project_key`.
- Step 2: read the project's *current* scheme associations first (workflow,
  permission, notification, priority, issue security, issue type scheme)
  before changing anything -- don't assume defaults.
- Step 3: the scheme associations to set are independent of each other once
  the project exists -- call this out as parallelizable and delegable
  (mirrors `content/issues.md`'s Step 3), with each one phrased agnostically
  ("search for how to assign a permission scheme to a project") and a
  cross-reference to the prompt that owns that scheme's full detail
  (`jira_workflow_workflows_statuses`, `jira_workflow_permissions_security`,
  `jira_workflow_issue_types_fields`) rather than duplicating it here.
- Step 4: verify each association took effect with a follow-up read.
- Step 5: summarize what was created/associated.
- Composing-with-other-workflows section, same as every other prompt.

Target length: 60-120 lines, matching the compound-workflow band
`content/issues.md` and `content/workflows_statuses.md` already established.

### Targeted enhancements (no new prompts, no new files)

Each existing `content/*.md` file gets one short addition, sized to keep
that file inside its already-established band (see the original plan's
"Content size and token economy" section, unchanged by this expansion):

- `content/issues.md`: a short "archiving" gotcha paragraph, and a
  one-sentence note on bulk creation's per-item-failure shape.
- `content/projects.md`: replace the vague "deciding what happens to
  referencing issues" line with the concrete release/unresolved-count and
  merge/remove-and-swap mechanism, plus a one-line project-archive/restore
  mention.
- `content/agile.md`: a short note that an issue's board estimate maps to
  whichever field that board is configured to use, plus a one-line mention
  of issue/epic ranking as its own operation.
- `content/search.md`: one sentence on filter sharing/visibility.
- `content/issue_collaboration.md`: one sentence on the manual-notify
  operation as distinct from watchers.
- `content/admin_monitoring.md`: extend the existing single paragraph with
  email-template customization as one more admin capability to search for
  -- stays a single paragraph, the shortest prompt.

### `master.md`

Add the `jira_workflow_project_setup` entry to the sub-workflow list
(alongside the existing 10 lines), keeping the file within its established
under-60-line target.

### README.md

`README.md` currently never mentions prompts at all -- it documents only
the `search`/`get`/`call` tools. Add a new top-level `## Guided workflows`
section (after `## Usage`, before `## Docker`, matching the doc's existing
top-level section ordering) covering:

- What MCP prompts are and why they exist here (parallels the intro
  sentence this repo already has for `search`/`get`/`call`, i.e.
  version-agnostic guidance layered on top of the 3 tools rather than a
  replacement for them).
- How to invoke them: `prompts/list` to discover, `prompts/get` with a
  `name` (and optional arguments) to fetch instructions -- one line each,
  not a protocol tutorial.
- The full list of 12 prompt names with a one-line description each
  (mirrors `master.md`'s own menu, kept in sync with it).
- A one-line pointer that arguments are optional everywhere and the prompt
  itself will ask for what's still missing, plus the delegation behavior
  (whole-sub-workflow delegation to an isolated sub-task where the calling
  environment supports it).

Keep it proportional to how the rest of `README.md` treats the existing
3-tool surface -- a few short paragraphs plus the name/description list, not
a restatement of every prompt's internal steps.

## Critical files

- `docs/mcp-prompts-workflow-expansion-plan.md` (new) -- this plan
- [src/prompts/mod.rs](../src/prompts/mod.rs) -- new `ProjectSetupWorkflowArgs` struct
- [src/prompts/router.rs](../src/prompts/router.rs) -- new `jira_workflow_project_setup_prompt` method
- `src/prompts/content/project_setup.md` (new)
- [src/prompts/content/issues.md](../src/prompts/content/issues.md), [projects.md](../src/prompts/content/projects.md), [agile.md](../src/prompts/content/agile.md), [search.md](../src/prompts/content/search.md), [issue_collaboration.md](../src/prompts/content/issue_collaboration.md), [admin_monitoring.md](../src/prompts/content/admin_monitoring.md), [master.md](../src/prompts/content/master.md) -- targeted enhancements
- [tests/prompts_workflow.rs](../tests/prompts_workflow.rs) -- bump expected prompt count to 12, add the new name to the assertion list
- [README.md](../README.md) -- new `## Guided workflows` section

## Sequencing

1. Add `jira_workflow_project_setup` (arg struct, router method, content
   file) and update `master.md` to reference it.
2. Apply the six targeted content enhancements.
3. Update `tests/prompts_workflow.rs`'s prompt-count/name assertions.
4. Update `README.md`.
5. Verify (see below), then follow this repo's established release process.

## Verification

- `cargo build` / `cargo test` (full suite, including the extended
  `tests/prompts_workflow.rs`).
- `cargo fmt --check` / `cargo clippy --all-targets`.
- Manual smoke check: `cargo run -- start` piped a `prompts/list` +
  `prompts/get` (`jira_workflow_project_setup`) request, confirming the
  response is well-formed and the master menu links to it, same shape as
  the original plan's manual verification step.
- Re-check every `content/*.md` file's line count still falls inside the
  size band its content-complexity tier established.

## Release

Following this repo's established convention (unchanged from the original
plan): once implementation is complete and `cargo test` passes, commit the
implementation with a conventional-commit message (e.g. `feat(prompts): add
project setup workflow and close catalog gaps` -- confirm the exact
`type(scope)` against recent history at commit time), commit this plan
document separately (`docs: ...`), bump the version and commit that
separately (`chore(release): bump version to X.Y.Z` -- this repo bumps
*minor* for `feat` commits per its own history, confirmed when the original
prompts feature shipped as `0.5.8` -> `0.6.0`, so default to `0.7.0` here
unless the actual diff argues otherwise), tag `vX.Y.Z`, then push the branch
and the tag.
