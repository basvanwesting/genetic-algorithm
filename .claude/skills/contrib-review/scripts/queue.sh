#!/usr/bin/env bash
# Open PRs and issues with ledger status. Usage: queue.sh
set -uo pipefail
DIR=${GA_REVIEW_DIR:-$HOME/.local/state/ga-review}
LEDGER=$DIR/ledger.tsv
mkdir -p "$DIR"; touch "$LEDGER"

echo "## open PRs (num, author, +/-, files, mergeable, fixes, ledger verdict, title)"
gh pr list --state open --limit 100 --json number,title,author,additions,deletions,changedFiles,mergeable,body \
  --jq '.[] | [.number, .author.login, "+\(.additions)/-\(.deletions)", "\(.changedFiles)f", .mergeable, ((.body | capture("(?i)(fixes|closes|resolves) #(?<n>[0-9]+)") | "#"+.n) // "-"), .title] | @tsv' \
  | sort -n | while IFS=$'\t' read -r n a s f m fx t; do
      v=$(awk -F'\t' -v n="$n" '$1==n{print $3": "$4}' "$LEDGER" | tail -1)
      printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$n" "$a" "$s" "$f" "$m" "$fx" "${v:-NEW}" "$t"
    done

echo
echo "## open issues without a PR referencing them"
PRFIX=$(gh pr list --state open --limit 100 --json body --jq '.[].body | capture("(?i)(fixes|closes|resolves) #(?<n>[0-9]+)") | .n' | sort -u)
gh issue list --state open --limit 100 --json number,title,author --jq '.[] | [.number, .author.login, .title] | @tsv' | sort -n \
  | while IFS=$'\t' read -r n a t; do echo "$PRFIX" | grep -qx "$n" || printf '%s\t%s\t%s\n' "$n" "$a" "$t"; done

echo
echo "ledger: $LEDGER ($(wc -l < "$LEDGER" | tr -d ' ') entries)"
