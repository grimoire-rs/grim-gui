---
paths:
  - "src/**"
  - "index.html"
  - "vite.config.ts"
---

# Subsystem: front end (`src/`)

Plain TypeScript, no UI framework, bundled by Vite into `dist/`, which the shell embeds at compile
time. The page runs under a strict CSP and talks to the shell only through `invoke`.

## Layout

| File | Role |
|---|---|
| `index.html` | the page; no inline script, no inline style |
| `src/main.ts` | on load: `invoke("grim_version")` once, pass the result to `render` |
| `src/view.ts` | `render(root, result)` with `result` `{ok: string}` or `{err: BridgeErrorJson}` |
| `src/styles.css` | the stylesheet, linked from `index.html` |
| `src/*.test.ts` | vitest with happy-dom |
| `vite.config.ts` | fixed dev port 1420, `clearScreen: false`, ignores `src-tauri/` |

## Invariants

1. **CSP-clean page.** No inline `<script>`, no inline `<style>`, no `style=` attribute, no
   remote origin. A feature that seems to need one changes the CSP in the shell's
   `tauri.conf.json` and `tests/config.rs` in the same commit, with a threat-model line.
2. **`textContent` only.** grim-derived strings (versions, names, messages) reach the DOM through
   `textContent` or `createElement`; never `innerHTML`, `outerHTML` or `insertAdjacentHTML`.
3. **Error shape.** `BridgeErrorJson` mirrors the shell's `{kind, code, message}`; switch on
   `kind`. States shown: ok `grim <version>`, not-found `grim not found` plus a hint (install grim
   or set `GRIM_BIN` to an absolute path) and the message, failed `grim failed (exit <code>)`.
4. **Render is a pure function of its input**, so vitest covers the three states without Tauri.
   `main.ts` is covered with `mockIPC` (the command name it invokes), cleared after each test.
5. **No framework, one runtime dependency** (`@tauri-apps/api`). Exact versions, `.npmrc`
   `save-exact=true`, `npm ci`, `engines.node >=24`. A new dependency needs a reason in the
   commit body.
6. **Strict TypeScript.** `tsc --noEmit` is the `web:lint` gate; no `any`, no `@ts-ignore`.
7. **Pass grim's JSON through.** Later screens read grim's fields as the shell returns them
   (R12, [`subsystem-shell.md`](./subsystem-shell.md)); do not rename or reshape them in a
   second model. Reuse of the vscode extension's renderers is the planned route.

## Testing

`task web:test` (vitest, host Node 24). The page text is proven here only; the end-to-end suite
proves the script ran under the CSP and reached the bridge, not what was rendered.

## Cross-references

- [`typescript-quality.md`](./typescript-quality.md) (installed), [`typescript-packaging.md`](./typescript-packaging.md)
- [`arch-threat-model.md`](./arch-threat-model.md) — invariants I2, I3
