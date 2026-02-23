# EXCEPTIONS

Este archivo controla bypasses. Si no está aquí, no existe.

## Reglas

1) Toda excepción debe incluir:
   - ID único
   - Alcance (archivo/módulo)
   - Justificación
   - Riesgo
   - Aprobador humano
   - Fecha de expiración
2) Si expira, debe eliminarse o renovarse con nueva aprobación.

---

## Plantilla

- Exception ID: EXC-YYYY-MM-DD-XXX
- Scope:
- What is bypassed:
- Why:
- Risk:
- Approved by:
- Expires:
- Links: (PR, issue)

---

## Ejemplo

- Exception ID: EXC-2026-02-23-001
- Scope: `src/parser.rs`
- What is bypassed: uso de `unwrap()` en ruta imposible según invariantes
- Why: micro-opt y simplificación; medido.
- Risk: `panic!` si invariante se rompe
- Approved by: @tech-lead
- Expires: 2026-03-23
- Links: PR #123

