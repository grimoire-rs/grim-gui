# Acceptance tests

Black-box pytest suite that starts the built `grim-gui` (directly, and through
`grim gui` plugin dispatch) as an external process under a virtual display and
reads the stderr lines the bridge writes. It asserts only on those lines, exit
codes and process state.

## Layout

- `conftest.py`: the `grim_binary` and `gui_binary` session fixtures
  (`GRIM_COMMAND` / `GRIM_GUI_COMMAND`, else `bin/`), and a cap on test id length.
- `tests/`: `test_gui.py` plus the harness meta tests.
- `src/gui.py`: `gui_env` (scrubbed environment, private `XDG_RUNTIME_DIR`) and
  `Proc` (stderr reader with a deadline).
- `tools/`: `accept_shard.py`, `accept_floor.py`, `require_all_passed.py`.
- `bin/` (gitignored): the staged `grim` and `grim-gui` binaries.

## Running

```sh
task test:accept   # stage grim (GRIM_SOURCE_BINARY) and grim-gui, run under xvfb in the build container
task test:floor    # judge results/junit-local-0-of-1.xml against the floor and ceilings
```

## Isolation

Every case gets a scrubbed environment: given `PATH`, no `GRIM_*` variables, a
private `XDG_RUNTIME_DIR`. WebKit runs without GPU or DMABUF on X11. A missing
binary aborts the session with exit 4 instead of skipping to green. No registry
is involved: the suite never leaves the machine.

## Limits

`SUITE_FLOOR`, `SKIP_CEILING` and `XFAIL_CEILING` hold the suite size and the
most skips and xfails CI accepts. Text I/O under `test/` names its encoding and
newline (`tests/test_text_io_hygiene.py` enforces it).
