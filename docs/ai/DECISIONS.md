# DECISIONS

Registro de decisiones técnicas. La IA debe consultarlo antes de imponer un estilo “nuevo”.

## Plantilla

- Date: YYYY-MM-DD
- Decision:
- Context:
- Rationale:
- Tradeoffs:
- Status: Proposed | Accepted | Deprecated

---

## 2026-02-23 — Verificación única (`verify.sh`) como gate

- Decision: `./tools/verify.sh` es la única fuente de verdad para “verde”.
- Context: Evitar que el agente elija combinaciones parciales de checks.
- Rationale: Un gate único reduce bypass y hace medible el ciclo.
- Tradeoffs: Puede ser más lento; se habilita `--fast` para iteración.
- Status: Accepted

## 2026-02-23 — Clippy en CI con warnings como error

- Decision: `cargo clippy ... -- -D warnings` en `verify.sh`.
- Context: warnings recurrentes tienden a convertirse en deuda.
- Rationale: Consistencia y reducción de bugs triviales.
- Tradeoffs: Aumenta fricción inicial; exige disciplina.
- Status: Accepted

## 2026-02-23 — Miri como check opcional

- Decision: Miri corre solo si `RUN_MIRI=1`.
- Context: Requiere nightly y puede ser lento.
- Rationale: Detecta UB en `unsafe`; útil pero no siempre disponible.
- Tradeoffs: Menos cobertura por defecto.
- Status: Accepted

