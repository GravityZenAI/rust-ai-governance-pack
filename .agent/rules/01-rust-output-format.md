# Rust Output Format (What the agent must return)

When completing a task, output must follow this structure:

## Summary
- 1–3 bullets: what changed and why (no marketing).

## Files changed
- List of paths.

## Commands executed (verbatim)
- `...`
- `...`

## Results (summarized, not invented)
- `cargo fmt --check`: PASS/FAIL (+ key lines if fail)
- `cargo clippy`: PASS/FAIL (+ key lints if fail)
- `cargo test`: PASS/FAIL (+ failing test names if fail)
- Supply-chain: PASS/FAIL (audit/deny/vet as configured)
- Miri (if applicable): PASS/FAIL

## Checklist
- [ ] Minimal diff
- [ ] Formatting gate
- [ ] Clippy gate
- [ ] Tests gate
- [ ] Supply-chain gate
- [ ] Unsafe policy respected
- [ ] Evidence included

## Risks / Follow-ups
- Any remaining risk, migration note, or future hardening.

If any gate fails, do NOT present the task as complete.
