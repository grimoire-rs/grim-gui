---
name: meta-validate-context
description: Use when auditing the freshness of `.claude/rules/subsystem-*.md` files against current codebase state, or when a subsystem has undergone significant change.
user-invocable: true
argument-hint: "all | subsystem-name"
triggers:
  - "validate context"
  - "audit subsystem rules"
  - "check subsystem rules"
  - "freshness of rules"
---

# Validate Context Rules

Check `.claude/rules/subsystem-*.md` files match current codebase.

## Workflow

For each `subsystem-*.md` rule file:

1. **Read the rule** — Extract type names, module paths, function names, error variants
2. **Grep the codebase** — Verify each reference exists
3. **Check for new additions** — Find new public types/modules not in rule
4. **Report** — List stale references + missing additions

## Subsystem Rules to Check

| Rule | Key References to Verify |
|------|------------------------|
| `subsystem-shell.md` | Files under `src-tauri/`, bridge commands and `BridgeError` shape, the three places a command is declared, CSP string, pinned versions |
| `subsystem-web.md` | Files under `src/`, `render` signature and states, `BridgeErrorJson`, dependency list |
| `subsystem-tests.md` | Fixture names in conftest.py, test file names, runner methods, scenario table |
| `subsystem-ci.md` | Workflow names, job names, the `GRIM_COMMIT` pin |
| `subsystem-taskfiles.md` | Taskfile names, the two-phase verify list |

Verify each rule against the current `src/` and `src-tauri/` as code lands.

## Verification Commands

```bash
# Check if a type still exists
grep -r "pub struct TypeName" src-tauri/src/
grep -r "pub enum TypeName" src-tauri/src/

# Check if a TypeScript export still exists
grep -rn "export function render" src/

# Check for new public types not in the rule
grep -rn "^pub struct\|^pub enum\|^pub trait" src-tauri/src/ | grep -v test
```

## Output Format

```markdown
## Context Validation Report

### subsystem-shell.md
- OK: [type] still present
- STALE: [type] — renamed to [new_name] or removed
- MISSING: [new_type] — not documented in rule

### subsystem-web.md
...
```

## When to Run

- After big refactors
- Before major feature branches
- Part of `/code-check` audits
- Monthly