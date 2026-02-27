# Plan de ejecución

Este plan asume que quieres un sistema “difícil de eludir” y medible.

## Fase 0 — Bootstrapping (30–60 min)

1) Copia este template al repo objetivo.
2) En Antigravity:
   - habilita Rules/Workflows/Skills desde el workspace (`.agent/`).
   - configura políticas de ejecución (ver `ANTIGRAVITY_SETUP.md`).
3) Ejecuta `./tools/verify.sh` una vez (solo para validar toolchain).

**Salida:** Antigravity ya ve las reglas y existe un verificador único.

## Fase 1 — CI y gates (1–2 h)

1) Copia `.github/workflows/ci.yml` (o adapta a tu CI actual).
2) Define status checks requeridos (branch protection).
3) Asegura que el gate es **solo** `./tools/verify.sh`.

**Criterio de éxito:** ninguna rama mergea sin `verify green`.

## Fase 2 — Normalizar estilo y lints (2–4 h)

1) Decide el nivel de severidad de Clippy:
   - mínimo: `-D warnings`
   - estricto: además negar lints de anti-patrones (documentado).
2) Ajusta `rustfmt.toml` (si necesitas).
3) Registra la decisión en `DECISIONS.md`.

**Criterio de éxito:** PRs sin “ruido” de formato; warnings no llegan a main.

## Fase 3 — Biblioteca de patrones (continuo, 1–2 h inicial)

1) Mantén `ERROR_PATTERNS.md` como “tabla de verdad”.
2) Regla operativa:
   - si aparece un error/lint 2 veces en 7 días → crear/actualizar entrada.
3) Cada entrada debe incluir:
   - causa raíz
   - fix canónico
   - snippet mínimo
   - test mínimo

**Criterio de éxito:** disminuye la repetición de errores (métrica semanal).

## Fase 4 — Entrenamiento (día 1 y luego mantenimiento)

1) Corre katas:
   ```bash
   cd training/kata_suite
   cargo test
   ```
2) El agente solo aprueba si:
   - tests verdes
   - `cargo clippy`/`cargo fmt --check` verdes
3) Registra métricas:
   - intentos por kata
   - tiempo a verde

**Criterio de éxito:** el agente alcanza >80% katas verdes sin intervención.

## Fase 5 — Mejora continua (operación)

Cadencia recomendada (semanal):
- revisar `DECISIONS.md` y eliminar excepciones expiradas.
- priorizar nuevas katas según los errores más frecuentes.
- revisar cambios de toolchain (edition/lints) y ajustar `verify.sh`.

