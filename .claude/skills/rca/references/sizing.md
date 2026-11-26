# Fix size rubric

Shared by `/rca` and `/fix`. Size by blast radius, not hours. State the basis beside the letter.

| Size | Criteria |
|---|---|
| **S** | One function or file. No interface surface. An existing test pattern covers it. Zero siblings. |
| **M** | Up to 5 files in one package, or the cause plus 1–3 sibling sites. New tests, no design choice. |
| **L** | Crosses the shell and the front end, or layers, **or** touches any interface surface below, **or** needs a choice between designs. |
| **XL** | Needs an ADR, the architectural assumption itself is wrong, a migration, or a concurrency/locking redesign. Split before building. |

**Bump +1** when the repro is flaky or Windows-only, or confidence is below medium.

## Interface surfaces

The bridge command names and arguments · the `BridgeError` wire shape (`{kind, code, message}`) · the stderr lines (`grim-gui: <command> <outcome>`) · the capability set and the CSP · `GRIM_BIN` handling · exit codes of `grim-gui` · what the front end reads of grim's JSON (grim's own interface, frozen in the grimoire repository).

Touching one is a decision, not a refactor. A break is prohibited (AGENTS.md › Principle 9): grim's contracts pass through unchanged, and our own surfaces evolve additively. A bridge command or capability change also updates `arch-threat-model.md`.
