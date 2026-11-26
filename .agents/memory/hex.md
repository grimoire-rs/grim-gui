# hex — swarm memory

Maintained by the hex skills. Small by contract: pointers and
preferences, not copies. Team-shared — commit it.

## Pointers

- Verification: `AGENTS.md` › "Build & Development Commands" — run
  `task verify` (full gate) before any merge; `task rust:verify`,
  `task web:lint`, `task web:test`, `task test:accept`,
  `task claude:verify` are the per-subsystem dev-loop gates. The Linux Rust
  build and the acceptance suite run in the build container
  (`scripts/in-container.sh`).
- Plan / ADR / spec conventions: `AGENTS.md` › "Workflow" › Planning flow.
  ADRs `.agents/adr/`, design specs `.agents/specs/`, plans (incl.
  `bugfix_plan_*`, `meta-plan_*`) `.agents/plans/`, research
  `.agents/research/`; one-off records at `.agents/` root. Templates:
  `.claude/templates/artifacts/` — the project's own, not hex's fallbacks.
- Spec home: `.agents/specs/` — the fold-back target; default ID-marker
  heading shape (no project-specific marker declared).
- Plan Status block: every `.agents/plans/plan_*.md` carries one; schema
  and per-skill mutation table in `.claude/rules/meta-ai-config.md` ›
  "Plan Status Protocol". Fast-path pointer `.claude/state/current_plan.md`
  is gitignored and per-worktree.
- Product knowledge: `.claude/rules/product-context.md` (identity, related
  repos; indexed from `AGENTS.md` › "Project Identity").
- Rule catalog: `.claude/rules.md` — "By concern" table routes to the rule
  for any given task before a file is open. Read it before planning.
- Key rules: `.claude/rules/arch-principles.md` (boundaries),
  `.claude/rules/subsystem-shell.md` and `subsystem-web.md` (the two code
  areas), `.claude/rules/arch-threat-model.md` (whom the IPC bridge defends
  against), `.claude/rules/quality-security.md` for security-sensitive
  surfaces.
- Security-sensitive paths: `src-tauri/**` (bridge commands, capabilities,
  CSP, the spawned grim), `src-tauri/tauri.conf.json`,
  `src-tauri/capabilities/**`, `index.html`, `.github/workflows/**`. See the
  `perspectives.always` rules below.
- Constitution: `AGENTS.md` › "Core Principles" — nine binding principles.
  Principle 9 (Respect grim's Contracts): this app consumes grim's frozen
  CLI and JSON contracts and passes grim's JSON on unchanged.
- Scaffold provenance: `scaffold-manifest.toml` and
  `tools/scaffold_manifest.py`; every copied file names its source commit.
- Discussions: `.agents/discussions/` (hex default) — `/hex-discuss`
  artifacts; `State: active` there arms the hex-state no-edit rule.
- Worktrees: agent worktrees at the hex default `.agents/worktrees/`
  (gitignored); human feature worktrees are siblings `../grim-gui-wt-<topic>`
  (`AGENTS.md` › "Workflow"). Whoever creates one removes it.

## Preferences

```yaml
# hex config, vocabulary v2. Unknown keys warn once and are ignored.
models:
  fast-balanced: sonnet
  deep-reasoning: opus
  overrides:
    # Mirrors the owner's global model-routing policy: review and
    # non-mechanical implementation run deep at every tier.
    reviewer:security: deep-reasoning
    reviewer:quality: deep-reasoning
    reviewer:spec: deep-reasoning
    builder:implement: deep-reasoning
adversary: codex:rescue

perspectives:
  always:
    # The IPC bridge: every command, capability and the CSP is a privilege.
    - role: reviewer:security
      when: "src-tauri/**"
    - role: reviewer:security
      when: "index.html"
    # Workflows run with tokens and build untrusted dependencies.
    - role: reviewer:security
      when: ".github/workflows/**"

research-axes:
  - Tauri 2 (IPC, capabilities, CSP, platform webviews, packaging)
  - webview security and plain-TypeScript front ends
  - grim JSON interface (what a screen can read, additive evolution)
  - cross-platform desktop CI (Linux, macOS, Windows builds)
```

- Security review here means the bridge, capabilities, CSP and the spawned
  grim named under Pointers — not the vitest code or the stylesheet.
- Contract fidelity is a review perspective in its own right: any diff that
  reshapes grim's JSON or maps one of grim's exit codes is checked against
  Principle 9 before anything else, regardless of tier.
- Codex reviews pass the model explicitly: `codex exec -m gpt-6.1-sol …`
  direct, `--model gpt-6.1-sol` on `codex:rescue`, because the global codex
  default is a different model. `nox-review` has no model flag and inherits
  that default.

## Memory

(empty — a new repository starts without history.)
