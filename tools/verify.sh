#!/usr/bin/env bash
set -euo pipefail

# Rust verifier: run all gates locally.
# Usage:
#   ./tools/verify.sh
# Optional env vars:
#   BOOTSTRAP=1      -> do not fail if optional tools missing; print install hints instead.
#   REQUIRE_MIRI=1   -> fail if unsafe is detected and Miri cannot be run.

BOOTSTRAP="${BOOTSTRAP:-0}"
REQUIRE_MIRI="${REQUIRE_MIRI:-0}"

fail_missing () {
  local msg="$1"
  if [[ "$BOOTSTRAP" == "1" ]]; then
    echo "WARN: $msg"
    return 0
  fi
  echo "ERROR: $msg" >&2
  exit 2
}

need_cmd () {
  local cmd="$1"
  command -v "$cmd" >/dev/null 2>&1 || fail_missing "Missing command: $cmd"
}

need_cmd cargo

echo "==> cargo fmt --check"
cargo fmt --all -- --check

echo "==> cargo clippy -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> cargo test"
cargo test --all-targets --all-features

# Supply-chain checks (optional tools, but recommended)
if cargo audit -V >/dev/null 2>&1; then
  echo "==> cargo audit"
  cargo audit
else
  fail_missing "cargo-audit not installed. Install: cargo install cargo-audit"
fi

if cargo deny --version >/dev/null 2>&1; then
  if [[ -f "deny.toml" ]]; then
    echo "==> cargo deny check"
    cargo deny check
  else
    echo "WARN: deny.toml not found; skipping cargo-deny."
  fi
else
  fail_missing "cargo-deny not installed. Install: cargo install cargo-deny"
fi

# Unsafe detection (best-effort)
UNSAFE_HITS="$(grep -R --include='*.rs' -n '\bunsafe\b' . 2>/dev/null || true)"
if [[ -n "$UNSAFE_HITS" ]]; then
  echo "==> unsafe detected:"
  echo "$UNSAFE_HITS" | head -n 50
  echo "==> Miri gate (nightly)"
  if cargo +nightly miri --version >/dev/null 2>&1; then
    cargo +nightly miri test
  else
    if [[ "$REQUIRE_MIRI" == "1" ]]; then
      echo "ERROR: unsafe present but Miri cannot run. Install nightly + miri component." >&2
      exit 3
    fi
    fail_missing "Miri not available. Install: rustup toolchain install nightly && rustup +nightly component add miri"
  fi
fi

echo "✅ All gates passed."
