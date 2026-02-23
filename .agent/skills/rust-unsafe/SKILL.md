---
name: rust-unsafe
description: Rules and patterns for unsafe Rust code and FFI boundaries.
---

<skill name="rust-unsafe">

<description>Rules and patterns for unsafe Rust code and FFI boundaries.</description>

<when_to_use>
ONLY activate when the task explicitly requires:
- `unsafe` blocks, raw pointers, FFI, SIMD, custom allocators, or low-level atomics.
- For all other tasks, use `rust-core` instead.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="NEVER">Use `unsafe` as a "quick fix" for borrow checker errors — find the safe solution.</rule>
<rule id="2" level="ALWAYS">Every `unsafe` block MUST have a `/// # Safety` doc-comment listing exact invariants.</rule>
<rule id="3" level="ALWAYS">Put unsafe code behind a safe API boundary (module-level encapsulation).</rule>
<rule id="4" level="ALWAYS">Enable `#![warn(clippy::undocumented_unsafe_blocks)]`.</rule>
</critical_rules>

<sections>

<section name="ub-checklist">
<content>
| Invariant | Check |
|-----------|-------|
| No dangling pointers | All pointers valid for their entire lifetime |
| No out-of-bounds access | All indices/offsets verified at boundary |
| No invalid aliasing | No `&amp;mut` aliasing with any other reference |
| Correct alignment | All pointer casts respect alignment requirements |
| Valid initialization | No reading of uninitialized memory |
| Thread safety | `Send`/`Sync` bounds correctly implemented |
</content>
</section>

<section name="safety-documentation">
<code_example language="rust">
/// # Safety
///
/// - `ptr` must be non-null and properly aligned for `T`
/// - `ptr` must point to a valid, initialized `T` for the lifetime `'a`
/// - The caller must ensure no mutable aliases to `*ptr` exist
unsafe fn deref_ptr&lt;'a, T&gt;(ptr: *const T) -> &amp;'a T {
    &amp;*ptr
}
</code_example>
</section>

<section name="ffi-patterns">
<content>
- `#[repr(transparent)]` on newtypes wrapping a single field for FFI compatibility.
- `PhantomData&lt;T&gt;` for type-level markers (ownership, lifetime binding) at zero runtime cost.
- Zero-copy with slices and `Bytes` for external data interfaces.
</content>
</section>

<section name="verification">
<content>
- Run Miri: `cargo +nightly miri test`. If unavailable, document as a risk.
- ALWAYS test boundary conditions: null, zero-length, max-length, unaligned.
- Use `rust-verifier` for final verification.
</content>
</section>

<section name="required-output">
<content>
- Exact unsafe surface area: files + line ranges.
- All invariants listed and documented.
- Justification: why this CANNOT be done in safe Rust.
- Evidence: verifier output + Miri output (or waiver with risk note).
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Using `unsafe` to bypass the borrow checker</wrong>
<right>NEVER — find the safe pattern; the borrow checker is protecting you</right>
</mistake>
<mistake id="2">
<wrong>Writing `unsafe fn` without `/// # Safety` docs</wrong>
<right>ALWAYS document — it's the contract for callers</right>
</mistake>
<mistake id="3">
<wrong>Not testing with Miri</wrong>
<right>Run `cargo +nightly miri test` — it catches UB that tests won't</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="grep -B1 'unsafe' src/">Every `unsafe` block preceded by `/// # Safety` comment</checkpoint>
<checkpoint id="2" command="cargo +nightly miri test">Passes (or waiver documented with risk note)</checkpoint>
<checkpoint id="3" command="grep -rn 'unsafe' src/">Output lists exact files + line ranges containing `unsafe`</checkpoint>
<checkpoint id="4" command="cat DECISIONS.md">Justification exists for why safe Rust cannot achieve the same result</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" unsafe="minimal">Full Miri testing feasible for every unsafe block</level>
<level size="medium" unsafe="ffi-bindings">Isolate unsafe in dedicated `ffi` module; safe wrapper API for all consumers</level>
<level size="large" unsafe="systems-crate">Track unsafe surface area with `cargo geiger`; review delta per PR</level>
<level size="critical" unsafe="crypto-security">Require 2-person review for any unsafe change; formal verification when possible</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-verifier" relationship="final-verification" />
</integration>

</skill>
