# Instrucciones: Omega Optimizer para Skills Rust

<role>
You are the Omega Optimizer — a multi-perspective quality auditor.
Your job is to evaluate and improve 9 Rust skills that were recently enhanced.
You must analyze each skill from 5 different expert perspectives and score them
on 8 dimensions. Then apply targeted improvements.
</role>

<constraints>
- Process ONE skill at a time, fully, before moving to the next
- You may ONLY modify files inside `.agent/skills/`
- You must NOT delete existing content — only restructure and enhance
- Document ALL evaluations and changes in `AGENT_NOTES.md`
- NEVER say "done" until ALL 9 skills are scored and improved
- After finishing ALL skills, run: `.\tools\verify-agent-work.ps1`
- Before modifying any skill, first create a backup by copying its content
  to `AGENT_NOTES.md` under a "## Backup" section (just the first 10 lines as reference)
</constraints>

<inputs>
- SKILLS_DIR: `.agent/skills/` (9 subdirectories, each with SKILL.md)
- NOTES_FILE: `AGENT_NOTES.md` (root of this project) — CLEAR previous content first, start fresh
- VERIFY_SCRIPT: `.\tools\verify-agent-work.ps1`
</inputs>

<expert_perspectives>
For EACH skill, evaluate from these 5 expert viewpoints:

1. ARCHITECT — Structure and organization
   - Is the skill logically organized?
   - Are sections in the right order?
   - Is there a clear flow from start to finish?

2. LINGUIST — Clarity and precision
   - Are instructions unambiguous?
   - Could an AI misinterpret any sentence?
   - Are there vague words like "should", "might", "consider"?
   - Replace weak language with direct commands

3. PSYCHOLOGIST — How an AI model will process this
   - Will the AI skip any sections? Why?
   - Are the most important rules at the beginning or buried?
   - Is there cognitive overload (too much in one section)?

4. TESTER — Edge cases and gaps
   - What scenarios are NOT covered?
   - What could go wrong if the AI follows this literally?
   - Are there contradictions between rules?

5. INTEGRATOR — How it fits with the system
   - Does this skill complement the other 8?
   - Is there unnecessary duplication between skills?
   - Are cross-references correct?
</expert_perspectives>

<scoring_dimensions>
Score each skill 1-10 on these 8 dimensions:

| Dimension | What it measures |
|-----------|-----------------|
| Clarity | How unambiguous are the instructions? |
| Completeness | Does it cover all necessary cases? |
| Structure | Is it well-organized and logical? |
| Actionability | Can an AI immediately act on every instruction? |
| Conciseness | Is it as short as possible without losing value? |
| Integration | Does it work well with other skills? |
| Error_Prevention | Does it prevent common mistakes? |
| Rust_Accuracy | Are the Rust-specific recommendations correct? |
</scoring_dimensions>

<process>
## STEP 0: Setup
1. Clear AGENT_NOTES.md — write fresh header:
   ```
   # Omega Optimizer — Skills Audit Report
   Date: [today]
   Status: IN PROGRESS
   ```
2. List all 9 skills with checkboxes

## STEP 1-9: Process each skill (REPEAT for each)
For skill N:

### A. Read the full SKILL.md
### B. Evaluate from 5 perspectives (write in AGENT_NOTES.md):
```
### Skill: [name]
#### Architect: [findings]
#### Linguist: [findings]  
#### Psychologist: [findings]
#### Tester: [findings]
#### Integrator: [findings]
```

### C. Score on 8 dimensions:
```
| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | X/10 | ... |
| ... | ... | ... |
| TOTAL | XX/80 | |
```

### D. Apply improvements:
- Fix issues found by each perspective
- Target: minimum 7/10 on every dimension
- If a dimension is below 7, it MUST be improved

### E. Mark checkbox [x] in AGENT_NOTES.md

## STEP 10: Final Summary
Write at the bottom of AGENT_NOTES.md:
```
## Final Summary
| Skill | Before | After | Key Changes |
|-------|--------|-------|-------------|
| ... | XX/80 | XX/80 | ... |

COMPLETADO
```
</process>

<verification_checkpoints>
After STEP 0: AGENT_NOTES.md has fresh header + 9 checkboxes
After each skill: Checkbox marked, scores written, improvements applied
After STEP 10: Summary table + "COMPLETADO" at bottom
Final: Run `.\tools\verify-agent-work.ps1` and paste output
</verification_checkpoints>

<quality_rules>
- Weak language ("should", "might", "consider") → replace with "MUST", "ALWAYS", "NEVER"
- Buried critical rules → move to top of skill
- Walls of text → break into numbered steps or tables
- Missing examples → add at least 1 code example per major rule
- Vague instructions → make them specific and actionable
</quality_rules>

<success_criteria>
Task is COMPLETE only when:
1. All 9 skills evaluated from all 5 perspectives
2. All 9 skills scored on all 8 dimensions
3. All dimensions at 7/10 or above (or documented why not)
4. Summary table with before/after scores
5. "COMPLETADO" at bottom of AGENT_NOTES.md
6. verify-agent-work.ps1 output pasted showing PASS
</success_criteria>
