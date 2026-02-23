<rule id="06" name="repo-memory" priority="medium">

<purpose>Maintain living documentation files so conventions and decisions persist across sessions and contributors.</purpose>

<requirements>
<req id="1" level="MUST">Keep `RUST_PLAYBOOK.md` updated with canonical Rust practices.</req>
<req id="2" level="MUST">Keep `ERROR_PATTERNS.md` updated with recurring errors and fixes.</req>
<req id="3" level="MUST">Keep `DECISIONS.md` updated with architecture/style decisions.</req>
<req id="4" level="MUST">Keep `EXCEPTIONS.md` updated with any bypass or rule exception.</req>
<req id="5" level="MUST">If you change conventions, update the memory files in the same diff.</req>
</requirements>

<examples>
<example type="correct">
Agent changes error handling convention → updates both the code AND RUST_PLAYBOOK.md in one commit.
</example>
<example type="violation">
Agent introduces a new pattern but does not update any memory file.
</example>
</examples>

<enforcement>Review diffs: any convention change must include corresponding memory file updates.</enforcement>

</rule>
