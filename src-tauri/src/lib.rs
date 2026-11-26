// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

//! Desktop front end of grim. Reaches grim only through `grim-sdk`.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod bridge;

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![bridge::grim_version])
        .run(tauri::generate_context!())
}
