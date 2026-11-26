// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

import { invoke } from "@tauri-apps/api/core";
import { type BridgeErrorJson, render } from "./view";

function isBridgeError(e: unknown): e is BridgeErrorJson {
  return typeof e === "object" && e !== null && "kind" in e && "message" in e;
}

async function load(root: HTMLElement): Promise<void> {
  try {
    render(root, { ok: await invoke<string>("grim_version") });
  } catch (err) {
    render(root, {
      err: isBridgeError(err) ? err : { kind: "failed", code: null, message: String(err) },
    });
  }
}

const root = document.getElementById("app");
if (root) void load(root);
