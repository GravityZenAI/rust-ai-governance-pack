# Arquitectura final

Objetivo: que una IA en Google Antigravity trabaje Rust con **competencia operativa real**:

- Converge rápido a **verde** (compila + tests + clippy + fmt).
- Reduce **errores repetidos** (registrando patrones y decisiones).
- Mantiene consistencia en refactors/estilo.

Métrica base (por PR / por sesión):
- `verify green rate` (porcentaje de ejecuciones exitosas de `verify.sh`).
- `kata pass rate` (porcentaje de katas resueltas sin intervención humana).
- `time-to-green` (tiempo/iteraciones desde “rojo” a “verde”).

---

## 1) Reglas permanentes (siempre activas)

Ubicación: `.agent/rules/`

Estas reglas son **guardrails**: no son “consejos”; son condiciones de operación.

### R0 — Compilador como juez
- No se acepta “debería compilar”. La IA **debe** correr el compilador y corregir hasta `cargo check`/`cargo test` verde.
- Prohibido cerrar una tarea sin evidencia de ejecución.

### R1 — Loop obligatorio
Para cualquier cambio Rust:
1) **Especificación** breve (inputs/outputs, invariantes, casos borde).
2) Implementación **incremental** (pasos pequeños).
3) `./scripts/verify.sh` (o `./scripts/verify.sh --fast` durante iteración).
4) Registrar patrones nuevos en `ERROR_PATTERNS.md`.

### R2 — Cero advertencias en CI
- En CI, `clippy` se ejecuta con warnings como error.
- `fmt` se valida con `--check`.

### R3 — Prohibiciones “de calidad” por defecto
- No introducir `unsafe` sin justificación y sin test específico.
- Evitar `unwrap()`/`expect()` en producción (permitidos solo con excepción explícita).
- Evitar indexing directo (`vec[i]`) si hay riesgo de out-of-range.

### R4 — Memoria del repo siempre actualizada
- Cambios en convenciones y excepciones **se registran** en `DECISIONS.md` / `EXCEPTIONS.md`.

---

## 2) Skills modulares (cargables por situación)

Ubicación: `.agent/skills/`

Las Skills son “módulos de operación” para reducir ambigüedad. Ejemplos incluidos:

- `rust-compile-loop`: implementación incremental + cómo usar el compilador.
- `rust-error-triage`: diagnóstico sistemático de errores rustc/clippy.
- `rust-refactor-safely`: refactors guiados por tests.
- `rust-kata-coach`: entrenamiento con `training/kata_suite`.

**Principio:** la IA no improvisa el proceso; selecciona una Skill y ejecuta su checklist.

---

## 3) Workflows invocables

Ubicación: `.agent/workflows/`

Workflows = secuencias activas.

Incluidos:
- `/verify`: ejecuta verificación y produce reporte.
- `/kata`: elige un ejercicio, corre tests y evalúa.
- `/log-decision`: agrega una entrada en `DECISIONS.md`.

---

## 4) Verificador único + CI gates

### `scripts/verify.sh`
Un solo script como fuente de verdad:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- opcional: `cargo miri test` (nightly) si `RUN_MIRI=1`
- opcional: `cargo deny check` / `cargo audit` si están instalados

### CI gate
- CI debe llamar `./scripts/verify.sh`.
- Se bloquea merge si falla.

---

## 5) Memoria persistente del repo

Archivos canónicos:
- `RUST_PLAYBOOK.md`: “cómo se hace Rust aquí”.
- `ERROR_PATTERNS.md`: biblioteca de errores y fixes.
- `DECISIONS.md`: decisiones con fecha, razón y tradeoffs.

---

## 6) Protocolo de excepción

Archivo: `EXCEPTIONS.md`

Bypass permitido solo si:
- queda documentado con **razón**, **alcance**, **expiry**, **aprobador humano**.
- no rompe CI (o CI se ajusta explícitamente con decisión registrada).

