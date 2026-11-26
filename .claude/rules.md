# grim-gui Rule Catalog

Entry point for `.claude/rules/`. Path-scoped rules fire only on file edit match — not enough during planning, research, or architecture before code exists.

## When to consult this catalog

- Plan/architecture phase — scan "By concern" before drafting
- Research phase — find quality criteria for topic
- Onboarding new area — scan "By subsystem" / "By language"
- Writing ADRs, RFCs, research artifacts — find relevant rules
- Updating AI config — see "AI config changes" row in "By concern"

Editing code + relevant rule auto-loaded via `paths:` glob → no need re-read from catalog. Catalog for cases where path-scoping cannot fire.

## How to update this catalog

Any change to `.claude/rules/` must reflect here in same commit:

- New rule → add row in relevant tables below
- Deleted rule → remove all references
- Changed `paths:` glob → update "By auto-load path" table
- New skill → add entry in "Skills by task topic"

Structural tests in `.claude/tests/test_ai_config.py` fail when catalog drifts from reality. Run `task claude:tests` locally before committing catalog changes.

## By concern — "if you care about X, consult these"

| Concern | Rules & skills |
|---|---|
| Product vision / positioning | [product-context.md](./rules/product-context.md) — canonical identity doc; update when positioning shifts (see Update Protocol section at bottom) |
| Adopting a third-party library or tool | [product-tech-strategy.md](./rules/product-tech-strategy.md), `quality-{lang}.md` |
| Starting any task (work-type routing) | [workflow-intent.md](./rules/workflow-intent.md) (global) — classifies work, checks GitHub context, routes to correct workflow |
| Designing a new feature | [arch-principles.md](./rules/arch-principles.md), [workflow-feature.md](./rules/workflow-feature.md), `subsystem-{target}.md`, `/hex-architect` |
| Fixing a bug | [workflow-bugfix.md](./rules/workflow-bugfix.md) — Reproduce → RCA → Regression Test → Fix → Verify; skill `bugfix` (guided, enforces failing-test-first gate); skill `rca` (read-only verdict on a ticket), skill `fix` (RCA, then an approved fix plan) |
| Refactoring code | [workflow-refactor.md](./rules/workflow-refactor.md) — Safety Net → Scope → Transform → Verify → Repeat |
| Security-sensitive change (IPC bridge, capabilities, CSP, the spawned grim) | [arch-threat-model.md](./rules/arch-threat-model.md) **first** — attackers T1–T5 are in scope, N1–N5 are non-goals; then [quality-security.md](./rules/quality-security.md), [subsystem-shell.md](./rules/subsystem-shell.md), skill `security-auditor` |
| Adding a bridge command | [subsystem-shell.md](./rules/subsystem-shell.md) — the R12 convention (return grim's JSON unchanged), the three places a command is declared; [subsystem-web.md](./rules/subsystem-web.md) for the renderer |
| Rendering artifact kinds in a screen | [artifact-kinds.md](./rules/artifact-kinds.md), [grim-compat-markers.md](./rules/grim-compat-markers.md) — both shared with the sibling repos |
| Writing tests | [subsystem-tests.md](./rules/subsystem-tests.md), [quality-python.md](./rules/quality-python.md), [quality-rust.md](./rules/quality-rust.md), skill `qa-engineer` |
| CI / workflows | [subsystem-ci.md](./rules/subsystem-ci.md), [quality-security.md](./rules/quality-security.md) |
| AI config changes | [meta-ai-config.md](./rules/meta-ai-config.md) + this catalog, skill `meta-maintain-config` |
| GitHub issues & PRs / planning artifacts | [workflow-github.md](./rules/workflow-github.md), [workflow-feature.md](./rules/workflow-feature.md) |
| Commits, branches, rebasing, landing on main | [workflow-git.md](./rules/workflow-git.md), skills `commit`, `finalize` |
| Plan progress tracking (Status block + `.claude/state/current_plan.md`) | [meta-ai-config.md](./rules/meta-ai-config.md) "Plan Status Protocol", `/hex-plan`, `/hex-execute`, `/hex-review`, skills `commit`, `finalize`, `next` |
| Multi-agent orchestration | [workflow-feature.md](./rules/workflow-feature.md) — the **hex** bundle (`/hex-plan`, `/hex-execute`, `/hex-review`, `/hex-architect`), installed at project scope via the root `grimoire.toml`; swarm memory `.agents/memory/hex.md` |
| Code quality audit | [quality-core.md](./rules/quality-core.md), `quality-{lang}.md`, skill `code-check` |
| Documentation work (README, CONTRIBUTING) | [docs-quality.md](./rules/docs-quality.md) (installed from `docs-essentials` — page declarations, blocking rules, runnable checks), skills `docs-plan`, `docs-instrument`, `docs-review` |
| Source comments, doc comments | [code-docs.md](./rules/code-docs.md) (installed), skill `code-docs-cleanup` for bulk shortening |
| Error type design (Rust) | [quality-rust.md](./rules/quality-rust.md), [quality-rust-errors.md](./rules/quality-rust-errors.md) |
| Exit code design (Rust) | [quality-rust.md](./rules/quality-rust.md), [quality-rust-exit_codes.md](./rules/quality-rust-exit_codes.md) |
| Taskfiles / build pipeline / caching | [subsystem-taskfiles.md](./rules/subsystem-taskfiles.md) |

## By language

| Language | Quality rule | Related |
|---|---|---|
| Rust | [quality-rust.md](./rules/quality-rust.md) | [quality-rust-errors.md](./rules/quality-rust-errors.md), [quality-rust-exit_codes.md](./rules/quality-rust-exit_codes.md), [arch-principles.md](./rules/arch-principles.md), [quality-core.md](./rules/quality-core.md) |
| TypeScript | [typescript-quality.md](./rules/typescript-quality.md) (installed) | [typescript-packaging.md](./rules/typescript-packaging.md) (installed), [subsystem-web.md](./rules/subsystem-web.md) |
| Python (acceptance tests) | [quality-python.md](./rules/quality-python.md) | [subsystem-tests.md](./rules/subsystem-tests.md) |
| Bash (tasks, hooks, scripts) | [quality-bash.md](./rules/quality-bash.md) | — |

Installed language rules co-fire with the first-party ones on the same globs:
[rust-quality.md](./rules/rust-quality.md) and [rust-cargo.md](./rules/rust-cargo.md) alongside
`quality-rust*.md`, [python-quality.md](./rules/python-quality.md) and
[python-packaging.md](./rules/python-packaging.md) alongside `quality-python.md`. See
"Installed rules" below — the overlap is real and unresolved.

## By subsystem

Catalog = single source of truth — `AGENTS.md` may summarize or reference by pointer.

| Subsystem | Rule | Path scope |
|---|---|---|
| Tauri shell: bridge, capabilities, CSP | [subsystem-shell.md](./rules/subsystem-shell.md) | `src-tauri/**` |
| Front end | [subsystem-web.md](./rules/subsystem-web.md) | `src/**`, `index.html`, `vite.config.ts` |
| Acceptance tests | [subsystem-tests.md](./rules/subsystem-tests.md) | `test/**` |
| CI / workflows | [subsystem-ci.md](./rules/subsystem-ci.md) | `.github/workflows/**`, `.github/dependabot.yml` |
| Taskfiles | [subsystem-taskfiles.md](./rules/subsystem-taskfiles.md) | `taskfile.yml`, `taskfiles/**/*.yml`, `**/taskfile.yml` |

## By auto-load path — "what fires when you edit"

| Edit path | Rules that auto-load |
|---|---|
| `**/*.rs` | [quality-rust.md](./rules/quality-rust.md), [quality-rust-errors.md](./rules/quality-rust-errors.md), [quality-rust-exit_codes.md](./rules/quality-rust-exit_codes.md), [rust-quality.md](./rules/rust-quality.md) (installed), [code-docs.md](./rules/code-docs.md) (installed) (+ [arch-principles.md](./rules/arch-principles.md) under `src-tauri/**/*.rs`) |
| `**/Cargo.toml`, `**/Cargo.lock` | [quality-rust.md](./rules/quality-rust.md) (+ [rust-cargo.md](./rules/rust-cargo.md) on `**/Cargo.toml`, `**/rustfmt.toml`, `**/deny.toml`, `**/rust-toolchain.toml`) |
| `src-tauri/**` | + [subsystem-shell.md](./rules/subsystem-shell.md), [arch-threat-model.md](./rules/arch-threat-model.md) |
| `src/**`, `index.html` | + [subsystem-web.md](./rules/subsystem-web.md), [arch-threat-model.md](./rules/arch-threat-model.md) |
| `vite.config.ts` | [subsystem-web.md](./rules/subsystem-web.md) |
| `src/main.ts`, `src/view.ts`, `src-tauri/src/bridge.rs` | + [artifact-kinds.md](./rules/artifact-kinds.md) |
| `src/**/*.ts` | + [grim-compat-markers.md](./rules/grim-compat-markers.md) |
| `**/*.ts`, `**/*.tsx`, `**/*.mts`, `**/*.cts`, `**/.npmrc` | [typescript-quality.md](./rules/typescript-quality.md) (installed), [code-docs.md](./rules/code-docs.md) (installed) |
| `**/package.json`, `**/tsconfig*.json`, `**/package-lock.json` | [typescript-packaging.md](./rules/typescript-packaging.md) (installed) |
| `test/**` | [subsystem-tests.md](./rules/subsystem-tests.md) |
| `**/*.py`, `**/pyproject.toml` | [quality-python.md](./rules/quality-python.md), [python-quality.md](./rules/python-quality.md) (installed), [code-docs.md](./rules/code-docs.md) (installed) |
| `**/pyproject.toml`, `**/uv.lock` | [python-packaging.md](./rules/python-packaging.md) (installed) |
| `**/*.sh`, `**/*.bash` | [quality-bash.md](./rules/quality-bash.md) |
| `.github/workflows/**`, `.github/dependabot.yml` | [subsystem-ci.md](./rules/subsystem-ci.md), [quality-security.md](./rules/quality-security.md) |
| `.github/ISSUE_TEMPLATE/**` | [workflow-github.md](./rules/workflow-github.md) |
| `cog.toml` | [workflow-git.md](./rules/workflow-git.md) |
| `taskfile.yml`, `taskfiles/**/*.yml`, `**/taskfile.yml` | [subsystem-taskfiles.md](./rules/subsystem-taskfiles.md) |
| `.agents/plans/**`, `.agents/adr/**`, `.agents/specs/**`, `.agents/research/**` | [product-context.md](./rules/product-context.md), [workflow-feature.md](./rules/workflow-feature.md) |
| `.claude/**` | [meta-ai-config.md](./rules/meta-ai-config.md) |

Globals (always loaded or imported into `AGENTS.md`): [quality-core.md](./rules/quality-core.md),
[product-tech-strategy.md](./rules/product-tech-strategy.md), [workflow-intent.md](./rules/workflow-intent.md),
[hex-state.md](./rules/hex-state.md) (installed), this catalog. Strict definition of the set: `meta-ai-config.md` › "Current Global Rules".

Source-work rules: [workflow-bugfix.md](./rules/workflow-bugfix.md) and [workflow-refactor.md](./rules/workflow-refactor.md)
load on `src/**`, `test/**`, `.claude/**`, `Cargo.toml` and `Cargo.lock`.

## Declared Path-Scope Overlaps

Every pair of rules sharing `paths:` pattern must be covered by declared group. Undeclared overlaps fail `test_path_overlaps_declared_or_absent`.

Exempt from overlap detection (intended broad coupling):

- Shareable `quality-*.md` — language quality rules co-fire with subsystem rules on language globs.
- `workflow-bugfix.md` / `workflow-refactor.md` — source-work-surface scope; co-firing with subsystem rules = intended coupling.

| Declared group | Shared scope |
|---|---|
| `subsystem-shell.md` + `arch-threat-model.md` | `src-tauri/**` (the shell is the privileged side of the bridge; the threat model co-fires with it by design) |
| `subsystem-web.md` + `arch-threat-model.md` | `src/**`, `index.html` (webview content is attacker T1) |
| `product-context.md` + `workflow-feature.md` | `.agents/plans/**`, `.agents/adr/**`, `.agents/specs/**`, `.agents/research/**` |
| `code-docs.md` + `rust-quality.md` + `python-quality.md` + `typescript-quality.md` | `README.md`, `CONTRIBUTING.md` and other generators' roots | [docs-quality.md](./rules/docs-quality.md) (installed) |
| `**/*.rs`, `**/*.py`, `**/*.pyi`, `**/*.ts`, `**/*.tsx`, `**/*.mts`, `**/*.cts` (installed comment rule co-fires with each language set; it owns what a comment keeps, they own syntax) |

## Installed rules — not authored here

Nine rules under `.claude/rules/` are installed from bundles by `grim install` (see `grimoire.toml`): their prose is **not authored here**, and a local edit is reverted by the next install. They declare provenance in frontmatter (`license:`), which is the marker `.claude/tests/test_ai_config.py::is_vendored` uses to exempt them from this repo's *authoring* standards (dead-glob detection, catalog authorship, skill description and body budgets). Fix upstream, then re-install. The same holds for the installed skills under `.claude/skills/` (`code-docs-cleanup`, `docs-*`, `hex-*`, `nox-review` and the `*-reference` support skills). `shell:verify` and the link check skip them (installed content is not authored here).

| Rule | Upstream bundle |
|---|---|
| [rust-quality.md](./rules/rust-quality.md), [rust-cargo.md](./rules/rust-cargo.md), [code-docs.md](./rules/code-docs.md) | `ghcr.io/ocx-sh/lore/rust-essentials` |
| [docs-quality.md](./rules/docs-quality.md) | `ghcr.io/ocx-sh/lore/docs-essentials` |
| [typescript-quality.md](./rules/typescript-quality.md), [typescript-packaging.md](./rules/typescript-packaging.md) | `ghcr.io/ocx-sh/lore/typescript-essentials` |
| [python-quality.md](./rules/python-quality.md), [python-packaging.md](./rules/python-packaging.md) | `ghcr.io/ocx-sh/lore/python-essentials` |
| [hex-state.md](./rules/hex-state.md) | `ghcr.io/michael-herwig/arcana/hex` |

**Open**: the installed `rust-quality.md` / `python-quality.md` duplicate the first-party `quality-rust.md` / `quality-python.md` on the same globs — two rule sets load for the same edit. Which set is authoritative is a decision for the owner, not for an install.

## Skills by task topic

| Task topic | Skill |
|---|---|
| Security review | `security-auditor` |
| Architecture decision | `/hex-architect` (hex bundle) |
| Code quality audit | `code-check` |
| Documentation planning, gates, review | `docs-plan`, `docs-instrument`, `docs-review` (installed) |
| Shortening source comments in bulk | `code-docs-cleanup` |
| Implementation / debugging | `builder` |
| Test strategy | `qa-engineer` |
| Fixing a bug (guided, failing-test-first) | `bugfix` |
| Root cause analysis of a ticket (read-only) | `rca` |
| Ticket to approved fix plan (RCA, then regression-test-first plan) | `fix` |
| Planning a feature (multi-agent) | `/hex-plan` (hex bundle) |
| Executing a feature (multi-agent) | `/hex-execute` (hex bundle) |
| Reviewing a fork or outside-contributor pull request (security-first) | `outside-pr-review` |
| Adversarial review | `/hex-review` (hex bundle; cross-model adversary configured in `.agents/memory/hex.md`) |
| AI config maintenance | `meta-maintain-config`, `meta-validate-context` |
| Commits (working phase) | `commit` |
| Finalize branch for merge onto main | `finalize` |
| Suggest next slash command from current state | `next` |
