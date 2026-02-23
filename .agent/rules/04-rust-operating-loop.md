<rule id="04" name="rust-operating-loop" priority="critical">

<purpose>Enforce a disciplined, incremental, evidence-based workflow for all Rust changes in this workspace.</purpose>

<requirements>
<req id="1" level="MUST">Spec first: state expected behavior, edge cases, and success criteria before writing code.</req>
<req id="2" level="MUST">Incremental implementation only: make the smallest change that can be compiled. Prefer smaller PR-sized edits.</req>
<req id="3" level="MUST">Compiler and tests are the judge: after each meaningful edit run `./scripts/verify.sh --fast`; before marking done run `./scripts/verify.sh`.</req>
<req id="4" level="MUST NOT">Make unverifiable claims. Never claim "fixed", "works", or "green" without showing: the exact commands executed, the relevant output, and the `git diff`.</req>
<req id="5" level="MUST">Error memory: if a rustc error code (E0xxx) or a clippy lint appears and is not already documented, add an entry to `ERROR_PATTERNS.md`.</req>
</requirements>

<examples>
<example type="correct">
1. State spec in 3 bullets.
2. Implement smallest compilable step.
3. Run `./scripts/verify.sh --fast`.
4. Show output + diff.
5. Add new E0382 pattern to ERROR_PATTERNS.md.
</example>
<example type="violation">
Agent writes 200 lines of code, then runs verify.sh once at the end without incremental checks.
</example>
</examples>

<enforcement>Every meaningful edit must be followed by `verify.sh --fast`. Final output must include commands + output + diff.</enforcement>

</rule>
