<rule id="01" name="rust-output-format" priority="high">

<purpose>Standardize the output format for completed Rust tasks so reviewers can quickly verify correctness.</purpose>

<requirements>
<req id="1" level="MUST">Output a Summary section with 1-3 bullets: what changed and why (no marketing).</req>
<req id="2" level="MUST">Output a Files Changed section listing all modified paths.</req>
<req id="3" level="MUST">Output a Commands Executed section with verbatim commands.</req>
<req id="4" level="MUST">Output a Results section with PASS/FAIL for each gate: `cargo fmt --check`, `cargo clippy`, `cargo test`, supply-chain (audit/deny/vet), Miri (if applicable). Include key lines on failure.</req>
<req id="5" level="MUST">Output a Checklist: minimal diff, formatting gate, clippy gate, tests gate, supply-chain gate, unsafe policy respected, evidence included.</req>
<req id="6" level="SHOULD">Output a Risks/Follow-ups section with remaining risks, migration notes, or future hardening.</req>
<req id="7" level="MUST NOT">Present the task as complete if any gate fails.</req>
</requirements>

<examples>
<example type="correct">
## Summary
- Added `Port` newtype with validation constructor.

## Files changed
- src/config.rs

## Commands executed
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Results
- cargo fmt: PASS
- cargo clippy: PASS
- cargo test: PASS (12 tests)

## Checklist
- [x] Minimal diff
- [x] All gates pass
</example>
<example type="violation">
Agent outputs "Done! Everything works." without commands or gate results.
</example>
</examples>

<enforcement>Review final output against the 7-section template. Any missing section = incomplete.</enforcement>

</rule>
