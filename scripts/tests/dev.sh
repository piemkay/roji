#!/usr/bin/env bash
# Tests of scripts/dev: hostile environment variables, damaged state files, stale process ids,
# failed database creation, and two worktrees side by side. Each case runs in a throwaway git
# repository that holds a copy of scripts/dev, so this worktree's own state is never touched.
# No `set -e`: a step that fails shows up as a failed check below instead of ending the run.
set -uo pipefail

scripts=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
work=$(mktemp -d)
sleeper=

cleanup() {
  local repo
  for repo in "$work"/repo-*; do
    [[ -x $repo/scripts/dev ]] && "$repo/scripts/dev" clean >/dev/null 2>&1
  done
  [[ -n $sleeper ]] && kill "$sleeper" 2>/dev/null
  rm -rf "$work"
}
trap cleanup EXIT

failures=0
check() {
  local name=$1
  shift
  if "$@"; then
    printf 'ok   %s\n' "$name"
  else
    printf 'FAIL %s\n' "$name"
    failures=$((failures + 1))
  fi
}

new_repo() {
  local repo="$work/repo-$1"
  mkdir -p "$repo/scripts"
  git -C "$repo" init -q
  cp "$scripts/dev" "$repo/scripts/dev"
  printf '%s\n' "$repo"
}

alive() { kill -0 "$1" 2>/dev/null; }
gone() { ! kill -0 "$1" 2>/dev/null; }
missing() { [[ ! -e $1 ]]; }
fails() { ! "$@" >/dev/null 2>&1; }

postmaster_of() { head -n 1 "$1/.dev/postgres/postmaster.pid"; }

# Variables that once leaked from the environment into the script, with hostile values.
hostile=(
  PORT_BASE=20500 PG_PORT= SERVER_PORT=x WEB_PORT=1
  "DB_NAME=x\$(touch $work/pwned)" 'port_base=1' 'db_name=my"db'
)

echo "scripts/dev"

a=$(new_repo a)
b=$(new_repo b)

started=$(env "${hostile[@]}" "$a/scripts/dev" db)
check "starts Postgres although the environment holds hostile values" [ "$started" = started ]
check "derives its own port block" grep -qxE 'PORT_BASE=2[0-9]{4}' "$a/.dev/state.env"
check "derives its own database name" grep -qxE 'DB_NAME=roji_repo_a_[0-9a-f]{8}' "$a/.dev/state.env"
env "${hostile[@]}" "$a/scripts/dev" env >/dev/null
check "never runs commands from the environment" missing "$work/pwned"
check "prints the URL of the test database" grep -qE \
  '^postgres://roji@127\.0\.0\.1:2[0-9]{4}/roji_repo_a_[0-9a-f]{8}_test$' <("$a/scripts/dev" url test)

"$b/scripts/dev" db >/dev/null
# The Postgres binaries in use, for the fake ones further down.
real_bin=$(dirname "$(tr '\0' '\n' <"/proc/$(postmaster_of "$b")/cmdline" | head -n 1)")
check "gives a second worktree its own ports" [ "$("$a/scripts/dev" url)" != "$("$b/scripts/dev" url)" ]
check "runs the first worktree's Postgres" [ "$("$a/scripts/dev" db)" = running ]
check "runs the second worktree's Postgres at the same time" [ "$("$b/scripts/dev" db)" = running ]

# A damaged state file is read, never executed, and clean still stops Postgres.
pg_a=$(postmaster_of "$a")
# shellcheck disable=SC2016 # the $(...) must reach the file as it is
printf 'PORT_BASE=$(touch %s/pwned)\nDB_NAME=x"; touch %s/pwned; "\n' "$work" "$work" >"$a/.dev/state.env"
check "refuses a damaged state file" fails "$a/scripts/dev" env
check "never runs commands from the state file" missing "$work/pwned"
"$a/scripts/dev" clean >/dev/null 2>&1
check "clean stops Postgres despite the damaged state" gone "$pg_a"
check "clean removes the state" missing "$a/.dev"

# Stale pid files that name an unrelated process: down must leave that process alone.
sleep 600 &
sleeper=$!
"$b/scripts/dev" db-stop
printf '%s\n' "$sleeper" >"$b/.dev/server.pid"
printf '%s 1 %s\n' "$sleeper" "$(cat /proc/sys/kernel/random/boot_id)" >"$b/.dev/web.pid"
printf '%s\n%s\n' "$sleeper" "$b/.dev/postgres" >"$b/.dev/postgres/postmaster.pid"
"$b/scripts/dev" down
check "down leaves an unrelated process named in stale pid files alone" alive "$sleeper"
check "down drops the stale server pid file" missing "$b/.dev/server.pid"
check "down drops the stale web pid file" missing "$b/.dev/web.pid"
check "db starts Postgres over a stale postmaster.pid" [ "$("$b/scripts/dev" db)" = started ]
check "the unrelated process is still alive" alive "$sleeper"
"$b/scripts/dev" clean >/dev/null 2>&1

# Single-user mode exits with 0 even when CREATE DATABASE fails; dev must notice.
fake_bin="$work/fake-bin"
mkdir -p "$fake_bin"
ln -s "$real_bin/initdb" "$fake_bin/initdb"
ln -s "$real_bin/pg_ctl" "$fake_bin/pg_ctl"
cat >"$fake_bin/postgres" <<EOF
#!/bin/sh
if [ "\$1" = "--single" ]; then
  cat >/dev/null
  echo "ERROR:  simulated failure"
  exit 0
fi
exec "$real_bin/postgres" "\$@"
EOF
chmod +x "$fake_bin/postgres"
c=$(new_repo c)
ROJI_PG_BIN=$fake_bin "$c/scripts/dev" db >"$work/c.out" 2>&1 || true
check "reports databases that single-user mode failed to create" \
  grep -q "creating the databases failed" "$work/c.out"
check "leaves no half-made cluster behind" missing "$c/.dev/postgres"

if ((failures > 0)); then
  echo "$failures scripts/dev test(s) failed"
  exit 1
fi
