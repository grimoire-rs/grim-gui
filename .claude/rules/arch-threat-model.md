---
paths:
  - "src-tauri/**"
  - "src/**"
  - "index.html"
---

# grim-gui Threat Model

**The trust boundary, stated once.** Every security finding in this repository is scoped by this
file. Without a written boundary, reviews churn: the same finding gets raised as a Block by one
reviewer and dismissed as a non-goal by the next, and neither is wrong because nobody said which
attacker we defend against.

Read this **before** filing or triaging a security finding, and before adding a bridge command,
a capability, a plugin or any content the webview loads. Companion:
[`quality-security.md`](./quality-security.md) (the OWASP/STRIDE checklist and attack surfaces).
This file says *whom* we defend against; that one says *how*. The model for grim itself lives in
the grimoire repository (`arch-threat-model.md` there); this file covers only what a desktop shell
adds.

## What grim-gui is

A Tauri 2 desktop app: a native process hosting a webview, which can call named Rust commands
(IPC). The shell spawns the user's `grim` binary through `grim-sdk` and returns what it prints.
Its risk class is a **local app with a privileged bridge**: whatever runs in the webview can
call whatever the bridge exposes.

## In scope — attackers we defend against

| ID | Attacker | What they control | What we must not do |
|---|---|---|---|
| T1 | **Content in the webview** | any text or markup that reaches the DOM: grim JSON fields (package names, descriptions, registry strings), error messages | execute it, or let it reach the IPC with more rights than the page has |
| T2 | **A page the webview navigates to** | a remote origin, if navigation or an external link ever loads | give it IPC access or the capability set |
| T3 | **Whoever sets the environment of the launch** | `GRIM_BIN`, `PATH` | treat the resolved binary as trusted code beyond "the user's grim"; never run it with elevated rights or from a relative path |
| T4 | **Output of the spawned grim** | stdout, stderr, exit code | trust its shape: parse as data, bound its size and time, never interpolate it into a shell or into `innerHTML` |
| T5 | **A malicious dependency** (npm or crates.io) | a transitive update | pull it in unreviewed: exact versions, committed lockfiles, `cargo deny`, `npm audit` in CI |

## Out of scope — attackers we deliberately do not defend against

| ID | Non-goal | Why |
|---|---|---|
| N1 | **Insiders with commit access** | they can change this file and the capability set; review is the control |
| N2 | **A compromised user account or OS** | the app runs as the user; anything that already is the user wins |
| N3 | **A hostile `grim` the user chose to run** | `GRIM_BIN` names the user's own binary; the grimoire threat model covers what grim does |
| N4 | **Physical access, screen capture, other local processes** | not a property of this app |
| N5 | **Update and signing trust** | no updater and no signed release exist yet; they get their own model when they land |

## Invariants

| ID | Invariant | Pinned by |
|---|---|---|
| I1 | The webview calls only commands named in `build.rs`; `capabilities/default.json` grants exactly those to window `main`; no plugin, no `core:default` | `src-tauri/tests/config.rs` |
| I2 | The CSP is `default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src ipc: http://ipc.localhost`. `index.html` has no inline script or style | `src-tauri/tests/config.rs`, vitest |
| I3 | The front end writes grim-derived text with `textContent` (or an equivalent that does not parse markup), never `innerHTML` | vitest case with markup in a field |
| I4 | The shell reaches grim only through `grim-sdk`: no `std::process::Command` of our own, no dependency on a grim crate | `src-tauri/tests/boundary.rs` |
| I5 | A failure to start grim is reported as `not-found` with the OS error; a non-zero exit is reported as `failed` with grim's code. The shell never turns one into the other and never invents success | bridge unit tests |
| I6 | No secret is read, stored or logged. The bridge writes one stderr line per call, carrying the outcome and the version or code only | bridge unit tests |
| I7 | No remote content: the app loads its own bundle only; opening an external link goes to the system browser, never into the webview | review; CSP |

## Applying it

1. Name the attacker (T1 to T5) the finding needs. None fits: it is a non-goal or a hardening
   suggestion, not a Block.
2. Check the invariant (I1 to I7) it breaks. A finding that breaks one is a Block.
3. A change that adds a command, a capability, a plugin or a way to load remote content
   updates this file in the same commit.

## See also

- [`quality-security.md`](./quality-security.md) — the checklist
- [`subsystem-shell.md`](./subsystem-shell.md), [`subsystem-web.md`](./subsystem-web.md) — the code the invariants live in
