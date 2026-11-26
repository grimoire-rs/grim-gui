---
paths:
  - test/**
---

# Test Subsystem

Black-box acceptance tests under `test/`: a small pytest suite that starts the real `grim-gui`
(through `grim gui` plugin dispatch, or directly) under a virtual display and reads the stderr
lines the bridge writes. Shell unit tests live in `src-tauri/` (nextest) and front-end tests next
to `src/` (vitest); this suite proves the assembled app starts, runs its script under the CSP and
reaches the bridge.

## Design Rationale

Pytest, not a WebDriver: `tauri-driver` runs on Linux and Windows only and is heavy for one page.
The suite runs inside the Linux build container (`scripts/in-container.sh`) under
`dbus-run-session xvfb-run -a` with `WEBKIT_DISABLE_DMABUF_RENDERER=1`, `LIBGL_ALWAYS_SOFTWARE=1`,
`GDK_BACKEND=x11` and a private `XDG_RUNTIME_DIR`. The same harness idea as grimoire's suite
(runner, binary lookup, floor and ceilings, meta suites), reduced to what a GUI needs.

## Structure

| Path | Purpose |
|------|---------|
| `test/conftest.py` | session fixtures: the `grim` and `grim-gui` binaries (`test/bin/`), the isolated environment |
| `test/src/` | the runner that starts the app, waits for a stderr line and stops the process |
| `test/tests/test_*.py` | the cases: `grim gui` ok, direct launch with grim absent, a fake grim that exits 78, the missing-docker check |
| `test/tests/test_accept_floor.py`, `test_accept_shard.py`, `test_text_io_hygiene.py` | meta suites for the tools and the text I/O rule |
| `test/tools/` | `accept_floor.py`, `accept_shard.py`, `require_all_passed.py` |
| `test/SUITE_FLOOR`, `SKIP_CEILING`, `XFAIL_CEILING` | counts enforced by `task test:floor` in PR CI |

## Scenarios the suite pins

| Case | Setup | Expected |
|---|---|---|
| ok | `grim gui` with `grim-gui` on `PATH` and the staged grim | stderr line `grim-gui: grim_version ok version=<v>` within 30 s, `<v>` = `grim --version` minus its prefix |
| not found | `grim-gui` direct, grim off `PATH`, `GRIM_BIN` unset | `not-found`, and the process keeps running until stopped |
| failed | `GRIM_BIN` names a fake grim that exits 78 | `failed code=69` |
| no docker | `scripts/in-container.sh true` with `docker` off `PATH` | exit 69 and `docker not found` |

The process is terminated after the line is seen. Skip ceiling 0, xfail ceiling 0.

## Rules

- Text I/O names its codec and newline: `test_text_io_hygiene.py` fails on a bare `read_text`,
  `write_text` or text-mode `open` under `test/` (except `.venv/` and `data/`).
- The staged grim comes from `GRIM_SOURCE_BINARY` (default: the one built in the grimoire
  checkout); a missing file fails the task and names the variable. Never use the host's grim.
- A case never leaves a process behind: stop the app in a `finally`, by PID.
- No sleep-and-hope: wait on the stderr line with a deadline.
- Never weaken a floor or a ceiling to make a run pass.

## Running

```bash
task test:accept   # stage grim (test:stage), run the suite in the container under xvfb
task test:floor    # enforce the floor and the ceilings against a junit report
```

Single test: `scripts/in-container.sh uv run --directory test pytest tests/<file>::<test> -v`.

## Quality Gate

During review-fix loops run `task test:accept`, not full `task verify`. See
[`quality-python.md`](./quality-python.md) for the Python rules.
