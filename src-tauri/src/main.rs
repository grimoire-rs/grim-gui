// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

// Prevents an extra console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
    match grim_gui_lib::run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("grim-gui: {err}");
            std::process::ExitCode::FAILURE
        }
    }
}
