<!-- doc_type: readme -->

# grim-gui

The desktop shell of [Grimoire](https://github.com/grimoire-rs/grimoire): a
[Tauri 2](https://tauri.app) application that runs next to the `grim` binary.
It is started by `grim gui`, or directly as `grim-gui`.

> **Status: skeleton.** The window shows the version of the `grim` it finds, or
> the text "grim not found". Screens, packaging, signing and auto-update are
> not built yet.

## How it works

The app only invokes `grim`. Nothing links grim's code. A Rust command in
`src-tauri/` asks the [`grim-sdk`](https://github.com/grimoire-rs/grimoire/tree/main/crates/grim-sdk)
crate to run `grim`. A plain TypeScript front end in `src/` renders the
result. grim is found through `GRIM_BIN` (an absolute path), else `grim` on
`PATH`.

## Build and run

You need [task](https://taskfile.dev), [ocx](https://ocx.sh) (it installs Node 24
and the other pinned tools from `ocx.toml`), a built `grim`, and, on Linux,
Docker. The Linux build runs in a container, so the host needs no webkit2gtk.

```sh
task rust:build           # builds the front end, then target/release/grim-gui
```

Run it against a grim you built from the grimoire repository:

```sh
GRIM_BIN=/absolute/path/to/grim ./target/release/grim-gui
```

or let grim start it, with `grim-gui` on `PATH`:

```sh
grim gui
```

Without a `grim` on `PATH` or in `GRIM_BIN` the window says "grim not found".
For front-end work, `task run` starts the app against the Vite dev server.

On macOS and Windows, build with the Tauri CLI from the repository root:

```sh
npm ci
npx tauri build --no-bundle
```

## Test

```sh
task verify               # the full gate
task web:test             # front-end unit tests (vitest)
task test:accept          # black-box suite: starts the real app under xvfb, in the container
```

The acceptance suite needs a `grim` that has `grim gui` plugin dispatch. It
takes it from `GRIM_SOURCE_BINARY`, by default the release build of the sibling
grimoire checkout.

## Project layout

| Path | Purpose |
|------|---------|
| `src-tauri/` | Rust shell: bridge commands, capabilities, content security policy |
| `src/`, `index.html` | Plain TypeScript front end |
| `test/` | Black-box acceptance suite |
| `docker/`, `scripts/` | The Linux build container |

For agent and contributor guidance see [AGENTS.md](AGENTS.md) and
[CONTRIBUTING.md](CONTRIBUTING.md). To report a vulnerability see
[SECURITY.md](SECURITY.md).

## License

Apache-2.0. See the `LICENSE` file.
