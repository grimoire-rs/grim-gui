// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

//! The GUI links `grim-sdk` and no other grim crate (C-029).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;
use std::process::Command;

#[test]
fn dependency_tree_has_the_sdk_and_no_other_grim_crate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let out = Command::new(cargo)
        .current_dir(&root)
        .args([
            "tree",
            "-p",
            "grim-gui",
            "-e",
            "normal",
            "--prefix",
            "none",
            "--offline",
        ])
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let tree = String::from_utf8(out.stdout).unwrap();
    let names: Vec<&str> = tree.lines().filter_map(|l| l.split_whitespace().next()).collect();
    assert!(names.contains(&"grim-sdk"), "grim-sdk missing:\n{tree}");
    for forbidden in ["grimoire", "grim-hook", "grimoire-hook-core", "grim-mirror"] {
        assert!(
            !names.contains(&forbidden),
            "{forbidden} is in the dependency tree:\n{tree}"
        );
    }
}
