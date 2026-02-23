# Instrucciones: Meta-Optimizer + Segunda Pasada

<role>
You have TWO tasks in sequence. Complete Task A FULLY before starting Task B.
Task A: Improve the Omega Optimizer process itself.
Task B: Use the improved optimizer to do a second pass on all 9 skills.
</role>

<constraints>
- Complete Task A FULLY before starting Task B
- Document ALL work in AGENT_NOTES.md (clear it first — write fresh)
- You may ONLY modify files inside `.agent/skills/` and `AGENT_INSTRUCTIONS.md`
- NEVER delete existing content — only restructure and enhance
- After EACH task, run: `.\tools\verify-agent-work.ps1`
- NEVER say "done" until BOTH tasks are complete
</constraints>

---

# TASK A: Meta-Optimize the Optimizer

<objective>
Evaluate and improve the Omega Optimizer process (this file's methodology).
You are optimizing HOW the optimization is done, not the skills themselves.
</objective>

<task_a_process>
## A1. Read the previous Omega Optimizer audit
Read the PREVIOUS `AGENT_NOTES.md` content (it was cleared but the pattern is in git history).
Read the skill files in `.agent/skills/` to understand what the optimizer produced.

## A2. Evaluate the Omega Optimizer methodology from 3 perspectives:

### CIRUJANO (Structure Expert)
- Are the 5 evaluation perspectives sufficient? Missing any?
- Are the 8 scoring dimensions the right ones? Should any be added/removed?
- Is the step-by-step process efficient or does it have unnecessary steps?
- Is the verification checkpoint system robust?

### FISCAL (Critic)
- What did the optimizer MISS in the first pass? (Read the skills — find gaps)
- Where are the instructions ambiguous?
- What allowed the optimizer to give high scores to mediocre improvements?
- What quality problems persist in the skills despite optimization?

### JUEZ (Decision Maker)
- Based on Cirujano's analysis and Fiscal's criticism:
- Which improvements to the methodology should be adopted?
- Which should be rejected (over-engineering)?
- Write the IMPROVED methodology

## A3. Write the improved Omega Optimizer v2
In AGENT_NOTES.md, write the complete improved methodology under:
```
## Omega Optimizer v2 (Improved)
[complete methodology here]
```

## A4. Mark Task A as [x] in AGENT_NOTES.md
</task_a_process>

---

# TASK B: Second Pass with Improved Optimizer

<objective>
Using the improved Omega Optimizer v2 methodology you just created,
run a SECOND optimization pass on all 9 skills.
Focus ONLY on what the first pass missed.
</objective>

<task_b_process>
## B1. For each skill (9 total):
1. Read the current SKILL.md
2. Apply your improved Omega v2 methodology
3. Focus on:
   - Gaps the first optimizer missed
   - Weak language that survived ("consider", "try to", "where possible")
   - Missing edge cases
   - Missing code examples for critical rules
   - Rules that contradict between skills
4. Score BEFORE and AFTER on your improved dimensions
5. Mark [x] in AGENT_NOTES.md

## B2. Final summary table
```
## Second Pass Results
| Skill | Before (v1 score) | After (v2 score) | Delta | Key fixes |
|-------|-------------------|-------------------|-------|-----------|
```

## B3. Run verification
Run: `.\tools\verify-agent-work.ps1`
Paste output in AGENT_NOTES.md

## B4. Write COMPLETADO at the end
</task_b_process>

<success_criteria>
COMPLETE only when:
1. Task A: Improved methodology written in AGENT_NOTES.md
2. Task B: All 9 skills processed with v2 methodology  
3. Before/after scores for all 9 skills
4. verify-agent-work.ps1 output pasted showing PASS
5. "COMPLETADO" at bottom of AGENT_NOTES.md
</success_criteria>
