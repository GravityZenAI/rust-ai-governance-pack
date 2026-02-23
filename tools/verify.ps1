# Rust verifier for Windows PowerShell
# Usage: powershell -ExecutionPolicy Bypass -File .\tools\verify.ps1
$ErrorActionPreference = "Stop"

function Require-Cmd($cmd, $hint) {
  if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
    throw "Missing command: $cmd. $hint"
  }
}

Require-Cmd cargo "Install Rust toolchain (rustup)."

Write-Host "==> cargo fmt --check"
cargo fmt --all -- --check

Write-Host "==> cargo clippy -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

Write-Host "==> cargo test"
cargo test --all-targets --all-features

# Supply-chain checks
try {
  cargo audit -V | Out-Null
  Write-Host "==> cargo audit"
  cargo audit
} catch {
  Write-Warning "cargo-audit not installed. Install: cargo install cargo-audit"
  throw
}

try {
  cargo deny --version | Out-Null
  if (Test-Path "deny.toml") {
    Write-Host "==> cargo deny check"
    cargo deny check
  } else {
    Write-Warning "deny.toml not found; skipping cargo-deny."
  }
} catch {
  Write-Warning "cargo-deny not installed. Install: cargo install cargo-deny"
  throw
}

# Unsafe detection
$unsafeHits = Select-String -Path . -Filter *.rs -Pattern "\bunsafe\b" -Recurse -ErrorAction SilentlyContinue
if ($unsafeHits) {
  Write-Host "==> unsafe detected (showing first 50):"
  $unsafeHits | Select-Object -First 50 | ForEach-Object { Write-Host $_.Path ":" $_.LineNumber ":" $_.Line }

  Write-Host "==> Miri gate (nightly)"
  try {
    cargo +nightly miri --version | Out-Null
    cargo +nightly miri test
  } catch {
    Write-Warning "Miri not available. Install: rustup toolchain install nightly; rustup +nightly component add miri"
    throw
  }
}

Write-Host "✅ All gates passed."
