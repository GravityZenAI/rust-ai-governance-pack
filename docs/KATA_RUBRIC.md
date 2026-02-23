# Kata Rubric

## Aprobar

- Tests verdes:
  - `cd training/kata_suite && cargo test`
- Formato:
  - `cargo fmt -- --check` (o `./scripts/verify.sh` desde root del repo principal)
- Lints:
  - `cargo clippy -- -D warnings`

## Fallar

- Cualquier error de compilación.
- Cualquier test fallido.
- Cualquier warning de clippy (tratado como error).
- Diferencias de formato en fmt check.

## Scoring sugerido (para medir convergencia)

Por kata:
- `iterations_to_green` = número de ejecuciones de `cargo test` hasta verde.
- `time_to_green` = minutos hasta verde.
- `new_error_patterns` = cuántas entradas nuevas se añadieron.

A nivel suite:
- `kata_pass_rate` = katas verdes / 20.

