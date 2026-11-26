# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 The Grimoire Authors
"""Binaries under test, staged by `task test:accept` into test/bin/."""

from __future__ import annotations

import os
import sys
from pathlib import Path

import pytest

BIN_DIR = Path(__file__).resolve().parent / "bin"


def _exe(name: str) -> Path:
    return BIN_DIR / (name + (".exe" if sys.platform == "win32" else ""))


@pytest.fixture(scope="session")
def grim_binary() -> Path:
    """The grim built elsewhere and copied here (GRIM_COMMAND overrides)."""
    path = Path(os.environ.get("GRIM_COMMAND") or _exe("grim"))
    if not path.is_file():
        pytest.exit(
            f"grim binary missing at {path}: run `task test:accept`", returncode=4
        )
    return path


@pytest.fixture(scope="session")
def gui_binary() -> Path:
    path = Path(os.environ.get("GRIM_GUI_COMMAND") or _exe("grim-gui"))
    if not path.is_file():
        pytest.exit(
            f"grim-gui binary missing at {path}: run `task test:accept`", returncode=4
        )
    return path


# A parametrize id that embeds a large payload stalls CI reports.
_MAX_TEST_ID = 1000


def pytest_collection_modifyitems(items: list[pytest.Item]) -> None:
    """Refuse to run when a test id is long enough to bloat CI output."""
    long_ids = [i.nodeid[:80] + "…" for i in items if len(i.nodeid) > _MAX_TEST_ID]
    if long_ids:
        raise pytest.UsageError(
            f"test ids over {_MAX_TEST_ID} chars (give the parameter an explicit id): {long_ids}"
        )
