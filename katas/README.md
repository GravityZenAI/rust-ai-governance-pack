# 🥋 Rust Training Katas

Practical exercises to train AI coding agents (and humans) on real Rust patterns.
Each kata includes tests that must pass and common errors that must be avoided.

## How Katas Work

1. The agent receives a kata as a task prompt.
2. The agent writes the implementation.
3. The implementation must pass all provided tests.
4. Additionally: `cargo fmt --check` and `cargo clippy -- -D warnings` must pass.
5. A kata is **PASSED** only when all tests + linters + formatting are green.

## Success Metric

**Kata Pass Rate** = (number of katas passed / total katas attempted) × 100%

This measures how well the agent produces correct, idiomatic Rust on the first try.

---

## Kata 01: Ownership Basics
**Goal:** Write a function that takes ownership correctly.
**Error to avoid:** E0382 (use after move).

```rust
/// Returns the length of a string WITHOUT consuming it.
/// The caller must be able to use the string after calling this.
fn string_length(s: &str) -> usize {
    // implement
    todo!()
}

#[test]
fn test_ownership() {
    let s = String::from("hello");
    assert_eq!(string_length(&s), 5);
    // s must still be usable here:
    assert_eq!(s, "hello");
}
```

---

## Kata 02: Error Handling (no unwrap)
**Goal:** Parse a port number safely.
**Error to avoid:** `unwrap_used` lint.

```rust
#[derive(Debug, PartialEq)]
enum PortError {
    NotANumber,
    OutOfRange,
}

/// Parse a string into a valid port number (1-65535).
/// No .unwrap() or .expect() allowed.
fn parse_port(s: &str) -> Result<u16, PortError> {
    // implement
    todo!()
}

#[test]
fn test_valid_port() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("443"), Ok(443));
}

#[test]
fn test_invalid_port() {
    assert_eq!(parse_port("abc"), Err(PortError::NotANumber));
    assert_eq!(parse_port("0"), Err(PortError::OutOfRange));
    assert_eq!(parse_port("99999"), Err(PortError::OutOfRange));
    assert_eq!(parse_port("-1"), Err(PortError::NotANumber));
}
```

---

## Kata 03: Structs and Traits
**Goal:** Implement `Display` and `Default` for a struct.
**Error to avoid:** E0277 (trait not implemented).

```rust
use std::fmt;

struct Config {
    host: String,
    port: u16,
    debug: bool,
}

// Implement Default: host="localhost", port=8080, debug=false
// Implement Display: "localhost:8080 (debug=false)"

#[test]
fn test_default() {
    let c = Config::default();
    assert_eq!(c.host, "localhost");
    assert_eq!(c.port, 8080);
    assert!(!c.debug);
}

#[test]
fn test_display() {
    let c = Config::default();
    assert_eq!(format!("{c}"), "localhost:8080 (debug=false)");
}
```

---

## Kata 04: Iterators (no manual loops)
**Goal:** Transform data using iterators, not index loops.
**Error to avoid:** E0599 (method not found — forgot `.iter()`).

```rust
/// Given a list of temperatures in Celsius, return only those above freezing
/// converted to Fahrenheit, sorted ascending.
fn warm_fahrenheit(temps: &[f64]) -> Vec<f64> {
    // Use iterators: .iter(), .filter(), .map(), .collect()
    // Formula: F = C * 9/5 + 32
    todo!()
}

#[test]
fn test_conversion() {
    let temps = vec![-5.0, 0.0, 10.0, 25.0, 100.0];
    let result = warm_fahrenheit(&temps);
    assert_eq!(result, vec![50.0, 77.0, 212.0]);
}
```

---

## Kata 05: Lifetimes
**Goal:** Return a reference with correct lifetime annotation.
**Error to avoid:** E0106 (missing lifetime), E0515 (returning local reference).

```rust
/// Return the longer of two string slices.
/// Must use lifetime annotations.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // implement
    todo!()
}

#[test]
fn test_longest() {
    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("world!");
        result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, "world!");
    }
}
```

---

## Kata 06: Generics with Bounds
**Goal:** Write a generic function with trait bounds.
**Error to avoid:** E0277 (missing trait bound).

```rust
/// Return the larger of two values.
/// Works with any type that implements PartialOrd.
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    // implement
    todo!()
}

#[test]
fn test_max_integers() {
    assert_eq!(max_of(3, 7), 7);
    assert_eq!(max_of(10, 2), 10);
}

#[test]
fn test_max_strings() {
    assert_eq!(max_of("apple", "banana"), "banana");
}
```

---

## Kata 07: Clippy Compliance
**Goal:** Fix all clippy warnings in this code.

```rust
// This code compiles and tests pass, but has clippy warnings.
// Fix ALL warnings without changing behavior.

fn process(data: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..data.len() {
        if data[i] > 0 {
            result.push(data[i].clone() * 2);
        }
    }
    return result;
}

#[test]
fn test_process() {
    assert_eq!(process(vec![-1, 2, -3, 4]), vec![4, 8]);
}
```

**Expected fix:** Use iterators, remove unnecessary clone, remove explicit `return`.

---

## Kata 08: Newtype Pattern
**Goal:** Use newtypes to prevent parameter confusion.

```rust
// Make it impossible to accidentally swap from/to or mix amounts with IDs.

struct AccountId(u64);
struct Amount(f64);

fn transfer(from: AccountId, to: AccountId, amount: Amount) -> Result<(), String> {
    // implement: just validate from != to and amount > 0
    todo!()
}

#[test]
fn test_transfer() {
    let from = AccountId(1);
    let to = AccountId(2);
    let amount = Amount(100.0);
    assert!(transfer(from, to, amount).is_ok());
}

#[test]
fn test_same_account() {
    let from = AccountId(1);
    let to = AccountId(1);
    assert!(transfer(from, to, Amount(50.0)).is_err());
}
```

---

## Adding More Katas

When creating new katas:
1. Focus on ONE concept per kata.
2. Include the specific error code the kata trains against.
3. Always provide failing and passing tests.
4. Keep implementations under 20 lines.
5. Test edge cases (empty input, zero, negative, overflow).
