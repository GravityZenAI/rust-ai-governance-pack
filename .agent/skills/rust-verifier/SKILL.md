# Skill: Rust Verifier Loop

## When to use
Use this skill to **verify finished work**:
- After editing `.rs` files, `Cargo.toml`, or `Cargo.lock`.
- After touching FFI, concurrency, parsing/serialization, crypto, auth, or I/O.
- For incremental implementation, use `rust-compile-loop` instead. This skill is for **final verification**.

## Critical rules
1. NEVER fabricate tool output — every claim MUST have evidence.
2. NEVER introduce `unsafe` as a "quick fix" — use `rust-unsafe` skill.
3. NEVER add dependencies without following `rust-supply-chain` policy.
4. NEVER rationalize a failing gate — fix the root cause.

## Verification gates

| Gate | Command | Failure action |
|------|---------|---------------|
| Compilation | `cargo check` | Fix the first error, re-run |
| Tests | `cargo test` | Fix failing test, use `rust-testing` |
| Lints | `cargo clippy` | Fix lint or justify `#[allow()]` |
| Formatting | `cargo fmt --check` | Run `cargo fmt`, commit |
| Dependencies | `cargo audit` + `cargo deny` | Use `rust-supply-chain` skill |
| UB detection | `cargo +nightly miri test` | Use `rust-unsafe` skill |

## Procedure

1. Restate the task constraints and Definition of DONE.
2. Plan the smallest safe diff.
3. Implement with tests first.
4. Run all gates:

```bash
./scripts/verify.sh
```

5. If ANY gate fails:
   - Fix the root cause (use `rust-error-triage` for diagnosis).
   - Re-run the verifier.
   - Repeat until ALL gates are green.
6. Produce final output using `.agent/rules/01-rust-output-format.md`.

## When stuck
- Use compiler diagnostics as ground truth.
- Run `rustc --explain <CODE>` (e.g., `rustc --explain E0502`).
- If a design is unclear, propose 2 options with tradeoffs, pick one, implement.

## Required lint levels

```rust
#![deny(clippy::correctness)]     // Logical errors — MUST fix
#![warn(clippy::suspicious)]       // Likely bugs
#![warn(clippy::perf)]             // Performance footguns
```

For workspaces: configure at `[workspace.lints]` so all crates inherit the same policy.

## Release profile

```toml
[profile.release]
lto = "fat"
codegen-units = 1
```

## Performance principle
- ALWAYS profile before optimizing. NEVER guess the bottleneck.
- Correctness and tests come first, performance second.

