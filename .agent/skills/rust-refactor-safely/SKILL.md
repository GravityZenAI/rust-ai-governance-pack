---
name: rust-refactor-safely
description: Refactor Rust code while keeping compile/tests/clippy/fmt green at each step.
---

<skill name="rust-refactor-safely">

<description>Refactor Rust code while keeping compile/tests/clippy/fmt green at each step.</description>

<when_to_use>
- Any structural change to existing Rust code (rename, extract, simplify, reorganize).
- For adding new code, use `rust-compile-loop` instead.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="NEVER">Make a refactor step that breaks compilation — every step MUST compile.</rule>
<rule id="2" level="ALWAYS">Add or confirm tests BEFORE refactoring untested behavior.</rule>
<rule id="3" level="NEVER">Combine multiple refactors in one step.</rule>
<rule id="4" level="ALWAYS">If a refactor breaks tests, REVERT and try a smaller step.</rule>
</critical_rules>

<sections>

<section name="protocol">
<content>
1. Identify the refactor goal (rename, extract, simplify types).
2. Add or confirm tests — use `rust-testing` skill if needed.
3. Refactor in **one tiny step**: one rename OR one extraction OR one signature change.
4. After each step: `./scripts/verify.sh --fast`
5. If red: REVERT the step, try a smaller change.
6. At the end: `./scripts/verify.sh`
7. Update `DECISIONS.md` if a new convention is introduced.
</content>
</section>

<section name="example">
<code_example language="rust">
// Step 1: change parameter type (one change only)
// BEFORE
fn process(items: &amp;Vec&lt;String&gt;) -> usize { items.len() }

// AFTER — verify compiles, then update all callers
fn process(items: &amp;[String]) -> usize { items.len() }
// No caller changes needed — &amp;Vec&lt;T&gt; auto-derefs to &amp;[T]
</code_example>
</section>

<section name="anti-patterns">
<content>
| Anti-pattern | Fix |
|-------------|-----|
| Excessive `.clone()` | Replace with `&amp;T`; investigate if scoping resolves the borrow |
| `&amp;Vec&lt;T&gt;` or `&amp;String` in params | Refactor to `&amp;[T]` and `&amp;str` |
| Manual `Into` impls | Replace with `From` impls — `Into` is auto-derived |
| Over-abstraction | NEVER add generics without a concrete second use case |
| Manual indexing loops | Replace with iterators for safety and clarity |
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Renaming + changing signature in one commit</wrong>
<right>Do ONE refactor per step; verify between each</right>
</mistake>
<mistake id="2">
<wrong>Refactoring code without tests</wrong>
<right>Add tests FIRST, then refactor — otherwise you can't verify behavior</right>
</mistake>
<mistake id="3">
<wrong>Not reverting when stuck</wrong>
<right>REVERT immediately; git stash is your friend</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="./scripts/verify.sh">Exits 0 after the complete refactor</checkpoint>
<checkpoint id="2" command="cargo test">Same number of tests as before (no tests lost)</checkpoint>
<checkpoint id="3" command="git diff --stat">Shows only the files that should have changed</checkpoint>
<checkpoint id="4" command="git stash">Each commit is individually compilable</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" files="1-5">Refactor directly; one PR per refactor</level>
<level size="medium" files="10-50">Split into multiple PRs if > 5 files affected</level>
<level size="large" files="50+" workspace="true">Refactor one crate at a time; use `cargo check -p <crate>` per step</level>
<level size="cross-crate">Change the definition first, then update callers crate-by-crate</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-testing" relationship="add-tests-before-refactor" />
<related_skill name="rust-compile-loop" relationship="new-code-alternative" />
</integration>

</skill>
