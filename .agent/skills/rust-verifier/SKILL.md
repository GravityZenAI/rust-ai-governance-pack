---
name: rust-verifier
description: Verification loop ensuring correctness through external judges (compiler, clippy, tests, audit).
---

# Skill: Rust Verifier Loop

> Inherits all rules from `rust-core`. This skill adds the verification gate system.

## When to use
- **Final verification** of finished work.
- After editing `.rs` files, `Cargo.toml`, or `Cargo.lock`.
- After touching FFI, concurrency, parsing/serialization, crypto, auth, or I/O.
- For incremental implementation, use `rust-compile-loop` instead.

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
| Dependencies | `cargo audit` + `cargo deny` | Use `rust-supply-chain` |
| UB detection | `cargo +nightly miri test` | Use `rust-unsafe` |

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

## Example: verify.sh output interpretation

```
$ ./scripts/verify.sh
[PASS] cargo check
[PASS] cargo test
[FAIL] cargo clippy -- warning: unused variable `x`
       ^^^ Fix this before proceeding
[PASS] cargo fmt --check

# Action: fix the clippy warning, then re-run
```

## When stuck
- `rustc --explain <CODE>` for detailed error explanations.
- Propose 2 options with tradeoffs, pick one, implement.

## Required lint levels

```rust
#![deny(clippy::correctness)]     // Logical errors — MUST fix
#![warn(clippy::suspicious)]       // Likely bugs
#![warn(clippy::perf)]             // Performance footguns
```

Workspaces: configure at `[workspace.lints]` so all crates inherit the same policy.

## Release profile

```toml
[profile.release]
lto = "fat"
codegen-units = 1
```

## Performance principle
- ALWAYS profile before optimizing. NEVER guess the bottleneck.
- Correctness and tests come first, performance second.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Fabricating "all gates passed" without running verify.sh | ALWAYS run the script and paste real output |
| Rationalizing a clippy warning as "not important" | Fix it or document WHY with `#[allow()]` + comment |
| Running only `cargo check` and skipping clippy/fmt | Run the FULL `verify.sh` — partial checks miss issues |

