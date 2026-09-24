#!/usr/bin/env bash
# Merge every open PR whose latest ledger verdict is MERGE, in number order.
# Usage: merge.sh [--dry-run]   Conflicting PRs are skipped; the contributor rebases them.
set -uo pipefail
DIR=${GA_REVIEW_DIR:-$HOME/.local/state/ga-review}; LEDGER=$DIR/ledger.tsv
DRY=${1:-}
gh pr list --state open --limit 100 --json number,mergeable --jq '.[] | "\(.number)\t\(.mergeable)"' | sort -n |
while IFS=$'\t' read -r n m; do
  v=$(awk -F'\t' -v n="$n" '$1==n{v=$3} END{print v}' "$LEDGER")
  [[ $v == MERGE ]] || continue
  for _ in 1 2 3 4 5 6; do [[ $m == UNKNOWN ]] || break; sleep 5; m=$(gh pr view "$n" --json mergeable --jq .mergeable); done
  if [[ $m != MERGEABLE ]]; then echo "skip #$n: $m (contributor rebases)"; continue; fi
  if [[ $DRY == --dry-run ]]; then echo "would merge #$n"; continue; fi
  if gh pr merge "$n" --rebase; then
    printf '%s\t%s\tMERGED\t%s\n' "$n" "$(date +%F)" "merged by merge.sh" >> "$LEDGER"
  else echo "FAILED #$n, stopping"; exit 1; fi
done
echo "done; now: git pull, cargo test, CHANGELOG"
