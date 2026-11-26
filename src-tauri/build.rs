// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

fn main() {
    let app = tauri_build::AppManifest::new().commands(&["grim_version"]);
    if let Err(err) = tauri_build::try_build(tauri_build::Attributes::new().app_manifest(app)) {
        eprintln!("tauri build failed: {err:#}");
        std::process::exit(1);
    }
}
