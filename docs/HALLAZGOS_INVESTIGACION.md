# Hallazgos clave

- **Bucle iterativo compilador–IA:** Estudios recientes muestran que usar los errores de compilación como “profesor” en un ciclo iterativo con un LLM mejora drásticamente la corrección del código. Por ejemplo, *RustAssistant* (ICSE 2025) extrae el mensaje de error y el contexto relevante, envía esto al LLM, aplica la corrección sugerida y vuelve a compilar hasta que pase【13†L334-L343】【13†L348-L354】. Este enfoque incremental logra ~74% de fixes correctos en pruebas reales, validando el esquema “implementación → compilador como juez → tests/linters → iterar”【13†L334-L343】.  

- **Herramientas de verificación en pipeline CI:** Se recomienda integrar *Cargo* con pruebas y linters en el pipeline CI. En la práctica, se suelen encadenar comandos como `cargo test && cargo clippy && cargo fmt` para cubrir pruebas funcionales, análisis estático y formato coherente【45†L168-L174】. Un ejemplo de CI con GitHub Actions corre en paralelo `cargo build`, `cargo test`, `cargo fmt --check` y `cargo clippy -- -D warnings` (fallando ante cualquier advertencia) en cada push/PR【56†L494-L502】【56†L511-L520】. Este flujo reduce errores de compilación repetidos y asegura consistencia de estilo.  

- **Linter agresivo para anti-patrones:** Clippy permite prohibir prácticas riesgosas que causan bugs comunes. Es habitual **denegar** lints como `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `unreachable`, etc., forzando un manejo explícito de `Option/Result`【77†L79-L88】. Por ejemplo, reescribir `.unwrap()` con `match` o `?`, y evitar acceso fuera de rango con índices directos. Esta política entrena a usar código idiomático y previene fallos en tiempo de ejecución【77†L79-L88】.  

- **Errores de Rust frecuentes:** Datos de JetBrains (RustRover) muestran que los errores más comunes involucran tipos, traits y gestión de memoria. Entre ellos destacan E0277 (falta impl. de trait), E0308 (tipo esperado distinto), E0382 (uso tras _move_), E0502/E0507/E0597 (préstamos inválidos) y varios “nombre no resuelto” (E0425, E0432, E0433)【69†L152-L161】【71†L298-L307】. Estos errores guían la biblioteca de patrones: cada código (E0xxx) debe documentarse con su causa raíz y solución canónica.  

- **Seguridad y dependencias:** Aunque secundaria, la seguridad se incluye: la base de datos RustSec y `cargo-audit` detectan vulnerabilidades conocidas en `Cargo.lock`【23†L15-L23】; `cargo-deny` amplía el análisis a licencias, fuentes y versiones duplicadas【63†L36-L40】; y `cargo-vet` verifica que cada dependencia esté auditada por fuentes confiables【27†L41-L49】. Estas herramientas se ejecutan en CI para evitar dependencias problemáticas. ANSSI también ofrece guías de prácticas seguras en Rust (p.ej. evitar `unsafe` innecesario o validación deficiente), que pueden incorporarse al *Playbook*【61†L61-L70】.

- **Antigravity Rules, Skills y Workflows:** Según la documentación de Antigravity, las **Rules** son restricciones pasivas siempre activas (guardrails) y se definen en `.agent/rules/`【39†L251-L259】. Las **Skills** son módulos especializados cargados bajo demanda cuando el agente detecta una tarea relevante【39†L261-L268】. Las **Workflows** son secuencias activas invocadas manualmente. Por ejemplo, una regla global puede requerir usar una Skill de verificación segura antes de cualquier commit, asegurando que el agente no omita pasos críticos【39†L274-L282】. Este diseño modular favorece la consistencia y dificulta bypass.

- **Riesgos de extensiones IDE:** Hay evidencia de que extensiones de VSCode (incluso forks AI) pueden filtrar secretos o entregar código malicioso a través de actualizaciones automáticas【82†L39-L47】. Por tanto, la configuración recomendada es minimizar extensiones no auditadas. Se sugiere operar principalmente en **terminal** y limitadas herramientas de IDE, para reducir la superficie de ataque. Si se usan extensiones, deben ser oficiales y de alta confianza【82†L39-L47】.  

---

## A) Arquitectura final

- **Reglas permanentes (“guardrails”)**: Definir en `.agent/rules/` reglas globales que *siempre* se apliquen. Ejemplos: “ANTES de cualquier commit en Rust, ejecutar `verify.sh`”, “nunca usar `.unwrap()` sin justificar” o “usar siempre `cargo fmt` y `clippy` en cada iteración”. Según la guía oficial, las rules se inyectan en el prompt del sistema y gobiernan el *cómo* de cada tarea【39†L251-L259】. Al ser **siempre activas**, evitan que el agente omita pasos críticos.

- **Skills modulares**: Crear Skills especializadas por contexto, ubicadas en `<repo>/.agent/skills/`. Por ejemplo, una `rust-compile` skill con instrucciones para compilar, otra `rust-test-lint` para ejecutar tests y linters, y una `rust-refactor` para tareas de refactorización. Las Skills solo se cargan cuando el agente identifica la intención adecuada【39†L261-L268】. Esto previene “tool bloat” y mantiene el contexto ligero. Un skill puede incluir scripts (Bash/Python) para automatizar pasos como `cargo clippy -- -D warnings` o `cargo fmt`.

- **Verificador único (`verify.sh`) + CI gates**: En el repositorio, incluir un script como `verify.sh` que ejecute localmente todos los checks (build, test, clippy, fmt, miri). Por ejemplo:
  ```bash
  #!/usr/bin/env bash
  set -e
  cargo check
  cargo test
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo +nightly miri test  # (si se usan pruebas de Miri)
  ```
  Este script se integra en la CI (por ejemplo, GitHub Actions) como *gate*. Un flujo recomendado (ver configuración de ejemplo) ejecuta jobs para **build**, **test**, **fmt**, **clippy** y documentación【56†L494-L502】【56†L511-L520】. Así, cada PR debe pasar `verify.sh` antes de mergear (“status checks”).

- **Memoria persistente en repo**: Mantener documentación viva en archivos del repositorio:
  - `RUST_PLAYBOOK.md`: Resumen de buenas prácticas (ownership, lifetimes, estilo, async recomendado, anti-patrones).  
  - `ERROR_PATTERNS.md`: Catálogo de patrones de error comunes (códigos E0xxx y lints), su causa y arreglo.  
  - `DECISIONS.md`: Registro de decisiones de diseño (por ejemplo, por qué se configura clippy de cierta forma).  
  Estos archivos versionados actúan como memoria del equipo/IA, ayudando a mantener consistencia y facilitar la onboarding. Son análogos a “wiki” de equipo, pero controlados en texto plano.  

- **Protocolo de excepción**: Definir formalmente qué bypasses están permitidos y quién los autoriza. Por ejemplo, permitir `unwrap()` solo si un ingeniero senior lo valida en un PR. Documentar esto en `EXCEPTIONS.md` o similar. Idealmente, todo bypass exige la aprobación manual de un humano (el “owner” o líder técnico). Antigravity mismo no fuerza autorizaciones, por lo que el protocolo es *organizacional*: exponerlo en la documentación y, en la regla global, indicar algo como “Si se requiere saltarse un paso, obtener firma de xxx”. Esto asegura que el agente no eluda la cadena (p.ej. no ignorar tests salvo aprobación).

> *Citas:* Los principios de Antigravity (Rules/Skills/Workflows) se detallan en la documentación de Google. Por ejemplo, se sugiere usar Rules para acciones que el agente **debe** seguir siempre【39†L251-L259】, y Skills para capacidades activadas dinámicamente al identificar la intención del usuario【39†L261-L268】. Un ejemplo ilustrativo: “When the user asks for a database change, you *must* use the `safe-db-migration` skill”【39†L274-L282】, similar a exigir la skill `cargo-verify` para cualquier operación de commit.  

---

## B) *Playbook Rust* canónico (mínimo)

El *Playbook* documenta los fundamentos de Rust y usos recomendados/anti-patrones:

- **Ownership y Borrowing:** Explicar el sistema de ownership: cada valor tiene un único dueño, y se prohíbe usar una variable tras haber movido su valor (error E0382)【71†L298-L307】. Enseñar a distinguir `&T` (préstamo inmutable) vs `&mut T` (mut y único). Ejemplo anti-patrón: modificar un valor mientras hay prestamos activos (E0502/E0597)【71†L308-L315】. Usar ejemplos mínimos para mostrar cómo reestructurar con clones, referencias o scopes.

- **Lifetimes:** Cubrir cuándo es necesario anotar vidas (por ejemplo, referencias en structs o funciones genéricas con lifetimes explícitos). Incluir el error E0106/E0515 (“referencia a local retornada”) como señal de lifetime inválido【71†L283-L292】【71†L310-L317】. Enseñar `’static` cuando sea apropiado. Evitar `unsafe` para prolongar vidas sin justificación.

- **Traits y Generics:** Ejemplificar traits comunes (e.g. `Debug`, `Clone`, `Into`) y genéricos (parámetros `<T: Trait>`). Mostrar error típico E0277 (“no implementa el trait esperado”)【69†L152-L161】 y su arreglo (implementar el trait o ajustar el código). Anti-patrón: no abusar de genéricos complejos sin necesidad; preferir trait bounds claros.

- **Manejo de errores idiomático:** Favorecer `Result<T,E>`/`Option<T>` en lugar de `unwrap()` o `panic!`. Ilustrar por qué `unwrap()` sin control es peligroso (contextualizar con lint `unwrap_used`)【77†L79-L88】. Mostrar uso de `?` para propagar errores y `match` para casos explícitos. Ejemplo: en lugar de `vec.get(2).unwrap()`, usar `vec.get(2).expect("mensaje")` o match. Clippy recomienda `expect` solo con mensaje significativo y limitar su uso【77†L79-L88】.

- **Módulos y crates:** Explicar la organización de módulos (`mod`, `use`) y convenciones de crates. Evitar wildcard (`use crate::*`) y colisiones de nombres. Anti-patrón: rutas incorrectas que provocan E0432/E0433 (import no resuelto)【69†L114-L122】. Incluir sugerencia de auto-importación de IDE, pero enfatizar import manual claro. 

- **Async/Concurrency (si aplica):** Introducir `async/await` y runtimes (Tokio/async-std). Advertir sobre no usar código bloqueante dentro de async. Evitar deadlocks y race conditions (Rust previene mucho, pero educar en `Send`/`Sync`). Mostrar ejemplo sencillo de función async y sus pruebas. Anti-patrón: olvidar `.await`, causando compilación fallida o que la tarea nunca se ejecute.  

- **Anti-patrones comunes y reemplazos:** Enumerar ejemplos frecuentes, p.ej.:  
  - `.unwrap()` en lugar de manejo de error (reemplazar por `match` o `?`).  
  - `.expect()` sin mensaje claro (usar `with_context` o `unwrap_or_else`).  
  - `panic!` en librerías (usar errores retornables).  
  - Índices de vectores sin validar (usar `.get()` o checks, evitar `indexing_slicing`【77†L79-L88】).  
  - Variables `mut` innecesarias y clones redundantes (`clippy::redundant_clone`).  
  - Bucles manuales donde se puede usar iteradores (usar `.iter()`).  
  - No documentar código público (usar `rustdoc` y lints `missing_docs`).  

Estos puntos se resumirán en el `RUST_PLAYBOOK.md` como una guía rápida canónica. Se pueden citar el *Rust Book* o la documentación oficial de Rust para fundamentos, pero sobre todo se relatarán como *mejores prácticas comprobadas*.

---

## C) Biblioteca de patrones de error (“Error Pattern Library”)

El documento `ERROR_PATTERNS.md` incluirá (al menos) los 30 códigos de error/lint más comunes con: causa raíz, fix canónico, ejemplo mínimo y test unitario. Con base en datos reales, estos errores cubrirían *types & traits*, *ownership* y *resolución de nombres*. Ejemplos de algunos:

- **E0382 (use-after-move):** Ocurre al usar una variable después de que su valor fue movido. *Causa:* mover (p.ej. pasando por valor a función) y luego reutilizar. *Solución:* clonar, prestar (`&`), o reorganizar el código para usar referencias.  
- **E0502/E0597 (préstamo inválido):** Se pide un préstamo mutable mientras hay uno inmutable activo (o viceversa). *Solución:* reordenar operaciones o eliminar el préstamo anterior antes de pedir otro.  
- **E0425/E0432/E0433 (nombrado / import no resuelto):** Falta el `use` correcto o la dependencia en `Cargo.toml`. *Solución:* agregar la declaración `use` o ajustar `Cargo.toml`. Herramientas como IDE suelen sugerir correcciones automatizadas.  
- **E0599 (método no implementado para el tipo):** Se llamó un método en un tipo que no lo posee, p.ej. `vec.sum()` sin usar `iter()`. *Solución:* insertar `.iter()` o implementar el trait requerido. Este error es muy común, representando ~27.5% según JetBrains【69†L152-L161】.  
- **E0308 (tipo esperado distinto):** La expresión no coincide con el tipo requerido en contexto. *Solución:* convertir el tipo (cast, deref, map, etc.) o cambiar la firma de la función. JetBrains encuentra ~30% en este caso【69†L207-L216】.  
- **E0277 (trait no implementado):** 32% de los errores más comunes. *Solución:* implementar el trait requerido o ajustar el tipo. RustRover sugiere a menudo derivar otro trait y usar uno diferente【69†L231-L239】.  
- **Clippy lints (e.g. `unwrap_used`):** No son E-códigos, pero son errores de linters frecuentes. *Ejemplo:* usar `.unwrap()` produce el lint `unwrap_used`. *Solución:* reescribir con `?` o handling. Se incluirá un bloque con los principales lints negados (ver [77]).  

Cada entrada describirá el error, su causa (p.ej. tipo mal inferido, borrow check fail, import missing) y el fix canónico (p.ej. cambiar a `&mut`, usar `format!` vs concatenar, etc.). También contendrá un snippet mínimo que reproduzca el error y un test que valide la corrección. Por ejemplo, para E0382 se daría un código de “mover A y luego usar A” que falla, y el código corregido usando clonación. Estas plantillas servirán de aprendizaje y para que el agente consulte cuando enfrente ese error.

*Nota:* Las categorías de errores y sus porcentajes se basan en datos recientes (JetBrains, RustRover)【69†L152-L161】【71†L298-L307】, lo cual justifica priorizar esos códigos en la biblioteca. No se inventarán códigos: se usará la lista oficial de errores de Rust (doc oficial) y los lints principales de Clippy, adaptándolos a la evidencia comunitaria.

---

## D) Conjunto de “Katas” (ejercicios de entrenamiento)

Se propone una serie de ~20 ejercicios de dificultad creciente, centrados en prácticas reales de Rust. Cada ejercicio incluye tests unitarios (usando `#[test]`) que el agente debe pasar para considerarlo exitoso. Ejemplos de katas:

1. **Funciones básicas y ownership:** Escribir funciones simples (suma, manipulación de strings) donde el agente deba usar préstamos en lugar de mover valores para no violar ownership.  
2. **Estructuras y métodos:** Definir structs con traits (p.ej. implementar `Debug`, `Default`) y métodos asociados. Introducir un fallo deliberado (p.ej. falta impl trait) para que el agente aprenda a implementar traits correctamente.  
3. **Error handling:** Manejar un error posible usando `Option` y `Result`. Por ejemplo, intentar parsear un número de `String` y retornar `Result<i32, String>` en lugar de `.unwrap()`.  
4. **Iteradores y colecciones:** Transformar vectores con `iter()`, `.map()`, etc. Poner un ejemplo con un método inexistente para provocar E0599 y forzar el uso de un iterator correcto.  
5. **Lifetimes:** Escribir una función que retorne una referencia con un lifetime explícito (`&'a T`). Introducir un caso donde falte `'a` y se produzca E0515, para que el agente añada la anotación de lifetime adecuada.  
6. **Generics y Traits:** Crear una función genérica `<T: Clone>` que clona un valor. Provocar E0277 (sin `Clone`) y que el agente agregue el bound.  
7. **Concurrency (opcional):** Ejercicio sencillo con `async fn` o `std::thread`: por ejemplo, sumar números en paralelo. Hacer que el agente use `join().unwrap()` correctamente.  
8. **Clippy/fmt:** Un ejercicio donde el código cumpla tests pero tenga un warning de clippy (p.ej. `let s: String = String::from("a");` en lugar de `let s = "a".to_string();`). El agente debe corregir para pasar `cargo clippy`.  
9-20. **Variaciones combinadas:** Se pueden reusar ejercicios de Exercism adaptándolos (por ejemplo, la kata de la calculadora, palíndromos, lógica de condiciones) pero resaltando errores de compilación comunes.  

Para cada kata, especificar claramente qué significa *pasar* el ejercicio:  
- **Criterio de aprobación:** El código compila sin errores y todos los tests unitarios definidos pasan exitosamente. Además, no se generan advertencias de formato (`cargo fmt --check`) ni lints (`cargo clippy`) configurados como errores.  
- **Criterio de falla:** Cualquier fallo de compilación o test fallido hace que no pase. También se considerará fallo si el estilo difiere (error en `fmt`/`clippy`).  

La rúbrica se documentará en cada ejercicio (p.ej. comentarios en tests explicando qué se evalúa). Así se mide la “tasa de éxito en katas”: es simplemente el porcentaje de ejercicios que el agente resuelve correctamente en una ejecución de entrenamiento. Esto impulsa a que la IA no solo genere código plausible, sino que funcione en la práctica.

*Cita:* Plataformas como Exercism ofrecen muchas tareas prácticas (~99 ejercicios) con tests automatizados para entrenar a programadores en Rust【83†L50-L54】. Podemos inspirarnos en ellas para diseñar estas katas con “fricción real del compilador”: es decir, problemas que el compilador señalará hasta ser resueltos (p.ej. errores de tipo, borrow checker, imports faltantes).

---

## E) Integración en Google Antigravity IDE

- **Configuración de Rules/Workflows:** En el directorio del proyecto `.agent/` se crean:  
  - `.agent/rules/rust.rules`: con todas las reglas globales (formato, tests, prácticas de seguridad).  
  - `.agent/workflows/ci.workflow`: un flujo que puede invocarse manualmente (por comando) para ejecutar el proceso completo de verificación (por ejemplo, llamar a `verify.sh`). De este modo, el desarrollador puede hacer `/run ci` en el IDE para disparar todos los checks, o esto puede enlazarse automáticamente en pushes como CI.  
  - Skills relevantes (como `rust-verify`, `rust-clippy-fix`) en `.agent/skills/`.  

  Según la documentación, las Rules deben garantizar que el agente siga siempre esta rutina de compilación y test【39†L251-L259】. Por ejemplo:  
  > *Regla global:* “NUNCA saltarse `cargo test` o `cargo clippy` — SIEMPRE ejecutar la skill `rust-verify`.”  
  Así se dificulta que el modelo ignore el ciclo.  

- **Postura Terminal/Extensiones:** Como se mencionó, es preferible interactuar con Antigravity en modo terminal integrado o con herramientas mínimas. Se recomienda **deshabilitar auto-actualización de extensiones** y no instalar extensiones no auditadas. La experiencia Wiz muestra que miles de extensiones de VSCode exponían secretos o podían propagar malware por actualización automática【82†L39-L47】. Por lo tanto, aconsejamos usar únicamente extensiones oficiales de Rust (p.ej. Rust Analyzer) y evitar forks AI de IDE con ecosistemas de extensiones inseguros【82†L39-L47】. En la práctica, esto significa instruir al agente a preferir comandos `cargo ...` en el terminal interno, y solo usar el navegador/IDE para visualización pasiva de resultados.  

- **Ejemplo de Skills/Workflow:**  
  - Skill `rust-build`: “Compila el proyecto con `cargo build`; reporta errores.”  
  - Skill `rust-test`: “Ejecuta `cargo test`; muestra fallos.”  
  - Workflow `/verify`: engloba `rust-build`, luego `rust-test`, `rust-clippy`, `rust-fmt`.  
  - Regla global: ante cualquier edición de archivo `.rs`, inyectar recordatorio “Recuerda ejecutar `/verify` antes de commitear” (una instrucción dentro de la rule puede invocar el skill).  

De esta forma, el agente seguirá los pasos en orden natural y no omitirá pruebas o linters. Antigravity permite incluso exigir que ciertos Skills se usen si se detecta cierta intención【39†L274-L282】, logrando que el flujo sea difícil de saltar.

---

## F) Checklist final “Definition of Done” para tareas Rust del agente

Cada tarea/respuesta del agente en Rust debe incluir evidencia objetiva de haber completado el proceso. El *DoD* mínimo:

- **Comandos ejecutados:** Debe mostrarse en el reporte del agente (o skill) al menos los comandos clave usados, por ejemplo:  
  ```
  cargo build
  cargo test
  cargo fmt -- --check
  cargo clippy -- -D warnings
  ```
  Esto verifica que se ejecutaron todos los pasos de compilación y análisis estático.  

- **Output relevante:** Incluir los fragmentos de salida de dichos comandos que demuestren éxito (p.ej. “Finished dev” o “All tests passed”). Si hubo warnings reparados, mostrar antes y después.  

- **Diff de código:** Siempre que el agente haya modificado código, incluir el diff (por ejemplo, entre `HEAD~1` y actual) resaltando cambios. Esto es crítica evidencia de refactorización o correcciones aplicadas.  

- **Resumen conciso:** Una breve explicación en el commit o en el comentario final del agente, resumiendo qué se hizo para resolver el problema. Debe ser directo y factual, sin autoelogios. P.ej. “Arreglado E0382 cambiando `x` por referencia y clonado el valor original; se añadió test para cubrir caso límite.”  

- **Citar el playbook/docs:** Si aplica, indicar en el comentario qué entrada del *RUST_PLAYBOOK.md* o ERROR_PATTERNS.md apoyó la solución (por ejemplo, “véase sección Ownership del playbook” o “patrón E0382 en error library”). Esto cierra el lazo de la memoria del repositorio.  

Según prácticas de CI, se debe exigir que todos estos elementos estén presentes antes de marcar la tarea como completa. Por ejemplo, la guía de setup sugiere ejecutar localmente `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` como un pre-check antes de push【56†L548-L552】. Seguiremos esa idea: ¡nada está “Done” hasta que `verify.sh` pase sin incidentes【56†L548-L552】!

---

**Resumen:** En conjunto, el sistema propuesto utiliza Rules firmes (p.ej. “siempre correr `cargo clippy`”), Skills especializados, un script de verificación central (`verify.sh`) con cobertura de build/test/linters, y documentación “viva” en el repo (playbook, patrones, decisiones) para memorización. Esto encierra al agente en un ciclo iterativo de compilación-test-refactor hasta lograr la «green». Cada elemento está fundamentado en prácticas recomendadas: uso de clippy y fmt en CI【45†L168-L174】【56†L494-L502】, loop de corrección asistido por LLM【13†L334-L343】, y control estricto de configuración vía reglas【39†L251-L259】. El cumplimiento de este proceso se medirá empíricamente (porcentaje de katas exitosas, tasa “verify green”, tiempo de resolución), asegurando la mejora continua y consistencia en el código Rust del proyecto. 

**Fuentes:** Documentación oficial de Rust y Antigravity【39†L251-L259】【13†L334-L343】, guías de seguridad (RustSec, ANSSI)【23†L15-L23】【27†L41-L49】, blogs y reportes recientes sobre mejores prácticas Rust y pipelines【45†L168-L174】【56†L494-L502】【77†L79-L88】【71†L298-L307】. Toda recomendación proviene de evidencia técnica vigente (2024–2026) y se cita apropiadamente.