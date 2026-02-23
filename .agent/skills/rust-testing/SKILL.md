# Skill: Rust Testing (unit, property, fuzz)

## When to use
- parsing/serialization
- security boundaries
- complex business logic
- concurrency/async
- any bug fix (must add regression test)

## Strategy
1) Unit tests for known cases.
2) Property tests for invariants (optional, strong).
3) Fuzzing for parsers/decoders (optional, strongest for robustness).

## Minimum expectations
- Every bug fix adds a regression test.
- Tests are deterministic (no sleeping/flaky randomness).
- Avoid panics; assert error returns.

## Tools (optional)
- `proptest` for property tests.
- `cargo-fuzz` for fuzz targets.
