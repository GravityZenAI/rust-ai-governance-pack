# Auditoría (web) del pack y la investigación: Antigravity + Rust (Feb 2026)

Este reporte resume:
- Qué partes del documento original y del governance pack están bien alineadas con fuentes externas.
- Huecos / riesgos.
- Correcciones concretas (tooling/CI/seguridad).
- Un blueprint “mejorado” para gobernar una IA que escribe Rust con alta seguridad.

> Nota: Este reporte está pensado para leerse dentro del repo (o como referencia) y no depende del histórico de la conversación.

---

## 1) Veredicto: lo sólido del enfoque

El núcleo del sistema está bien planteado:
- Rust se “domina” operativamente no por “saber teoría”, sino por un **loop verificable**: `fmt` → `clippy` → `test` → (si hay `unsafe`) `miri` → supply-chain (`audit/deny/vet`) → (si aplica) fuzzing.
- Antigravity se presta a esto porque permite:
  - reglas y workflows por workspace,
  - ejecución de comandos de terminal con políticas de revisión,
  - y controles de seguridad como deny-lists / sandbox (según plataforma y settings).

---

## 2) Huecos críticos detectados (y por qué importan)

### 2.1 CI roto cuando aparece `unsafe`
Problema típico: si `unsafe` aparece y el pipeline exige Miri, **CI debe instalar nightly + miri y correr `cargo miri setup`**; si no, fallará por infraestructura, no por el código.

### 2.2 Reproducibilidad: falta `--locked` + política clara de `Cargo.lock`
Sin `--locked`, CI puede fallar por cambios externos (resolución de dependencias). Para un agente, esto es veneno: el agente “persigue fantasmas” que no vienen del commit.

### 2.3 Seguridad del entorno: extensiones y recomendadores
Hay reportes recientes de **riesgo supply-chain** en forks de VS Code cuando recomiendan extensiones inexistentes (y un atacante puede ocupar esos nombres en Open VSX). Esto es un riesgo real si el flujo permite instalar extensiones “porque el IDE lo sugirió”.

### 2.4 “Strict mode / Sandbox / Request review”: términos y comportamiento varían
Tu doc usa términos que existen, pero el comportamiento depende de:
- plataforma (por ejemplo, sandboxing en macOS primero),
- versión (features agregadas en releases recientes),
- y settings (p. ej. Strict Mode fija sandbox ON y network OFF).

### 2.5 Supply chain: audit/deny bien… pero falta “social proof”
`cargo-audit`/`cargo-deny` detectan advisories/licencias, pero no prueban que una dependencia sea “segura”. Para proyectos serios conviene sumar `cargo-vet` (auditoría de dependencias con criterios).

### 2.6 Unsafe en Rust 2024: falta endurecer “unsafe_op_in_unsafe_fn”
En Rust 2024, el lint `unsafe_op_in_unsafe_fn` avisa por defecto: obliga a marcar con bloques `unsafe {}` las operaciones unsafe incluso dentro de `unsafe fn`. Si quieres que la IA sea consistente, conviene fijarlo como regla.

---

## 3) Correcciones concretas recomendadas

### 3.1 Verifier (local)
- Agregar `--locked` cuando exista `Cargo.lock` (y preferir requerirlo).
- Si hay `unsafe`, correr:
  - `cargo +nightly miri setup`
  - `cargo +nightly miri test ...`
- Excluir `target/` y `.git/` del detector de `unsafe`.

### 3.2 Workflow CI
- Instalar toolchain estable (fmt/clippy) y toolchain nightly (miri).
- Instalar `cargo-audit`/`cargo-deny` con `--locked`.
- Correr `./tools/verify.sh`.

### 3.3 Antigravity opsec (reglas)
- Terminal en “request review” por defecto.
- Sandbox ON (cuando disponible) + network OFF por defecto.
- Deny-list de comandos destructivos/exfiltración.
- No instalar extensiones sugeridas sin verificar publisher/namespace.

### 3.4 Supply chain extra (opcional pero fuerte)
- Adoptar `cargo-vet` (requiere `supply-chain/`).
- Considerar `cargo-auditable` si vas a auditar binarios en producción.

---

## 4) Canon de fuentes (las “4 columnas”)

1) The Rust Book (canonical): https://doc.rust-lang.org/book/
2) The Rust Reference: https://doc.rust-lang.org/reference/
3) Rustonomicon (unsafe): https://doc.rust-lang.org/nightly/nomicon/
4) ANSSI Secure Rust Guidelines: https://github.com/ANSSI-FR/rust-guide

Complementos operativos:
- Miri: https://github.com/rust-lang/miri
- RustSec / advisory-db: https://rustsec.org/ | https://github.com/RustSec/advisory-db
- cargo-deny: https://github.com/EmbarkStudios/cargo-deny
- cargo-vet: https://github.com/mozilla/cargo-vet
- cargo-fuzz / Rust Fuzz Book: https://github.com/rust-fuzz/cargo-fuzz | https://rust-fuzz.github.io/book/
- proptest: https://github.com/proptest-rs/proptest

---

## 5) Entregable
- Pack actualizado: `antigravity-rust-governance-pack-v2.zip` (ver adjunto en el chat).

