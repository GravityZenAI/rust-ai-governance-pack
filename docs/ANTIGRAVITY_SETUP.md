# Integración en Google Antigravity IDE

> Nota: este repo incluye una configuración basada en el modelo de **Rules / Workflows / Skills**.

## 1) Dónde viven las customizaciones

### Workspace (en el repo)
- Rules: `.agent/rules/`
- Workflows: `.agent/workflows/`
- Skills: `.agent/skills/`

### Global (en tu máquina)
- Rules globales: `~/.gemini/GEMINI.md`
- Workflows globales: `~/.gemini/antigravity/global_workflows/`
- Skills globales: `~/.gemini/antigravity/global_skills/`

## 2) Qué hace cada cosa

- **Rules**: guardrails siempre activos (cómo debe operar el agente).
- **Skills**: módulos especializados cargados bajo demanda (ej. loop Rust).
- **Workflows**: recetas activas invocables (ej. /verify).

## 3) Settings recomendados (pragmático)

- Modo de desarrollo: preferir el modo que fuerza revisión (review-driven) si tu equipo lo usa.
- Terminal execution:
  - Si priorizas velocidad: permitir ejecución automática **pero** con denylist de comandos destructivos.
  - Si priorizas control: requerir aprobación por comando.

## 4) Extensiones

- Instala el mínimo necesario.
- Evita extensiones no auditadas; trata el ecosistema de extensiones como supply-chain.

## 5) Cómo operar

- Para cambios en Rust: usar la Skill `rust-compile-loop`.
- Antes de cerrar: ejecutar `/verify` (workflow) o `./tools/verify.sh` en terminal.

