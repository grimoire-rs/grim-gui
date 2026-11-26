---
paths:
  - src-tauri/**/*.rs
---

# grim-gui Architecture Principles

Auto-loads on every Rust file edit under `src-tauri/`. Provides stable
architectural context, the "why" behind the design. For dynamic discovery of
current code state, launch an architecture-explorer worker (hex spawns one
during Discover).

## Layout

grim-gui is a **cargo workspace with one member**, `src-tauri` (package and
binary `grim-gui`, library `grim_gui_lib`). The front end is plain TypeScript
in `src/`, bundled by Vite into `dist/` and embedded by `generate_context!`.
Black-box acceptance tests live under `test/`.

```
grim gui ─▶ grim-gui (Tauri process)
              ├─ webview ── src/main.ts ── invoke("grim_version")
              └─ shell (src-tauri) ── bridge.rs ── grim_sdk::GrimDriver ── spawns `grim`
```

## Design Principles

| Principle | Intent |
|-----------|--------|
| **Invoke, never link** | The app reaches grim only through the `grim` binary, by way of `grim-sdk`; nothing links grim's code |
| **Thin bridge** | A command is a few lines over a `grim-sdk` call; logic lives in a plain function the unit tests call with a fake grim |
| **Pass JSON on unchanged** | A command that returns grim JSON returns grim's `serde_json::Value`; the front end reads grim's schema, not ours |
| **Least privilege** | One named command per capability grant; no `core:default`, no plugin, a strict CSP |
| **Typed errors, one wire shape** | `BridgeError` serializes as `{"kind", "code", "message"}`; the front end switches on `kind` |

## Boundaries

| Boundary | Rule |
|---|---|
| shell to grim | only `grim-sdk`; `src-tauri/tests/boundary.rs` pins the dependency tree |
| webview to shell | only commands in `build.rs` and `capabilities/default.json` |
| webview content | own bundle only (CSP `'self'`); no remote content, no inline script |

## Code Style

- `#[tauri::command]` functions wrap a plain `pub fn` taking the driver as a
  parameter, so a unit test passes a fake grim script.
- No `unwrap` or `expect` in non-test code (workspace lint); `run()` returns
  `tauri::Result<()>` and `main` maps an error to exit 1.
- Each call writes one stderr line (`grim-gui: <command> <outcome>`); the
  acceptance suite reads these lines.

## Where New Features Land

| Feature type | Where it lands |
|--------------|----------------|
| New grim call | a bridge function and command in `src-tauri/src/bridge.rs`, its entry in `build.rs` and `capabilities/default.json`, a renderer in `src/view.ts` |
| New screen | `src/` module plus a vitest case; the page stays CSP-clean |
| Any new `grim-sdk` need | a change in the grimoire repository first (Principle 9) |

## ADR Index

Architecture decisions are recorded as `.agents/adr/adr_*.md`. Read
the relevant ADRs before making decisions in the same domain. The repository
split that created this repository is recorded in grimoire's
`adr_repository_split.md`; none is local yet.

| ADR | Decision |
|-----|----------|
