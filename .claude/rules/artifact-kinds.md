---
paths:
  - 'src/main.ts'
  - 'src/view.ts'
  - 'src-tauri/src/bridge.rs'
---

# Artifact kinds: interfaces, sites and consumers

Shared with the sibling repositories (vscode, indexer), which carry the full
version of this rule; this copy is cut to what grim-gui faces today. grim's own
kind list is `ArtifactKind::ALL` in the grimoire repository, and its rule
`artifact-kinds.md` there owns the CLI side: skill, rule, agent, mcp, bundle,
hook, settings, mod.

## Interfaces this app faces

Today one: `grim --version`, through `grim-sdk` (`GrimDriver::version()`),
shown by `src/view.ts`. No artifact kind is rendered yet. The first screen that
lists artifacts reads grim's `--format json` output (`search`, `status`,
`describe`) through a bridge command that returns grim's `serde_json::Value`
unchanged (R12, [`subsystem-shell.md`](./subsystem-shell.md)).

## When a screen renders kinds

1. Take the kind list from grim's JSON, never a hard-coded copy that can fall
   behind: an unknown kind renders as plain text with its name.
2. Every kind is supported and tested on every interface the screen uses, except
   refusals recorded in the vscode rule's "Kind x interface" table. Copy that
   table here when a screen needs it, with the checked consumer.
3. Fields grim adds are optional: guard each at its read site; never gate on a
   version ([`grim-compat-markers.md`](./grim-compat-markers.md)).
4. Nothing may key on a harness name: names come from grim (`[a-z0-9]+`).

## Other repos and release order

- `grimoire-rs/grimoire` (grim): every interface above. Its JSON is additive.
- `grimoire-rs/vscode`: renders the same JSON; reuse its plain-TypeScript
  renderers before writing new ones.
- `grimoire-rs/indexer`: read only through grim.

Release order for a new kind: indexer, then grim, then the clients. The CI pin
(`GRIM_COMMIT` in `.github/workflows/verify-basic.yml`) moves to the grim
release that ships the kind.
