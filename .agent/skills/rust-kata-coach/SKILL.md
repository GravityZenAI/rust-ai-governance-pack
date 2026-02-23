---
name: rust-kata-coach
description: Run and solve Rust katas with compiler-driven feedback and scoring.
---

<skill name="rust-kata-coach">

<description>Run and solve Rust katas with compiler-driven feedback and scoring.</description>

<when_to_use>
- Training exercises in `training/kata_suite`.
- For error diagnosis during katas, use `rust-error-triage`.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="ALWAYS">Pick ONE failing test at a time — fix it, verify, then move to the next.</rule>
<rule id="2" level="ALWAYS">Write a 1-line learning note in the kata file after each solution.</rule>
<rule id="3" level="ALWAYS">Add new error patterns to ERROR_PATTERNS.md if encountered.</rule>
<rule id="4" level="NEVER">Fix multiple tests simultaneously.</rule>
</critical_rules>

<sections>

<section name="how-to-run">
<content>
```bash
cd training/kata_suite
cargo test
```
</content>
</section>

<section name="coaching-loop">
<content>
1. Pick ONE failing test (the first one in output).
2. Implement the minimal fix.
3. Re-run only that test: `cargo test <name>`.
4. When the kata passes:
   - Write a 1-line learning note in the kata file.
   - If a new error pattern was encountered, add it to `ERROR_PATTERNS.md`.
5. If ALL tests fail, start with the simplest test (shortest name or first alphabetically).
</content>
</section>

<section name="example">
<code_example language="rust">
// WRONG — fighting the borrow checker
fn longest(words: &amp;Vec&lt;String&gt;) -> String {
    let mut best = words[0].clone();
    for w in words {
        if w.len() > best.len() { best = w.clone(); }
    }
    best
}

// RIGHT — idiomatic Rust
fn longest(words: &amp;[String]) -> &amp;str {
    words.iter().max_by_key(|w| w.len()).map(|w| w.as_str()).unwrap_or("")
}
</code_example>
</section>

<section name="test-structure">
<content>
Follow Arrange/Act/Assert:
1. **Arrange**: set up input data.
2. **Act**: call the function under test.
3. **Assert**: check the result.

Naming pattern: `test_<function>_<scenario>_<expected_result>`
</content>
</section>

<section name="scoring">
<content>
| Result | Criteria |
|--------|----------|
| PASS | All tests green + `cargo fmt --check` green + `cargo clippy` green |
| FAIL | Any compile error, failing test, clippy warning, or fmt drift |
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Fixing multiple tests at once</wrong>
<right>Fix ONE test, verify, then move to the next</right>
</mistake>
<mistake id="2">
<wrong>Using `.clone()` to pass every kata</wrong>
<right>First solve idiomatically; clone only if borrowing fails</right>
</mistake>
<mistake id="3">
<wrong>Not recording the learning</wrong>
<right>ALWAYS write a 1-line learning note — it builds ERROR_PATTERNS.md</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="cargo test">Target kata passes in `training/kata_suite`</checkpoint>
<checkpoint id="2" command="cargo fmt --check && cargo clippy">Shows 0 issues</checkpoint>
<checkpoint id="3" command="grep 'learning' kata_file.rs">Kata file contains a learning note comment</checkpoint>
<checkpoint id="4" command="cat ERROR_PATTERNS.md">New entry exists if a new error pattern was encountered</checkpoint>
</verification_checkpoints>

<scalability>
<level size="single-kata">Follow the coaching loop as written</level>
<level size="kata-suite" count="10+">Pick katas by difficulty order — simplest first</level>
<level size="team-training">Each member works on a different kata; merge ERROR_PATTERNS.md after</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-error-triage" relationship="error-diagnosis" />
</integration>

</skill>
