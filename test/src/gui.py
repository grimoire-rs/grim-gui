# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 The Grimoire Authors
"""Start grim or grim-gui as a child process and read its stderr lines."""

from __future__ import annotations

import os
import queue
import subprocess
import tempfile
import threading
import time
from collections.abc import Sequence
from pathlib import Path

# WebKit in a container: no GPU, no DMABUF, X11 only (the suite runs under xvfb).
WEBKIT_ENV = {
    "WEBKIT_DISABLE_DMABUF_RENDERER": "1",
    "LIBGL_ALWAYS_SOFTWARE": "1",
    "GDK_BACKEND": "x11",
}


def gui_env(
    path: Sequence[Path | str], extra: dict[str, str] | None = None
) -> dict[str, str]:
    """A scrubbed environment: given PATH, no GRIM_*, a private XDG_RUNTIME_DIR."""
    runtime = tempfile.mkdtemp(prefix="grim-gui-xdg-")
    os.chmod(runtime, 0o700)
    env = {
        k: v
        for k, v in os.environ.items()
        if k in {"DISPLAY", "XAUTHORITY", "DBUS_SESSION_BUS_ADDRESS", "HOME", "LANG"}
    }
    env.update(WEBKIT_ENV)
    env["PATH"] = os.pathsep.join(str(p) for p in path)
    env["XDG_RUNTIME_DIR"] = runtime
    env.update(extra or {})
    return env


class Proc:
    """A child whose stderr is read line by line on a thread."""

    def __init__(self, argv: Sequence[Path | str], env: dict[str, str]) -> None:
        self.lines: list[str] = []
        self._queue: queue.Queue[str | None] = queue.Queue()
        self.proc = subprocess.Popen(
            [str(a) for a in argv],
            env=env,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
        )
        threading.Thread(target=self._pump, daemon=True).start()

    def _pump(self) -> None:
        assert self.proc.stderr is not None
        for line in self.proc.stderr:
            self._queue.put(line.rstrip("\n"))
        self._queue.put(None)

    def wait_line(self, prefix: str, timeout: float = 30.0) -> str:
        """The first stderr line starting with ``prefix``; fails with all lines seen on timeout."""
        deadline = time.monotonic() + timeout
        while True:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise AssertionError(
                    f"no stderr line starting {prefix!r} in {timeout}s; saw: {self.lines}"
                )
            try:
                line = self._queue.get(timeout=remaining)
            except queue.Empty:
                continue
            if line is None:
                raise AssertionError(
                    f"process ended before {prefix!r}; saw: {self.lines}"
                )
            self.lines.append(line)
            if line.startswith(prefix):
                return line

    def stop(self) -> None:
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait()
