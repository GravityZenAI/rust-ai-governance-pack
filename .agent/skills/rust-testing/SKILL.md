---
name: rust-testing
description: Unit, property, and fuzz testing strategies for Rust code.
---

# Skill: Rust Testing (unit, property, fuzz)

> Inherits all rules from `rust-core`. This skill adds testing strategies.

## When to use
- Parsing/serialization, security boundaries, complex business logic, concurrency/async.
- ALWAYS add a regression test with every bug fix.
- Referenced by `rust-refactor-safely` — ALWAYS add tests before refactoring untested code.

## Critical rules
1. Every bug fix MUST add a regression test — no exceptions.
2. Tests MUST be deterministic — NEVER use sleep, system time, or non-seeded randomness.
3. `unwrap()` is allowed on happy-path assertions in tests. For error-path checks, use `assert!(result.is_err())` or pattern matching.
4. ALWAYS use `#[cfg(test)] mod tests { }` with `use super::*;`.

## Test structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input_returns_value() {
        // Arrange
        let input = "42";
        // Act
        let result = parse(input).unwrap(); // unwrap OK — happy path
        // Assert
        assert_eq!(result, 42);
    }

    #[test]
    fn parse_empty_input_returns_error() {
        // Arrange
        let input = "";
        // Act
        let result = parse(input);
        // Assert — NEVER unwrap here
        assert!(result.is_err());
    }
}
```

## Strategy (choose by risk level)

| Risk level | Strategy | Tool |
|-----------|----------|------|
| Standard | Unit tests for known inputs and edge cases | `#[test]` |
| High (invariants) | Property-based testing for mathematical properties | `proptest` |
| Critical (parsers) | Fuzzing for untrusted input | `cargo-fuzz` |

## Test naming
- Pattern: `test_<function>_<scenario>_<expected_result>`
- Example: `test_parse_empty_input_returns_error`

## Integration tests
- Place in `tests/` directory, one file per logical area.
- Integration tests test the public API as separate binaries.

## Mocking
- Design dependencies behind traits to enable mocking.
- Use `mockall` for trait mocking when dependencies need isolation.

## Async testing
- Use `#[tokio::test]` for async functions — NEVER manually build a runtime.

## Benchmarking
- Use `criterion` for micro-benchmarks with statistical analysis.
- Keep `/// # Examples` doc comments as executable tests.

## Cleanup
- Use RAII pattern (`Drop`) for test cleanup.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Using `unwrap()` to assert an error path | Use `assert!(result.is_err())` or `assert_matches!` |
| Testing implementation details instead of behavior | Test PUBLIC API behavior, not internal methods |
| Non-deterministic tests (system time, random) | Seed randomness; inject time as a parameter |

