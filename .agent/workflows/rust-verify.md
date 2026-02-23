# rust-verify

Goal: run the full Rust verification pipeline and report results with evidence.

## Steps
1) Run `./tools/verify.sh` (or `./tools/verify.ps1` on Windows).
2) Paste the command output (or attach logs) so we can verify pass/fail.
3) If any gate fails:
   - Explain the root cause.
   - Propose the minimal fix.
   - Apply the fix as a small, reviewable diff.
   - Re-run the failing gate(s) until green.

## Rules
- Never claim “all good” unless the commands actually ran and you saw the output.
- Prefer **Planning mode** if the fix spans multiple files or requires more than 2 steps.
- Do not introduce `unsafe` unless explicitly required and justified.
- If `unsafe` exists, ensure `cargo +nightly miri test` is executed and green.
