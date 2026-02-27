<document name="architecture" type="architecture">

<summary>Blueprint for AI-governed Rust development: rules, skills, workflows, verification, and repo memory.</summary>

<sections>

<section name="goal">
<content>
Goal: enable an AI agent (e.g., Google Antigravity) to work on Rust with real operational competence:
- Converge quickly to green (compiles + tests + clippy + fmt).
- Reduce repeated errors (by recording patterns and decisions).
- Maintain consistency across refactors and style.
</content>
</section>

<section name="metrics">
<content>
Baseline metrics (per PR / per session):
- `verify green rate`: percentage of successful `verify.sh` executions.
- `kata pass rate`: percentage of katas solved without human intervention.
- `time-to-green`: time/iterations from "red" to "green".
</content>
</section>

<section name="permanent-rules">
<content>
Location: `.agent/rules/`

These rules are guardrails: they are not "advice"; they are operating conditions.

### R0 — Compiler as judge
"Should compile" is not accepted. The AI MUST run the compiler and fix until green.
Closing a task without execution evidence is prohibited.

### R1 — Mandatory loop
1) Brief specification (inputs/outputs, invariants, edge cases).
2) Incremental implementation (small steps).
3) `./tools/verify.sh` (or `--fast` during iteration).
4) Record new patterns in `ERROR_PATTERNS.md`.

### R2 — Zero warnings in CI
Clippy with warnings as errors. `fmt` validated with `--check`.

### R3 — Quality prohibitions
No `unsafe` without justification + test. Avoid `unwrap()`/`expect()` in production.
Avoid direct indexing if there is risk of out-of-range.

### R4 — Repo memory always up to date
Changes in conventions and exceptions are recorded in `DECISIONS.md` / `EXCEPTIONS.md`.
</content>
</section>

<section name="modular-skills">
<content>
Location: `.agent/skills/`

Skills are "operation modules" to reduce ambiguity:
- `rust-compile-loop`: incremental implementation + how to use the compiler.
- `rust-error-triage`: systematic diagnosis of rustc/clippy errors.
- `rust-refactor-safely`: test-guided refactors.
- `rust-kata-coach`: training with `training/kata_suite`.

Principle: the AI does not improvise the process; it selects a Skill and executes its checklist.
</content>
</section>

<section name="workflows">
<content>
Location: `.agent/workflows/`

Workflows = active sequences:
- `/verify`: executes verification and produces a report.
- `/kata`: picks an exercise, runs tests, and evaluates.
- `/log-decision`: adds an entry in `DECISIONS.md`.
</content>
</section>

<section name="verifier-and-ci">
<content>
### `tools/verify.sh`
A single script as the source of truth:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- optional: `cargo geiger` (unsafe footprint in dependencies)
- optional: `cargo vet` (supply-chain vetting)
- optional: `cargo miri test` (nightly) if `RUN_MIRI=1`
- optional: `cargo deny check` / `cargo audit` if installed

### CI gate
CI must call `./tools/verify.sh`. Merge is blocked if it fails.
</content>
</section>

<section name="repo-memory">
<content>
Canonical files:
- `RUST_PLAYBOOK.md`: "how we do Rust here".
- `ERROR_PATTERNS.md`: error library with fixes.
- `DECISIONS.md`: decisions with date, rationale, and tradeoffs.
</content>
</section>

<section name="exception-protocol">
<content>
File: `EXCEPTIONS.md`

Bypass is allowed only if:
- it is documented with reason, scope, expiry, and human approver.
- it does not break CI (or CI is explicitly adjusted with a recorded decision).
</content>
</section>

</sections>

</document>
