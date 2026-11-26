// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

beforeEach(() => {
  document.body.innerHTML = '<main id="app"></main>';
  vi.resetModules();
});

afterEach(() => clearMocks());

const settle = () => new Promise((r) => setTimeout(r, 0));

describe("main", () => {
  it("invokes grim_version once and renders the version", async () => {
    const calls: string[] = [];
    mockIPC((cmd) => {
      calls.push(cmd);
      return "0.17.0";
    });
    await import("./main");
    await settle();
    expect(calls).toEqual(["grim_version"]);
    expect(document.body.textContent).toContain("grim 0.17.0");
  });

  it("renders a bridge error", async () => {
    mockIPC(() => {
      throw { kind: "not-found", code: null, message: "gone" };
    });
    await import("./main");
    await settle();
    expect(document.body.textContent).toContain("grim not found");
  });
});
