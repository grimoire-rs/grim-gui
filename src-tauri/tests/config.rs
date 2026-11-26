// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

//! Pins the security configuration: CSP, the one granted permission, no plugins.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn read(rel: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)).unwrap()
}

fn json(rel: &str) -> Value {
    serde_json::from_str(&read(rel)).unwrap()
}

#[test]
fn csp_is_the_pinned_string() {
    let conf = json("tauri.conf.json");
    assert_eq!(
        conf["app"]["security"]["csp"],
        "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src ipc: http://ipc.localhost; object-src 'none'; base-uri 'none'; form-action 'none'"
    );
}

#[test]
fn identifiers_are_pinned() {
    let conf = json("tauri.conf.json");
    assert_eq!(conf["productName"], "grim-gui");
    assert_eq!(conf["identifier"], "rs.grimoire.gui");
    assert_eq!(conf["app"]["windows"][0]["label"], "main");
}

#[test]
fn exactly_one_capability_grants_only_the_bridge_command_to_main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("capabilities");
    let files: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    assert_eq!(files.len(), 1, "{files:?}");
    let cap: Value = serde_json::from_str(&fs::read_to_string(&files[0]).unwrap()).unwrap();
    assert_eq!(cap["windows"], serde_json::json!(["main"]));
    assert_eq!(cap["permissions"], serde_json::json!(["allow-grim-version"]));
}

#[test]
fn build_script_declares_only_the_bridge_command() {
    let build = read("build.rs");
    assert!(build.contains(r#"commands(&["grim_version"])"#), "{build}");
}

#[test]
fn no_tauri_plugin_is_a_dependency() {
    assert!(!read("Cargo.toml").contains("tauri-plugin"));
    let lock = read("../Cargo.lock");
    assert!(
        !lock.contains("name = \"tauri-plugin"),
        "a plugin entered the lock file"
    );
}

#[test]
fn tauri_crates_are_pinned_exactly() {
    let manifest: toml::Table = read("Cargo.toml").parse().unwrap();
    // The pin shape, not the version: a dependency bump must not fail this test.
    for (table, name) in [("dependencies", "tauri"), ("build-dependencies", "tauri-build")] {
        let version = manifest[table][name].as_str().unwrap_or_default();
        assert!(version.starts_with('='), "{name} is not pinned exactly: {version:?}");
    }
}
