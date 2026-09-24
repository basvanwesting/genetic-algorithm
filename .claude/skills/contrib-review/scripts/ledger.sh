#!/usr/bin/env bash
# Record a verdict. Usage: ledger.sh <pr> <MERGE|CHANGES|REJECT|DISCUSS|MERGED|CLOSED> "<one-line note>"
set -uo pipefail
DIR=${GA_REVIEW_DIR:-$HOME/.local/state/ga-review}; mkdir -p "$DIR"
printf '%s\t%s\t%s\t%s\n' "$1" "$(date +%F)" "$2" "$3" >> "$DIR/ledger.tsv"
