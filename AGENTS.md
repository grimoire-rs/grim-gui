# AGENTS.md

Single project-context file for **every** AI agent working in this repo —
Claude Code, Codex CLI, and any other `AGENTS.md`-aware tool.

> **Edit this file, never `CLAUDE.md`.** `CLAUDE.md` is a one-line pointer
> that imports this one, so a second copy would only drift. The same goes
> for anything it links: fix the source file, not a restatement of it.

Lines beginning `@` are Claude Code file imports and are expanded into context automatically; other agents should open those paths directly.

## What is grim-gui

`grim-gui` is the desktop shell of Grimoire: a Tauri 2 application whose
binary `grim-gui` is started by `grim gui` (plugin dispatch) or directly. It
**only invokes the `grim` binary** — nothing links grim's code. The Rust shell
(`src-tauri/`) reaches grim through the `grim-sdk` crate, and a plain
TypeScript front end (`src/`) renders what the shell returns.

> **Status: skeleton, Unstable.** One bridge command (`grim_version`) shows the
> version of the grim it finds, or "grim not found". Screens, packaging,
> signing and auto-update are not built yet — see "Next work" below. The
> contracts this repo honours are grim's, not its own.

## Project Identity

Product vision, users, positioning, related repos →
[`product-context.md`](./.claude/rules/product-context.md). Consult when
reasoning about direction, scope trade-offs or ADR motivation. Canonical —
keep current (update protocol at the bottom of that file).

## Rule Catalog

Before planning, research, or an architectural decision, scan "By concern"
in the catalog below. Path-glob rules fire only when a matching file is
already open; the catalog covers everything before that.

@.claude/rules.md

## Build & Development Commands

**Task runner**: [`task`](https://taskfile.dev) (Taskfile v3). **Always check
`task --list` before inventing ad-hoc commands.** Tools (`task`, Node 24,
`uv`, `cargo-deny`, …) are pinned in `ocx.toml`.

```sh
task                           # fast check (format + clippy + cargo check)
task verify                    # full quality gate (lint, then build + tests)
task --force verify            # bypass caching — run everything
task docs:check                # docs-quality checks over README.md
task rust:verify               # Rust-only gate (runs in the build container)
task web:lint                  # TypeScript type check
task web:test                  # vitest + happy-dom
task test:accept               # black-box pytest suite under xvfb (container)
task claude:tests              # AI config structural tests
task ci:lint                   # actionlint over .github/workflows
```

**The Linux build runs in a container.** The host needs no webkit2gtk:
`scripts/in-container.sh <cmd…>` runs a command in the image built from
`docker/linux-build.Dockerfile`; every Rust build, clippy, unit test and
acceptance run goes through it. Front-end lint and tests run on host Node 24.

**Always run `task verify` after implementation is done.** Always run
`cargo fmt` before commit. `rust:verify`, `claude:verify` are AI dev-loop
gates for the code changed; full `task verify` is the final gate before
commit. Conventions → [subsystem-taskfiles.md](./.claude/rules/subsystem-taskfiles.md).

## Architecture

**Layout**: a cargo workspace with one member, `src-tauri` (package and
binary `grim-gui`, library `grim_gui_lib`, edition 2024). The front end is
plain TypeScript under `src/` (no UI framework), built by Vite into `dist/`,
which `generate_context!` embeds at compile time. Black-box tests under
`test/`; the Linux build environment under `docker/` and `scripts/`.

| Path | Subsystem rule |
|---|---|
| `src-tauri/**` | [subsystem-shell.md](./.claude/rules/subsystem-shell.md) — bridge commands, capabilities, CSP, the R12 convention |
| `src/**`, `index.html`, `vite.config.ts` | [subsystem-web.md](./.claude/rules/subsystem-web.md) — plain TypeScript, CSP-clean page, vitest |
| `test/**` (pytest) | [subsystem-tests.md](./.claude/rules/subsystem-tests.md) |
| `.github/workflows/**` | [subsystem-ci.md](./.claude/rules/subsystem-ci.md) |
| `taskfile.yml`, `taskfiles/**` | [subsystem-taskfiles.md](./.claude/rules/subsystem-taskfiles.md) |

Beyond the path map, two rules carry weight on any change:
[`arch-principles.md`](./.claude/rules/arch-principles.md) (design
principles, boundaries, ADR index) and `quality-core.md` (SOLID/DRY/YAGNI,
Block/Warn/Suggest severity tiers). Shareable quality guidance lives in
`.claude/rules/quality-*.md` — load the one matching the language you are
editing (`quality-rust.md`, `quality-python.md`, `quality-bash.md`; the
TypeScript set is the installed `typescript-quality.md`), and
`quality-security.md` before any security review.
[`arch-threat-model.md`](./.claude/rules/arch-threat-model.md) decides whether
a finding about webview content, IPC or the spawned grim is a defect.
`artifact-kinds.md` and `grim-compat-markers.md` are shared with the sibling
repos (vscode, indexer) and track facts of grim itself.

## Environment Variables

| Variable | Purpose | Default |
|---|---|---|
| `GRIM_BIN` | Absolute path of the `grim` binary the bridge runs (read by `grim_sdk::bin_from_env`). A relative or empty value is ignored. **Set by grim, never by this app**, when `grim gui` dispatches here; set it by hand to point a direct launch at another grim | (unset → `grim` on `PATH`) |
| `WEBKIT_DISABLE_DMABUF_RENDERER`, `LIBGL_ALWAYS_SOFTWARE`, `GDK_BACKEND` | Test-only: set by the acceptance harness so the webview starts under xvfb in the container | (unset) |
| `GRIM_SOURCE_BINARY` | Build-only: the grim the acceptance suite stages into `test/bin/`; default is the one built in the grimoire checkout | (task default) |

## Core Principles

Nine principles distill every rule, skill, and standard in the framework.
Follow them and everything else follows.

### 1. Understand First

Read before write. Grep before create. Never modify code not read — before
changing a function, grep all callers. Check what exists before building
new.

### 2. Prove It Works

Write tests for the use case first. Run them before commit. Every bug fix
gets a regression test. All quality gates must pass.

### 3. Keep It Safe

No secrets in code. Validate all external input. Least privilege
everywhere. Flag vulnerabilities immediately.

### 4. Keep It Simple

Small functions, single responsibility. No premature abstraction. Delete
dead code. Comments explain *why*, never *what*.

### 5. Don't Repeat Yourself

Check `.claude/skills/` before ad-hoc generation. Follow existing patterns.
Single source of truth for logic. Extract only when duplication is real.

### 6. Ship It

Work on a branch, never main. Commit iteratively. **Never push to remote**
— the human decides when to push. Push triggers CI, real cost.

### 7. Leave a Trail

Planning artifacts go under `./.agents/`. Document architectural
decisions in ADRs. Name things so the next person understands.

### 8. Learn and Adapt

When you get user feedback or corrections, evaluate whether the insight
should persist as an AI config update (rules, skills, agents).


### 9. Respect grim's Contracts

grim's CLI, JSON output, exit codes and layouts are frozen contracts of
another repository. This app consumes them and never reshapes them: a bridge
command returns grim's `serde_json::Value` unchanged, and an exit code from
grim stays that code. Evolution on our side is additive — a command, a field,
a screen. A change that needs grim to change goes to the grimoire repository
first.

## Tech Stack

Golden-path technology choices — no deviations →
[`product-tech-strategy.md`](./.claude/rules/product-tech-strategy.md). The
Tauri 2 shell with a TypeScript front end is the recorded deviation from the
Rust-only golden path. A frontmatter-less rule: Claude Code already loads it
every session, so it is linked here rather than `@`-imported a second time.

## Workflow

**Worktrees**: `grim-gui` is the primary checkout (`main`). Feature work
happens in ad hoc worktrees — human ones as siblings `../grim-gui-wt-<topic>`
on a matching `<type>/<topic>` branch (e.g. `../grim-gui-wt-first-screen` on
`feat/first-screen`), agent-spawned ones under `.agents/worktrees/<topic>`
(gitignored). Created and torn down per task, not a fixed roster; whoever
creates one removes it. `git worktree list` shows what is currently checked
out. (`../grimoire-wt-<topic>` is the same convention in the main repo.)

**Commits**: [Conventional Commits](https://www.conventionalcommits.org/)
(`feat:`, `fix:`, `ci:`, `chore:`, …), scopes optional, no `Co-Authored-By`.
**Always sign off** (DCO): the `-s` flag on every commit; CI blocks without it
(`task git:dco`). `chore:` for AI settings, skills, agent context, tooling
kept out of the changelog.

**Landing a feature**: run `/finalize` to clean branch history into a
sequence of Conventional Commits ready to fast-forward onto `main`. Two-phase
model (`/commit` during dev, `/finalize` before landing) →
[workflow-git.md](./.claude/rules/workflow-git.md).

**Planning flow**: ADR → Design Spec → Plan → Implementation. Planning docs
are committed under `./.agents/`: `adr/`, `specs/`, `plans/`, `research/`,
one-off records at the root. Templates in `./.claude/templates/artifacts/`.

**Scaffold provenance**: this repository was scaffolded by copying files from
grimoire (and two rules from vscode). `scaffold-manifest.toml` records every
copy with its source commit and hash; `python3 tools/scaffold_manifest.py
check` is part of `task claude:tests`. A `copied` file stays byte-identical to
its source; changing one means changing its row to `rewritten`.

## Skills & Personas

Multi-agent orchestration is the **hex** bundle, installed at project scope
from the committed `grimoire.toml`/`grimoire.lock` (with `nox` and the lore
bundles `rust-essentials`, `typescript-essentials`, `python-essentials`, `docs-essentials`).
Swarm memory: `.agents/memory/hex.md`. Commands: `/hex-init`, `/hex-plan`,
`/hex-execute`, `/hex-review`, `/hex-architect`.
Project-local skills in `.claude/skills/`: `/builder`, `/qa-engineer`,
`/security-auditor`, `/code-check`, `/bugfix`, `/rca`, `/fix`,
`/outside-pr-review`, `/commit`, `/finalize`, `/next`, `/meta-maintain-config`,
`/meta-validate-context`. Full map → "Skills by task topic" in
[.claude/rules.md](./.claude/rules.md). Check `.claude/skills/` before ad-hoc generation.

## Next work (for the GUI agent)

1. **Install progress as JSON events.** Check first whether `grim install`
   can emit progress as JSON events (grimoire open question 4). If it can,
   the bridge streams them; if not, the first screen polls `grim status`.
2. **Reuse the vscode webview code.** The VS Code extension already renders
   artifact lists and details from grim's JSON; reuse its model and
   renderers where they are plain TypeScript, before writing new ones.
3. **Prove the macOS lint.** The workspace forbids `unsafe_code`; Tauri's macOS
   dev build embeds an Info.plist through macros whose expansion may trip it.
   Only the macOS CI job can show it. If it fails, apply the narrow fallback
   in `subsystem-shell.md` and record a manifest deviation.
4. **Later commands** return grim's JSON unchanged (R12 convention in
   `subsystem-shell.md`); add each to `build.rs`'s command list and to the
   capability file in the same change.

## Starting Work

Every task starts with
[workflow-intent.md](./.claude/rules/workflow-intent.md) — classify work
(feature, bug fix, refactoring), check GitHub for related issues/PRs, then
follow the appropriate workflow. Also:
[workflow-feature.md](./.claude/rules/workflow-feature.md),
[workflow-bugfix.md](./.claude/rules/workflow-bugfix.md),
[workflow-refactor.md](./.claude/rules/workflow-refactor.md).

## Adversarial Review Guidance

When a cross-model reviewer is delegated an adversarial pass (hex's
adversary gate, or a direct hand-off to another model):

1. **Load this file and the subsystem rules** for the touched paths before
   flagging anything — the author was likely working against them.
2. **Challenge design choices**, not style. `cargo fmt` handles
   formatting; question whether the approach is right, what assumptions it
   depends on, and where it fails under real-world conditions.
3. **Do not critique load-bearing conventions** stated here or in
   `product-tech-strategy.md` (Tokio, Rust 2024, OCI-backed storage,
   never-push-to-remote, commit format). They are decisions, not
   invitations.
4. Return findings with **concrete file paths and line numbers**, grouped
   by severity (Block / Warn / Suggest — definitions in
   `.claude/rules/quality-core.md`).

---

# General Agent Safety (applies to any external agent)

## Non-interactive shell commands

`cp`, `mv`, `rm` may be aliased to `-i` on some systems, which hangs an
agent waiting for y/n. Always use non-interactive flags:

```bash
cp -f source dest           # NOT: cp source dest
mv -f source dest           # NOT: mv source dest
rm -f file                  # NOT: rm file
rm -rf directory            # NOT: rm -r directory
```

Also: `scp -o BatchMode=yes`, `ssh -o BatchMode=yes`, `apt-get -y`, `HOMEBREW_NO_AUTO_UPDATE=1 brew …`.

## Session hygiene

- **Never push to remote** — the human decides when to push (CI has real cost).
- All changes must be committed locally on a feature branch; **never commit directly to `main`**.
- Run `task verify` after any implementation change.
