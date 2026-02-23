---
name: rust-core
description: Foundation skill for safe, idiomatic Rust — ownership, error handling, API design.
---

<skill name="rust-core">

<description>Foundation skill for safe, idiomatic Rust. All other Rust skills inherit these rules.</description>

<when_to_use>
- API design, ownership/borrowing, error handling, iterators, slices, strings, modules/crates layout.
</when_to_use>

<critical_rules>
<rule id="1" level="ALWAYS">Prefer `&amp;T` / `&amp;mut T` over `.clone()`.</rule>
<rule id="2" level="ALWAYS">Accept `&amp;[T]` instead of `&amp;Vec&lt;T&gt;`, and `&amp;str` instead of `&amp;String` in parameters.</rule>
<rule id="3" level="ALWAYS">Use iterators over index loops — they eliminate bounds checks and are clearer.</rule>
<rule id="4" level="NEVER">Use `unwrap()` or `expect()` in library code. In `#[cfg(test)]`: allowed on happy-path assertions.</rule>
<rule id="5" level="ALWAYS">Return `Result&lt;T, E&gt;` for operations that can fail — NEVER panic on expected errors.</rule>
</critical_rules>

<sections>

<section name="ownership-patterns">
<content>
- Use `Cow&lt;'a, T&gt;` when a function must sometimes own and sometimes borrow.
- Derive `Copy` for small, trivial types (≤ 16 bytes, no heap allocation).
- Move large data instead of cloning — ownership transfer is zero-cost.
- Rely on lifetime elision; add explicit lifetimes only when the compiler requires them.
- Make every `.clone()` call intentional — each one is a potential heap allocation.
</content>
</section>

<section name="error-handling">
<content>
### Libraries
Use `thiserror` with `#[from]` for automatic conversion. ALWAYS create custom error types — NEVER use `Box&lt;dyn Error&gt;` in public APIs.

### Applications
Use `anyhow` with `.context("what failed")`. Use `?` for propagation — NEVER write manual `match` on `Result` unless adding context.
</content>
<code_example language="rust">
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("config parse failed")]
    Config(#[from] serde_json::Error),
    #[error("network timeout after {0}s")]
    Timeout(u64),
}
</code_example>
</section>

<section name="api-design">
<content>
### Type safety (highest priority)
- Use newtypes to encode invariants: `struct Port(u16)` with validating constructor.
- Add `#[must_use]` to functions returning `Result` or types that must not be silently discarded.
- Implement `From&lt;T&gt;`, not `Into&lt;T&gt;` — `Into` is auto-derived.
- Derive `Debug`, `Clone`, `PartialEq` on all public types.
- Implement `Default` when there is a sensible default value.

### Patterns for complex APIs
- Builder pattern for types with 3+ optional parameters.
- Typestate for compile-time state machines.
- Parse, don't validate: convert raw data into validated types at boundaries.

### Future-proofing
- `#[non_exhaustive]` on public enums and structs.
- Accept `impl AsRef&lt;Path&gt;` / `impl Into&lt;String&gt;` for flexible input types.
- Return owned data when the return value must outlive the input borrows.
</content>
</section>

<section name="memory-awareness">
<content>
- `Vec::with_capacity(n)` when the size is known or estimable.
- `clone_from()` instead of `x = y.clone()` to reuse allocations.
- Box large enum variants to keep the enum's stack size small.
</content>
</section>

<section name="performance">
<content>
- NEVER micro-optimize until tests exist and a profiler confirms the bottleneck.
- NEVER use `unsafe` for performance without a benchmark proving the gain.
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Using `Box&lt;dyn Error&gt;` in a library's public API</wrong>
<right>Define a custom error enum with `thiserror`</right>
</mistake>
<mistake id="2">
<wrong>Adding `.clone()` without checking if &amp; works</wrong>
<right>Try `&amp;T` first; clone only when ownership is truly needed</right>
</mistake>
<mistake id="3">
<wrong>Using `Vec&lt;T&gt;` parameter when `&amp;[T]` suffices</wrong>
<right>Change signature to `&amp;[T]` — avoids forcing callers to allocate</right>
</mistake>
<mistake id="4">
<wrong>Forgetting `#[must_use]` on Result-returning fns</wrong>
<right>Add `#[must_use]` — prevents silent error dropping</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="cargo clippy -- -W clippy::all">Shows 0 warnings about clone, unwrap, or Box&lt;dyn Error&gt;</checkpoint>
<checkpoint id="2" command="grep -rn '&amp;Vec&lt;' src/">Returns 0 matches — no `&amp;Vec&lt;T&gt;` in function signatures</checkpoint>
<checkpoint id="3" command="grep -rn 'pub struct' src/">All public types have `#[derive(Debug)]`</checkpoint>
<checkpoint id="4" command="grep -rn '#[must_use]' src/">All Result-returning public functions tagged</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" crates="1">Apply all rules directly — low overhead</level>
<level size="medium" crates="workspace">Define error types per crate; use `thiserror` in each lib, `anyhow` in bins</level>
<level size="large" crates="50+">Create a shared `error` crate; enforce via `[workspace.lints]`</level>
<level size="open-source">Every rule applies; additionally enforce `#[non_exhaustive]` and SemVer compliance</level>
</scalability>

<integration>
<related_skill name="rust-compile-loop" relationship="inherits-from-this" />
<related_skill name="rust-verifier" relationship="inherits-from-this" />
<related_skill name="rust-testing" relationship="inherits-from-this" />
<related_skill name="rust-error-triage" relationship="inherits-from-this" />
</integration>

</skill>
