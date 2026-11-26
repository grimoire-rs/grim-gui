// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

import { beforeEach, describe, expect, it } from "vitest";
import { render } from "./view";

let root: HTMLElement;

beforeEach(() => {
  document.body.innerHTML = '<main id="app"></main>';
  root = document.getElementById("app") as HTMLElement;
});

describe("render", () => {
  it("shows the grim version", () => {
    render(root, { ok: "0.17.0" });
    expect(root.textContent).toContain("grim 0.17.0");
  });

  it("shows not found with a hint and the message", () => {
    render(root, { err: { kind: "not-found", code: null, message: "no such file" } });
    expect(root.textContent).toContain("grim not found");
    expect(root.textContent).toContain("GRIM_BIN");
    expect(root.textContent).toContain("no such file");
  });

  it("shows the exit code of a failed grim", () => {
    render(root, { err: { kind: "failed", code: 78, message: "bad" } });
    expect(root.textContent).toContain("grim failed (exit 78)");
  });

  it("never interprets the message as markup", () => {
    render(root, { err: { kind: "not-found", code: null, message: "<img src=x onerror=alert(1)>" } });
    expect(root.querySelector("img")).toBeNull();
  });
});
