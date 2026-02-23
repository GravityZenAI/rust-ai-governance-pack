# Skill: Rust Testing (unit, property, fuzz)

## When to use
- Parsing/serialization, security boundaries, complex business logic, concurrency/async.
- ALWAYS add a regression test with every bug fix.
- Referenced by `rust-refactor-safely` — ALWAYS add tests before refactoring untested code.

## Critical rules
1. Every bug fix MUST add a regression test — no exceptions.
2. Tests MUST be deterministic — NEVER use sleep, system time, or non-seeded randomness.
3. NEVER use `unwrap()` in tests to check error cases — use `assert!(result.is_err())` or pattern matching.
4. ALWAYS use `#[cfg(test)] mod tests { }` with `use super::*;`.

## Test structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_input_returns_error() {
        // Arrange
        let input = "";

        // Act
        let result = parse(input);

        // Assert
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
- Use descriptive names: `test_parse_empty_input_returns_error`, not `test1`.
- Pattern: `test_<function>_<scenario>_<expected_result>`.

## Integration tests
- Place in `tests/` directory, one file per logical area.
- Integration tests run as separate binaries — they test the public API.

## Mocking
- Design dependencies behind traits to enable mocking.
- Use `mockall` for trait mocking when dependencies need isolation.

## Async testing
- Use `#[tokio::test]` for async functions — NEVER manually build a runtime.

## Benchmarking
- Use `criterion` for micro-benchmarks with statistical analysis.
- Keep `/// # Examples` doc comments as executable tests.

## Cleanup
- Use RAII pattern (`Drop`) for test cleanup — create a guard struct that cleans up on drop.

