# Instrucciones: Conversion a XML

<important_paths>
BEFORE starting any work, confirm you can access BOTH paths:

1. Omega Optimizer skill (Task A):
   C:\Users\negro\.gemini\antigravity\skills\omega-optimizer\SKILL.md

2. Rust repo skills (Task B):
   C:\Users\negro\Sol\proyectos\rust-ai-governance-pack\.agent\skills\

Write in AGENT_NOTES.md:
- "Task A file accessible: YES/NO"
- "Task B files accessible: YES/NO"

If Task A file is NOT accessible, skip Task A and do Task B only.
</important_paths>

<role>
You have TWO tasks in sequence. Complete Task A FULLY before starting Task B.
Task A: Convert the Omega Optimizer skill to XML structure.
Task B: Convert the 9 Rust skills in this repo to XML structure.
</role>

<constraints>
- Complete Task A FULLY before starting Task B
- Document ALL work in AGENT_NOTES.md (clear it first — write fresh)
- NEVER delete content — restructure using XML tags while preserving all rules
- XML tags guide AI parsing. Keep content in natural language inside the tags.
- After EACH task, run: `.\tools\verify-agent-work.ps1`
- NEVER say "done" until BOTH tasks are complete
</constraints>

<why_xml>
XML tags help AI agents follow instructions more precisely because:
1. Clear section boundaries — AI knows exactly where rules start/end
2. Hierarchy — nested tags show priority and relationships
3. Parseable — AI can extract specific sections without reading everything
4. Proven — AGENT_INSTRUCTIONS.md used XML and agents followed it perfectly
</why_xml>

---

# TASK A: Convert Omega Optimizer to XML

<source_file>
C:\Users\negro\.gemini\antigravity\skills\omega-optimizer\SKILL.md
</source_file>

<task_a_instructions>
1. Read the entire Omega Optimizer SKILL.md
2. Restructure it using XML tags. The structure should be:

```xml
<skill name="omega-optimizer" version="3.0">
  <description>...</description>

  <workflow>
    <phase name="diagnosis">...</phase>
    <phase name="research">...</phase>
    <phase name="repair">...</phase>
    <phase name="verification">...</phase>
    <phase name="adversarial">...</phase>
  </workflow>

  <agents>
    <agent id="1" name="analyst">
      <role>...</role>
      <objective>...</objective>
      <instructions>...</instructions>
      <tools>...</tools>
      <constraints>...</constraints>
      <inputs>...</inputs>
      <outputs>...</outputs>
      <success_criteria>...</success_criteria>
    </agent>
    <!-- repeat for all 5 agents -->
  </agents>

  <dimensions count="10">
    <dimension id="1" name="clarity">...</dimension>
    <!-- repeat for all 10 -->
  </dimensions>

  <scoring>
    <scale min="0" max="10" />
    <total max="100" />
    <levels>
      <level range="0-30" label="critical" />
      <level range="31-50" label="improvable" />
      <level range="51-75" label="good" />
      <level range="76-100" label="excellent" />
    </levels>
  </scoring>

  <usage>
    <basic>...</basic>
    <advanced>...</advanced>
  </usage>

  <lessons_learned>...</lessons_learned>
</skill>
```

3. Keep YAML frontmatter at the top (required by the system)
4. Keep ALL content — just wrap it in XML tags
5. Test that it is valid XML structure (tags properly opened/closed)
6. Mark [x] Task A in AGENT_NOTES.md
</task_a_instructions>

---

# TASK B: Convert 9 Rust Skills to XML

<task_b_instructions>
1. For each of the 9 skills in `.agent/skills/*/SKILL.md`:
2. Restructure using XML tags. The structure for REFERENCE skills is:

```xml
<skill name="skill-name">
  <description>...</description>
  <when_to_use>...</when_to_use>
  <inherits from="rust-core" />  <!-- if applicable -->

  <critical_rules>
    <rule id="1" level="ALWAYS">...</rule>
    <rule id="2" level="NEVER">...</rule>
  </critical_rules>

  <sections>
    <section name="...">
      <content>...</content>
      <code_example language="rust">...</code_example>
    </section>
  </sections>

  <common_mistakes>
    <mistake id="1">
      <wrong>...</wrong>
      <right>...</right>
    </mistake>
  </common_mistakes>

  <verification_checkpoints>
    <checkpoint id="1" command="...">...</checkpoint>
  </verification_checkpoints>

  <scalability>
    <level size="small">...</level>
    <level size="large">...</level>
  </scalability>

  <integration>
    <related_skill name="..." relationship="..."/>
  </integration>
</skill>
```

3. Keep YAML frontmatter at the top
4. Keep ALL existing content
5. Mark [x] per skill in AGENT_NOTES.md
</task_b_instructions>

<skills_list>
1. rust-compile-loop
2. rust-core
3. rust-error-triage
4. rust-kata-coach
5. rust-refactor-safely
6. rust-supply-chain
7. rust-testing
8. rust-unsafe
9. rust-verifier
</skills_list>

<success_criteria>
COMPLETE only when:
1. Task A: Omega Optimizer converted to XML structure
2. Task B: All 9 skills converted to XML structure
3. verify-agent-work.ps1 output pasted showing PASS
4. "COMPLETADO" at bottom of AGENT_NOTES.md
</success_criteria>
