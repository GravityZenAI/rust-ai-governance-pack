#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-ARTIFACTS}"
mkdir -p "$OUT_DIR"

TS="$(date +%Y%m%d-%H%M%S)"
LOG="$OUT_DIR/verify-$TS.log"
DIFF="$OUT_DIR/diff-$TS.patch"

{
  echo "# verify evidence ($TS)"
  echo
  echo "## git status"
  git status || true
  echo
  echo "## verify.sh"
  ./scripts/verify.sh
} | tee "$LOG"

git diff > "$DIFF" || true

echo "Evidence written:" >&2
echo "- $LOG" >&2
echo "- $DIFF" >&2
