# Definition of Done (Rust Agent)

Una tarea Rust no está “Done” hasta que exista evidencia objetiva.

## Evidencia obligatoria (siempre)

1) **Comandos ejecutados** (copiados tal cual):
   - `./scripts/verify.sh` (o `./scripts/verify.sh --fast` durante iteración)
2) **Output relevante**:
   - fragmento que muestre éxito: fmt OK, clippy OK, tests OK.
3) **Diff**:
   - `git diff` (o PR diff) mostrando cambios.
4) **Resumen factual**:
   - qué cambió y por qué.
5) **Referencia a memoria del repo**:
   - si arreglaste un error recurrente: link/entrada en `ERROR_PATTERNS.md`.
   - si tomaste una decisión: entrada en `DECISIONS.md`.
   - si hiciste bypass: entrada en `EXCEPTIONS.md`.

## Criterios de aprobación

- `./scripts/verify.sh` termina en **exit code 0**.
- No hay warnings en clippy (tratados como error).
- `cargo fmt` pasa en modo check.
- Tests pasan.

## Criterios de falla

- “Funciona en mi cabeza”.
- No hay evidencia de ejecución.
- Se salta el verificador sin excepción aprobada.

