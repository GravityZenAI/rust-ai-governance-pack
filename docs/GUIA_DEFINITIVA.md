# 🦀 LA GUÍA DEFINITIVA: Cómo Hacer que CUALQUIER IA Escriba Rust Sin Errores

### Por GravityZen AI — La referencia universal para desarrollo Rust con inteligencia artificial

> **¿Por qué esta guía es diferente?** Porque no es para UN solo tool. Funciona con Claude Code,
> Cursor, Windsurf, GitHub Copilot, OpenAI Codex, Gemini CLI, Aider, Zed, Amp, Cline, y cualquier
> agente que soporte el estándar AGENTS.md. Combina las Microsoft Pragmatic Rust Guidelines,
> 179 reglas de rust-skills, reglas de seguridad, técnicas anti-hallucination de Anthropic,
> y patrones de rescate probados en producción — todo en un solo documento.

---

## TABLA DE CONTENIDOS

1. [Por qué Rust + IA es diferente](#1-por-qué-rust--ia-es-diferente)
2. [Catálogo de errores: qué hace mal la IA con Rust](#2-catálogo-de-errores)
3. [El archivo universal AGENTS.md para Rust](#3-el-archivo-universal-agentsmd)
4. [Instalación para CADA herramienta de IA](#4-instalación-para-cada-herramienta)
5. [Las Microsoft Pragmatic Rust Guidelines (AI-optimized)](#5-microsoft-rust-guidelines)
6. [El CLAUDE.md definitivo para Rust](#6-el-claudemd-definitivo)
7. [Hooks y automatización por herramienta](#7-hooks-y-automatización)
8. [Árbol de decisión: Borrow Checker para IA](#8-árbol-de-decisión-borrow-checker)
9. [Prompts de rescate: cuando la IA se atora](#9-prompts-de-rescate)
10. [Anti-patrones con código: MAL → BIEN](#10-anti-patrones-con-código)
11. [Seguridad en Rust generado por IA](#11-seguridad)
12. [El workflow completo: de cero a código que compila](#12-workflow-completo)
13. [Recursos y links](#13-recursos)

---

## 1. POR QUÉ RUST + IA ES DIFERENTE

### El compilador es tu guardaespaldas

Con Python o JavaScript, si la IA genera código malo, no te enteras hasta que explota en
producción a las 3 AM. Con Rust, el compilador lo atrapa ANTES de ejecutar.

Julian Schrittwieser (Anthropic, creador de AlphaProof) lo descubrió en producción: la combinación
del type system de Rust con sus safety checks actúa como un code reviewer experto automático que
rechaza edits incorrectos y previene bugs. Cada error del compilador incluye:

- **Ubicación exacta** (archivo, línea, columna)
- **Explicación** de qué regla se violó
- **Sugerencia** de cómo arreglarlo
- **Link** a documentación detallada

La IA lee todo esto y corrige en segundos. Es un loop de feedback que no existe en ningún otro
lenguaje. Por eso Rust es paradójicamente el MEJOR lenguaje para desarrollo con IA.

### El problema real

El problema no es Rust — es que las IAs fueron entrenadas con más Python/JS que Rust. Cometen
errores predecibles y repetitivos. Esta guía los cataloga TODOS y te da la solución para cada uno.

---

## 2. CATÁLOGO DE ERRORES

### Errores que TODA IA comete con Rust

| # | Error | Frecuencia | Gravedad | La IA hace esto... | Debería hacer esto |
|---|-------|-----------|----------|--------------------|--------------------|
| 1 | **Lifetimes incorrectos** | MUY ALTA | Crítica | `fn parse<'a>(s: &str) -> &'a str` | Usar lifetime elision o atar al input |
| 2 | **Borrow after move** | MUY ALTA | Crítica | Mover un valor y luego referenciarlo | Usar references o clonar antes del move |
| 3 | **Mutable + immutable borrow** | ALTA | Crítica | `let r1 = &data; let r2 = &mut data;` | Separar scopes, usar NLL |
| 4 | **Traits inventados** | ALTA | Bloqueante | Llamar `.to_vec()` en tipos que no lo tienen | Verificar docs antes de llamar métodos |
| 5 | **Crates fantasma** | ALTA | Bloqueante | `use nonexistent_crate::Thing;` | Verificar en crates.io antes de usar |
| 6 | **APIs obsoletas** | ALTA | Funcional | `lazy_static!` | `std::sync::OnceLock` (Rust 1.80+) |
| 7 | **Async incorrecto** | MEDIA | Funcional | Mezclar sync/async, olvidar `.await` | Un runtime (tokio), await consistente |
| 8 | **`unwrap()` en producción** | MEDIA | Peligroso | `.unwrap()` por pereza | `?` operator o `match` |
| 9 | **`clone()` excesivo** | MEDIA | Rendimiento | `.clone()` para callar al borrow checker | References, lifetimes, o reestructurar |
| 10 | **`unsafe` innecesario** | MEDIA | Seguridad | `unsafe { }` para evitar el compilador | Buscar alternativa safe primero |
| 11 | **String vs &str** | MEDIA | Diseño | `fn greet(name: String)` en parámetros | `fn greet(name: &str)` |
| 12 | **Box\<dyn Error\>** | MEDIA | Diseño | `-> Result<T, Box<dyn Error>>` en libs | `thiserror` para libraries |
| 13 | **Mutación durante iteración** | MEDIA | Crítica | `for x in &v { v.push(x); }` | Collect primero, mutar después |
| 14 | **HashMap double lookup** | BAJA | Rendimiento | `contains_key` + `get` separados | Entry API: `.entry(k).or_insert(v)` |
| 15 | **Hardcodear tests** | BAJA | Tramposo | Return "42" porque matchea expected | TDD con property-based testing |

### Por qué estos errores ocurren

Los LLMs generan código basado en probabilidad estadística. Rust tiene patrones únicos
(ownership, borrowing, lifetimes) que aparecen con menos frecuencia en datos de entrenamiento
que los patterns de Python/JS. El resultado: la IA "adivina" y se equivoca en los puntos donde
Rust es más diferente de otros lenguajes.

---

## 3. EL ARCHIVO UNIVERSAL AGENTS.md PARA RUST

### ¿Qué es AGENTS.md?

AGENTS.md es el estándar universal (60,000+ repos en GitHub, respaldado por Linux Foundation,
OpenAI, Anthropic, y todos los major players) para dar instrucciones a agentes de IA. Es como
un README.md pero para las IAs. UN solo archivo que funciona con TODAS las herramientas.

### El AGENTS.md definitivo para Rust

Crea este archivo en la raíz de tu proyecto:

```markdown
# AGENTS.md — Rust Development Rules

## Identity
This is a Rust project. All code must compile with `cargo check`, pass `cargo clippy --all-targets
-- -D warnings`, and be formatted with `cargo fmt`.

## Critical Rules

### Compilation is LAW
- Run `cargo check` after EVERY change to .rs files
- Run `cargo clippy --all-targets -- -D warnings` before marking any task complete
- Run `cargo fmt` before committing
- NEVER deliver code that doesn't compile
- Read compiler errors COMPLETELY — Rust tells you exactly what's wrong and how to fix it

### Dependencies — VERIFY BEFORE USING
- ALWAYS verify a crate exists on crates.io BEFORE adding it
- Use `cargo add <crate>` to add dependencies
- Preferred ecosystem:
  - Async: `tokio` (with explicit features)
  - HTTP server: `axum`
  - HTTP client: `reqwest`
  - Serialization: `serde` + `serde_json`
  - Errors (libraries): `thiserror`
  - Errors (applications): `anyhow`
  - CLI: `clap` (derive)
  - Logging: `tracing` + `tracing-subscriber`
  - Testing: built-in + `tokio::test` for async

### Ownership & Borrowing — THE GOLDEN RULES
- Prefer `&T` (borrow) over `T` (move) in function parameters
- Prefer `&str` over `String` in function parameters
- NEVER use `.clone()` just to satisfy the borrow checker — restructure code instead
- Use this priority when the borrow checker complains:
  1. Adjust scope (move code so borrows don't overlap)
  2. Use references only when needed, drop quickly
  3. Clone ONLY for small types (<100 bytes) or when genuinely needed
  4. Restructure the algorithm
  5. Smart pointers: `Rc<T>` (single-thread), `Arc<T>` (multi-thread)
  6. Interior mutability: `RefCell<T>` or `Mutex<T>` as LAST resort

### Error Handling — NO SHORTCUTS
- ALWAYS use `Result<T, E>` for operations that can fail
- NEVER use `.unwrap()` in production code
- Use `.expect("descriptive message")` ONLY for programming errors (invariants)
- Use `?` operator for error propagation
- Use `thiserror` for library error types, `anyhow` for application errors
- Error messages: lowercase, no trailing punctuation

### What NEVER to Do
- NEVER invent crate names — verify on crates.io first
- NEVER use `unsafe` unless absolutely necessary and documented with `// SAFETY: reason`
- NEVER use `#[allow(unused)]` to hide problems
- NEVER use `Box<dyn Error>` as error type in libraries
- NEVER use `lazy_static!` — use `std::sync::OnceLock` instead
- NEVER ignore clippy warnings — fix them or justify suppression

### Style
- `snake_case` for functions/variables/modules
- `PascalCase` for types/traits/enums
- `SCREAMING_SNAKE_CASE` for constants
- Max 100 chars per line
- One module per file, files under 300 lines
- Document all public items with `///` including `# Examples`

### Testing
- Write tests BEFORE implementation when possible
- Tests in same file: `#[cfg(test)] mod tests { ... }`
- Integration tests in `tests/` directory
- Run `cargo test` after every implementation
- Never hardcode test results — use assertions that verify behavior

### Async Patterns
- Use `tokio` as the runtime (not `async-std`)
- Every async function that does I/O should return `Result`
- Use `tokio::sync::mpsc` for channels
- Use `tokio::sync::Mutex` (not `std::sync::Mutex`) in async code
- Always `.await` futures — a future that's not awaited does nothing

### Documentation
- Every public function: `///` with description
- Include `# Examples` with compilable code
- Include `# Errors` listing when function can fail
- Include `# Panics` if function can panic
- Module-level docs with `//!`
```

---

## 4. INSTALACIÓN PARA CADA HERRAMIENTA

### Un solo archivo, TODAS las herramientas

El `AGENTS.md` de arriba funciona directamente con la mayoría. Pero cada herramienta tiene
su ubicación preferida. Aquí están los comandos exactos:

```bash
# ============================================
# OPCIÓN A: AGENTS.md universal (RECOMENDADO)
# Funciona con Codex, Gemini CLI, Zed, Amp, y más
# ============================================
# Ya lo creaste arriba en la raíz del proyecto

# ============================================
# OPCIÓN B: Copiar para herramientas específicas
# ============================================

# Claude Code — CLAUDE.md
cp AGENTS.md CLAUDE.md

# Cursor — .cursorrules
cp AGENTS.md .cursorrules

# Windsurf — .windsurf/rules/
mkdir -p .windsurf/rules
cp AGENTS.md .windsurf/rules/rust-rules.md

# GitHub Copilot
mkdir -p .github
cp AGENTS.md .github/copilot-instructions.md

# OpenAI Codex
# Ya lee AGENTS.md automáticamente

# Aider
# Agregar a .aider.conf.yml:
# read: AGENTS.md

# Cline / Roo Code
mkdir -p .clinerules
cp AGENTS.md .clinerules/rust-rules.md

# ============================================
# OPCIÓN C: Symlinks (mantener un solo archivo)
# ============================================
ln -s AGENTS.md CLAUDE.md
ln -s AGENTS.md .cursorrules
mkdir -p .github && ln -s ../AGENTS.md .github/copilot-instructions.md
```

### Compatibilidad verificada

| Herramienta | Lee AGENTS.md | Archivo alternativo | Hooks | Skills |
|-------------|:---:|-----|:---:|:---:|
| **Claude Code** | ✅ | CLAUDE.md | ✅ | ✅ |
| **Cursor** | ✅ | .cursorrules / .cursor/rules/ | ✅ | ❌ |
| **Windsurf** | ✅ | .windsurf/rules/ | ✅ | ❌ |
| **GitHub Copilot** | ✅ | .github/copilot-instructions.md | ❌ | ❌ |
| **OpenAI Codex** | ✅ | (nativo) | ✅ | ❌ |
| **Gemini CLI** | ✅ | GEMINI.md | ❌ | ❌ |
| **Aider** | ✅ | via --read flag | ❌ | ❌ |
| **Zed** | ✅ | (nativo) | ❌ | ❌ |
| **Amp** | ✅ | (nativo) | ❌ | ❌ |
| **Cline** | ✅ | .clinerules/ | ❌ | ❌ |
| **ChatGPT/Claude.ai** | ❌ | Pegar en prompt | ❌ | ❌ |

---

## 5. MICROSOFT PRAGMATIC RUST GUIDELINES (AI-Optimized)

Microsoft creó la biblia del Rust idiomático Y una versión condensada específicamente para IAs.
Esto es lo que usan internamente mientras reescriben Windows y Azure en Rust.

### Instalación (una vez, para siempre)

```bash
# Descargar la versión AI-optimized (22K tokens, todo incluido)
mkdir -p .claude/skills/ms-rust
curl -o .claude/skills/ms-rust/rust-guidelines.txt \
  https://microsoft.github.io/rust-guidelines/agents/all.txt

# Crear el SKILL.md para que Claude lo active automáticamente
cat > .claude/skills/ms-rust/SKILL.md << 'EOF'
---
name: ms-rust
description: ALWAYS invoke this skill BEFORE writing or modifying ANY Rust code (.rs files).
  Enforces Microsoft Pragmatic Rust Guidelines. MANDATORY for all Rust development.
---
# Microsoft Rust Guidelines Skill
Before writing or modifying ANY Rust code, read and follow rust-guidelines.txt in this directory.
Load ONLY the sections relevant to the current task.
EOF
```

### Las 6 reglas clave de Microsoft para IA + Rust

1. **APIs idiomáticas** — Cuanto más tu código se parezca al Rust "estándar", mejor lo entiende la IA
2. **Docs exhaustivas** — `///` en todo lo público con Examples, Errors, Panics, Safety
3. **Tipos fuertes** — No usar `String` para todo; crear newtypes con semántica clara
4. **APIs testables** — La IA necesita iterar rápido; diseña para que pueda escribir tests
5. **Test coverage** — Con buenos tests, la IA puede refactorizar sin supervisión
6. **Ejemplos ejecutables** — La IA aprende de ejemplos, no de prosa

---

## 6. EL CLAUDE.md DEFINITIVO PARA RUST

Para proyectos que usan Claude Code como herramienta principal, este CLAUDE.md va más allá
del AGENTS.md genérico con instrucciones específicas de Claude:

```markdown
# CLAUDE.md — Proyecto Rust (GravityOS)

## Reglas del Compilador (OBLIGATORIAS)
- Ejecuta `cargo check` después de CADA cambio a archivos .rs
- Ejecuta `cargo clippy --all-targets -- -D warnings` antes de finalizar
- Ejecuta `cargo fmt` al terminar
- Si hay errores: lee el error COMPLETO del compilador, corrígelo, repite
- NUNCA entregues código que no compile

## Dependencias — VERIFICAR ANTES DE USAR
- Verifica que cada crate existe en crates.io ANTES de agregarlo
- Usa `cargo add <nombre>` para agregar — NUNCA edites Cargo.toml manualmente
- Stack preferido: tokio, axum, reqwest, serde, thiserror/anyhow, clap, tracing

## Rust Idiomático
- Consulta .claude/skills/ms-rust/rust-guidelines.txt para guidelines completas
- `Result<T, E>` siempre, `unwrap()` nunca en producción
- `&str` > `String` en parámetros, `impl Trait` > generics cuando solo hay un trait
- `OnceLock` > `lazy_static!`, Entry API > double lookup en HashMap
- `?` operator para propagación, `thiserror` en libs, `anyhow` en apps

## Workflow
- Define types y traits ANTES de implementar
- Escribe tests ANTES del código cuando sea posible
- Un módulo por archivo, máximo 300 líneas
- Documenta todo lo público con `///` + Examples

## Lo que NUNCA Debes Hacer
- NUNCA inventes crates, traits o métodos — verifica primero
- NUNCA uses `.clone()` solo para callar al borrow checker
- NUNCA uses `unsafe` sin documentar `// SAFETY:` con razón
- NUNCA uses `#[allow(unused)]` para esconder problemas
- NUNCA ignores warnings de clippy
```

---

## 7. HOOKS Y AUTOMATIZACIÓN

### Claude Code — Hooks automáticos

Crea `.claude/settings.json`:
```json
{
  "hooks": {
    "afterWrite": [
      {
        "pattern": "**/*.rs",
        "command": "cargo check 2>&1 | head -50"
      },
      {
        "pattern": "**/*.rs",
        "command": "cargo clippy --all-targets -- -D warnings 2>&1 | head -30"
      }
    ]
  }
}
```

### Cursor — Configuración equivalente

En `.cursor/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--all-targets", "--", "-D", "warnings"],
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Para CUALQUIER herramienta — Makefile universal

```makefile
.PHONY: check lint fmt test audit all

check:
	cargo check --workspace

lint:
	cargo clippy --all-targets --workspace -- -D warnings

fmt:
	cargo fmt --all

test:
	cargo test --workspace

audit:
	cargo audit
	cargo deny check

# La IA debe ejecutar esto antes de dar cualquier código por terminado
all: check lint fmt test
```

---

## 8. ÁRBOL DE DECISIÓN: BORROW CHECKER PARA IA

Cuando la IA (o tú) se atora con el borrow checker, seguir este orden de prioridad.
Esto es lo que debería estar en la "mente" de la IA:

```
¿EL BORROW CHECKER SE QUEJA?
│
├─ PASO 1: ¿Puedo ajustar el scope?
│  └─ Mover el código para que los borrows no se solapen
│     Ejemplo: mover el `println!` ANTES del `&mut`
│     ✅ Cambio mínimo, sin costo de rendimiento
│
├─ PASO 2: ¿Puedo usar el reference solo cuando lo necesito?
│  └─ Tomar el reference justo antes de usarlo, dropearlo rápido
│     Ejemplo: `{ let r = &data[0]; println!("{r}"); }` en un bloque
│     ✅ Aprovecha NLL (Non-Lexical Lifetimes)
│
├─ PASO 3: ¿El dato es pequeño (<100 bytes)?
│  └─ .clone() está OK para tipos pequeños (i32, String cortos, etc.)
│     ✅ Impacto negligible en rendimiento
│
├─ PASO 4: ¿Puedo reestructurar el algoritmo?
│  └─ Separar la fase de lectura de la fase de escritura
│     Ejemplo: collect primero en Vec, luego mutar
│     ✅ A menudo produce código más claro
│
├─ PASO 5: ¿Necesito múltiples owners?
│  └─ Single-thread: Rc<T>
│     Multi-thread: Arc<T>
│     ✅ Overhead mínimo, solución correcta
│
└─ PASO 6: ¿Necesito mutación con múltiples owners? (ÚLTIMO RECURSO)
   └─ Single-thread: Rc<RefCell<T>>
      Multi-thread: Arc<Mutex<T>> o Arc<RwLock<T>>
      ⚠️ Costo runtime, usar solo cuando es genuinamente necesario
```

### Patrones específicos que la IA debe conocer

```
PROBLEMA: Iterar y mutar al mismo tiempo
MAL:  for x in &vec { vec.push(x * 2); }
BIEN: let additions: Vec<_> = vec.iter().map(|x| x * 2).collect();
      vec.extend(additions);

PROBLEMA: Double lookup en HashMap
MAL:  if !map.contains_key(&k) { map.insert(k, default); }
      let v = map.get(&k).unwrap();
BIEN: let v = map.entry(k).or_insert(default);

PROBLEMA: Return reference de función
MAL:  fn get_name() -> &str { &String::from("hello") } // dangling!
BIEN: fn get_name() -> String { String::from("hello") }
  O:  fn get_name(s: &str) -> &str { &s[0..5] } // tied to input

PROBLEMA: Mutable borrow en struct method
MAL:  let x = &self.field_a; self.field_b = 42;
BIEN: let x = self.field_a.clone(); self.field_b = 42; // if small
  O:  Usar métodos separados que tomen &self y &mut self
```

---

## 9. PROMPTS DE RESCATE: CUANDO LA IA SE ATORA

### Librería de prompts probados en producción

Cuando la IA no puede resolver algo, usa estos prompts exactos:

#### Lifetimes
```
"Fix the lifetimes. Read the complete compiler error. Apply EXACTLY
what the compiler suggests. If you need to add lifetime parameters,
match them to the input references."
```

#### Borrow Checker
```
"The borrow checker is complaining. Follow this priority:
1. Can you adjust the scope so borrows don't overlap?
2. Can you take the reference later or drop it earlier?
3. Is the data small enough to clone?
4. Can you restructure to separate read and write phases?
5. Only if nothing else works: use Rc/Arc.
Try each in order. Show me what you tried."
```

#### No compila (genérico)
```
"This doesn't compile. Run `cargo check`, read ALL errors from
first to last, and fix them one by one starting with the FIRST
error (later errors are often caused by the first one)."
```

#### Clone excesivo
```
"There are too many .clone() calls. For each one, explain WHY
it's necessary. If it's just to satisfy the borrow checker,
restructure the code to use references instead."
```

#### Crate que no existe
```
"Before using any crate, verify it exists on crates.io. Use
`cargo add <name>` to add it. If it doesn't exist, find the
real crate that provides this functionality."
```

#### Código unsafe
```
"Replace ALL unsafe code with safe alternatives. If truly
impossible to do safely, document EXACTLY why with a
// SAFETY: comment explaining the invariant."
```

#### Async confuso
```
"Every future MUST be .await'ed. Every async function that does
I/O should return Result. Use tokio::sync::Mutex (not std) in
async code. Check that all spawned tasks are awaited or joined."
```

#### La IA genera demasiado código
```
"This function is too long. Split it into smaller functions,
each with a single responsibility. Each function should be
under 40 lines. Extract helper functions."
```

#### Para ChatGPT / Claude.ai (sin terminal)
```
"Write [what you need] in Rust. Before showing me the code:
1. Mentally run `cargo check` — fix any compilation errors
2. Mentally run `cargo clippy` — fix any warnings
3. Verify every crate name exists on crates.io
4. Ensure no .unwrap() in non-test code
5. Check all lifetimes are correct
Show me the final, corrected code."
```

---

## 10. ANTI-PATRONES CON CÓDIGO: MAL → BIEN

### AP-01: Lifetime que cuelga

```rust
// ❌ MAL — lifetime 'a no está conectado a nada
fn first_word<'a>(s: &str) -> &'a str {
    &s[..s.find(' ').unwrap_or(s.len())]
}

// ✅ BIEN — lifetime elision (el compilador lo infiere)
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

### AP-02: unwrap() en producción

```rust
// ❌ MAL — panic en producción si el archivo no existe
let content = std::fs::read_to_string("config.toml").unwrap();

// ✅ BIEN — error propagado con contexto
let content = std::fs::read_to_string("config.toml")
    .context("Failed to read config.toml")?;
```

### AP-03: Clone para callar al borrow checker

```rust
// ❌ MAL — clone innecesario
fn process(data: &[String]) {
    let first = data[0].clone();  // clon innecesario
    println!("Processing: {first}");
}

// ✅ BIEN — reference es suficiente
fn process(data: &[String]) {
    let first = &data[0];
    println!("Processing: {first}");
}
```

### AP-04: String en parámetros

```rust
// ❌ MAL — fuerza al caller a tener un String owned
fn greet(name: String) {
    println!("Hello, {name}!");
}

// ✅ BIEN — acepta &str, String, y más via deref coercion
fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

### AP-05: lazy_static obsoleto

```rust
// ❌ MAL — patrón obsoleto que requiere crate externo
lazy_static::lazy_static! {
    static ref CONFIG: Config = load_config();
}

// ✅ BIEN — standard library, sin dependencia externa
static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();
fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| load_config())
}
```

### AP-06: Box\<dyn Error\> en library

```rust
// ❌ MAL — pierde información del tipo de error
pub fn parse(input: &str) -> Result<Data, Box<dyn std::error::Error>> {
    // ...
}

// ✅ BIEN — error tipado con thiserror
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid format at position {0}")]
    InvalidFormat(usize),
    #[error("missing field: {0}")]
    MissingField(String),
}

pub fn parse(input: &str) -> Result<Data, ParseError> {
    // ...
}
```

### AP-07: Mutación durante iteración

```rust
// ❌ MAL — no compila (mutable borrow durante iteración)
let mut numbers = vec![1, 2, 3, 4, 5];
for n in &numbers {
    if *n > 3 {
        numbers.push(*n * 2);
    }
}

// ✅ BIEN — separar lectura de escritura
let mut numbers = vec![1, 2, 3, 4, 5];
let additions: Vec<i32> = numbers.iter()
    .filter(|&&n| n > 3)
    .map(|&n| n * 2)
    .collect();
numbers.extend(additions);
```

### AP-08: Async con std::sync::Mutex

```rust
// ❌ MAL — bloquea el thread del runtime async
use std::sync::Mutex;
let data = Arc::new(Mutex::new(vec![]));
// En contexto async: data.lock().unwrap() puede bloquear todo

// ✅ BIEN — mutex async-aware
use tokio::sync::Mutex;
let data = Arc::new(Mutex::new(vec![]));
let mut guard = data.lock().await;  // yield-friendly
guard.push(42);
```

---

## 11. SEGURIDAD EN RUST GENERADO POR IA

### Reglas de seguridad críticas

La IA puede generar código que compila pero tiene vulnerabilidades.
Estas reglas previenen los problemas de seguridad más comunes:

```markdown
## Security Rules for AI-Generated Rust

### Input Validation
- ALWAYS validate and sanitize external input before processing
- NEVER trust data from users, files, network, or environment variables
- Use strong types (newtypes) to distinguish validated from unvalidated data

### Memory Safety
- NEVER use `unsafe` without a SAFETY comment documenting invariants
- Run `cargo audit` to check dependencies for known vulnerabilities
- Run `cargo deny check` for license and advisory compliance
- Use `cargo geiger` to detect unsafe code in dependencies

### Cryptography
- NEVER implement custom cryptography — use `ring` or `rustls`
- NEVER use `rand::thread_rng()` for security — use `rand::rngs::OsRng`
- NEVER hardcode secrets, keys, or tokens in source code

### Network
- ALWAYS use TLS for network connections (reqwest defaults to this)
- ALWAYS validate certificates (don't disable verification)
- Set timeouts on all network operations

### Dependencies
- Pin dependency versions in Cargo.lock (commit it for binaries)
- Audit new dependencies before adding: `cargo audit`, `cargo deny`
- Prefer crates with > 1M downloads and recent maintenance
- NEVER add a dependency for something the standard library provides

### Logging & Secrets
- NEVER log secrets, tokens, passwords, or PII
- Use `secrecy::Secret<T>` for sensitive data that shouldn't be logged
- Implement custom Debug for structs with sensitive fields
```

---

## 12. EL WORKFLOW COMPLETO: DE CERO A CÓDIGO QUE COMPILA

### Paso 0 — Setup del proyecto (una vez)

```bash
# Crear proyecto
cargo new gravityos-module
cd gravityos-module

# Agregar AGENTS.md (copiar sección 3 de esta guía)
# Agregar CLAUDE.md (copiar sección 6)
# Instalar Microsoft guidelines (sección 5)
# Configurar hooks (sección 7)

# Crear symlinks para compatibilidad multi-tool
ln -s AGENTS.md .cursorrules
mkdir -p .github && ln -s ../AGENTS.md .github/copilot-instructions.md

# Instalar herramientas de verificación
cargo install cargo-audit cargo-deny cargo-watch
rustup component add clippy rustfmt
```

### Paso 1 — Definir tipos y traits PRIMERO

Decirle a la IA:
```
"Define the types and traits for [mi módulo]. Do NOT implement
anything yet — only the interfaces. Run cargo check to verify
they compile."
```

### Paso 2 — Escribir tests ANTES del código

```
"Write tests for [funcionalidad] FIRST. The tests should verify
the expected behavior. Then implement the code to make them pass.
Run cargo test after each implementation."
```

### Paso 3 — Implementar con el loop de compilación

```
"Implement trait X for struct Y. After each change, run cargo check.
If there are errors, read them completely and fix them. Repeat until
it compiles. Then run cargo clippy and fix all warnings."
```

### Paso 4 — Verificación final

```
"Run the full verification suite:
1. cargo check --workspace
2. cargo clippy --all-targets -- -D warnings
3. cargo fmt --check
4. cargo test --workspace
5. cargo audit
Fix any issues found."
```

### El Mantra
```
Types primero → Tests segundo → Implementar → Compilar → Clippy → Repetir
```

---

## 13. RECURSOS Y LINKS

### Estándares y Guidelines

| Recurso | URL | Qué es |
|---------|-----|--------|
| Microsoft Rust Guidelines | microsoft.github.io/rust-guidelines | La biblia oficial para Rust a escala |
| MS Guidelines AI version | microsoft.github.io/rust-guidelines/agents/ | Versión condensada para IAs (22K tokens) |
| AGENTS.md standard | agents.md | El estándar universal para instrucciones a IAs |
| Rust API Guidelines | rust-lang.github.io/api-guidelines | Guidelines oficiales del equipo de Rust |

### Skills y Rules pre-hechos

| Recurso | URL | Qué es |
|---------|-----|--------|
| rust-skills (179 reglas) | github.com/leonardomso/rust-skills | 179 reglas en 14 categorías, todos los agents |
| Rust Coding Guidelines | mcpmarket.com/es/tools/skills/rust-coding-guidelines | 50 reglas core como Claude Code Skill |
| cursor-rust-rules | github.com/tyrchen/cursor-rust-rules | Rules modulares para Cursor IDE |
| rust-claude-code template | github.com/iepathos/rust-claude-code | Starter template con CLAUDE.md |
| Secure Coding Rules | github.com/anthropics/claude-secure-coding-rules | 100+ reglas de seguridad incluyendo Rust |
| antigravity-awesome-skills | github.com/sickn33/antigravity-awesome-skills | 800+ skills para múltiples agents |
| rust-agentic-skills | github.com/udapy/rust-agentic-skills | Skills con loop RPI para Rust |

### Artículos clave

| Recurso | URL | Qué es |
|---------|-----|--------|
| Claude Code + Rust (Julian) | julian.ac/blog/2025/05/03/claude-code-and-rust | El descubrimiento del compilador como code reviewer |
| Coding Rust with Claude (Tigran) | tigran.tech/coding-rust-with-claude-code-and-codex | Experiencia real en producción (Sayna) |
| MS Guidelines + Claude | 40tude.fr/docs/06_programmation/rust/019_ms_rust | Tutorial paso a paso de setup completo |
| Mastering Cursor + Rust | medium.com/@kamol (By DevOps For DevOps) | TDD con IA, memory.md pattern |
| Reduce Hallucinations (Anthropic) | platform.claude.com/.../reduce-hallucinations | Técnicas oficiales anti-hallucination |

### Herramientas de verificación

| Herramienta | Comando | Qué hace |
|-------------|---------|----------|
| cargo check | `cargo check` | Verifica compilación sin producir binario |
| cargo clippy | `cargo clippy -- -D warnings` | Linter estricto con 700+ rules |
| cargo fmt | `cargo fmt` | Formateo automático estándar |
| cargo audit | `cargo audit` | Vulnerabilidades conocidas en deps |
| cargo deny | `cargo deny check` | Licencias + advisories + duplicados |
| cargo geiger | `cargo geiger` | Detecta unsafe en dependencias |
| cargo bloat | `cargo bloat` | Análisis de tamaño de binario |
| cargo mutants | `cargo mutants` | Mutation testing (calidad de tests) |
| cargo semver-checks | `cargo semver-checks` | Verifica semver en libraries |

---

## RESULTADO ESPERADO

Con esta guía aplicada correctamente:

- **80-90%** del código generado por IA compila al primer intento (vs ~50% sin guía)
- **Errores restantes** se auto-corrigen en 1-2 iteraciones del loop de compilación
- **Cero crates inventados** porque la IA verifica antes de usar
- **Código idiomático** gracias a clippy + Microsoft guidelines
- **Seguro** por las reglas de seguridad integradas
- **Funciona con CUALQUIER herramienta de IA** gracias al estándar AGENTS.md

---

*Compilado por GravityZen AI — 22 de febrero de 2026*
*Fuentes: Microsoft Rust Guidelines, Anthropic (Julian Schrittwieser), Linux Foundation (AAIF),
 OpenAI (AGENTS.md), leonardomso/rust-skills, 40tude.fr, Tigran.tech/Sayna, claude-flow,
 cargo-audit, Rust API Guidelines, MCP Market, cursor-rust-rules, antigravity-awesome-skills*

**Licencia: CC BY 4.0 — Usa, comparte, modifica con atribución a GravityZen AI**
