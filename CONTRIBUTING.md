# Contributing to grim-gui

## Prerequisites

- **Rust** (edition 2024) — install via [rustup](https://rustup.rs/); the toolchain is pinned in `rust-toolchain.toml`
- **[task](https://taskfile.dev)** — primary task runner (`brew install go-task` or see docs)
- **[ocx](https://ocx.sh)** — installs the pinned tools of `ocx.toml` (Node 24, uv, cargo-deny, actionlint, …)
- **[Docker](https://docs.docker.com/get-docker/)** — on Linux, the Rust build and the acceptance suite run in a container, so the host needs no webkit2gtk
- **A built `grim`** — the app and its tests need the `grim` binary of the grimoire repository (see below)

## Layout

| Path | Purpose |
|------|---------|
| `src-tauri/` | The Rust shell: package and binary `grim-gui`, the bridge commands, the Tauri configuration |
| `src/`, `index.html` | The plain TypeScript front end, bundled by Vite into `dist/` |
| `test/` | Python (pytest) black-box acceptance suite: starts the real app under a virtual display |
| `docker/`, `scripts/` | The Linux build container and the wrapper that runs commands in it |
| `.claude/`, `.agents/` | AI-assisted development config (rules, skills, hooks) and planning artifacts |
| `taskfiles/` | Task automation modules |

## Building

```sh
task rust:build                # release build in the container: target/release/grim-gui
task web:build                 # front end only: dist/
task run                       # app against the Vite dev server
```

The shell depends on the `grim-sdk` crate of the grimoire repository, pinned by
revision in the root `Cargo.toml`.

## Running Tests

```sh
task web:test                  # front-end unit tests (vitest)
task rust:test:unit            # shell unit tests (nextest, in the container)
task test:accept               # black-box suite under xvfb, in the container
```

The acceptance suite stages a grim binary from `GRIM_SOURCE_BINARY` (default:
the release build in the sibling grimoire checkout). Set the variable to use
another build. Do not use a released `grim` older than the one that ships
`grim gui` plugin dispatch.

## Code Style

```sh
cargo fmt              # format (max_width=120, see rustfmt.toml)
task rust:clippy:check # clippy in the container
task web:lint          # tsc --noEmit
```

Format before every commit. CI enforces all of it.

## Commit Conventions

All commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: show the installed artifacts
fix: report a failing grim with its exit code
refactor: extract the error renderer
ci: add the macOS build
```

Scopes are optional. cocogitto validates commit messages in CI.

Every commit also needs a `Signed-off-by` line — see [License](#license).
The `-s` flag of `git commit` adds it for you.

## Branch Model

- Branch from `main` — never commit directly to `main`.
- Keep commits atomic and complete — no WIP commits on shared branches.

## Before Submitting

```sh
task verify    # fmt check + clippy + shell, AI-config, type and workflow lint, then build + unit tests + vitest + acceptance
```

All checks must pass before opening a pull request.

## License

grim-gui is licensed under the [Apache License, Version 2.0](LICENSE), and
contributions are accepted under that same license — inbound matches outbound,
as Apache-2.0 §5 already presumes. Nothing you contribute is relicensed, and
you keep the copyright in your own work.

**There is no CLA.** Instead, sign off your commits under the
[Developer Certificate of Origin](https://developercertificate.org/) — a
one-line statement that you wrote the patch, or otherwise have the right to
submit it under this license. The `-s` flag of `git commit` appends
`Signed-off-by: Your Name <you@example.com>`.

The name and email must be real, and the sign-off address must match the one
that authored the commit. If you are contributing work owned by an employer,
make sure you have their permission before you sign off.

CI checks this on every pull request. If you forget, `git rebase --signoff
main..HEAD` fixes the whole branch at once. Run it yourself with:

```sh
task git:dco                       # checks main..HEAD
```

The copyright holder named in `LICENSE` is **The Grimoire Authors** — that is
every person with a commit in this repository, as listed by:

```sh
git shortlog -sne
```

No separate contributor list is maintained; git history is the record.
