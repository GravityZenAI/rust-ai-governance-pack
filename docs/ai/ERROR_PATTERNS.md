# ERROR_PATTERNS (persistent)

Known error patterns with root cause, canonical fix, and test guidance.
Based on JetBrains RustRover empirical data (2024-2025) and Rust official docs.

## How to Use This File
When the agent encounters an error code, look it up here for the canonical fix.
Each entry includes: error code, frequency, root cause, fix, and example.

---

## Types & Traits (~60% of all errors)

### E0277 — Trait not implemented (~32%)
**Root cause:** Calling a function or using a type that requires a trait the type doesn't implement.
**Fix:** Derive or manually implement the required trait.
```rust
// ❌ Triggers E0277
fn print_it<T: Display>(val: T) { println!("{val}"); }
print_it(vec![1, 2, 3]);  // Vec doesn't implement Display

// ✅ Fix: use Debug instead, or implement Display
fn print_it<T: Debug>(val: T) { println!("{val:?}"); }
```

### E0308 — Mismatched types (~30%)
**Root cause:** Expression type doesn't match expected type.
**Fix:** Convert, cast, or adjust function signature.
```rust
// ❌ Triggers E0308
fn get_count() -> u32 { "five" }

// ✅ Fix: return correct type
fn get_count() -> u32 { 5 }
```

### E0599 — Method not found (~27.5%)
**Root cause:** Calling a method that doesn't exist on the type.
**Fix:** Add `.iter()`, import the right trait, or use the correct method.
```rust
// ❌ Triggers E0599
let total: i32 = vec![1, 2, 3].sum();

// ✅ Fix: use iterator
let total: i32 = vec![1, 2, 3].iter().sum();
```

---

## Ownership & Borrowing (~25% of errors)

### E0382 — Use after move
**Root cause:** Using a variable after its value was moved.
**Fix:** Clone, borrow (`&`), or restructure to use references.
```rust
// ❌ Triggers E0382
let s = String::from("hello");
let s2 = s;
println!("{s}");  // moved!

// ✅ Fix: clone or borrow
let s = String::from("hello");
let s2 = s.clone();
println!("{s}");
```

### E0502 — Borrow conflict
**Root cause:** Mutable borrow while immutable borrow is active.
**Fix:** Reorder operations or scope the borrow.
```rust
// ❌ Triggers E0502
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);        // mutable borrow while first is alive
println!("{first}");

// ✅ Fix: use first before mutating
let mut v = vec![1, 2, 3];
let first = v[0]; // copy the value, no borrow
v.push(4);
println!("{first}");
```

### E0507 — Cannot move out of borrowed content
**Fix:** Clone, use `std::mem::take`, or restructure.

### E0597 — Value does not live long enough
**Fix:** Extend scope or return owned data instead of reference.

---

## Name Resolution (~15% of errors)

### E0425 — Unresolved name
**Root cause:** Variable or function not in scope.
**Fix:** Check spelling, add `use`, or declare the variable.

### E0432 — Unresolved import
**Root cause:** Wrong `use` path.
**Fix:** Verify module path and `Cargo.toml` dependencies.

### E0433 — Unresolved path
**Root cause:** Module or type not found.
**Fix:** Add dependency to `Cargo.toml` or fix the `use` path.

---

## Lifetimes

### E0106 — Missing lifetime specifier
**Fix:** Add lifetime annotation `'a` to function signature.

### E0515 — Returning reference to local
**Fix:** Return owned data or restructure to take reference as input.

---

## Clippy Lints (not E-codes, but frequent)

| Lint | Severity | Fix |
|------|:--------:|-----|
| `unwrap_used` | deny | Replace with `?` or `match` |
| `expect_used` | warn | Use `?` with context or `unwrap_or_else` |
| `panic` | deny | Return `Result` instead |
| `indexing_slicing` | warn | Use `.get()` with bounds check |
| `redundant_clone` | warn | Remove unnecessary `.clone()` |
| `todo` | deny | Implement before merging |
| `unimplemented` | deny | Implement or return error |

---

## Adding New Patterns
When you encounter a new error pattern:
1. Document the error code and frequency (if known).
2. Identify the root cause.
3. Write a minimal reproduction.
4. Document the canonical fix.
5. Add a test that validates the fix.
6. Add it to this file.

## Sources
- JetBrains RustRover error frequency data (2024-2025)
- Rust compiler error index: https://doc.rust-lang.org/error_codes/
- Clippy lint list: https://rust-lang.github.io/rust-clippy/master/
