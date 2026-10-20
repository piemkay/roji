#!/usr/bin/env bash
# Tests of scripts/check-scope in a throwaway repository: a module is a feature area across the
# layers, and shared files do not count.
set -euo pipefail

scripts=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

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
passes() { "$@" >"$work/out" 2>&1; }
fails() { ! "$@" >"$work/out" 2>&1; }

touch_files() {
  local file
  for file in "$@"; do
    mkdir -p "$(dirname "$file")"
    echo "// change" >>"$file"
  done
}

repo="$work/repo"
mkdir -p "$repo/scripts"
cd "$repo"
git init -q -b main
git config user.name test
git config user.email test@example.invalid
cp "$scripts/check-scope" scripts/
touch_files crates/domain/src/lib.rs crates/store/src/lib.rs crates/server/src/lib.rs \
  crates/server/src/main.rs crates/store/tests/boundary.rs web/src/main.tsx web/src/App.tsx \
  crates/domain/src/tasks.rs web/src/tasks/List.tsx Cargo.toml README.md
git add -A
git commit -qm base
git switch -q -c work
export ROJI_BASE_REF=main

echo "scripts/check-scope"

reset() {
  git reset -q --hard main
  git clean -qfd
}

touch_files crates/domain/src/tasks.rs crates/store/src/tasks/mod.rs crates/server/src/tasks.rs \
  crates/server/tests/tasks.rs web/src/tasks/List.tsx web/src/tasks.test.ts
touch_files crates/server/src/lib.rs web/src/App.tsx Cargo.toml Cargo.lock README.md
check "passes a slice of one module across domain, store, server and web" passes scripts/check-scope
check "names that module" grep -q "one module: tasks" "$work/out"
reset

touch_files crates/domain/src/tasks.rs web/src/projects/List.tsx
check "fails a change to two modules" fails scripts/check-scope
# shellcheck disable=SC2016 # $1 belongs to the inner shell
check "lists both modules" bash -c 'grep -q "^  projects$" "$1" && grep -q "^  tasks$" "$1"' _ "$work/out"
reset

touch_files crates/domain/src/recurring_tasks.rs web/src/recurring-tasks/Card.tsx
check "reads - as _ in module names" passes scripts/check-scope
reset

touch_files crates/domain/src/projects.rs crates/store/src/projects.rs
git add -A
git commit -qm "new module"
check "counts a new module like any other" passes scripts/check-scope
touch_files crates/domain/src/tasks.rs
check "fails a new module together with another module" fails scripts/check-scope
reset

touch_files crates/domain/src/lib.rs crates/server/src/main.rs crates/store/tests/boundary.rs \
  web/src/main.tsx web/src/App.tsx tools/boundaries/src/lib.rs scripts/dev .github/workflows/ci.yml
check "treats entry points, boundary tests and tooling as shared" passes scripts/check-scope
check "says that no module changed" grep -q "no module" "$work/out"
reset

touch_files web/src/app/tasks/List.tsx web/src/app/projects/List.tsx
check "fails two modules under web/src/app/" fails scripts/check-scope
check "names the modules under app/" grep -q "^  projects$" "$work/out"
reset

touch_files web/src/app/tasks/List.tsx crates/domain/src/tasks.rs web/src/app/App.tsx \
  crates/server/src/bin/roji.rs
check "reads the folder below app/ as the module" passes scripts/check-scope
check "names the module below app/" grep -q "one module: tasks" "$work/out"
reset

touch_files crates/domain/src/tasks.rs web/src/vite-env.d.ts web/src/index.css web/src/App.css \
  web/src/logo.svg web/src/index.ts
check "treats styles, images, type declarations and index files as shared" passes scripts/check-scope
check "still names the one module" grep -q "one module: tasks" "$work/out"
reset

touch_files crates/domain/src/tasks.rs web/src/projects/List.tsx
check "falls back to a local main without origin/main" \
  fails env -u ROJI_BASE_REF -u GITHUB_BASE_REF scripts/check-scope
check "compares with that main" grep -q "compared with main" "$work/out"
reset

if ((failures > 0)); then
  echo "$failures scripts/check-scope test(s) failed"
  exit 1
fi
