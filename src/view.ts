// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

export interface BridgeErrorJson {
  kind: "not-found" | "failed";
  code: number | null;
  message: string;
}

export type GrimResult = { ok: string } | { err: BridgeErrorJson };

function line(tag: string, text: string, className?: string): HTMLElement {
  const el = document.createElement(tag);
  el.textContent = text;
  if (className) el.className = className;
  return el;
}

export function render(root: HTMLElement, result: GrimResult): void {
  root.replaceChildren();
  if ("ok" in result) {
    root.append(line("p", `grim ${result.ok}`));
  } else if (result.err.kind === "not-found") {
    root.append(
      line("p", "grim not found"),
      line("p", "Install grim or set GRIM_BIN to an absolute path.", "hint"),
      line("p", result.err.message, "hint"),
    );
  } else {
    root.append(line("p", `grim failed (exit ${result.err.code ?? "unknown"})`), line("p", result.err.message, "hint"));
  }
}
