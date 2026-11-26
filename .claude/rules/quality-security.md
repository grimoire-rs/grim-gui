---
paths:
  - ".github/workflows/**"
  - ".github/actions/**"
  - ".github/dependabot.yml"
---

# Security Standards

Deep-dive reference for security reviews. See Core Principle 3 ("Keep It Safe") in AGENTS.md for essentials.

## Security Checklist

- [ ] No hardcoded secrets or credentials
- [ ] All user input validated + sanitized
- [ ] SQL queries use parameterized statements
- [ ] Auth + authorization properly implemented
- [ ] Sensitive data encrypted at rest + in transit
- [ ] Error messages no expose internal details
- [ ] Dependencies up to date + vuln-free

## OWASP Top 10 2021

| Category | Check For |
|----------|-----------|
| Broken Access Control | Missing authorization checks |
| Cryptographic Failures | Unencrypted sensitive data |
| Injection | SQL, Command, XSS vulnerabilities |
| Insecure Design | Missing threat modeling |
| Security Misconfiguration | Default credentials, debug enabled |
| Vulnerable Components | Outdated/CVE-affected packages |
| Auth Failures | Weak passwords, session issues |
| Integrity Failures | Unsigned updates, untrusted deserialization |
| Logging Failures | Missing audit trails |
| SSRF | Unvalidated URLs in server requests |

## Severity Classification

| Severity | Definition | Action |
|----------|------------|--------|
| Critical | Exploitable vulnerability, data loss risk, high impact | MUST fix before merge |
| High | Exploitable vulnerability, breaking change, moderate impact, major bug | MUST fix before merge |
| Medium | Requires conditions to exploit, performance issue, code smell | SHOULD fix, can negotiate |
| Low | Best practice violation, style, minor improvement | COULD fix, optional |

## CWE References

Reference CWE (Common Weakness Enumeration) IDs for standardized vuln classification. Example: `CWE-89` for SQL Injection, `CWE-798` for hardcoded credentials.

## Dependency Safety

- Warn on deprecated/vulnerable deps
- Audit new deps before adding
- Keep deps updated
- Use automated scanning (Trivy, Snyk, Dependabot)

## Output Guidelines

- Never expose actual secrets in analysis output
- Give specific file locations + line numbers
- Include concrete remediation steps
- Check code AND config files

## grim-gui Attack Surfaces

Recurring attack surfaces in this repository. Use as STRIDE scoping checklist for any audit; whom
we defend against is decided in [`arch-threat-model.md`](./arch-threat-model.md).

### IPC bridge
- The webview may call only commands named in `build.rs`, granted per command in
  `capabilities/default.json` to window `main`. No `core:default`, no `tauri-plugin-*`
- A new command is a new privilege: it needs a threat-model line and a test in
  `src-tauri/tests/config.rs`
- Command arguments come from the webview: validate them as untrusted input

### Webview content and CSP
- `security.csp` forbids inline script and style and any origin but `'self'` (plus the IPC
  endpoints). Weakening it is a Block
- grim-derived text (package names, descriptions, error messages) is attacker-influenced:
  render with `textContent`, never `innerHTML`
- The page never navigates to or embeds remote content

### The spawned grim
- Resolved by `grim-sdk` from `GRIM_BIN` (absolute only) or `PATH`; stdin is null and every
  call has a deadline
- Its stdout and stderr are data: bound the size, parse defensively, never feed them to a shell
- Its exit code passes through unchanged; do not map a failure to success

### Supply chain
- npm and crates.io dependencies are pinned exactly with committed lockfiles; `cargo deny`
  and `npm audit --audit-level=high` run in CI
- GitHub Actions are pinned by commit SHA; `tauri-action` is not used

### Secrets
- The app reads, stores and logs none. grim's own credential handling is not ours

## grim-gui Audit Checklist

- [ ] New or changed bridge command: named in `build.rs`, granted by one permission, tested
- [ ] CSP unchanged or stricter; no inline script or style in `index.html`
- [ ] grim-derived text written with `textContent`
- [ ] Spawned grim reached only through `grim-sdk`; deadline and null stdin intact
- [ ] Dependency versions exact, lockfiles committed, `cargo deny` and `npm audit` clean
- [ ] Workflow actions pinned by SHA; permissions minimal; inputs through `env:`
