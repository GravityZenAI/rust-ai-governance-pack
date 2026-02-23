Hallazgos (auditoría) — lo que está bien, lo que está mal, lo que faltaba

* **El núcleo del sistema (loop “compilador como juez”) es correcto y está respaldado por evidencia.** El enfoque de “iterar con mensajes de error del compilador y reintentar” está directamente alineado con *RustAssistant* (ICSE 2025 / MSR), que reporta una tasa alta de fixes correctos cuando se usa ese loop de manera sistemática (pico ~74%). ([Microsoft][1])
* **Tus “gates” (fmt + clippy + tests) son la dirección correcta**, pero faltaba convertirlo en un **único verificador** reproducible y obligatorio. En particular:

  * `cargo fmt` en CI típicamente se valida como `cargo fmt --all -- --check`. ([GitHub][2])
  * `cargo clippy` en CI se endurece con `-D warnings` para romper el build con warnings. ([Documentación de Rust][3])
* **La parte de Antigravity estaba conceptualmente bien, pero tus citas no eran verificables.** Antigravity soporta customizaciones por **workspace** en `.agent/rules/`, `.agent/workflows/`, `.agent/skills/` y también rutas **globales** bajo `~/.gemini/...`. ([Google Codelabs][4])
* **Miri / UB-check:** correcto incluirlo, pero debe ser **opcional** (requiere nightly y puede ser lento). El flujo real es `cargo miri test` a través de `cargo-miri`. ([GitHub][5])
* **Supply-chain / advisories:** es correcto mencionar RustSec y herramientas de auditoría. RustSec mantiene la advisory database (base para tooling como cargo-audit). ([RustSec][6])

  * `cargo-deny` existe y cubre checks (incluyendo licencias/otras políticas según configuración). ([Embark Studios][7])
* **Errores frecuentes:** tu lista (E0382/E0502/E0597/…) es consistente con errores típicos en Rust y con los códigos que el propio RustAssistant menciona como comunes al reparar builds. ([Microsoft][8])
* **Lo que faltaba para que tu doc se convierta en un sistema “difícil de eludir”:**

  1. Un `verify.sh` real + CI gate que lo ejecute (si no, el agente “declara victoria”).
  2. Un paquete de **Rules/Workflows/Skills** listo en `.agent/` (para que Antigravity lo cargue). ([Google Codelabs][4])
  3. Memoria persistente “de repo” (playbook/patrones/decisiones/excepciones) con plantillas y enforcement.
  4. Un set de entrenamiento con tests (katas) que obligue a pasar por el compilador.

Lo que NO viste (riesgos ocultos que te van a romper el sistema)

* **Bypass #1 (el más común):** el agente optimiza por “texto convincente”, no por “verde real”. Sin un gate único (verify) + CI obligatorio + plantilla de PR con evidencia, el sistema se degrada.
* **Bypass #2:** “habilitar más herramientas” sin control hace que el agente cambie de método cada vez. Necesitas **una sola ruta** (verify) y el resto como checks opcionales con flags (Miri, audits). ([Documentación de Rust][9])
* **Bypass #3:** sin `ERROR_PATTERNS.md` y `DECISIONS.md`, el agente repite los mismos errores (esto es lo que *RustAssistant* explota: iteración sistemática + patrones). ([Microsoft][8])

Tu siguiente paso físico (no negociable)

1. Descarga el template y descomprímelo en el root de tu repo Rust.
2. Abre Antigravity y asegúrate de que el workspace está usando `.agent/` (rules/workflows/skills). ([Google Codelabs][4])
3. Ejecuta el gate local una vez:

```bash
./scripts/verify.sh
```

4. Empieza el entrenamiento (katas):

```bash
cd training/kata_suite
cargo test
```

Entrega: todos los archivos listos (A–F) + plan de ejecución
[Download el template completo (zip)](sandbox:/mnt/data/rust-antigravity-agent-template.zip)

Árbol de archivos (lo que estás recibiendo)

* **A) Arquitectura final:** `ARCHITECTURE.md`
* **Plan de ejecución:** `EXECUTION_PLAN.md`
* **B) Playbook Rust:** `RUST_PLAYBOOK.md`
* **C) Error Pattern Library (Top 30):** `ERROR_PATTERNS.md`
* **D) Kata Suite (20 ejercicios con tests):** `training/kata_suite/` + `KATA_RUBRIC.md`
* **E) Integración Antigravity:** `ANTIGRAVITY_SETUP.md` + `.agent/*`
* **F) Definition of Done:** `DEFINITION_OF_DONE.md` + `.github/pull_request_template.md`
* Verificador único: `scripts/verify.sh`
* CI gate: `.github/workflows/ci.yml`

A) Arquitectura final (1–2 páginas)
Archivo: `ARCHITECTURE.md` (ya creado en el zip).
Incluye: rules permanentes, skills modulares, workflows, verify.sh + CI, memoria del repo, protocolo de excepción.

B) Playbook Rust mínimo (canónico)
Archivo: `RUST_PLAYBOOK.md`.
Incluye: ownership/borrowing, lifetimes, traits/generics, manejo de errores, módulos, concurrencia, anti‑patrones y reemplazos, y el hábito de usar `rustc --explain` y el índice oficial de errores. ([Documentación de Rust][10])

C) Error Pattern Library (Top 30)
Archivo: `ERROR_PATTERNS.md`.
Incluye 30 patrones (E0xxx + lints) con causa raíz, fix canónico, repro mínimo y test mínimo, orientado a los códigos comunes reportados en literatura y al índice oficial. ([Microsoft][8])

D) Kata Suite (entrenamiento)
Directorio: `training/kata_suite/`

* 20 ejercicios con `todo!()` + tests en `tests/katas.rs`.
* Rubric: `KATA_RUBRIC.md`.

E) Integración en Antigravity IDE
Archivo: `ANTIGRAVITY_SETUP.md` + carpetas `.agent/`.
Incluye rutas oficiales de workspace/global para rules/workflows/skills y postura de settings (con tradeoff velocidad vs control). ([Google Codelabs][4])

F) Checklist “Definition of Done”
Archivo: `DEFINITION_OF_DONE.md` + PR template que fuerza evidencia.

Notas explícitas de verificación (lo que NO pude cerrar al 100% con fuentes dentro del material consultado)

* **ANSSI (Rust) y cargo-vet:** tu requerimiento los menciona; en el set de fuentes recuperado aquí no entraron páginas primarias específicas para ambos (por tanto no los “certifico” en este paquete). El template deja la puerta abierta vía la skill `rust-security-audit`, con RustSec/cargo-audit y cargo-deny ya respaldados por fuentes. ([RustSec][6])

Si quieres que lo convierta en “imposible de esquivar” al máximo dentro de Antigravity, el ajuste adicional (sin discusión) es:

* CI obligatorio con `./scripts/verify.sh` como único gate.
* PR template obligatorio con evidencia.
* Regla: “sin output no hay done”.

[1]: https://www.microsoft.com/en-us/research/publication/fixing-rust-compilation-errors-using-llms/ "https://www.microsoft.com/en-us/research/publication/fixing-rust-compilation-errors-using-llms/"
[2]: https://github.com/rust-lang/rustfmt "https://github.com/rust-lang/rustfmt"
[3]: https://doc.rust-lang.org/stable/clippy/usage.html "https://doc.rust-lang.org/stable/clippy/usage.html"
[4]: https://codelabs.developers.google.com/getting-started-google-antigravity "https://codelabs.developers.google.com/getting-started-google-antigravity"
[5]: https://github.com/rust-lang/miri "https://github.com/rust-lang/miri"
[6]: https://rustsec.org/ "https://rustsec.org/"
[7]: https://embarkstudios.github.io/cargo-deny/cli/init.html "https://embarkstudios.github.io/cargo-deny/cli/init.html"
[8]: https://www.microsoft.com/en-us/research/uploads/prod/2024/08/paper.pdf "https://www.microsoft.com/en-us/research/uploads/prod/2024/08/paper.pdf"
[9]: https://doc.rust-lang.org/beta/cargo/commands/cargo-miri.html "https://doc.rust-lang.org/beta/cargo/commands/cargo-miri.html"
[10]: https://doc.rust-lang.org/stable/error_codes/ "https://doc.rust-lang.org/stable/error_codes/"
