---
name: rust-unsafe
description: Rules and patterns for unsafe Rust code and FFI boundaries.
---

# Skill: Rust Unsafe / FFI

> Inherits all rules from `rust-core`. This skill adds unsafe-specific controls.

## When to activate
ONLY activate when the task explicitly requires:
- `unsafe` blocks, raw pointers, FFI, SIMD, custom allocators, or low-level atomics.
- For all other tasks, use `rust-core` instead.

## Critical rules
1. NEVER use `unsafe` as a "quick fix" for borrow checker errors — find the safe solution.
2. EVERY `unsafe` block MUST have a `/// # Safety` doc-comment listing exact invariants.
3. ALWAYS put unsafe code behind a safe API boundary (module-level encapsulation).
4. ALWAYS enable `#![warn(clippy::undocumented_unsafe_blocks)]`.

## UB checklist (verify ALL before declaring done)

| Invariant | Check |
|-----------|-------|
| No dangling pointers | All pointers valid for their entire lifetime |
| No out-of-bounds access | All indices/offsets verified at boundary |
| No invalid aliasing | No `&mut` aliasing with any other reference |
| Correct alignment | All pointer casts respect alignment requirements |
| Valid initialization | No reading of uninitialized memory |
| Thread safety | `Send`/`Sync` bounds correctly implemented |

## Required Safety documentation

```rust
/// # Safety
///
/// - `ptr` must be non-null and properly aligned for `T`
/// - `ptr` must point to a valid, initialized `T` for the lifetime `'a`
/// - The caller must ensure no mutable aliases to `*ptr` exist
unsafe fn deref_ptr<'a, T>(ptr: *const T) -> &'a T {
    &*ptr
}
```

## FFI patterns
- `#[repr(transparent)]` on newtypes wrapping a single field for FFI compatibility.
- `PhantomData<T>` for type-level markers (ownership, lifetime binding) at zero runtime cost.
- Zero-copy with slices and `Bytes` for external data interfaces.

## Verification
- Run Miri: `cargo +nightly miri test`. If unavailable, document this as a risk.
- ALWAYS test boundary conditions: null, zero-length, max-length, unaligned.
- Use `rust-verifier` for final verification.

## Required output
- Exact unsafe surface area: files + line ranges.
- All invariants listed and documented.
- Justification: why this CANNOT be done in safe Rust.
- Evidence: verifier output + Miri output (or waiver with risk note).

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Using `unsafe` to bypass the borrow checker | NEVER — the borrow checker is protecting you; find the safe pattern |
| Writing `unsafe fn` without `/// # Safety` docs | ALWAYS document — it's the contract for callers |
| Not testing with Miri | Run `cargo +nightly miri test` — it catches UB that tests won't |

