# Skill: Rust Unsafe / FFI

## When to load
Only load when:
- the task explicitly requires `unsafe`, raw pointers, FFI, SIMD, custom allocators, or low-level atomics.

## Core rule
Unsafe code is allowed only if you can state the invariants precisely and enforce them.

## Required structure
- Put unsafe behind a safe API boundary (module-level encapsulation).
- Every unsafe function/block must have a `/// # Safety` section that lists invariants.
- Add tests that exercise the boundary conditions.
- Run Miri when possible.

## UB checklist
- no dangling pointers
- no out-of-bounds
- no invalid aliasing (mutable aliasing)
- correct alignment
- valid initialization
- thread-safety invariants (Send/Sync)

## What to output
- exact unsafe surface area (files + line ranges)
- invariants
- why it cannot be done in safe Rust
- evidence: verifier output + Miri output (or explicit waiver)
