<rule id="07" name="command-safety" priority="medium">

<purpose>Prevent accidental data loss or system damage from terminal commands.</purpose>

<requirements>
<req id="1" level="SHOULD">Prefer read-only / build commands (`cargo`, `rustc`, `git status`, `git diff`).</req>
<req id="2" level="MUST NOT">Run destructive commands unless explicitly requested and reviewed: `rm -rf`, `sudo`, `dd`, `mkfs`, mass delete, credential tools.</req>
<req id="3" level="MUST">If a workflow requires bypassing this rule, log an exception in `EXCEPTIONS.md`.</req>
</requirements>

<examples>
<example type="correct">
Agent runs `cargo test` and `git diff` without asking — these are safe read-only/build commands.
</example>
<example type="violation">
Agent auto-runs `rm -rf target/` without human approval.
</example>
</examples>

<enforcement>Any destructive command must have human approval. Exceptions logged in `EXCEPTIONS.md`.</enforcement>

</rule>
