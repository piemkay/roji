# Working in this repository

The project is in planning: there is no code, and no technology has been chosen. Don't
introduce languages, frameworks or tooling until the plan calls for them.

- `design/` holds the design tokens; `design/README.md` explains them.
- `main` is protected: every change is a pull request.
- Run `scripts/check` before every commit. CI runs the same command, and it must pass. For now
  it scans for secrets (gitleaks, downloaded into `~/.cache/roji`) and tests that scan.
