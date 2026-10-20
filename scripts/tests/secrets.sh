#!/usr/bin/env bash
# Tests of scripts/check-secrets in a throwaway repository with a main branch. The tokens are
# random strings in the shape of a GitHub token, made up at run time; none of them is real.
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
lacks() { ! grep -q "$1" "$2"; }
no_downloads() { ! compgen -G "$1/roji/download.*" >/dev/null; }

fake_token() { printf 'ghp_%s' "$(head -c 600 /dev/urandom | LC_ALL=C tr -dc 'A-Za-z0-9' | head -c 36)"; }

repo="$work/repo"
mkdir -p "$repo/scripts"
cd "$repo"
git init -q -b main
git config user.name test
git config user.email test@example.invalid
cp "$scripts/check-secrets" scripts/
echo "# test" >README.md
git add -A
git commit -qm base
export ROJI_BASE_REF=main

echo "scripts/check-secrets"

# A secret on another local branch belongs to that branch, not to this one.
git switch -q -c other
printf 'token = "%s"\n' "$(fake_token)" >config.txt
git add config.txt
git commit -qm "secret on another branch"
git switch -q -c work main
echo "work" >work.txt
git add work.txt
git commit -qm "clean work"
check "ignores a secret committed on another local branch" passes scripts/check-secrets

printf 'token = "%s"\n' "$(fake_token)" >notes.txt
check "finds a secret in an uncommitted file" fails scripts/check-secrets
check "names the rule and the file" grep -q "notes.txt" "$work/out"

git add notes.txt
git commit -qm "add notes"
git rm -q notes.txt
git commit -qm "remove notes"
check "finds a secret committed on this branch and removed again" fails scripts/check-secrets
git reset -q --hard HEAD~2

echo "DATABASE_URL=postgres://localhost/roji" >.env
git add -f .env
check "refuses a tracked .env file" fails scripts/check-secrets
git rm -q --cached .env
rm .env

check "passes again once the branch is clean" passes scripts/check-secrets

printf 'token = "%s"\n' "$(fake_token)" >staged.txt
git add staged.txt
rm staged.txt
check "finds a secret that is only staged" fails scripts/check-secrets
git rm -q --cached staged.txt

status=0
env -u HOME -u XDG_CACHE_HOME scripts/check-secrets >"$work/out" 2>&1 || status=$?
check "runs without HOME" [ "$status" -eq 0 ]
check "reports no unbound variable without HOME" lacks "unbound" "$work/out"

# A download that fails (here: through a proxy that refuses) stops the check and leaves no
# temporary folder behind.
mkdir -p "$work/empty-cache"
check "fails when the gitleaks download fails" fails env XDG_CACHE_HOME="$work/empty-cache" \
  HTTPS_PROXY=http://127.0.0.1:1 https_proxy=http://127.0.0.1:1 scripts/check-secrets
check "stops before scanning" lacks "Scanning" "$work/out"
check "leaves no download folder behind" no_downloads "$work/empty-cache"

if ((failures > 0)); then
  echo "$failures scripts/check-secrets test(s) failed"
  exit 1
fi
