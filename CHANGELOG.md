# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

This file was reconstructed retroactively from `git log` and the 44
`vX.Y.Z` release tags (2026-07-05 through 2026-07-26); it was not maintained
incrementally during those releases.

## [Unreleased]

No changes since v0.7.11.

## [0.7.11] - 2026-07-26

### Fixed

- Multi-arch GHCR image now builds `linux/amd64` on `ubuntu-24.04` and
  `linux/arm64` natively on `ubuntu-24.04-arm` (matrix build-by-digest, then
  a merge job combines both into one manifest), replacing the previous
  single-job QEMU cross-build.

## [0.7.10] - 2026-07-26

### Fixed

- Published container image is now genuinely multi-arch (`linux/amd64` +
  `linux/arm64`) — the release workflow's `docker/build-push-action` step
  never set `platforms:`, so only an amd64 image was ever pushed and pulls
  on arm64 hosts (Apple Silicon, arm64 CI runners) failed with "no matching
  manifest". (This first fix used QEMU emulation; see v0.7.11 for the
  follow-up that replaced it with native runners.)

## [0.7.9] - 2026-07-26

### Fixed

- Skip the disk-lock (`remove_file`) test on GitHub Actions' Windows
  runners instead of continuing to widen its retry budget — three
  successive escalations (1s → 10s → 60s) still didn't clear reliably
  there, pointing at runner-level file-handle contention (e.g. a
  background AV scanner) outside this crate's control. A short retry is
  kept for genuine transient contention on platforms where it does clear.

## [0.7.8] - 2026-07-26

### Fixed

- Widened the Windows store-lock retry from 10s (100 × 100ms) to 60s
  (600 × 100ms) — still not enough headroom for GitHub Actions' Windows
  runners in practice.

## [0.7.7] - 2026-07-26

### Fixed

- Serialized `login_authenticates_and_saves_credentials` against the
  credential-storage test that also mutates the shared, process-wide
  `HOME` environment variable, using an async-safe (`tokio::sync::Mutex`)
  lock so the guard can be held across an `.await` without tripping
  `clippy::await_holding_lock`.
- Widened the Windows store-lock retry from 1s (50 × 20ms) to 10s
  (100 × 100ms).

## [0.7.6] - 2026-07-26

### Fixed

- `cargo fmt --check` failures across five files.
- `test-connection`'s test now binds and drops a real `TcpListener` to get
  a genuinely free port, instead of assuming `http://127.0.0.1` (port 80)
  is free — that assumption collided with IIS's Default Web Site on
  GitHub's `windows-latest` runners and made the test wrongly report
  success.
- Retry `remove_file` in the store lock-release test to tolerate Windows
  handle-release timing.

## [0.7.5] - 2026-07-26

### Fixed

- Catalog input/output schemas (`mcp_store.db`, `generated_schemas.json`)
  now fully inline every `$ref`, instead of pointing at a locally-embedded
  `$defs` block — a leftover from an earlier partial `$ref`-localization
  fix. Genuinely cyclic references (e.g. `LinkGroupBean`, which nests
  itself) are confirmed to have no finite fully-inlined form and are left
  as-is.

## [0.7.4] - 2026-07-22

No user-facing changes — this release is test-coverage work only (CLI
call-path tests, MCP protocol test stabilization, auth-strategy and
prompts-workflow unit tests).

## [0.7.3] - 2026-07-21

### Fixed

- Both `release.yml`'s build job and `ci.yml`'s test job now have an
  explicit `timeout-minutes` (45/10 and 20 respectively) instead of
  falling back to GitHub Actions' 6-hour default job ceiling on a hang.

## [0.7.2] - 2026-07-21

### Fixed

- Recover from mutex poisoning on the shared store-connection lock at
  every lock site instead of propagating it (safe, since all access there
  is read-only) — a panic while holding that lock previously made every
  later `search`/`get`/`call` (and CLI invocation) sharing the connection
  panic too. Also wraps the MCP protocol test in a 30s timeout so a future
  hang fails fast.

## [0.7.1] - 2026-07-20

### Changed

- Renamed every MCP prompt from the `jira_workflow_*` underscore scheme to
  a plain `jira-*` hyphenated scheme (e.g. `jira_workflow_issues` →
  `jira-issues`, `jira_workflow` → `jira`), updating every cross-reference
  in prompt content, README, tests, and the server's advertised
  instructions.

## [0.7.0] - 2026-07-20

### Added

- New `jira-project-setup` guided-workflow prompt: bootstraps a project
  and walks through wiring up its workflow, permission, notification,
  priority, and issue-security scheme associations — previously spread
  across four separate prompts with no single guided path through all of
  them.
- Closed real gaps found in a full re-review of the embedded operation
  catalog across all 6 supported API versions: issue archiving and
  bulk-create semantics, version release/merge/archive mechanics, board-
  specific estimation fields and backlog/epic ranking, saved-filter
  sharing, manual (non-watcher) notifications, and email-template
  customization are now called out in their respective prompts.

### Documentation

- Documents the whole MCP prompts feature in `README.md` for the first
  time, in a new "Guided workflows" section.
- Adds `docs/mcp-prompts-workflow-expansion-plan.md`, the design record
  for this release's prompt additions.

## [0.6.0] - 2026-07-20

### Added

- New MCP **prompts** capability alongside the existing `search`/`get`/
  `call` tools: a `jira_workflow` master prompt plus 10 domain
  sub-workflows (issues, issue collaboration, search/JQL, projects, agile,
  workflows & statuses, issue types & fields, permissions & security,
  users & groups, admin/monitoring). Guides callers through multi-step
  Jira tasks — gathering required parameters, gating on verified state
  changes, and delegating whole sub-workflows or individual steps to
  isolated sub-tasks where the environment supports it.

### Documentation

- Adds `docs/mcp-prompts-workflow-plan.md`, the design record for the new
  prompts feature.

## [0.5.8] - 2026-07-19

### Fixed

- Retry the store rename on a cross-process race (another process — a
  concurrently-starting server instance, a `populate_embeddings` run —
  can briefly hold the destination file open) instead of adding
  cross-process locking: 5 attempts, 50ms × attempt backoff.
- Skip the `setup` smoke test on GitHub Actions' Windows runners, where
  `inquire`'s Windows backend reads raw console input rather than
  treating closed/non-console stdin as immediate EOF and blocks
  indefinitely; still runs on a real Windows machine.

### Changed

- CI now installs `cargo-dist` from a prebuilt binary/installer script
  instead of compiling it from source on every release run (several
  minutes on Linux/macOS, roughly double that on Windows).

## [0.5.7] - 2026-07-19

### Documentation

- Adds a sponsorship callout to the README and a `FUNDING.yml`.

## [0.5.6] - 2026-07-19

### Fixed

- Stop the stdio smoke test from hanging indefinitely (previously
  requiring manual cancellation after ~5 hours) on GitHub Actions' Windows
  runners: `tokio`'s stdin reader there is a background blocking thread
  that doesn't observe an already-EOF-closed child stdin the way Unix
  does. Skipped via a runtime `GITHUB_ACTIONS`+`windows` check rather than
  a blanket `#[cfg(windows)]`, so real Windows machines still get full
  coverage.

## [0.5.5] - 2026-07-19

### Fixed

- Relaxed a stdio-transport test's exact-error-text assertion to Unix-only
  — Windows's handling of an immediately-closed child stdin doesn't
  surface the same `rmcp` error message, though the platform-independent
  property (handshake never completes, non-zero exit) is still checked
  everywhere.

## [0.5.4] - 2026-07-19

No user-facing changes — this release closes the remaining production
test-coverage gap (`AuthManager::normalize_credentials`, `ApiClient::execute`'s
full happy-path and circuit-breaker-error paths, the `call` tool against a
real known `operationId`), raising coverage from 82.58% to 86.40%.

## [0.5.3] - 2026-07-19

### Fixed

- Eliminated a Windows-only race in `resolve_store_path`: the embedded
  SQLite store was re-extracted and renamed into place on *every* call,
  unguarded, so concurrent tool calls could race a rename against another
  thread's open connection — Windows refuses that rename outright, unlike
  POSIX. The resolved path is now cached per `api_version` behind a mutex,
  so extraction genuinely happens once per process as intended.
- Fixed a related test that deleted a fixture file out from under its own
  still-open connection (allowed on POSIX, not on Windows).

## [0.5.2] - 2026-07-19

### Fixed

- `dist host --steps=create` (the real release-tag trigger) refused to
  run against this repo's hand-simplified, single-job `release.yml`,
  flagging it "out of date" against cargo-dist's own multi-job generated
  shape. `allow-dirty` now tells `dist` the divergence is intentional.

## [0.5.1] - 2026-07-19

### Fixed

- Stopped the setup-wizard test from requiring a real TTY: with more than
  one supported API version, it exercised `inquire::Select`, which panics
  with "The input device is not a TTY" outside an interactive terminal —
  breaking `cargo test` in CI on every push. The test now constructs its
  inputs directly instead of driving the interactive prompts.

## [0.5.0] - 2026-07-19

### Changed

- Re-synced against mcpify's current code-generation templates. Notable
  changes picked up: every embedded store is now zstd-compressed instead
  of raw (~3.7MB `.db` files become ~1.3MB `.db.zst`); hardened auth/
  validation/schema handling that had been hand-patched directly into
  this repo is now generated correctly by default; a simpler hand-rolled
  release workflow replaces cargo-dist's full autogenerated pipeline; new
  coverage/profiling scaffolding.

### Documentation

- Restores README sections (Observability & Resilience, License, install
  instructions) that this repo's own hand-authored copies had drifted
  from — mcpify's generator now produces the equivalent content
  correctly by default.
- Adds a "Connect an MCP client" section to the README with real
  stdio/HTTP `mcpServers` JSON configs.

### Fixed

- Broke an over-long empty-body guard clause to satisfy `cargo fmt --check`.

## [0.4.4] - 2026-07-17

### Fixed

- Completed the OAuth2 code-exchange fallback: the credentials cascade
  only tried `refresh_token()` when validation failed, never the initial
  `authenticate()` exchange, so a credential blob with neither an access
  token nor a refresh token failed with a generic "no active credentials"
  error instead of re-deriving credentials from what was actually stored.
- Send an explicit empty body with `Content-Length: 0` on body-less PUT/
  POST/DELETE requests — some operations sent neither a body nor a
  `Content-Length` header, which strict APIs reject with `411 Length
  Required` before ever reaching auth or business logic.

## [0.4.3] - 2026-07-16

### Fixed

- Extract `mcp_store.db` atomically (write to a uniquely-named sibling
  file, then rename into place) to avoid a race where concurrent MCP tool
  calls could read the shared temp-dir copy mid-truncate — SQLite treats
  a momentarily-empty file as a valid empty database, surfacing as
  confusing "no such table" errors instead of a clear one.

## [0.4.2] - 2026-07-16

### Fixed

- `call` no longer rejects an otherwise-successful request just because
  the live response doesn't match its documented output schema (logs a
  warning and returns the real response instead) — upstream OpenAPI specs
  are frequently wrong about response shape. Input validation is
  unchanged and still rejects invalid arguments.
- Validation error messages now include the actual JSON Schema violations
  instead of just a generic "invalid input" / "unexpected response shape"
  string.

## [0.4.1] - 2026-07-16

No functional changes — `cargo fmt` formatting only.

## [0.4.0] - 2026-07-15

### Fixed

- `JIRA_DC_MCP_TOKEN`/`JIRA_DC_MCP_API_KEY` (and username/password)
  environment variables are now actually read before falling back to the
  stored credential — previously documented but never wired up.
- Fall back to the encrypted credential file when the OS keychain cleanly
  reports "not found," not only on a hard error.
- Resolve the home directory via `HOME` then `USERPROFILE` on Windows, so
  `credentials.enc` resolves consistently there.
- Setup wizard now prompts for global vs. local config persistence and
  writes YAML the loader actually reads back.
- `call`'s `arguments` now default to `{}` instead of `null`.
- `populate-embeddings` now defaults to populating and verifying every
  API version's store (row-count parity between `endpoints` and
  `semantic_endpoints`), failing loudly instead of silently
  under-populating; `search` now warns when a store is incomplete.
- The `api_key` auth-header branch now uses the scheme's configured
  header name instead of a hardcoded literal (no behavior change for this
  repo, since Jira's header name already was `X-Api-Key`).

### Documentation

- Documented that `JIRA_DC_MCP_URL` must include the `/rest` suffix.

## [0.3.0] - 2026-07-14

### Changed

- Adopted current mcpify Rust-generator parity: renamed the Docker
  release workflow, tightened the release-tag pattern to `v*.*.*`,
  applied least-privilege permissions and `--locked` to CI steps for
  reproducible builds, and added a proper `.dockerignore`.

## [0.2.12] - 2026-07-13

### Documentation

- Fixed the `call` usage example (it showed a stale operation and arg
  shape) and refreshed the stated operation count.

## [0.2.11] - 2026-07-10

### Fixed

- Docker build now copies every embedded version's `.db` file, not just
  the default — `store.rs` embeds all of them via `include_bytes!`, but
  the Dockerfile only `COPY`'d one, breaking non-default-version builds.
- Fixed `cargo fmt --check` drift left over from an earlier add-version/
  remove-version run.
- Release target matrix adjusted for the embedded ONNX Runtime (`ort`)
  dependency: dropped `x86_64-apple-darwin` (no prebuilt ONNX Runtime for
  x64 macOS anymore), pinned Linux builds to `ubuntu-24.04` (older glibc
  is too old for the prebuilt ONNX Runtime), and fixed a Windows CRT
  mismatch (`msvc-crt-static = false`).

## [0.2.10] - 2026-07-10

### Added

- Prebuilt binaries for macOS, Linux, and Windows, published to GitHub
  Releases (with shell/PowerShell installers) via `cargo-dist`, triggered
  by pushing a version tag. crates.io publishing and the GHCR image push
  moved into their own workflows.

## [0.2.9] - 2026-07-08

### Documentation

- Fixed the `call` usage example (it used a fabricated `--some-arg`
  syntax instead of the real `-a`/`--args` JSON flag). Added an
  Observability & Resilience section and a License section to the README.

## [0.2.8] - 2026-07-08

### Fixed

- Credentials are now normalized into a usable request-header shape
  (deriving `authorization_header` from a raw `access_token` when needed)
  immediately after being fetched, cached, or refreshed, and the
  normalized form is persisted back to storage.

## [0.2.7] - 2026-07-06

### Fixed

- Structured logs now go to stderr instead of stdout — the stdio MCP
  transport speaks JSON-RPC over stdout, so log lines interleaved into it
  broke strict MCP clients (e.g. VS Code) that parse stdout as JSON-RPC
  frames.

## [0.2.6] - 2026-07-06

### Fixed

- Install the `aws-lc-rs` rustls crypto provider once at process start —
  the OTLP span exporter's transitive `reqwest` client panicked with "No
  rustls crypto provider is configured" the first time it built, since
  nothing had selected a default provider yet.

## [0.2.5] - 2026-07-06

### Removed

- Dropped API versions `10.0`–`10.5` (keeping the default `11.3` plus the
  5 newer versions) — embedding every version's store made the published
  crate exceed crates.io's package size limit (confirmed by two failed
  upload attempts); now verifies at 7.9MiB compressed. Older versions can
  still be added back locally via `mcpify add-version`.

## [0.2.4] - 2026-07-06

### Fixed

- Backfilled semantic-search embeddings for every supported `api_version`'s
  store — previously only the default version (`11.3`) had any embeddings
  at all, so `search` against any other configured version returned
  nothing useful.

## [0.2.3] - 2026-07-06

### Fixed

- The temp-dir extraction of the embedded store no longer skips writing
  when a file already exists at that path — a stale copy from a previous
  install could otherwise linger and silently serve outdated data after a
  rebuild.
- `populate_embeddings` now accepts an explicit path, or `--all` to
  backfill every known version, instead of being hardcoded to
  `mcp_store.db` only.

## [0.2.2] - 2026-07-06

### Fixed

- Every supported version's `mcp_store*.db` is now embedded directly into
  the compiled binary (`include_bytes!`) and extracted to the OS temp
  directory on first use, so an installed binary works regardless of
  where it's invoked from or whether its source checkout still exists.
  The now-redundant `CARGO_MANIFEST_DIR`/exe-dir path-guessing fallback
  chain was removed in favor of looking up the embedded bytes directly.

## [0.2.1] - 2026-07-06

### Fixed

- An installed binary invoked from outside the project checkout can now
  find its bundled `mcp_store*.db` by falling back to the directory
  containing the running executable, not just the current working
  directory.

## [0.2.0] - 2026-07-06

### Added

- Personal Access Token (PAT) authentication support for Jira Data
  Center's bearer-token auth (the OpenAPI spec this server was generated
  from only declares Basic auth) — a new `PatAuthStrategy`, an
  `AuthMethod::Pat` config value, and setup-wizard support for choosing
  between the two.

### Documentation

- Added a reference for the API schema/DB versions and their source
  Swagger v3 URLs.

## [0.1.4] - 2026-07-05

### Fixed

- Namespaced the `populate-embeddings` helper binary to
  `jira-dc-mcp-populate-embeddings`, avoiding a `PATH` collision when
  multiple mcpify-generated servers are installed side by side.

## [0.1.2] - 2026-07-05

### Added

- Published the crate to crates.io.

## [0.1.1] - 2026-07-05

### Added

- Initial release of the `jira-dc-mcp` Rust MCP server: core runtime
  (config, cache, circuit breaker, rate limiting, health checks, OTEL),
  auth strategies, CLI commands, HTTP layer, `search`/`get`/`call` tool
  handlers, embedding service, schema validation, and Docker/CI setup.

### Fixed

- Narrowed an over-broad `*.db` `.gitignore` rule (meant for runtime/temp
  files) that was excluding the versioned `mcp_store*.db` semantic-store
  databases the Dockerfile and `store.rs` depend on, so the Docker build
  failed with them missing from the checkout.

[Unreleased]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.11...HEAD
[0.7.11]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.10...v0.7.11
[0.7.10]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.9...v0.7.10
[0.7.9]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.8...v0.7.9
[0.7.8]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.8...v0.6.0
[0.5.8]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.4.4...v0.5.0
[0.4.4]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.12...v0.3.0
[0.2.12]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.1.2...v0.1.4
[0.1.2]: https://github.com/guercheLE/jira-dc-mcp-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/guercheLE/jira-dc-mcp-rs/releases/tag/v0.1.1
</content>
