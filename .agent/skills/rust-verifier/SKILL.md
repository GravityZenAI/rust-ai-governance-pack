# Skill: Rust Verifier Loop

## When to use
Use this skill whenever:
- you edit `.rs` files, `Cargo.toml`, or `Cargo.lock`;
- you touch FFI, concurrency, parsing/serialization, crypto, auth, or I/O.

## Objective
Force correctness through **external judges**:
- `rustc/cargo` for compilation
- `clippy` for lint + footguns
- tests for behavior
- supply-chain tools for dependency risk
- Miri for UB when `unsafe` exists

## Procedure
1) Restate the task constraints and the Definition of DONE.
2) Plan the smallest safe diff.
3) Implement with tests first when feasible.
4) Run `tools/verify.sh`.
5) If any gate fails:
   - do not rationalize,
   - fix the root cause,
   - re-run the verifier,
   - repeat until green.
6) Produce final output using `.agent/rules/01-rust-output-format.md`.

## What to do when stuck
- Use compiler diagnostics as ground truth.
- If an error is unclear, run: `rustc --explain <CODE>` (e.g. `E0502`).
- If a design is unclear, propose 2 options with tradeoffs, then pick one and implement.

## Never do
- Never fabricate tool output.
- Never introduce `unsafe` as a “quick fix”.
- Never add dependencies without following the dependency policy.
