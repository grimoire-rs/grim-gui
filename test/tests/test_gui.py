# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 The Grimoire Authors
"""S-005 to S-008: the GUI starts, finds grim, and reports what it finds."""

from __future__ import annotations

import os
import stat
import subprocess
from pathlib import Path

from src.gui import Proc, gui_env

SYSTEM_PATH = ["/usr/bin", "/bin"]
REPO_ROOT = Path(__file__).resolve().parents[2]


def _version(grim: Path) -> str:
    out = subprocess.run(
        [str(grim), "--version"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    )
    return out.stdout.strip().removeprefix("grim ")


def test_grim_gui_subcommand_shows_the_found_version(
    grim_binary: Path, gui_binary: Path, tmp_path: Path
) -> None:
    """S-005: plugin dispatch starts grim-gui, which reports the dispatching grim's version.

    A decoy grim with another version comes first on PATH: only GRIM_BIN, set by the
    dispatch, can make the app report the real one.
    """
    assert grim_binary.parent == gui_binary.parent, (
        "both binaries are staged in test/bin"
    )
    decoy_dir = tmp_path / "decoy"
    decoy_dir.mkdir()
    decoy = decoy_dir / "grim"
    decoy.write_text(
        "#!/bin/sh\necho 'grim 0.0.0-decoy'\n", encoding="utf-8", newline="\n"
    )
    decoy.chmod(decoy.stat().st_mode | stat.S_IXUSR)
    assert _version(grim_binary) != "0.0.0-decoy"
    env = gui_env(
        [decoy_dir, grim_binary.parent, *SYSTEM_PATH],
        {"GRIM_HOME": str(grim_binary.parent / "home")},
    )
    proc = Proc([grim_binary, "gui"], env)
    try:
        line = proc.wait_line("grim-gui: grim_version ")
        assert line == f"grim-gui: grim_version ok version={_version(grim_binary)}"
    finally:
        proc.stop()


def test_without_grim_the_app_reports_not_found_and_stays_open(
    gui_binary: Path,
) -> None:
    """S-006: no grim on PATH, GRIM_BIN unset."""
    proc = Proc([gui_binary], gui_env(SYSTEM_PATH))
    try:
        assert (
            proc.wait_line("grim-gui: grim_version ")
            == "grim-gui: grim_version not-found"
        )
        try:
            proc.proc.wait(timeout=2)
        except subprocess.TimeoutExpired:
            pass
        assert proc.proc.poll() is None, "the app exited after reporting not-found"
    finally:
        proc.stop()


def test_a_failing_grim_is_reported_as_failed(gui_binary: Path, tmp_path: Path) -> None:
    """S-007: GRIM_BIN names a grim that exits 78; the SDK folds that into 69."""
    fake = tmp_path / "grim"
    fake.write_text("#!/bin/sh\nexit 78\n", encoding="utf-8", newline="\n")
    fake.chmod(fake.stat().st_mode | stat.S_IXUSR)
    proc = Proc([gui_binary], gui_env(SYSTEM_PATH, {"GRIM_BIN": str(fake)}))
    try:
        assert (
            proc.wait_line("grim-gui: grim_version ")
            == "grim-gui: grim_version failed code=69"
        )
    finally:
        proc.stop()


def test_in_container_wrapper_refuses_without_docker(tmp_path: Path) -> None:
    """S-008: with docker off PATH the wrapper exits 69 and says so."""
    bash = next(p for p in ("/usr/bin/bash", "/bin/bash") if os.path.exists(p))
    env = {"PATH": str(tmp_path)}  # an empty directory: no docker, no coreutils
    out = subprocess.run(
        [bash, str(REPO_ROOT / "scripts" / "in-container.sh"), "true"],
        env=env,
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=False,
    )
    assert out.returncode == 69
    assert "docker not found" in out.stderr
