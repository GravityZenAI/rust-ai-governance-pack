# Instrucciones: Optimizar y convertir a XML el resto del repositorio

<role>
You have THREE tasks in sequence. Complete each FULLY before starting the next.
Task A: Convert the 8 Rules to XML structure.
Task B: Convert docs and root markdown files to XML structure.
Task C: Fix the CI workflow — make ALL katas compile and pass tests.
</role>

<constraints>
- Complete Task A FULLY before starting Task B, and Task B before Task C
- Document ALL work in AGENT_NOTES.md (clear it first — write fresh)
- NEVER delete content — restructure using XML tags while preserving all rules
- XML tags guide AI parsing. Keep content in natural language inside the tags.
- After ALL tasks, run: `.\tools\verify-agent-work.ps1`
- NEVER say "done" until ALL three tasks are complete
</constraints>

---

# TASK A: Convert 8 Rules to XML

<source_dir>.agent/rules/</source_dir>

<files>
1. 00-rust-contract.md
2. 01-rust-output-format.md
3. 02-rust-dependency-policy.md
4. 03-antigravity-ops-security.md
5. 04-rust-operating-loop.md
6. 05-rust-quality-bar.md
7. 06-repo-memory.md
8. 07-command-safety.md
</files>

<task_a_instructions>
Rules are CONTRACTS — the AI MUST follow them. Use this XML structure:

```xml
<rule id="XX" name="rule-name" priority="critical|high|medium">
  <purpose>What this rule prevents or ensures</purpose>

  <requirements>
    <req id="1" level="MUST">...</req>
    <req id="2" level="MUST NOT">...</req>
    <req id="3" level="SHOULD">...</req>
  </requirements>

  <examples>
    <example type="correct">...</example>
    <example type="violation">...</example>
  </examples>

  <enforcement>How to verify compliance</enforcement>
</rule>
```

Keep YAML frontmatter if present. Preserve ALL content.
Mark [x] per rule in AGENT_NOTES.md.
</task_a_instructions>

---

# TASK B: Convert Docs and Root Files to XML

<files>
1. README.md (keep as-is — this is for humans on GitHub, NOT for AI)
2. ARCHITECTURE.md
3. docs/DEFINITION_OF_DONE.md
4. docs/EXCEPTIONS.md
5. docs/KATA_RUBRIC.md
6. CONTRIBUTING.md
</files>

<task_b_instructions>
IMPORTANT: Do NOT convert README.md to XML — it must stay human-readable for GitHub.

For the other files, use this XML structure:

```xml
<document name="file-name" type="architecture|policy|rubric|guide">
  <summary>One-line purpose</summary>

  <sections>
    <section name="...">
      <content>...</content>
    </section>
  </sections>

  <checklists>
    <checklist name="...">
      <item id="1" required="true">...</item>
    </checklist>
  </checklists>
</document>
```

Mark [x] per file in AGENT_NOTES.md.
</task_b_instructions>

---

# TASK C: Fix CI — Make Katas Compile

<critical>
The GitHub Actions CI runs `cargo fmt --check`, `cargo clippy -- -D warnings`,
and `cargo test` on `training/kata_suite/`. ALL THREE must pass.
</critical>

<task_c_instructions>
1. Navigate to `training/kata_suite/`
2. Run `cargo check` — fix ALL compilation errors
3. Run `cargo fmt --check` — fix ALL formatting issues
4. Run `cargo clippy -- -D warnings` — fix ALL warnings
5. Run `cargo test` — fix ALL failing tests
6. ALL FOUR commands must exit with code 0

DO NOT skip any kata. If a kata has a bug, fix it.
If a kata is intentionally broken (for training), add `#[ignore]` to the test
and document why in AGENT_NOTES.md.

After all katas pass, record the full output of `cargo test` in AGENT_NOTES.md.
</task_c_instructions>

---

<checklist>
## Task A: Rules to XML
- [ ] 00-rust-contract.md
- [ ] 01-rust-output-format.md
- [ ] 02-rust-dependency-policy.md
- [ ] 03-antigravity-ops-security.md
- [ ] 04-rust-operating-loop.md
- [ ] 05-rust-quality-bar.md
- [ ] 06-repo-memory.md
- [ ] 07-command-safety.md

## Task B: Docs to XML
- [ ] ARCHITECTURE.md
- [ ] docs/DEFINITION_OF_DONE.md
- [ ] docs/EXCEPTIONS.md
- [ ] docs/KATA_RUBRIC.md
- [ ] CONTRIBUTING.md

## Task C: Fix CI
- [ ] cargo check passes
- [ ] cargo fmt --check passes
- [ ] cargo clippy -- -D warnings passes
- [ ] cargo test passes
</checklist>

<success_criteria>
COMPLETE only when:
1. All 8 rules converted to XML
2. All 5 docs converted to XML (NOT README)
3. All 4 cargo commands pass with exit code 0
4. verify-agent-work.ps1 output pasted showing PASS
5. "COMPLETADO" at bottom of AGENT_NOTES.md
</success_criteria>
