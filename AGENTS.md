# Working in this repository

The project is in planning: there is no code, and no technology has been chosen. Don't
introduce languages, frameworks or tooling until the plan calls for them.

- `design/` holds the design tokens; `design/README.md` explains them.
- `main` is protected: every change is a pull request.
- Run `scripts/check` before every commit. CI runs the same command, and it must pass. For now
  it scans for secrets (gitleaks, downloaded into `~/.cache/roji`) and tests that scan.

## Agent skills

### Issue tracker

Issues live in GitHub Issues for `piemkay/roji`, managed with `gh`. See `docs/agents/issue-tracker.md`.

### Triage labels

The five default triage labels (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one `GLOSSARY.md` and `docs/adr/` at the repo root. See `docs/agents/domain.md`.
