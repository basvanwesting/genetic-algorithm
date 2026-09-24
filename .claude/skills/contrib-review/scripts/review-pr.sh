#!/usr/bin/env bash
# Mechanical half of a PR review. Proves the bug on main, checks the PR branch.
# Usage: review-pr.sh <pr-number>   |   review-pr.sh --clean
# Serial by design: one worktree, one shared cargo target dir (~2 GB total).
set -uo pipefail

REPO=$(git rev-parse --show-toplevel)
DIR=${GA_REVIEW_DIR:-$HOME/.local/state/ga-review}
WT=$DIR/wt
BASE=origin/main
export CARGO_TARGET_DIR=$DIR/target
mkdir -p "$DIR/reports"

if [[ ${1:-} == --clean ]]; then
  git -C "$REPO" worktree remove --force "$WT" 2>/dev/null
  git -C "$REPO" branch --list 'pr-*' | xargs -r git -C "$REPO" branch -q -D
  rm -rf "$DIR/target"
  echo "cleaned $DIR (ledger and reports kept)"; exit 0
fi

N=${1:?pr number}; BR=pr-$N; OUT=$DIR/reports/pr-$N.md
cd "$REPO"
git fetch -q origin main "+pull/$N/head:$BR" || { echo "fetch failed"; exit 1; }
git worktree remove --force "$WT" 2>/dev/null
git worktree add -q --detach "$WT" "$BR" || { echo "worktree failed"; exit 1; }

{
echo "# PR $N mechanical report ($(date +%F))"
echo "base $(git rev-parse --short $BASE), head $(git rev-parse --short $BR), commits $(git rev-list --count $BASE..$BR), merge-base $(git merge-base $BASE $BR | cut -c1-8)"
echo
echo "## files"
git diff --stat $BASE $BR | tail -n +1
OUTSIDE=$(git diff --name-only $BASE $BR | grep -vE '^(src|tests|benches|examples)/' || true)
[[ -n $OUTSIDE ]] && { echo; echo "outside src/tests/benches/examples:"; echo "$OUTSIDE" | sed 's/^/  /'; }
echo
echo "## conflict with $BASE"
if git merge-tree --write-tree $BASE $BR >/dev/null 2>&1; then echo "clean"; else echo "CONFLICT"; fi

SRC=$(git diff --name-only $BASE $BR -- src Cargo.toml)
NEWTESTS=$(git diff   -- tests | grep -E '^\+[[:space:]]*fn [[:alnum:]_]+\(' | sed -E 's/^\+[[:space:]]*fn ([[:alnum:]_]+)\(.*/\1/' | sort -u)
echo
echo "## bug proof: PR tests against $BASE source"
if [[ -z $SRC ]]; then echo "no src changes, skipped"
elif [[ -z $NEWTESTS ]]; then echo "PR adds no test fns, nothing to prove; reviewer must judge from code"
else
  echo "tests: $(echo $NEWTESTS | tr '\n' ' ')"
  ( cd "$WT" && git checkout -q $BASE -- $SRC 2>/dev/null
    timeout 300 cargo test -q -- $NEWTESTS > "$DIR/proof.log" 2>&1; RC=$?
    git checkout -q $BR -- $SRC
    case $RC in
      0)   echo "RESULT: PASS on main source => bug NOT reproduced by these tests";;
      124) echo "RESULT: HANG on main source (timeout 300s) => confirmed hang";;
      *)   if grep -qE '^error\[E[0-9]+\]|could not compile' "$DIR/proof.log"; then echo "RESULT: does not COMPILE against main source (tests use new API?), inconclusive"
           else echo "RESULT: FAIL on main source => bug confirmed"; fi;;
    esac
    grep -E "panicked at|assertion|left:|right:|^test .* FAILED|thread .* overflow" "$DIR/proof.log" | head -8 | sed 's/^/  /' )
fi

echo
echo "## PR branch checks"
cd "$WT"
timeout 900 cargo test -q > "$DIR/test.log" 2>&1; RC=$?
echo "cargo test: exit $RC; $(grep -E '^test result' "$DIR/test.log" | awk '{p+=$4; f+=$6} END{print p" passed, "f" failed"}')"
grep -E '^test .* FAILED|panicked at' "$DIR/test.log" | head -5 | sed 's/^/  /'
W=$(cargo clippy -q --all-targets 2>&1 | grep -cE '^(warning|error)'); echo "clippy --all-targets: $W warnings/errors"
cargo fmt --check >/dev/null 2>&1 && echo "fmt: clean" || echo "fmt: NOT clean"
cargo build -q --examples 2>&1 | grep -E '^error' | head -3; echo "examples: built"
cd "$REPO"

echo
echo "## conventions"
DOC=$(git diff $BASE $BR | awk '/^\+\s*\/\/\//{buf=buf"\n"$0; next} /^\+/{ if(buf!="" && $0 !~ /^\+\s*(pub|#\[|impl|\/\/)/) print buf"\n"$0" <== /// on non-pub?"; buf=""; next} {buf=""}')
[[ -n $DOC ]] && echo "$DOC" || echo "no /// on non-pub items detected"
PUB=$(git diff $BASE $BR -- src | grep -E '^\+\s*pub (fn|struct|enum|trait|type|const|mod)' || true)
[[ -n $PUB ]] && { echo; echo "new pub items:"; echo "$PUB" | sed 's/^/  /'; }
EXP=$(git diff $BASE $BR -- tests | grep -E '^-' | grep -vE '^---|^-\s*$' | wc -l | tr -d ' ')
echo; echo "removed lines in tests: $EXP (nonzero usually means expected values were edited; PR body must justify)"
echo
echo "worktree left at $WT for reading; run review-pr.sh --clean when the batch is done"
} | tee "$OUT"
