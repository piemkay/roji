# Working in this repository

Run `scripts/check` before every commit. CI runs the same command, and it must pass.

## What the checks enforce

- **Formatting, lint and types.** rustfmt; Clippy with warnings as errors; Prettier, oxlint and
  TypeScript for the web app.
- **No unsafe Rust.** The workspace forbids `unsafe`.
- **Crate boundaries.** Each crate's `tests/boundary.rs` states what it may use:
  - `roji-domain` and `roji-parse` are I/O-free: no async runtime, database or web crate, no
    external crate that is not on their list, and no `std::fs`, `io`, `net`, `path`,
    `process`, `env`, `thread`, `os`, clock, console macros or `extern` blocks.
  - `roji-domain` uses no other crate of the workspace; `roji-parse` may use `roji-domain`.
  - `roji-store` owns the database and may use `roji-domain`, but no HTTP or web crate.
  - `roji-server` may use the other crates, reaches the database only through `roji-store`, and
    nothing depends on it.
- **Web boundary.** Code in `web/src` imports only from `web/src` and the `dependencies` in
  `web/package.json` (tests may also use `devDependencies`), never Node built-ins or anything
  outside `web/`. It reaches the server only over HTTP at `/api`.
- **No secrets in the repository.** gitleaks scans the commits of this branch that are not on
  `main`, and the uncommitted files; no `.env` file may be tracked.
- **One change, one module.** Compared with `main`, a branch changes at most one module. A
  module is a feature area across the layers: `tasks` is `src/tasks.rs` or `src/tasks/` and
  `tests/tasks.rs` in any crate, and `web/src/tasks/`. Crate roots and entry points (`lib`,
  `main`, `mod`, `bin`, `App`), boundary tests, manifests, lock files, `scripts/`, `tools/`,
  CI and docs are shared and don't count.

## Running it

- `scripts/dev up` starts Postgres, the server and the web app for this worktree, on ports and
  a database of its own; `scripts/dev status`, `scripts/dev down`, `scripts/dev clean`.
- `eval "$(scripts/dev env test)"` points `cargo test` at this worktree's test database.
