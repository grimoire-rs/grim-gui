---
paths:
  - taskfile.yml
  - taskfiles/**/*.yml
  - "**/taskfile.yml"
---

# Taskfiles Subsystem

[Taskfile](https://taskfile.dev) v3 = primary task runner for grim-gui. Root taskfile includes per-subsystem taskfiles under `taskfiles/`. Each subsystem own pipeline, expose contract back to root.

## File Layout

| Path | Owner | Loaded as |
|---|---|---|
| `taskfile.yml` | root | entry point -- `task verify`, `task default`, etc. |
| `.taskrc.yml` | root | project config -- `failfast: true` |
| `taskfiles/rust.taskfile.yml` | Rust shell | `rust:` -- format, clippy, build, test:unit; build and test run in the container |
| `taskfiles/shell.taskfile.yml` | Shell subsystem | `shell:` -- verify (shellcheck + shfmt, skipped if absent) |
| `taskfiles/web.taskfile.yml` | Front end | `web:` -- `lint` (`tsc --noEmit`), `test` (vitest), `build` (`vite build` into `dist/`) |
| `taskfiles/git.taskfile.yml` | cross-cutting | `git:` -- `dco`, `merge` |
| `taskfiles/ci.taskfile.yml` | workflows | `ci:` -- `lint` (actionlint over `.github/workflows/`, pinned in `ocx.toml`) |
| `taskfiles/docs.taskfile.yml` | docs | `docs:` -- `check` (docs-quality checks over `README.md`) |
| `taskfiles/gui.taskfile.yml` | build container | `gui:` -- `image` (builds the Linux build image) |
| `.claude/taskfile.yml` | `.claude/` subsystem | `claude:` |
| `test/taskfile.yml` | acceptance tests | `test:` -- `stage` (copies grim into `test/bin/`), `accept`, `floor` |

A taskfile name differing from this table is a drift to fix here in the same change.

## Two-Phase Verify

`task verify` run as two-phase pipeline via internal `.verify:lint` + `.verify:build-test` tasks, then `.verify:mark`:

1. **Phase 1 (parallel `deps:`)**: `rust:format:check`, `rust:clippy:check`, `shell:verify`, `claude:verify`, `web:lint`, `ci:lint`, `docs:check`
2. **Phase 2 (sequential `cmds:`)**: `rust:build`, `rust:test:unit`, `web:test`, `test:accept`
3. **`.verify:mark`** writes `.claude/hooks/.state/commit-verified`, the stamp the commit guard checks (300 s). Never write it by hand.

`rust:build` and `rust:clippy:check` depend on `web:build`, because `generate_context!` reads `dist/` at compile time. Every Rust step on Linux runs through `scripts/in-container.sh`, which never takes a build lock: the caller owns it.

## AI Quality Gate Pattern

Each subsystem rule has Quality Gate section specifying which verify task to run during review-fix loops. Full `task verify` runs only as final gate before commit. Stop expensive acceptance tests running every iteration.

## Taskfile Features in Use

| Feature | Purpose |
|---|---|
| `output: group` + `error_only: true` | Clean verify output; only show failures |
| `.taskrc.yml` | Project-wide `failfast: true` (abort on first failure) |
| Include-with-vars templates | reusable taskfiles included with different `vars:`; use `requires:` for mandatory vars |
| `internal: true` on includes | Template includes hidden from `task --list` |
| `:` prefix cross-references | Child taskfiles call root helpers like `:.ensure-cargo-tool` |
| `--force` | Bypass all caching for one run (replaces `rm -rf .task/`) |
| `--status` | Check freshness without running |
| `--summary` | Show task detail (e.g., `task --summary verify`) |
| `aliases:` | Backward compat after renames |
| `for:` loops | Iteration in cmds/deps |

## Conventions

1. **`verify` is a subsystem contract.** Every subsystem taskfile expose `verify` task. Root `verify` calls `<subsystem>:verify` instead of inlining subtasks.
2. **Composite tasks aggregate subtasks.** Each linter/formatter has own subtask so independently runnable. `verify` references composite tasks only -- one entry per concern.
3. **Helpers use dot-prefix + `internal: true`.** Internal tasks named with dot prefix (e.g., `.ensure-cargo-tool`, `.verify:lint`) -- like GitLab CI hidden jobs. Visually distinct + hidden from `task --list`.
4. **Tool installs use `status:` for idempotency.** `status: - cargo nextest --version` skips install when tool already present. Cross-include dedup pattern -- `run: once` does **not** work reliably across namespaced includes (go-task issue #852).
5. **Subsystem env overrides go on task, not include.** When the root sets a global `env:`, a subsystem task needing a different value overrides it with a per-task `env:`.

## Caching Contract

### `sources:` + `status:` + `method:`

| Field | Purpose | Recommendation |
|---|---|---|
| `sources:` | Files that force re-run when changed | Broad include + targeted excludes. Survive new file types. |
| `status:` | Exit 0 = up-to-date, skip | **Load-bearing for cache.** Use for output file checks. |
| `generates:` | Output declaration (docs only) | **Not load-bearing** (go-task #2181). Declare for discoverability; add `status:` if skip-on-present needed. |
| `method:` | `checksum` (default), `timestamp`, `none` | Only specify `timestamp` for tools with own incremental build (e.g. `cargo build`). |

**Sources relative to task's `dir:`.** Set `dir: '{{.TASKFILE_DIR}}'` or rely on include's `dir:` so globs use plain `**/*`.

**Two canonical patterns:**

```yaml
# Pattern A -- skip when output exists, re-run when sources change
generate:
  cmds: [scripts/in-container.sh some-command > {{.OUT}}]
  sources: [src-tauri/**/*.rs, Cargo.lock]
  status: [test -f {{.OUT}}]

# Pattern B -- wrap a tool with its own incremental build
build:
  cmd: cargo build --release --locked
  sources: [src-tauri/**/*.rs, Cargo.lock]
  status: [test target/release/grim-gui -nt Cargo.lock]
  method: timestamp
```

In a cargo workspace the source list names every member (`src-tauri/**/*.rs`), and a
whole-workspace command takes `--workspace` (`cargo clippy --workspace --locked`),
so a change in a member re-runs the task. The shell build also depends on the
front end: list `dist/**` or depend on `web:build`.

### Sharing source lists: `x-shared:` anchors

Multiple tasks validate same input set → define source list once as YAML anchor under `x-shared:` (JSON-Schema extension convention), reference with `*alias`. Do not anchor under consuming task -- implies ownership data structure does not have.

```yaml
x-shared:
  claude-sources: &claude-sources
    - '**/*'
    - '../CLAUDE.md'
    - '../AGENTS.md'
    - '../.agents/plans/*.md'
    - exclude: '**/__pycache__/**'

tasks:
  tests:
    sources: *claude-sources
  lint:links:
    sources: *claude-sources
```

### `.task/` cache directory

Task stores fingerprint state in `.task/`. **Add to `.gitignore`** before introducing any `sources:`/`method:` task.

## Reports and Artifacts (`REPORTS_DIR`)

Tasks that validate or check should emit machine-readable reports.

- **Location**: `target/reports/<subsystem>/` (already gitignored under `target/`).
- **Root var**: `REPORTS_ROOT: '{{.ROOT_DIR}}/target/reports'` in root taskfile.
- **Subsystem var**: Each subsystem resolves own `REPORTS_DIR` with `{{if/else}}` so standalone invocation works:

  ```yaml
  vars:
    REPORTS_DIR: '{{if .REPORTS_ROOT}}{{.REPORTS_ROOT}}/claude{{else}}{{.TASKFILE_DIR}}/reports{{end}}'
  ```

  Parent + child vars must have **different names** (`REPORTS_ROOT` vs `REPORTS_DIR`) to avoid double-suffixing.
- **Format**: JUnit XML (preferred) or JSON. Create dir with `mkdir -p {{.REPORTS_DIR}}` as first command.
- **Declare `generates:`** for docs even though not load-bearing.

## Includes and `dir:` Scoping

| Variable | Resolves to |
|---|---|
| `{{.ROOT_DIR}}` | Directory of entry-point (root) taskfile |
| `{{.TASKFILE_DIR}}` | Directory of currently executing taskfile |
| `{{.USER_WORKING_DIR}}` | Directory from which `task` was invoked |

Set `dir:` on include block when all tasks should run relative to sub-taskfile's directory (`.claude/`, `test/`). Override per-task only when needed.

**Gotchas:** `${{.TASKFILE_DIR}}` wrong (becomes shell var). When root sets global `env:`, included tasks inherit -- override per-task if needed.

## Preconditions vs Status

| Field | Use | Semantics |
|---|---|---|
| `preconditions:` + `msg:` | Guard -- abort if not ready | Failure aborts; downstream tasks don't run |
| `status:` | Cache -- skip if done | Exit 0 = up-to-date, skip silently |

## Task Contracts

- **Tasks** that run the compiled binary should depend on it via `deps: [build]` + `sources: [target/release/grim-gui]`, NOT on Rust source file lists. Cargo already tracks source deps — duplicating in the Taskfile = maintenance overhead.

## Sources

- [Taskfile schema reference](https://taskfile.dev/docs/reference/schema)
- [Taskfile usage](https://taskfile.dev/usage/) -- sources/status/method, run modes, env, preconditions
- [Taskfile templating](https://taskfile.dev/docs/reference/templating) -- special vars
- [go-task #2181](https://github.com/go-task/task/issues/2181) -- `generates:` globs not fingerprinted
- [go-task #852](https://github.com/go-task/task/issues/852) -- `run: once` cross-include dedup broken