---
name: rust-compile-loop
description: Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.
---

<skill name="rust-compile-loop">

<description>Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.</description>

<when_to_use>
- Any task that modifies `.rs` files.
- For **incremental implementation**. For final verification, use `rust-verifier`.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="ALWAYS">Fix ONLY the first compiler error — later ones are often cascading.</rule>
<rule id="2" level="ALWAYS">Run `./scripts/verify.sh --fast` after each fix step.</rule>
<rule id="3" level="ALWAYS">Provide evidence: commands + output + diff when claiming green.</rule>
<rule id="4" level="NEVER">Never try to fix ALL errors at once.</rule>
</critical_rules>

<sections>

<section name="operating-protocol">
<content>
1. Restate the spec in 3–6 bullets.
2. Identify the smallest compilable step.
3. Implement that step.
4. Run `./scripts/verify.sh --fast`.
5. If red (compile error, test failure, clippy lint, or fmt drift):
   - Read the **first** error only.
   - Fix only what is required for that error.
   - Re-run `--fast`.
   - Repeat until green.
6. When green:
   - Run full `./scripts/verify.sh`.
   - Provide evidence: commands + output + diff.
</content>
</section>

<section name="heuristics">
<content>
| Error type | Fix strategy |
|------------|-------------|
| Borrow checker (E0382, E0505) | Reduce scope; introduce bindings; reorder operations |
| Type mismatch (E0308) | Inspect types; add annotations only where needed |
| Trait not satisfied (E0277) | Add bounds or implement the trait; do NOT over-generalize |
| Incompatible error types | Implement `From` or use `.map_err()` |
| Format/clippy | `cargo fmt` to fix; for clippy, apply the fix or `#[allow()]` with justification |
</content>
</section>

<section name="example">
<code_example language="rust">
// WRONG — borrow lives too long
fn process(data: &amp;mut Vec&lt;i32&gt;) -> &amp;i32 {
    data.push(42);
    data.last().unwrap()  // borrows data while we just mutated it
}

// RIGHT — introduce a binding to reduce borrow scope
fn process(data: &amp;mut Vec&lt;i32&gt;) -> i32 {
    data.push(42);
    *data.last().unwrap()  // copy the value, borrow ends
}
</code_example>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Trying to fix ALL errors at once</wrong>
<right>Fix ONLY the first error — later ones are often cascading</right>
</mistake>
<mistake id="2">
<wrong>Adding `.clone()` to silence the borrow checker</wrong>
<right>Try reducing borrow scope first; clone only as last resort</right>
</mistake>
<mistake id="3">
<wrong>Skipping `--fast` and running full verify every time</wrong>
<right>Use `--fast` during iteration, full verify only once at the end</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="./scripts/verify.sh --fast">Exits with code 0</checkpoint>
<checkpoint id="2" command="cargo test">Shows 0 failures</checkpoint>
<checkpoint id="3" command="cargo clippy">Shows 0 warnings (or all have `#[allow()]` with justification)</checkpoint>
<checkpoint id="4" command="git diff">Contains ONLY the changes described in the spec bullets from step 1</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" files="1-5">Run `cargo check` directly; `--fast` flag optional</level>
<level size="medium" files="5-50">Use `--fast` to skip slow tests during iteration; full verify at end</level>
<level size="large" files="50+">Run `cargo check -p <crate>` on the affected crate only; full verify on CI</level>
<level size="monorepo">Use `cargo check --workspace` only at final step; iterate on single crate</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-verifier" relationship="final-verification" />
<related_skill name="rust-error-triage" relationship="error-diagnosis" />
</integration>

</skill>
