---
paths:
  - "src-tauri/**"
---

# Subsystem: Tauri shell (`src-tauri/`)

The Rust side of the app: package and binary `grim-gui`, library `grim_gui_lib`. It hosts the
webview, exposes the bridge commands and reaches grim only through `grim-sdk`. Threat model:
[`arch-threat-model.md`](./arch-threat-model.md).

## Layout

| File | Role |
|---|---|
| `src/main.rs` | entry; calls `grim_gui_lib::run()`, maps an error to exit 1, no `expect` |
| `src/lib.rs` | `run() -> tauri::Result<()>`: builder, `invoke_handler`, `generate_context!` |
| `src/bridge.rs` | bridge commands and the plain functions behind them |
| `build.rs` | `AppManifest::commands(&[...])`: the one list of callable commands |
| `capabilities/default.json` | grants `allow-<command>` to window `main`; no `core:default`, no plugin |
| `tauri.conf.json` | `identifier` `rs.grimoire.gui`, `productName` `grim-gui`, CSP, `frontendDist` `../dist` |
| `tests/config.rs`, `tests/boundary.rs` | pin the security configuration and the dependency tree |

## Invariants

1. **Commands wrap plain functions.** `#[tauri::command] fn grim_version()` calls
   `version_with(&GrimDriver)`; unit tests pass a driver pointing at a fake grim script. The
   deadline is the SDK's and is tested there.
2. **One wire error.** `BridgeError` derives `Serialize` with `#[serde(tag = "kind",
   rename_all = "kebab-case")]`: `{"kind": "not-found" | "failed", "code": int|null, "message": str}`.
   `code: None` from the SDK (grim could not start) is `not-found`; any other `DriverError` is `failed`.
3. **One stderr line per call**: `grim-gui: grim_version ok version=<v>`, `… not-found`,
   `… failed code=<n>`. The acceptance suite matches these lines; change them with the suite.
4. **The R12 convention.** Every later command that returns grim JSON returns grim's
   `serde_json::Value` unchanged: no struct re-declaring grim's schema, no renamed or dropped
   field, no wrapping envelope. A command that needs a shape grim does not provide goes to the
   grimoire repository first. `grim_version` is the one exception: grim prints its version as
   text, so it returns the string `GrimDriver::version()` gives.
5. **Least privilege.** A new command is added in three places in one change: `build.rs`,
   `capabilities/default.json` (`allow-<name>`), `tests/config.rs`. No `tauri-plugin-*`, no
   `core:default`. If `invoke` fails at run time for want of a core permission, add the narrowest
   one and record it as a deviation in `scaffold-manifest.toml`.
6. **CSP is fixed**: `default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self'
   data:; connect-src ipc: http://ipc.localhost`. `tests/config.rs` asserts the string.
7. **Only `grim-sdk` reaches grim.** No `std::process::Command` here; `tests/boundary.rs` asserts
   the tree names `grim-sdk` and no other grim crate (`grimoire`, `grim-hook`, `grimoire-hook-core`).
8. **`unsafe_code = "forbid"` holds** (workspace lint) on the Linux build. macOS is unproven until
   CI. Narrow fallback, only on evidence of a failing build: `src-tauri/Cargo.toml` sets its own
   `[lints.rust] unsafe_code = "deny"` and `lib.rs` puts `#[allow(unsafe_code)]` on the single
   `run()` that holds `generate_context!`, with a one-line why. Record it as a manifest deviation.
9. **`tauri` and `tauri-build` are pinned with `=`**; npm `@tauri-apps/*` move with them
   (dependabot groups them).

## Testing

- `cargo nextest` unit tests with fake grim scripts (unix): ok, not found, exit 78, garbage output.
- `tests/config.rs` parses `tauri.conf.json` and `capabilities/default.json`.
- The end-to-end proof is the black-box suite in `test/` ([`subsystem-tests.md`](./subsystem-tests.md)).
- A new bridge command needs a unit test, an acceptance case and a README update in the same change.
- On the Linux host every Rust command runs through `scripts/in-container.sh`.

## Cross-references

- [`subsystem-web.md`](./subsystem-web.md) — the front end that calls these commands
- [`arch-principles.md`](./arch-principles.md) — design principles
- [`quality-rust.md`](./quality-rust.md), [`quality-rust-errors.md`](./quality-rust-errors.md)
