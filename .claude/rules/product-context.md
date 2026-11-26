---
paths:
  - '.agents/plans/**'
  - '.agents/adr/**'
  - '.agents/specs/**'
  - '.agents/research/**'
---

# grim-gui Product Context

> The desktop shell of Grimoire: a window onto what `grim` already does.

> **Status: skeleton, Unstable.** One bridge command shows the version of the
> grim it finds, or "grim not found". Statements below are maintained
> positioning — flag drift via the Update Protocol at the bottom of this file.

`grim-gui` is a Tauri 2 desktop application for people who prefer a window to
a terminal when they install, inspect and update AI-agent configuration
(skills, rules, agents, MCP servers) with Grimoire. It is started by
`grim gui` through grim's plugin dispatch, or launched directly.

## The Problem

`grim` is a CLI. The VS Code extension serves editor users; a terminal-averse
user, or a user with no VS Code, has no graphical way to browse, install and
update packages. The GUI fills that gap without a second implementation of
grim.

## Principles

1. **Only the binary.** The app runs `grim` and reads its JSON. It links no
   grim code, so grim's frozen CLI and JSON contracts are the whole interface.
2. **Pass grim's JSON on unchanged.** A bridge command returns grim's
   `serde_json::Value`; the front end renders it. No second schema.
3. **Least privilege.** The webview may call the commands this repository
   lists, nothing else (`subsystem-shell.md`).
4. **Plain TypeScript.** No UI framework, so grim's JSON and the VS Code
   extension's renderers carry over.

## Target Users

- **Primary**: engineers who manage AI-agent configuration and want a GUI.
- **Non-target**: users who need the GUI to work without grim installed. The
  app finds grim, or says "grim not found".

## Technical Overview

- **Language**: Rust 2024 shell (`src-tauri/`), TypeScript front end (`src/`)
- **Layout**: cargo workspace with one member, package and binary `grim-gui`
- **grim access**: `crates/grim-sdk` of the grimoire repository, pinned by
  commit; resolves `GRIM_BIN` or `grim` on `PATH`
- **Testing**: nextest for the shell, vitest for the front end, a black-box
  pytest suite that starts `grim gui` under xvfb

## Related Repositories

| Repo | Relationship |
|---|---|
| `grimoire-rs/grimoire` | The `grim` CLI and the `grim-sdk` crate this app depends on; the scaffold source (see `scaffold-manifest.toml`) |
| `grimoire-rs/grim-mirror` | Sibling tool repository from the same split, same scaffold |
| `grimoire-rs/vscode` | VS Code extension; renders the same grim JSON, source of `artifact-kinds.md` and `grim-compat-markers.md` |
| `grimoire-rs/indexer` | Scaffolds package-index repositories |
| `arcana` | The `hex` and `nox` bundles installed here |

## Update Protocol

This file is the single source of truth for grim-gui product identity.

**When to update** — any of these triggers an edit in the same commit:

1. A screen or capability is added that changes what the app is for
2. A principle is added, dropped or reworded
3. The related-repository set changes

**Who must check** — a researcher after evaluating a library, an architect
after an ADR, a doc writer after README edits.
