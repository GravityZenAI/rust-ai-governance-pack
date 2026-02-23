---
name: rust-testing
description: Unit, property, and fuzz testing strategies for Rust code.
---

<skill name="rust-testing">

<description>Unit, property, and fuzz testing strategies for Rust code.</description>

<when_to_use>
- Parsing/serialization, security boundaries, complex business logic, concurrency/async.
- ALWAYS add a regression test with every bug fix.
- Referenced by `rust-refactor-safely` — ALWAYS add tests before refactoring untested code.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="ALWAYS">Every bug fix MUST add a regression test — no exceptions.</rule>
<rule id="2" level="ALWAYS">Tests MUST be deterministic — NEVER use sleep, system time, or non-seeded randomness.</rule>
<rule id="3" level="ALWAYS">`unwrap()` is allowed on happy-path assertions in tests. For error-path checks, use `assert!(result.is_err())` or pattern matching.</rule>
<rule id="4" level="ALWAYS">Use `#[cfg(test)] mod tests { }` with `use super::*;`.</rule>
</critical_rules>

<sections>

<section name="test-structure">
<code_example language="rust">
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
</code_example>
</section>

<section name="strategy-by-risk">
<content>
| Risk level | Strategy | Tool |
|-----------|----------|------|
| Standard | Unit tests for known inputs and edge cases | `#[test]` |
| High (invariants) | Property-based testing for mathematical properties | `proptest` |
| Critical (parsers) | Fuzzing for untrusted input | `cargo-fuzz` |
</content>
</section>

<section name="naming">
<content>
Pattern: `test_<function>_<scenario>_<expected_result>`
Example: `test_parse_empty_input_returns_error`
</content>
</section>

<section name="integration-tests">
<content>
- Place in `tests/` directory, one file per logical area.
- Integration tests test the public API as separate binaries.
</content>
</section>

<section name="advanced">
<content>
- **Mocking**: Design dependencies behind traits; use `mockall` for trait mocking.
- **Async**: Use `#[tokio::test]` — NEVER manually build a runtime.
- **Benchmarking**: Use `criterion` for micro-benchmarks with statistical analysis.
- **Doc tests**: Keep `/// # Examples` doc comments as executable tests.
- **Cleanup**: Use RAII pattern (`Drop`) for test cleanup.
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Using `unwrap()` to assert an error path</wrong>
<right>Use `assert!(result.is_err())` or `assert_matches!`</right>
</mistake>
<mistake id="2">
<wrong>Testing implementation details instead of behavior</wrong>
<right>Test PUBLIC API behavior, not internal methods</right>
</mistake>
<mistake id="3">
<wrong>Non-deterministic tests (system time, random)</wrong>
<right>Seed randomness; inject time as a parameter</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="cargo test">Exits with 0 failures</checkpoint>
<checkpoint id="2" command="cargo test && cargo test">Running twice gives same result (determinism check)</checkpoint>
<checkpoint id="3" command="git log --oneline -1">Bug fix commits include at least one new `#[test]` function</checkpoint>
<checkpoint id="4" command="grep -rn 'fn test_' src/">Test names follow `test_<function>_<scenario>_<expected_result>`</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" crates="1">All tests in `#[cfg(test)] mod tests` within each file</level>
<level size="medium" crates="workspace">Unit tests per crate; integration tests in `tests/` per crate</level>
<level size="large" crates="50+">Shared test utilities crate; run `cargo test -p <crate>` for focused testing</level>
<level size="ci-pipeline">Run `cargo test --workspace` with `--release` for full coverage; cache target dir</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-refactor-safely" relationship="test-before-refactor" />
</integration>

</skill>
