# Tech Strategy - Golden Paths

**SINGLE SOURCE OF TRUTH** for tech choices this project.

## Compliance

1. **Follow This File**: Use tech listed below
2. **No Deviations**: No alternatives unless told
3. **Latest Stable**: Latest stable version unless pinned

## Language Golden Paths

### Rust (Shell)

| Component | Choice |
|-----------|--------|
| Edition | Rust 2024 |
| Toolchain | 1.95.0, pinned in `rust-toolchain.toml` |
| Desktop framework | Tauri 2, `tauri` and `tauri-build` pinned with `=` |
| grim access | the `grim-sdk` crate only; nothing links grim's code |
| Runtime | no async runtime of our own; Tauri's |

**Deviation: Tauri 2 with a TypeScript front end.** The golden path of the
sibling repositories is Rust-only (Tokio, Mold). This repository is a desktop
shell, so it carries a Rust shell in `src-tauri/` and a plain TypeScript front
end in `src/`. The shell has no `tauri-plugin-*` dependency and grants the
webview one command (see `subsystem-shell.md`). Rationale: the owner's
decision in the repository-split discussion; the front end is plain
TypeScript so grim's JSON reaches it unchanged.

### TypeScript (Front end)

| Component | Choice |
|-----------|--------|
| Runtime | Node 24 (pinned in `ocx.toml`, `engines.node` `>=24`) |
| Bundler | Vite |
| Tests | vitest with happy-dom |
| Framework | none; plain TypeScript, no UI library |
| Dependencies | `@tauri-apps/api` only at run time; exact versions, `npm ci` |

### Python (Acceptance Tests)

| Component | Choice |
|-----------|--------|
| Runtime | Python 3.13+ |
| Tooling | uv (Manager), Ruff (Linter) |
| Testing | pytest, black-box, under xvfb in the build container |

## Infrastructure

| Component | Choice |
|-----------|--------|
| Linux build and test | Docker image `docker/linux-build.Dockerfile` (the host needs no webkit2gtk) |
| Secrets | GitHub Secrets |

## CI/CD

| Component | Choice |
|-----------|--------|
| Platform | GitHub Actions |
| Auth | OIDC |
| Build | `npx tauri build --no-bundle` on Linux, macOS and Windows; `tauri-action` is not used |
| Workflow lint | actionlint, pinned in `ocx.toml` |
