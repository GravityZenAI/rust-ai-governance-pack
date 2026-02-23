---
name: rust-verifier
description: Verification loop ensuring correctness through external judges (compiler, clippy, tests, audit).
---

<skill name="rust-verifier">

<description>Verification loop ensuring correctness through external judges (compiler, clippy, tests, audit). Capstone skill that connects all others.</description>

<when_to_use>
- **Final verification** of finished work.
- After editing `.rs` files, `Cargo.toml`, or `Cargo.lock`.
- After touching FFI, concurrency, parsing/serialization, crypto, auth, or I/O.
- For incremental implementation, use `rust-compile-loop` instead.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="NEVER">Fabricate tool output — every claim MUST have evidence.</rule>
<rule id="2" level="NEVER">Introduce `unsafe` as a "quick fix" — use `rust-unsafe` skill.</rule>
<rule id="3" level="NEVER">Add dependencies without following `rust-supply-chain` policy.</rule>
<rule id="4" level="NEVER">Rationalize a failing gate — fix the root cause.</rule>
</critical_rules>

<sections>

<section name="verification-gates">
<content>
| Gate | Command | Failure action |
|------|---------|---------------|
| Compilation | `cargo check` | Fix the first error, re-run |
| Tests | `cargo test` | Fix failing test, use `rust-testing` |
| Lints | `cargo clippy` | Fix lint or justify `#[allow()]` |
| Formatting | `cargo fmt --check` | Run `cargo fmt`, commit |
| Dependencies | `cargo audit` + `cargo deny` | Use `rust-supply-chain` |
| UB detection | `cargo +nightly miri test` | Use `rust-unsafe` |
</content>
</section>

<section name="procedure">
<content>
1. Restate the task constraints and Definition of DONE.
2. Plan the smallest safe diff.
3. Implement with tests first.
4. Run all gates: `./scripts/verify.sh`
5. If ANY gate fails:
   - Fix the root cause (use `rust-error-triage` for diagnosis).
   - Re-run the verifier.
   - Repeat until ALL gates are green.
6. Produce final output using `.agent/rules/01-rust-output-format.md`.
</content>
</section>

<section name="example">
<code_example language="bash">
$ ./scripts/verify.sh
[PASS] cargo check
[PASS] cargo test
[FAIL] cargo clippy -- warning: unused variable `x`
       ^^^ Fix this before proceeding
[PASS] cargo fmt --check

# Action: fix the clippy warning, then re-run
</code_example>
</section>

<section name="when-stuck">
<content>
- `rustc --explain <CODE>` for detailed error explanations.
- Propose 2 options with tradeoffs, pick one, implement.
</content>
</section>

<section name="lint-levels">
<code_example language="rust">
#![deny(clippy::correctness)]     // Logical errors — MUST fix
#![warn(clippy::suspicious)]       // Likely bugs
#![warn(clippy::perf)]             // Performance footguns
</code_example>
<content>
Workspaces: configure at `[workspace.lints]` so all crates inherit the same policy.
</content>
</section>

<section name="release-profile">
<code_example language="toml">
[profile.release]
lto = "fat"
codegen-units = 1
</code_example>
</section>

<section name="performance-principle">
<content>
ALWAYS profile before optimizing. NEVER guess the bottleneck.
Correctness and tests come first, performance second.
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Fabricating "all gates passed" without running verify.sh</wrong>
<right>ALWAYS run the script and paste real output</right>
</mistake>
<mistake id="2">
<wrong>Rationalizing a clippy warning as "not important"</wrong>
<right>Fix it or document WHY with `#[allow()]` + comment</right>
</mistake>
<mistake id="3">
<wrong>Running only `cargo check` and skipping clippy/fmt</wrong>
<right>Run the FULL `verify.sh` — partial checks miss issues</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="./scripts/verify.sh">Exits with code 0 (ALL 6 gates pass)</checkpoint>
<checkpoint id="2" command="cat deliverable.md">Output is pasted as evidence — no fabrication</checkpoint>
<checkpoint id="3" command="grep -rn '#\[allow' src/">Every `#[allow]` has inline justification comment</checkpoint>
<checkpoint id="4" command="cat .agent/rules/01-rust-output-format.md">Final output follows the required format</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" crates="1">Run `./scripts/verify.sh` — covers all gates in seconds</level>
<level size="medium" crates="workspace">Run verify per crate during development; full workspace verify before merge</level>
<level size="large" crates="50+">CI runs full verify; local dev uses `cargo check -p <crate>` + clippy per crate</level>
<level size="monorepo" mixed="true">Rust verify.sh covers Rust only; integrate with top-level CI for other languages</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-compile-loop" relationship="incremental-alternative" />
<related_skill name="rust-error-triage" relationship="diagnosis-on-failure" />
<related_skill name="rust-testing" relationship="test-gate" />
<related_skill name="rust-supply-chain" relationship="dependency-gate" />
<related_skill name="rust-unsafe" relationship="ub-gate" />
</integration>

</skill>
