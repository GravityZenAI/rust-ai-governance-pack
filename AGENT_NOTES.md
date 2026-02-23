# Omega Optimizer — Skills Audit Report
Date: 2026-02-23
Status: IN PROGRESS

## Skills a evaluar:
- [x] 1. rust-compile-loop
- [x] 2. rust-core
- [x] 3. rust-error-triage
- [x] 4. rust-kata-coach
- [x] 5. rust-refactor-safely
- [x] 6. rust-supply-chain
- [x] 7. rust-testing
- [x] 8. rust-unsafe
- [x] 9. rust-verifier

---

## Backup (first 10 lines of each skill before modification)

### rust-compile-loop (pre)
```
---
name: rust-compile-loop
description: Incremental Rust implementation using compiler/tests/clippy/fmt as the judge.
---
# Rust Compile Loop
## When to use
- Any task that modifies `.rs` files.
## Operating protocol
1) Restate the spec in 3–6 bullets.
2) Identify the smallest compilable step.
```

### rust-core (pre)
```
# Skill: Rust Core (Safe, idiomatic)
## Use for
- API design
- ownership/borrowing issues
- error handling
- iterators, slices, strings
- modules/crates layout
## Safe defaults
- Prefer `&T` / `&mut T` over cloning.
- Accept `&[T]` instead of `&Vec<T>`, and `&str` instead of `&String`.
```

### rust-error-triage (pre)
```
---
name: rust-error-triage
description: Systematic diagnosis of rustc error codes and Clippy lints.
---
# Rust Error Triage
## Inputs
- Compiler output (`cargo check` / `cargo test` / clippy output)
## Procedure
1) Extract: error code, file:line, snippet
2) Classify: Ownership/Borrowing/Types/Modules/Mutability/Temporaries
```

### rust-kata-coach (pre)
```
---
name: rust-kata-coach
description: Run and solve Rust katas with compiler-driven feedback and scoring.
---
# Rust Kata Coach
## How to run
cd training/kata_suite
cargo test
## Coaching loop
1) Pick ONE failing test.
```

### rust-refactor-safely (pre)
```
---
name: rust-refactor-safely
description: Refactor Rust code while keeping compile/tests/clippy/fmt green.
---
# Rust Refactor Safely
## Goals
- Preserve behavior.
- Keep the repo green after each small commit.
## Protocol
1) Identify the refactor goal.
```

### rust-supply-chain (pre)
```
# Skill: Rust Supply Chain Hardening
## When to use
Whenever Cargo.toml / dependency versions change.
## Objective
Prevent vulnerable or risky dependencies.
## Required actions
1) Minimize dependency surface
2) Run checks: cargo audit, cargo deny, cargo vet
3) Document decisions in DECISIONS.md
## Red flags
```

### rust-testing (pre)
```
# Skill: Rust Testing (unit, property, fuzz)
## When to use
- parsing/serialization
- security boundaries
- complex business logic
- concurrency/async
- any bug fix (must add regression test)
## Strategy
1) Unit tests for known cases.
2) Property tests for invariants.
```

### rust-unsafe (pre)
```
# Skill: Rust Unsafe / FFI
## When to load
Only load when: unsafe, raw pointers, FFI, SIMD, allocators, atomics.
## Core rule
Unsafe code is allowed only if invariants are precise.
## Required structure
- Put unsafe behind safe API boundary.
- Every unsafe block must have /// # Safety.
- Add tests for boundary conditions.
- Run Miri when possible.
```

### rust-verifier (pre)
```
# Skill: Rust Verifier Loop
## When to use
- edit .rs files, Cargo.toml, Cargo.lock
- FFI, concurrency, parsing, crypto, auth, I/O
## Objective
Force correctness through external judges.
## Procedure
1) Restate constraints and Definition of DONE.
2) Plan smallest safe diff.
3) Implement with tests first.
```

---

## Evaluations

### Skill 1: rust-compile-loop

#### Architect
Structure: Good linear flow (spec→implement→verify). Added: "Critical rules" section at top for immediate pickup. Differentiated from verifier with clear "when to use" guidance.

#### Linguist
Fixed: "consider" → removed in favor of direct instructions. Added ALWAYS/NEVER language. Replaced prose heuristics with actionable table.

#### Psychologist
Moved critical ownership/error rules to top so AI reads them first. Protocol steps remain clear and sequential with low cognitive load.

#### Tester
Added: coverage for clippy and fmt errors (previously missing). Added table entry for format/clippy fix strategy. Added Cow pattern guidance.

#### Integrator
Added: differentiation from rust-verifier ("incremental implementation" vs "final verification"). Cross-reference established.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | ALWAYS/NEVER language, table format |
| Completeness | 6 | 8 | Added fmt/clippy coverage |
| Structure | 7 | 9 | Critical rules at top |
| Actionability | 7 | 9 | Every instruction immediately actionable |
| Conciseness | 8 | 8 | Clean, no bloat |
| Integration | 5 | 8 | Differentiated from verifier |
| Error_Prevention | 6 | 8 | Table of fix strategies |
| Rust_Accuracy | 8 | 9 | Added From/.map_err() pattern |
| **TOTAL** | **54/80** | **68/80** | |

---

### Skill 2: rust-core

#### Architect
Restructured into clear hierarchy: Critical rules → Ownership → Error handling (lib vs app) → API design (3 sub-sections) → Memory → Performance. Each section has distinct focus.

#### Linguist
Replaced weak "Prefer" (x3) with ALWAYS. Added code example for thiserror. Strong imperative throughout.

#### Psychologist
11-item API patterns list split into 3 sub-sections (Type safety, Complex APIs, Future-proofing) to reduce cognitive overload. Critical rules moved to top.

#### Tester
Added code example for thiserror pattern. Explicit rule about Box<dyn Error> being forbidden in public APIs.

#### Integrator
Established as "foundation skill" that others reference. Cross-reference structure clear.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 6 | 9 | ALWAYS/NEVER, no ambiguity |
| Completeness | 7 | 9 | Comprehensive ownership/error/API |
| Structure | 5 | 9 | Sub-sections reduce overload |
| Actionability | 6 | 9 | Numbered critical rules |
| Conciseness | 7 | 8 | Added code example, still focused |
| Integration | 6 | 8 | Foundation skill declared |
| Error_Prevention | 7 | 9 | thiserror example prevents misuse |
| Rust_Accuracy | 8 | 9 | All patterns verified |
| **TOTAL** | **52/80** | **70/80** | |

---

### Skill 3: rust-error-triage

#### Architect
Added classification table with error codes and typical fixes. Clear lookup structure.

#### Linguist
Procedure steps now bolded. "Warnings vs errors" section added to prevent ambiguity.

#### Psychologist
Table format for classification reduces decision paralysis. Error codes give concrete anchors.

#### Tester
Added common error codes (E0382, E0502, E0505, etc.) for quick reference. Added warnings handling.

#### Integrator
Added relationship to rust-compile-loop: "use this to diagnose, compile-loop to iterate."

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | Table with codes + fixes |
| Completeness | 5 | 8 | Error codes, warnings coverage |
| Structure | 7 | 9 | Classification table |
| Actionability | 7 | 9 | Exact codes to look for |
| Conciseness | 8 | 8 | Tight |
| Integration | 5 | 8 | Connected to compile-loop |
| Error_Prevention | 6 | 8 | Warnings policy added |
| Rust_Accuracy | 7 | 9 | Real error codes mapped |
| **TOTAL** | **52/80** | **68/80** | |

---

### Skill 4: rust-kata-coach

#### Architect
Rules moved to top. Scoring converted to table.

#### Linguist
"never" → "NEVER". All rules numbered and imperative.

#### Psychologist
Rules first means AI adopts the mindset before starting katas. Low overload maintained.

#### Tester
Added: what to do when ALL tests fail. AAA pattern detailed with steps.

#### Integrator
Added reference to rust-error-triage for diagnosis.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | Numbered rules |
| Completeness | 6 | 8 | All-fail scenario covered |
| Structure | 6 | 9 | Rules first, scoring table |
| Actionability | 7 | 9 | Specific patterns to practice |
| Conciseness | 8 | 8 | Short and focused |
| Integration | 5 | 7 | Error-triage referenced |
| Error_Prevention | 6 | 8 | Naming pattern given |
| Rust_Accuracy | 8 | 8 | Correct patterns |
| **TOTAL** | **53/80** | **66/80** | |

---

### Skill 5: rust-refactor-safely

#### Architect
Anti-patterns moved before protocol (catch mistakes early). Protocol refined with revert step.

#### Linguist
All weak language replaced. "wherever possible" → absolute rules.

#### Psychologist
Anti-patterns table is scannable. Revert strategy prevents panic when refactor breaks.

#### Tester
Added: rollback step (REVERT if red). Differentiated from compile-loop.

#### Integrator
Added references to rust-testing and rust-compile-loop.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 6 | 9 | Table format, NEVER/ALWAYS |
| Completeness | 6 | 8 | Revert strategy added |
| Structure | 7 | 9 | Anti-patterns before protocol |
| Actionability | 6 | 9 | Concrete fix for each anti-pattern |
| Conciseness | 7 | 8 | Tight |
| Integration | 5 | 8 | Cross-refs to testing, compile-loop |
| Error_Prevention | 6 | 9 | Revert policy prevents damage |
| Rust_Accuracy | 8 | 8 | Correct patterns |
| **TOTAL** | **51/80** | **68/80** | |

---

### Skill 6: rust-supply-chain

#### Architect
Added decision documentation checklist. Checks in table with actions.

#### Linguist
"avoid" → NEVER. All rules imperative.

#### Psychologist
Table of checks reduces cognitive load. Documentation checklist ensures completeness.

#### Tester
Added: yanked versions, remediation steps, dry-run updates.

#### Integrator
Connected to rust-verifier for pipeline integration.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | NEVER/ALWAYS, table |
| Completeness | 6 | 9 | Documentation checklist, yanked |
| Structure | 7 | 9 | Table with failure actions |
| Actionability | 6 | 9 | Exact commands + actions |
| Conciseness | 7 | 8 | Focused |
| Integration | 5 | 8 | Verifier referenced |
| Error_Prevention | 7 | 9 | Remediation for each check |
| Rust_Accuracy | 8 | 8 | Correct tooling |
| **TOTAL** | **53/80** | **69/80** | |

---

### Skill 7: rust-testing

#### Architect
Added code example. Strategy table by risk level.

#### Linguist
"optional" removed. "Avoid" → NEVER. All rules imperative.

#### Psychologist
Code example immediately shows expected pattern. Table reduces decision fatigue.

#### Tester
Added: test naming pattern, explicit example, risk-based strategy selection.

#### Integrator
Bidirectional reference with rust-refactor-safely established.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | Code example, table |
| Completeness | 7 | 9 | Risk levels, naming pattern |
| Structure | 7 | 9 | Logical flow: rules→example→strategy |
| Actionability | 6 | 9 | Copy-paste code example |
| Conciseness | 6 | 8 | Removed "optional" padding |
| Integration | 5 | 8 | Refactor-safely reference |
| Error_Prevention | 7 | 9 | Naming pattern prevents test1 |
| Rust_Accuracy | 8 | 9 | Correct test structure |
| **TOTAL** | **53/80** | **70/80** | |

---

### Skill 8: rust-unsafe

#### Architect
UB checklist converted to table. Added Safety documentation example.

#### Linguist
"Only load when" → "ONLY activate when". "when possible" eliminated.

#### Psychologist
Example shows exact expected Safety doc format — AI can replicate immediately.

#### Tester
Added: Miri unavailability fallback. Explicit boundary testing guidance.

#### Integrator
Cross-references to rust-verifier and rust-core established.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | Table + example |
| Completeness | 7 | 9 | Miri fallback, boundary testing |
| Structure | 7 | 9 | Logical: rules→checklist→example→verification |
| Actionability | 6 | 9 | Copy-paste Safety example |
| Conciseness | 8 | 8 | Focused |
| Integration | 6 | 8 | Connected to verifier, core |
| Error_Prevention | 8 | 9 | UB table comprehensive |
| Rust_Accuracy | 9 | 9 | Correct unsafe patterns |
| **TOTAL** | **58/80** | **70/80** | |

---

### Skill 9: rust-verifier

#### Architect
Verification gates in table. Capstone skill connecting to all others.

#### Linguist
NEVER rules at top. ALWAYS for profiling. Imperative throughout.

#### Psychologist
Gate table makes verification checklist scannable. Failure actions direct to other skills.

#### Tester
Table covers all 6 gates with specific failure actions. No gaps.

#### Integrator
References all other skills: compile-loop, error-triage, testing, supply-chain, unsafe.

| Dimension | Before | After | Notes |
|-----------|--------|-------|-------|
| Clarity | 7 | 9 | NEVER/ALWAYS, gate table |
| Completeness | 7 | 9 | All 6 gates covered |
| Structure | 7 | 9 | Rules→Gates→Procedure→Lint→Profile |
| Actionability | 7 | 9 | Failure actions for each gate |
| Conciseness | 6 | 8 | Removed prose, added tables |
| Integration | 6 | 9 | Hub connecting all skills |
| Error_Prevention | 7 | 9 | "NEVER rationalize" — strong |
| Rust_Accuracy | 8 | 9 | Correct lint levels, profile settings |
| **TOTAL** | **55/80** | **71/80** | |

---

## Final Summary

| Skill | Before | After | Key Changes |
|-------|--------|-------|-------------|
| rust-compile-loop | 54/80 | 68/80 | Critical rules top, heuristics table, verifier differentiation |
| rust-core | 52/80 | 70/80 | Sub-sections, code example, ALWAYS/NEVER, foundation declared |
| rust-error-triage | 52/80 | 68/80 | Error code table, warnings policy, compile-loop connection |
| rust-kata-coach | 53/80 | 66/80 | Rules first, scoring table, all-fail scenario |
| rust-refactor-safely | 51/80 | 68/80 | Anti-patterns table, revert policy, testing/compile-loop refs |
| rust-supply-chain | 53/80 | 69/80 | Checks table, documentation checklist, yanked versions |
| rust-testing | 53/80 | 70/80 | Code example, risk-level strategy, naming pattern |
| rust-unsafe | 58/80 | 70/80 | UB table, Safety example, Miri fallback |
| rust-verifier | 55/80 | 71/80 | Gates table, all-skills hub, failure actions |
| **AVERAGE** | **53.4/80** | **68.9/80** | **+29% improvement across all skills** |

COMPLETADO

---

## Verification Script Output

```
========================================
  VERIFICADOR DE TRABAJO DE AGENTE
  Mini-CI para colmenas y skills
========================================

--- Checkboxes ---
  Total:        9
  Completados:  9
  Pendientes:   0
  En progreso:  0

--- Skills modificadas ---
  [MOD] rust-compile-loop - modificado 09:57
  [MOD] rust-core - modificado 09:57
  [MOD] rust-error-triage - modificado 09:58
  [MOD] rust-kata-coach - modificado 09:58
  [MOD] rust-refactor-safely - modificado 09:59
  [MOD] rust-supply-chain - modificado 09:59
  [MOD] rust-testing - modificado 10:00
  [MOD] rust-unsafe - modificado 10:00
  [MOD] rust-verifier - modificado 10:01

  Skills modificadas hoy: 9 / 9

========================================
  VEREDICTO
========================================

  [PASS] Archivo de notas existe
  [PASS] Marca COMPLETADO encontrada
  [PASS] Todos los checkboxes completados (9/9)
  [PASS] Todas las skills (9) fueron modificadas

  === RESULTADO: PASS ===
  El agente completo su trabajo correctamente.
```

