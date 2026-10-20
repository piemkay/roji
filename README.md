# Roji

A calm to-do app for individuals, households and teams. Capture a task in plain text, see only what matters today, and let the app recognise dates, people and places in your words. Self-hosted or hosted: one deployment can serve many separate spaces.

Status: scaffold. The repository builds, checks and runs a health page; no features yet.

## Stack

- Rust server around a domain core with no I/O, so the same rules can later run in the browser (WebAssembly), on Android (UniFFI) and in a desktop shell (Tauri).
- React web app (PWA), with typed API clients generated from the Rust definitions.
- Postgres, with one space per household or team and row-level security.
- Models are called by the server through a configurable gateway or provider API key. Rules run first; the model adds proposals.
- Runs behind a reverse proxy, on a private network or hosted.

## Milestones

Each milestone ships on its own.

| | Milestone | Content |
|---|---|---|
| W1 | Backend and web UI | Everything for daily use in the browser (Chrome and Edge, desktop and Android). Online only. Contract specs first, then capture, Today, Next, Projects, Recurring, Lists, search, export, web push. |
| W2 | Web refinements | Features that wait for real use: templates, check-ins, follow-through, duplicate detection, a sync protocol and test client. Runs next to the connectors. |
| C1 | Gmail and Inbox | Connector framework, tasks proposed from mail, the Inbox. |
| C2 | Google calendar | Busy times, calendar entries. |
| C3 | Contacts and places | Contacts and opening hours found for a task. |
| C4 | Weather and conditions | Weather brings a task up. |
| C5 | Local agents | MCP and CLI. |
| D1 | Desktop app | Windows and Linux. Hotkey capture, tray, local copy and sync, on-device model setting. |
| D2 | Ishi | Always-on-top focus companion and drift catch. |
| A1 | Android app | Native, with its own local copy and the on-device model setting. |
| AP | Apple support | Safari, iPhone, Mac, iCloud calendar. At the very end. |

Not scheduled: sharing, groups and invitations (the data model carries spaces from the start) and Microsoft 365 connectors. The connectors need only the server. The desktop app starts when the web app is good enough.

## Decisions

- Web app first, online only. Each native client brings its own local copy and sync.
- No Safari or iOS work until AP.
- Two environments from one image: staging (synthetic data, connectors off) and production. Merges to `main` deploy to staging; production is promoted by hand.
- Backups are handled by the deployment platform; the operator chooses the tooling.
- Built by coding agents. Work is specified as GitHub issues. `main` is protected: every change is a pull request, merged one at a time.
- MIT licence.

## Development

Develop on Linux, or on Windows through WSL. You need:

- [rustup](https://rustup.rs) and a C linker (`cc`, for example from `build-essential`). `rust-toolchain.toml` selects the Rust version.
- Node.js 24, the major version in `.node-version` (CI installs exactly that version), and pnpm 12. `web/package.json` names the exact pnpm version.
- `git`, `curl`, `tar`, `xz`, `unzip`, `sha256sum` and `setsid`, which most Linux distributions include.

Postgres 18 and gitleaks are downloaded on first use into `~/.cache/roji` and checked against pinned SHA-256 sums. To use your own Postgres 18, set `ROJI_PG_BIN` to its `bin` directory.

| Command | What it does |
|---|---|
| `scripts/check` | The one check: formatting, lint, type checks, boundary checks and tests for the Rust workspace and the web app, plus the secret scan and the one-module rule. CI runs the same command. |
| `scripts/dev up` | Starts Postgres, the server and the web app for this worktree and checks that they are connected. Each worktree gets its own ports and database, so several can run at once. |
| `scripts/dev status` | Shows ports, processes and health. |
| `scripts/dev down` | Stops all three; the data stays in `.dev/`. |
| `scripts/dev clean` | Stops all three and deletes this worktree's data. |

`AGENTS.md` lists the rules the checks enforce. The Rust crates are in `crates/` (`domain`, `parse`, `store`, `server`), the React and TypeScript web app (built with Vite) is in `web/`, and the crate boundary checks are in `tools/boundaries`.

## Licence

MIT, see `LICENSE`.
