#!/usr/bin/env bash
set -euo pipefail

# Best-effort installer for Rust dev tools.
# Requires network access.

echo "==> Installing cargo-audit"
cargo install cargo-audit

echo "==> Installing cargo-deny"
cargo install cargo-deny

echo "==> Installing cargo-geiger"
cargo install cargo-geiger

echo "==> Installing cargo-fuzz (optional)"
cargo install cargo-fuzz || true

echo "==> Installing proptest (library; add via Cargo.toml when needed)"
echo "Note: proptest is a crate dependency, not a cargo subcommand."

echo "==> Installing nightly + miri (optional)"
rustup toolchain install nightly
rustup +nightly component add miri

echo "Done."
