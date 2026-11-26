# Security Policy

## Reporting a vulnerability

Report privately through GitHub's [private vulnerability reporting][pvr] —
the **Report a vulnerability** button on the [Security tab][security-tab].
That keeps the report and the fix in one place and lets us credit you when it
lands.

Please do not open a public issue, discussion, or pull request for a suspected
vulnerability, and do not post it to social media before there is a fix.

grim-gui is maintained by one person. Expect a first response within about a
week; there is no formal SLA, and inventing one nobody can hold to would be
worse than saying this. If a report goes unanswered for two weeks, feel free to
nudge it.

If you are unsure whether something is a vulnerability, report it anyway.
Over-communication is welcome — a duplicate costs a few minutes, a missed
report costs users.

## Supported versions

The latest release. Fixes land on `main` and go out in the next release. There
is no backport branch, and older versions receive no patches. No release exists
yet: the repository holds a skeleton.

## What grim-gui guarantees, and what it does not

Read this before deciding whether something is a vulnerability. grim-gui is a
desktop window around the `grim` binary: a webview that may call a short list
of Rust commands, which in turn run `grim`.

**The webview may call only the commands the repository names.** The Tauri
capability file grants window `main` one permission per command, and no plugin
and no core permission is granted. Reaching a command that is not listed, or
reaching the operating system from the webview, is in scope.

**The page runs under a strict content security policy.** Only the app's own
bundle loads: no inline script, no inline style, no remote origin. Text that
comes from `grim` (names, descriptions, error messages) is written into the
page as text, never as markup. A way to run script in the webview from such
text is in scope.

**grim is the user's own binary.** The app runs the `grim` found on `PATH`, or
the absolute path in `GRIM_BIN`. It passes grim's output and exit code on
unchanged. What grim itself does, including what it downloads and installs,
is governed by the [grimoire threat model][grim-security], not this policy.

**No credential is read, stored or logged.** The app writes one line to stderr
per bridge call, carrying the outcome and the grim version or exit code.

Out of scope: an attacker who already controls the user account or the
environment the app starts in (`PATH`, `GRIM_BIN`), a malicious `grim` the user
chose to run, and people with commit access to this repository.

## Known limitations

Not vulnerabilities, but worth knowing:

- **No release, signing or update channel exists yet.** Builds are unsigned,
  and the app has no auto-update.
- **macOS is unproven for the unsafe-code lint.** The Linux build forbids
  `unsafe` code in this repository; the macOS build is verified only in CI.

<!-- external -->
[pvr]: https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability
[security-tab]: https://github.com/grimoire-rs/grim-gui/security
[grim-security]: https://github.com/grimoire-rs/grimoire/security
