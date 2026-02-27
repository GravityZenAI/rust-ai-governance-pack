<document name="definition-of-done" type="policy">

<summary>Objective evidence required before any Rust task can be marked DONE.</summary>

<checklists>

<checklist name="mandatory-evidence">
<item id="1" required="true">Commands executed (copied verbatim): `./tools/verify.sh` (or `--fast` during iteration).</item>
<item id="2" required="true">Output relevante: fragment showing success — fmt OK, clippy OK, tests OK.</item>
<item id="3" required="true">Diff: `git diff` (or PR diff) showing changes.</item>
<item id="4" required="true">Factual summary: what changed and why.</item>
<item id="5" required="true">Repo memory reference: if you fixed a recurring error → entry in `ERROR_PATTERNS.md`; if you made a decision → entry in `DECISIONS.md`; if you bypassed a rule → entry in `EXCEPTIONS.md`.</item>
</checklist>

<checklist name="approval-criteria">
<item id="1" required="true">`./tools/verify.sh` exits with code 0.</item>
<item id="2" required="true">No warnings in clippy (treated as errors).</item>
<item id="3" required="true">`cargo fmt` passes in check mode.</item>
<item id="4" required="true">All tests pass.</item>
</checklist>

<checklist name="failure-criteria">
<item id="1" required="true">"Funciona en mi cabeza" — no evidence of execution.</item>
<item id="2" required="true">No evidence of tool execution pasted.</item>
<item id="3" required="true">Verifier skipped without an approved exception.</item>
</checklist>

</checklists>

</document>
