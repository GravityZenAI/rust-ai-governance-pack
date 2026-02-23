Hallazgos clave (verificados en fuentes)

* **Antigravity soporta “Rules” (siempre activas) y “Workflows” (prompts guardados)**; pueden ser **globales** o **por workspace**, y los workflows se disparan con `/` desde el panel del agente. ([Google Codelabs][1])
* Ubicaciones verificadas:

  * **Global rule:** `~/.gemini/GEMINI.md`
  * **Global workflow:** `~/.gemini/antigravity/global_workflows/global-workflow.md`
  * **Workspace rules:** `your-workspace/.agent/rules/`
  * **Workspace workflows:** `your-workspace/.agent/workflows/` ([Google Codelabs][1])
* Antigravity controla autonomía del agente con **Terminal Command Auto Execution** (Off/Auto/Turbo) y permite **Allow List** y **Deny List** para comandos. ([Google Codelabs][1])
* Antigravity también permite un **Browser URL Allowlist** (archivo `HOME/.gemini/antigravity/browserAllowlist.txt`) para mitigar riesgos como prompt injection al navegar. ([Google Codelabs][1])
* Para “competencia real” en Rust, lo que más reduce errores no es “memorizar”, sino un loop con herramientas canónicas:

  * `cargo fmt` (rustfmt) ([GitHub][2])
  * `cargo clippy` (+ CI con `-D warnings` si quieres convergencia fuerte) ([GitHub][3])
  * `cargo test` (incluye unit/integration/doctests; argumentos con `--`) ([Documentación de Rust][4])
  * `cargo miri` (solo **nightly**, componente opcional) si hay `unsafe` o UB sospechosa ([Documentación de Rust][5])
* Las bases canónicas para enseñar/normalizar el lenguaje (lo que el agente debe tratar como “verdad”) siguen siendo:

  * Ownership/borrowing ([Documentación de Rust][6])
  * Lifetimes ([Documentación de Rust][7])
  * Generics/traits (Book + Reference para bounds) ([Documentación de Rust][8])
  * Módulos/crates ([Documentación de Rust][9])
  * Error handling en Rust Book (Result, etc.) ([Documentación de Rust][10])
* Supply-chain (secundario, pero útil como “gate” opcional):

  * `cargo-audit` (RustSec) ([docs.rs][11])
  * `cargo-deny` (advisories/licencias/fuentes/bans) ([GitHub][12])
  * `cargo-vet` (auditoría humana/criterios) ([mozilla.github.io][13])
* Punto ciego que suele matar estos sistemas: creer que “instrucciones perfectas” reemplazan al compilador. Tu mejor defensa es que **nada se acepta si `verify` no está verde**, y que el agente esté obligado por reglas + CI a vivir dentro de ese loop.

---

# A) Arquitectura final (1–2 páginas)

## Objetivo operativo

Que el agente “sepa Rust” de forma **medible**: converge a verde rápido, reduce repetición de errores, mantiene estilo consistente. Eso se logra haciendo que el **compilador + tests + lint + fmt** sean la autoridad, y que el agente no pueda “declarar victoria” sin evidencia.

## Enfoque ganador (por evidencia, no opinión)

**Mejor enfoque:** *tool-driven agent* con “un solo juez” (`verify.sh`) y “gates” (CI + reglas).
Razón: Rust ya trae el feedback más valioso (borrow checker, type checker) y Clippy aporta idiomaticidad; si el agente está obligado a iterar hasta `verify green`, la competencia es verificable. (Las herramientas y flags concretos están documentados en rustfmt/Clippy/Cargo/Miri). ([GitHub][2])

## Componentes

### 1) Rules permanentes (siempre activas)

Ubicación: `your-workspace/.agent/rules/` (Antigravity las aplica por workspace). ([Google Codelabs][1])

Reglas “core” (inviolables):

* **No “DONE” sin `verify green`.**
* **Iteración incremental:** especificación → patch mínimo → `cargo check/test` → ajustar → `clippy` → `fmt` → `verify`.
* **Evidencia obligatoria:** comandos ejecutados + output relevante + diff + resumen.
* **Aprendizaje acumulativo:** cuando aparece un error nuevo, se registra en `docs/ai/ERROR_PATTERNS.md`.

### 2) Skills modulares (cargables por situación)

En Antigravity, el mecanismo verificable que tienes hoy para “skills a demanda” son **Workflows** (prompts guardados invocables con `/`). ([Google Codelabs][1])
“Skill” aquí = workflow + checklist + mini-plantillas que fuerzan el loop correcto.

Skills sugeridas:

* `rust-implement-loop`
* `rust-error-triage`
* `rust-refactor-safe`
* `rust-deps-change`
* `rust-unsafe-miri`

### 3) Verificador único: `tools/verify.sh`

Una sola entrada que decide si el trabajo “existe”.
Debe correr lo mínimo para detectar:

* formateo
* lint
* tests
* (opcional) miri y supply-chain

### 4) CI gates

El CI solo corre `tools/verify.sh`. Si falla, no hay merge.

### 5) Memoria persistente en repo

* `docs/ai/RUST_PLAYBOOK.md` (canon operativo)
* `docs/ai/ERROR_PATTERNS.md` (biblioteca viva, empieza con Top 30)
* `docs/ai/DECISIONS.md` (decisiones + excepciones aprobadas)

### 6) Protocolo de excepción (bypass)

Permitido **solo** si:

* está escrito en `DECISIONS.md` con: motivo, alcance, fecha, responsable, plan de remediación.
* bypass típico: `miri`/fuzz en PRs no críticas por costo de runtime.
* bypass **prohibido**: `fmt`, `clippy` (al menos `clippy::correctness`), `cargo test`.

### 7) Antigravity “difícil de eludir”

Config recomendada por defecto (segura sin romper productividad):

* **Review-driven development** como balance recomendado por el codelab (el agente pide revisión con frecuencia). ([Google Codelabs][1])
* Terminal: **Auto** para entrenamiento/iteración rápida; **Off + Allow List** para entornos sensibles. La existencia de Off/Auto/Turbo y allow/deny lists está documentada. ([Google Codelabs][1])
* Browser URL allowlist activado con dominios de confianza (docs oficiales, repos oficiales). ([Google Codelabs][1])

---

# E) Integración en Antigravity IDE (Rules/Workflows/Settings)

## 1) Dónde van Rules y Workflows

Crea estos directorios en tu repo (workspace):

* `./.agent/rules/`
* `./.agent/workflows/` ([Google Codelabs][1])

En el UI: `Customizations → Rules/Workflows` (según el codelab). ([Google Codelabs][1])

## 2) Settings de terminal (postura recomendada)

Antigravity soporta:

* **Off:** no auto-ejecuta comandos salvo allowlist
* **Auto:** decide según modelos internos, pide permiso si ve riesgo
* **Turbo:** auto-ejecuta salvo denylist ([Google Codelabs][1])

Para tu objetivo (dominio Rust + mínima tasa de error):

* **Modo Entrenamiento/Katas:** Terminal = **Auto** (reduce fricción de iteración).
* **Modo Producción/Repo crítico:** Terminal = **Off** y allowlist mínima (por ejemplo `./tools/verify.sh`, `cargo test`, `cargo clippy`, `cargo fmt`) — el resto requiere review.

El codelab describe cómo configurar allowlist/denylist y da ejemplos (`rm`, `sudo`, `curl`, `wget` en denylist). ([Google Codelabs][1])

## 3) Browser allowlist (para navegar docs sin riesgo)

El codelab documenta que puedes abrir el archivo:
`HOME/.gemini/antigravity/browserAllowlist.txt` y poner dominios confiables. ([Google Codelabs][1])

Recomendación práctica de allowlist (mínima):

* doc.rust-lang.org
* github.com/rust-lang
* github.com/RustSec
* rustsec.org
* github.com/mozilla
* github.com/EmbarkStudios
* rust-fuzz.github.io (si usas fuzz)

---

# A) Reglas permanentes + Workflows + Verify + CI (listo para copiar/pegar)

A continuación te dejo el “pack mínimo” como archivos. Copias tal cual.

## `./.agent/rules/00-rust-contract.md`

```md
# RUST CONTRACT (siempre activo)

## Objetivo
Producir cambios Rust que compilen, pasen tests y mantengan estilo consistente.

## Reglas inviolables
1) No declares "DONE" ni "READY" sin:
   - ejecutar `./tools/verify.sh`
   - pegar el output relevante (o resumirlo con líneas clave)
   - y confirmar que está green.

2) Loop obligatorio (en este orden):
   Spec breve -> patch mínimo -> `cargo check` -> `cargo test` -> `cargo clippy` -> `cargo fmt` -> `./tools/verify.sh`.

3) Cambios incrementales:
   - Si el PR es grande, divídelo en pasos pequeños, cada uno con verify green.

4) Si aparece un error nuevo de rustc/clippy:
   - registra el patrón en `docs/ai/ERROR_PATTERNS.md` (causa raíz + fix canónico + ejemplo).

5) Rust idiomático:
   - Evita `unwrap/expect` en código de librería o paths de error; prefiere `Result` + `?`.
```

(Nota: preferir `Result` y el operador `?` está alineado con guías de buenas prácticas (ANSSI) y con el enfoque de error handling documentado. ([ANSSI][14]))

## `./.agent/rules/01-rust-style-and-lint.md`

```md
# Rust style & lint

- Formato: `cargo fmt` (rustfmt) debe estar limpio.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings` como gate por defecto.
- No habilitar `clippy::restriction` completo sin justificar; usar lints caso por caso.
```

(La ejecución de rustfmt vía `cargo fmt` está documentada en rustfmt. ([GitHub][2])
El gate de Clippy en CI con `--all-targets --all-features -- -D warnings` está documentado en el README de rust-clippy y en la documentación de uso. ([GitHub][3]))

## `./.agent/workflows/rust-implement-loop.md`

```md
# /rust-implement-loop

Sigue este protocolo exacto:

1) SPEC (máx 8 líneas)
- Qué cambia
- Qué NO cambia
- Qué tests/archivos afectan

2) Plan incremental (3-7 pasos). Cada paso debe compilar.

3) Implementa el paso 1 con patch mínimo.

4) Ejecuta:
- cargo check
- cargo test
Si falla, corrige primero compile errors (rustc), luego tests.

5) Cuando todo pase:
- cargo clippy --all-targets --all-features -- -D warnings
- cargo fmt
- ./tools/verify.sh

6) Entrega:
- comandos ejecutados + output relevante
- diff/archivos tocados
- resumen final (qué y por qué)
- si hubo errores nuevos, actualiza docs/ai/ERROR_PATTERNS.md
```

(Workflows se invocan con `/` y se guardan en `your-workspace/.agent/workflows/`. ([Google Codelabs][1]))

## `./.agent/workflows/rust-error-triage.md`

```md
# /rust-error-triage

Input requerido: pega el error completo (rustc o clippy).

Proceso:
1) Clasifica: compile / test / clippy / fmt.
2) Si es rustc:
   - identifica: ownership/borrow, lifetimes, traits, modules, types.
3) Propón fix con patch mínimo.
4) Re-ejecuta el comando que falló.
5) Registra patrón si se repite:
   docs/ai/ERROR_PATTERNS.md
```

## `./tools/verify.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "== verify: format (rustfmt) =="
cargo fmt --all -- --check

echo "== verify: build lock determinism =="
# Falla si Cargo.lock está desactualizado/cambia durante resolución
cargo build --locked

echo "== verify: clippy (strict) =="
cargo clippy --all-targets --all-features -- -D warnings

echo "== verify: tests =="
cargo test --all-features

echo "== verify: done =="
```

(El uso de `cargo fmt` está documentado en rustfmt. ([GitHub][2])
El gate recomendado de Clippy con `--all-targets --all-features -- -D warnings` está documentado. ([GitHub][3])
`cargo build --locked` está documentado en Cargo Book como modo para asegurar que el lockfile no cambie. ([Documentación de Rust][15])
`cargo test` y su semántica/flags (`--`) están documentados en Cargo Book. ([Documentación de Rust][4]))

## CI gate (GitHub Actions) `./.github/workflows/rust-verify.yml`

```yml
name: rust-verify

on:
  pull_request:
  push:
    branches: [ main ]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - name: Verify
        run: ./tools/verify.sh
```

(Esto es plantilla; si no usas GitHub, el concepto es el mismo: CI solo corre `verify.sh`.)

---

# B) “Playbook Rust” mínimo (canónico)

Guárdalo como `docs/ai/RUST_PLAYBOOK.md`.

## `docs/ai/RUST_PLAYBOOK.md`

```md
# RUST_PLAYBOOK (canónico)

## Regla de oro
Si algo no compila, no existe. Rust se aprende operando el loop del compilador.

## 1) Ownership / Borrowing (lo que más rompe)
- Por defecto: valores tienen un único dueño; mover transfiere ownership.
- Referencias `&T` prestan sin mover; `&mut T` es exclusivo.
- Heurística:
  - Si solo lees: toma `&T`.
  - Si modificas sin tomar propiedad: `&mut T`.
  - Si necesitas guardar o devolver ownership: toma `T` y devuelve `T`.
(Ownership y borrowing están explicados en el Rust Book). 

## 2) Lifetimes (solo cuando el compilador te obliga)
- Si devuelves referencias, el compilador necesita relacionar “de dónde vienen”.
- Regla práctica:
  - Evita devolver `&T` si puedes devolver `T` (clonar o mover si procede).
  - Si sí debes devolver referencia: ata lifetime del output a una entrada.
(Lifetimes en Rust Book).

## 3) Traits y generics (idiomático)
- Usa traits para expresar capacidades, no jerarquías.
- Preferir bounds en `where` cuando crecen.
- Si un método necesita un comportamiento, exprésalo como bound.
(Generics/traits en Rust Book; bounds en Rust Reference).

## 4) Módulos / crates
- Diseña “API pública” pequeña: `pub` mínimo.
- Usa módulos para encapsular invariantes.
- Evita `pub use` caótico: re-export solo lo que es API.
(Módulos/crates en Rust Book).

## 5) Error handling idiomático
- Usa `Result<T, E>` para fallos esperables.
- Usa `?` para propagar fallos con legibilidad.
- Evita `panic!/unwrap/expect` en paths de producción (salvo pruebas o prototipos).
(Rust Book error handling; ANSSI recomienda Result y `?` y evitar panics).

## 6) Testing (mínimo exigible)
- Cada cambio debe traer tests o explicar por qué no aplica.
- `cargo test` corre unit + integration + doctests.
- Si el output importa, usa `cargo test -- --show-output` o `-- --nocapture`.
(Cargo Book + Rust Book testing).

## 7) Concurrencia (std)
- Preferir message passing cuando sea simple.
- Si compartes estado: `Arc<Mutex<T>>`.
(Concurrencia en Rust Book).

## 8) Unsafe (solo con razón)
- `unsafe` no es “rápido”, es “tú te haces responsable”.
- Si aparece unsafe: documenta invariantes + considera `cargo miri test` en CI.
(Miri requiere nightly; Miri docs describen `cargo miri test`).
```

Fuentes: Rust Book (ownership, lifetimes, traits, modules), Cargo testing, y Miri. ([Documentación de Rust][6])

---

# C) Error Pattern Library (Top 30) — práctica y “operativa”

Guárdalo como `docs/ai/ERROR_PATTERNS.md`.

Importante: sin tu codebase real no puedo garantizar “frecuencia real” de cada uno, pero estos 30 cubren la mayoría de fricción inicial/intermedia (types, moves, borrows, lifetimes, traits, modules, unsafe) y son los que más rápido convierten a un agente en alguien que “opera Rust”.

## `docs/ai/ERROR_PATTERNS.md`

```md
# ERROR_PATTERNS (Top 30 inicial)

Formato por patrón:
- Síntoma
- Causa raíz
- Fix canónico
- Ejemplo (malo) / Ejemplo (bueno)
- Test mínimo

---

## 1) E0308 — mismatched types
- Causa: inferiste/retornaste tipo distinto al esperado.
- Fix: ajusta firma o convierte explícitamente (`as`, `into`, `parse`, `to_string`, etc).
- Malo:
  let x: i32 = "5";
- Bueno:
  let x: i32 = "5".parse().unwrap();
- Test: `cargo check`

## 2) E0382 — use of moved value
- Causa: moviste un valor (p.ej. `String`) y lo usaste después.
- Fix: pasa `&T`, clona si es correcto, o reestructura para no usar tras el move.
- Malo:
  let s = String::from("hi");
  takes(s);
  println!("{s}");
- Bueno:
  let s = String::from("hi");
  takes(&s);
  println!("{s}");
- Test: `cargo check`

## 3) E0502 — cannot borrow as mutable because also borrowed as immutable
- Causa: tienes `&` activo y luego quieres `&mut`.
- Fix: reduce scope del préstamo inmutable (bloque `{}`), o reordena para mutar antes.
- Malo:
  let x = &v[0];
  v.push(1);
  println!("{x}");
- Bueno:
  let x0 = v[0];
  v.push(1);
  println!("{x0}");
- Test: `cargo check`

## 4) E0499 — cannot borrow as mutable more than once at a time
- Causa: dos `&mut` vivos al mismo tiempo.
- Fix: usa `split_at_mut`, reestructura, o reduce scopes.
- Test: `cargo check`

## 5) E0597 — borrowed value does not live long enough
- Causa: referencia apunta a valor que muere antes que la referencia.
- Fix: extiende vida del dueño (bind a variable), o devuelve owned (`String`) en vez de `&str`.
- Test: `cargo check`

## 6) E0106 — missing lifetime specifier
- Causa: firma devuelve referencia sin relacionarla con input.
- Fix: agrega lifetime `'a` y átalo a input, o cambia a retorno owned.
- Test: `cargo check`

## 7) E0277 — trait bound not satisfied
- Causa: llamas algo que requiere `T: Trait` y no lo cumple.
- Fix: añade bound (`where T: Trait`), o implementa trait, o cambia tipo.
- Test: `cargo check`

## 8) E0282 — type annotations needed
- Causa: el compilador no puede inferir un tipo genérico.
- Fix: anota tipo (`let x: Vec<i32> = vec![];`) o especifica turbofish (`parse::<i32>()`).
- Test: `cargo check`

## 9) E0432 — unresolved import
- Causa: `use foo::bar` no existe / feature gating / path incorrecto.
- Fix: corrige path; revisa `Cargo.toml` deps/features; revisa `pub mod`.
- Test: `cargo check`

## 10) E0433 — failed to resolve: use of undeclared crate/module
- Causa: intentas usar módulo/crate no disponible en ese scope.
- Fix: `use` correcto; agrega dependencia; corrige `mod`.
- Test: `cargo check`

## 11) E0583 — file not found for module
- Causa: declaraste `mod x;` pero no existe `x.rs` o `x/mod.rs`.
- Fix: crea archivo correcto o ajusta `mod`.
- Test: `cargo check`

## 12) E0603 — module is private
- Causa: intentas acceder a módulo no `pub`.
- Fix: expón API mínima: `pub mod`, `pub(crate) mod`, o re-export selectivo.
- Test: `cargo check`

## 13) E0616 — field is private
- Causa: intentas leer campo privado de struct.
- Fix: agrega getter, cambia visibilidad, o mueve código al módulo.
- Test: `cargo check`

## 14) E0624 — method is private
- Fix igual que campos: API explícita o reubicar código.
- Test: `cargo check`

## 15) E0412 — cannot find type in this scope
- Causa: tipo no importado o no definido.
- Fix: `use`, o define tipo, o califica `crate::path::Type`.
- Test: `cargo check`

## 16) E0425 — cannot find value in this scope
- Causa: nombre fuera de scope / typo.
- Fix: corrige nombre o mueve definición.
- Test: `cargo check`

## 17) E0596 — cannot borrow as mutable
- Causa: variable no es `mut` pero intentas mutarla o pedir `&mut`.
- Fix: `let mut x = ...` o cambia la API a inmutabilidad.
- Test: `cargo check`

## 18) E0507 — cannot move out of borrowed content
- Causa: intentas mover `T` desde `&T`.
- Fix: clona (`T: Clone`), o usa `std::mem::take`/`replace` si tienes `&mut`.
- Test: `cargo check`

## 19) E0716 — temporary value dropped while borrowed
- Causa: tomaste referencia a temporal.
- Fix: guarda el owned en variable antes de referenciar.
- Test: `cargo check`

## 20) E0004 — non-exhaustive patterns
- Causa: `match` no cubre todos los casos.
- Fix: agrega arms faltantes o `_ => ...`.
- Test: `cargo check`

## 21) E0133 — call to unsafe function requires unsafe block
- Causa: llamaste función unsafe sin `unsafe {}`.
- Fix: encapsula unsafe con invariantes documentadas.
- Test: `cargo check`

## 22) E0658 — use of unstable feature
- Causa: usaste feature nightly en stable.
- Fix: elimina feature o migra a alternativa estable; si es requerido, documenta toolchain.
- Test: `cargo check`

## 23) E0107 — wrong number of generic arguments
- Causa: `Foo<T, U>` pero diste otro número.
- Fix: revisa definición y ajusta.
- Test: `cargo check`

## 24) E0119 — conflicting implementations of trait
- Causa: dos impls que se solapan.
- Fix: restringe con nuevos tipos (newtype), o ajusta bounds.
- Test: `cargo check`

## 25) E0207 — the type parameter is not constrained
- Causa: parámetro genérico no aparece donde el compilador lo pueda fijar.
- Fix: úsalo en tipo/función, o remuévelo, o usa PhantomData.
- Test: `cargo check`

## 26) (Clippy) `-D warnings` rompe el build “por cualquier warning”
- Síntoma: CI falla aunque sea warning menor.
- Fix: decide política:
  - “estricto total”: mantener `-D warnings`
  - “estricto solo clippy”: usar `-D clippy::all` o `#![deny(clippy::all)]`
- Test: `cargo clippy ...`
(Clippy documenta que `-D warnings` falla por *cualquier* warning, incluyendo rustc). 

## 27) (Clippy) `cargo clippy --fix` cambia código automáticamente
- Riesgo: arreglos automáticos deben revisarse.
- Fix: permitir `--fix` solo en branches locales; siempre revisar diff.
(Clippy documenta `cargo clippy --fix` y que implica `--all-targets`). 

## 28) (Clippy) categoría `clippy::restriction` no debe activarse completa
- Síntoma: ruido, falsos positivos, contradicciones.
- Fix: habilitar lints selectivos.
(Advertencia está documentada en rust-clippy). 

## 29) (Miri) `cargo miri` solo nightly
- Síntoma: “command only available on nightly”.
- Fix: instalar nightly + componente miri; correr `cargo miri setup` y `cargo miri test`.
(Miri/Cargo documenta nightly + comandos). 

## 30) (Tests) confusion con argumentos de `cargo test`
- Síntoma: flags no aplican como esperas.
- Fix: recuerda el separador `--`: antes va a Cargo, después al test binary.
(Cargo book documenta el separador `--`). 
```

Soporte documental del bloque Clippy/Miri/Tests (lo importante): ([GitHub][3])
(Nota: el error-code index oficial existe y cada E0xxx tiene documentación; ejemplo E0502 está publicado. ([Documentación de Rust][16]))

---

# D) Kata Suite (20 ejercicios con tests) + rúbrica

Diseño: un crate `katas` con 20 funciones stub + tests. El agente debe implementar hasta que `cargo test` esté verde.

## `katas/Cargo.toml`

```toml
[package]
name = "katas"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## `katas/src/lib.rs`

```rust
//! Kata Suite (básico -> intermedio)
//! Implementa las funciones. No cambies los tests.

use std::collections::HashMap;

pub fn kata01_sum(xs: &[i32]) -> i32 {
    todo!()
}

pub fn kata02_count_words(s: &str) -> usize {
    todo!()
}

pub fn kata03_parse_positive_i64(s: &str) -> Result<i64, String> {
    todo!()
}

pub fn kata04_first_nonempty<'a>(xs: &'a [&'a str]) -> Option<&'a str> {
    todo!()
}

pub fn kata05_dedup_sorted_in_place(xs: &mut Vec<i32>) {
    todo!()
}

pub fn kata06_group_by_key(pairs: &[(String, i32)]) -> HashMap<String, Vec<i32>> {
    todo!()
}

pub fn kata07_collect_results<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    todo!()
}

pub fn kata08_find_and_remove(xs: &mut Vec<String>, needle: &str) -> Option<String> {
    todo!()
}

pub fn kata09_median(xs: &mut [i32]) -> Option<f64> {
    todo!()
}

pub fn kata10_histogram_chars(s: &str) -> HashMap<char, usize> {
    todo!()
}

pub fn kata11_safe_div(a: i64, b: i64) -> Result<i64, String> {
    todo!()
}

pub fn kata12_transpose(m: Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, String> {
    todo!()
}

pub fn kata13_unique_preserve_order(xs: &[i32]) -> Vec<i32> {
    todo!()
}

pub fn kata14_map_values<K: Eq + std::hash::Hash, V, U>(
    m: &HashMap<K, V>,
    f: impl Fn(&V) -> U,
) -> HashMap<K, U>
where
    K: Clone,
{
    todo!()
}

pub fn kata15_parse_kv_lines(s: &str) -> Result<HashMap<String, String>, String> {
    todo!()
}

pub fn kata16_two_mut_slices(xs: &mut [i32], mid: usize) -> Result<(&mut [i32], &mut [i32]), String> {
    todo!()
}

pub fn kata17_word_wrap(s: &str, width: usize) -> Vec<String> {
    todo!()
}

pub fn kata18_bounded_push(xs: &mut Vec<i32>, cap: usize, x: i32) -> Result<(), String> {
    todo!()
}

pub fn kata19_top_k(xs: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

pub fn kata20_run_length_encode(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t01_sum() {
        assert_eq!(kata01_sum(&[]), 0);
        assert_eq!(kata01_sum(&[1, 2, 3]), 6);
        assert_eq!(kata01_sum(&[-1, 2, -3]), -2);
    }

    #[test]
    fn t02_count_words() {
        assert_eq!(kata02_count_words(""), 0);
        assert_eq!(kata02_count_words("a"), 1);
        assert_eq!(kata02_count_words("a b  c\n d\t\te"), 5);
    }

    #[test]
    fn t03_parse_positive_i64() {
        assert!(kata03_parse_positive_i64("0").is_err());
        assert!(kata03_parse_positive_i64("-5").is_err());
        assert_eq!(kata03_parse_positive_i64("42").unwrap(), 42);
        assert!(kata03_parse_positive_i64("nope").is_err());
    }

    #[test]
    fn t04_first_nonempty() {
        let xs = ["", "  ", "hi", "there"];
        assert_eq!(kata04_first_nonempty(&xs), Some("hi"));
        let ys = ["", "   "];
        assert_eq!(kata04_first_nonempty(&ys), None);
    }

    #[test]
    fn t05_dedup_sorted_in_place() {
        let mut xs = vec![1, 1, 2, 2, 2, 3, 3];
        kata05_dedup_sorted_in_place(&mut xs);
        assert_eq!(xs, vec![1, 2, 3]);

        let mut ys = vec![];
        kata05_dedup_sorted_in_place(&mut ys);
        assert_eq!(ys, vec![]);
    }

    #[test]
    fn t06_group_by_key() {
        let pairs = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("a".to_string(), 3),
        ];
        let m = kata06_group_by_key(&pairs);
        assert_eq!(m.get("a").unwrap(), &vec![1, 3]);
        assert_eq!(m.get("b").unwrap(), &vec![2]);
    }

    #[test]
    fn t07_collect_results() {
        let ok: Vec<Result<i32, String>> = vec![Ok(1), Ok(2)];
        assert_eq!(kata07_collect_results(ok).unwrap(), vec![1, 2]);

        let bad: Vec<Result<i32, String>> = vec![Ok(1), Err("x".into()), Ok(3)];
        assert_eq!(kata07_collect_results(bad).unwrap_err(), "x");
    }

    #[test]
    fn t08_find_and_remove() {
        let mut xs = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(kata08_find_and_remove(&mut xs, "b"), Some("b".into()));
        assert_eq!(xs, vec!["a".to_string(), "c".to_string()]);
        assert_eq!(kata08_find_and_remove(&mut xs, "z"), None);
    }

    #[test]
    fn t09_median() {
        let mut xs = vec![3, 1, 2];
        assert_eq!(kata09_median(&mut xs), Some(2.0));

        let mut ys = vec![1, 2, 3, 4];
        assert_eq!(kata09_median(&mut ys), Some(2.5));

        let mut zs: Vec<i32> = vec![];
        assert_eq!(kata09_median(&mut zs), None);
    }

    #[test]
    fn t10_histogram_chars() {
        let h = kata10_histogram_chars("ababa");
        assert_eq!(h.get(&'a').copied().unwrap_or(0), 3);
        assert_eq!(h.get(&'b').copied().unwrap_or(0), 2);
    }

    #[test]
    fn t11_safe_div() {
        assert_eq!(kata11_safe_div(10, 2).unwrap(), 5);
        assert!(kata11_safe_div(10, 0).is_err());
    }

    #[test]
    fn t12_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let t = kata12_transpose(m).unwrap();
        assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);

        let bad = vec![vec![1, 2], vec![3]];
        assert!(kata12_transpose(bad).is_err());
    }

    #[test]
    fn t13_unique_preserve_order() {
        assert_eq!(kata13_unique_preserve_order(&[1, 2, 1, 3, 2]), vec![1, 2, 3]);
    }

    #[test]
    fn t14_map_values() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 2);
        m.insert("b".to_string(), 3);
        let out = kata14_map_values(&m, |v| v * 10);
        assert_eq!(out.get("a").copied(), Some(20));
        assert_eq!(out.get("b").copied(), Some(30));
    }

    #[test]
    fn t15_parse_kv_lines() {
        let s = "a=1\nb=two\nc=3";
        let m = kata15_parse_kv_lines(s).unwrap();
        assert_eq!(m.get("a").unwrap(), "1");
        assert_eq!(m.get("b").unwrap(), "two");

        let bad = "nope\nx=1";
        assert!(kata15_parse_kv_lines(bad).is_err());
    }

    #[test]
    fn t16_two_mut_slices() {
        let mut xs = vec![1, 2, 3, 4];
        let (a, b) = kata16_two_mut_slices(&mut xs, 2).unwrap();
        a[0] = 10;
        b[0] = 30;
        assert_eq!(xs, vec![10, 2, 30, 4]);

        assert!(kata16_two_mut_slices(&mut xs, 99).is_err());
    }

    #[test]
    fn t17_word_wrap() {
        let out = kata17_word_wrap("a bb ccc dddd", 3);
        assert_eq!(out, vec!["a bb".to_string(), "ccc".to_string(), "dddd".to_string()]);
    }

    #[test]
    fn t18_bounded_push() {
        let mut xs = vec![1, 2];
        assert!(kata18_bounded_push(&mut xs, 2, 3).is_err());
        assert!(kata18_bounded_push(&mut xs, 3, 3).is_ok());
        assert_eq!(xs, vec![1, 2, 3]);
    }

    #[test]
    fn t19_top_k() {
        assert_eq!(kata19_top_k(&[5, 1, 5, 2, 9], 3), vec![9, 5, 5]);
        assert_eq!(kata19_top_k(&[1, 2], 5), vec![2, 1]);
    }

    #[test]
    fn t20_rle() {
        assert_eq!(kata20_run_length_encode("aaabbc"), "a3b2c1");
        assert_eq!(kata20_run_length_encode(""), "");
        assert_eq!(kata20_run_length_encode("abcd"), "a1b1c1d1");
    }
}
```

### Rúbrica de evaluación (aprobado / fallado)

Aprobado:

* `cargo test` verde (100% tests pasan).
* Sin `clippy` warnings si corres clippy con `-D warnings` (cuando aplique).
* Soluciones idiomáticas: no clonar por defecto; usar referencias; `Result/?` donde corresponde.

Fallado:

* Tests rojos.
* Soluciones “anti-Rust”: clonar todo para “silenciar” borrow checker, o panics en rutas normales.

(El comportamiento y ergonomía de `cargo test`/tests está documentado en Rust Book + Cargo Book). ([Documentación de Rust][17])

---

# F) Checklist final “Definition of Done” (para tareas Rust del agente)

Cópialo como `docs/ai/DEFINITION_OF_DONE.md` o como parte de tus rules.

```md
# Definition of Done (Rust Agent)

## Evidencia obligatoria
- [ ] `./tools/verify.sh` ejecutado y GREEN
- [ ] Output relevante pegado o resumido (líneas clave: fmt/clippy/test)
- [ ] Diff claro: qué archivos cambiaron y por qué

## Calidad técnica
- [ ] No hay warnings (Clippy con `-D warnings` si es política del repo)
- [ ] Tests cubren el cambio (unit o integration o doctest)
- [ ] Error handling idiomático: `Result` + `?` donde aplique (sin `unwrap` en producción)

## Aprendizaje
- [ ] Si apareció un error nuevo: registrado en `docs/ai/ERROR_PATTERNS.md`
- [ ] Si se tomó una decisión irreversible: registrada en `docs/ai/DECISIONS.md`

## Excepciones
- [ ] Si se omitió algo (p.ej. miri/fuzz): existe entrada en DECISIONS con responsable + fecha + razón.
```

---

## Lo que NO conviene creer (punto ciego típico)

* No existe “prompt perfecto” que haga a un agente escribir Rust sin errores **sin** un juez (compilador/tests/lints). Lo que sí existe es un sistema donde el agente **no tiene salida**: o converge a `verify green` o no pasa.

---

## Siguiente paso físico (para que esto se ejecute hoy)

1. En tu repo, crea:

   * `./.agent/rules/`
   * `./.agent/workflows/`
   * `./tools/verify.sh` (y dale permisos: `chmod +x tools/verify.sh`)
   * `./docs/ai/` con los 3 md (playbook, error patterns, decisions)
2. En Antigravity:

   * activa **Review-driven development** si quieres control (recomendado por el codelab) ([Google Codelabs][1])
   * set Terminal a **Auto** (entrenamiento) o **Off + Allow List** (crítico) ([Google Codelabs][1])
3. Corre: `./tools/verify.sh` y usa eso como tu “semáforo”.

Si quieres, pega aquí el contenido de tu `Cargo.toml` y la estructura del repo (árbol de carpetas). Te ajusto `verify.sh` para tu caso (workspace, features, bins, etc.) sin introducir gates que te rompan el flujo.

[1]: https://codelabs.developers.google.com/getting-started-google-antigravity "https://codelabs.developers.google.com/getting-started-google-antigravity"
[2]: https://github.com/rust-lang/rustfmt "https://github.com/rust-lang/rustfmt"
[3]: https://github.com/rust-lang/rust-clippy "https://github.com/rust-lang/rust-clippy"
[4]: https://doc.rust-lang.org/cargo/commands/cargo-test.html "https://doc.rust-lang.org/cargo/commands/cargo-test.html"
[5]: https://doc.rust-lang.org/beta/cargo/commands/cargo-miri.html "https://doc.rust-lang.org/beta/cargo/commands/cargo-miri.html"
[6]: https://doc.rust-lang.org/beta/book/ch04-01-what-is-ownership.html "https://doc.rust-lang.org/beta/book/ch04-01-what-is-ownership.html"
[7]: https://doc.rust-lang.org/beta/book/ch10-03-lifetime-syntax.html "https://doc.rust-lang.org/beta/book/ch10-03-lifetime-syntax.html"
[8]: https://doc.rust-lang.org/book/ch10-00-generics.html "https://doc.rust-lang.org/book/ch10-00-generics.html"
[9]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html "https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html"
[10]: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html "https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html"
[11]: https://docs.rs/cargo-audit/latest/cargo_audit/index.html "https://docs.rs/cargo-audit/latest/cargo_audit/index.html"
[12]: https://github.com/EmbarkStudios/cargo-deny "https://github.com/EmbarkStudios/cargo-deny"
[13]: https://mozilla.github.io/cargo-vet/config.html "https://mozilla.github.io/cargo-vet/config.html"
[14]: https://anssi-fr.github.io/rust-guide/04_language.html "https://anssi-fr.github.io/rust-guide/04_language.html"
[15]: https://doc.rust-lang.org/cargo/commands/cargo-build.html "https://doc.rust-lang.org/cargo/commands/cargo-build.html"
[16]: https://doc.rust-lang.org/nightly/nomicon/ "https://doc.rust-lang.org/nightly/nomicon/"
[17]: https://doc.rust-lang.org/stable/book/ch11-00-testing.html "https://doc.rust-lang.org/stable/book/ch11-00-testing.html"
