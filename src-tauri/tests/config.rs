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
        "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src ipc: http://ipc.localhost"
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
fn capability_grants_only_the_bridge_command_to_main() {
    let cap = json("capabilities/default.json");
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
    assert_eq!(manifest["dependencies"]["tauri"].as_str(), Some("=2.12.5"));
    assert_eq!(manifest["build-dependencies"]["tauri-build"].as_str(), Some("=2.7.4"));
}
