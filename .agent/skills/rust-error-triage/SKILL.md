---
name: rust-error-triage
description: Systematic diagnosis of rustc error codes and Clippy lints; updates ERROR_PATTERNS.md.
---

<skill name="rust-error-triage">

<description>Systematic diagnosis of rustc error codes and Clippy lints; updates ERROR_PATTERNS.md.</description>

<when_to_use>
- Any time `cargo check`, `cargo test`, or `cargo clippy` produces errors.
- Complements `rust-compile-loop` — use this to diagnose, compile-loop to iterate fixes.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="ALWAYS">Read the FULL error message — the compiler suggests the fix.</rule>
<rule id="2" level="ALWAYS">Classify each error into exactly ONE category before fixing.</rule>
<rule id="3" level="ALWAYS">Add new patterns to ERROR_PATTERNS.md with root cause, fix, snippet, and test.</rule>
<rule id="4" level="NEVER">Suppress clippy with `#[allow]` without a documented justification comment.</rule>
</critical_rules>

<sections>

<section name="procedure">
<content>
1. **Extract** from the error output: error code (E0xxx), file:line, highlighted snippet.
2. **Classify** into exactly ONE category (see table below).
3. **Apply** the canonical fix — consult ERROR_PATTERNS.md first.
4. **Validate**: `./scripts/verify.sh --fast`.
5. If still failing, repeat from step 1 on the **new first error**.
</content>
</section>

<section name="classification-table">
<content>
| Category | Common codes | Typical fix |
|----------|-------------|-------------|
| Ownership/move | E0382, E0507 | Clone, borrow, restructure scope |
| Borrowing/lifetime | E0502, E0505, E0597 | Reduce borrow scope, add lifetime |
| Types/traits | E0277, E0308 | Add bound, implement trait, cast |
| Modules/imports | E0432, E0433 | Fix path, add `use`, check visibility |
| Mutability | E0596, E0594 | Add `mut`, use `RefCell`/`Cell` |
| Temporaries | E0716 | Bind to variable before borrowing |
</content>
</section>

<section name="example">
<code_example language="rust">
// Error E0502: cannot borrow `data` as mutable because also borrowed as immutable
let first = &amp;data[0]; // immutable borrow
data.push(42);        // mutable borrow — CONFLICT
println!("{}", first);

// Fix: finish using the immutable borrow before mutating
let first = data[0];  // copy the value (no borrow kept)
data.push(42);        // now safe
</code_example>
</section>

<section name="fix-patterns">
<content>
### Library errors
- `thiserror` with `#[error("...")]` + `#[from]` for auto-conversion.
- `#[source]` to preserve error chains.
- `Box` large payloads: `ParseError(Box&lt;serde_json::Error&gt;)`.

### Application errors
- `anyhow` with `.context("why it failed")`.
- `.context()` at module boundaries, NOT on every call.

### Warnings vs errors
- Clippy warnings: fix immediately or `#[allow(clippy::...)]` with justification.
- NEVER suppress warnings without documented justification.
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Trying to fix error by type before reading the message</wrong>
<right>Read the FULL error message — the compiler suggests the fix</right>
</mistake>
<mistake id="2">
<wrong>Ignoring `rustc --explain Exxxx`</wrong>
<right>Run it — it gives examples and detailed explanations</right>
</mistake>
<mistake id="3">
<wrong>Suppressing clippy with `#[allow]` without a comment</wrong>
<right>ALWAYS add `// Reason: ...` next to `#[allow]`</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="cat ERROR_PATTERNS.md">New entry has: root cause, canonical fix, minimal snippet, minimal test</checkpoint>
<checkpoint id="2" command="./scripts/verify.sh --fast">Exits 0 after applying the fix</checkpoint>
<checkpoint id="3" command="cargo test">New minimal test in ERROR_PATTERNS.md compiles and passes</checkpoint>
<checkpoint id="4" command="grep -rn '#\[allow' src/">Every `#[allow]` has an inline comment explaining why</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" errors="few">Follow the full 5-step procedure for each error</level>
<level size="medium" errors="10+">Group errors by category first, then batch-fix same-category errors</level>
<level size="large" errors="100+">Fix by category priority: Ownership → Types → Modules → Mutability</level>
<level size="ci-pipeline">Integrate ERROR_PATTERNS.md as a lookup table for automated fix suggestions</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-compile-loop" relationship="iterates-fixes" />
<related_skill name="rust-verifier" relationship="final-check" />
</integration>

</skill>
