# 🦀 Ecosistema Completo de Rust para IA — Febrero 2026

> Todas las herramientas, agentes, frameworks, SDKs y crates disponibles para construir sistemas de IA en Rust.

---

## 1. FRAMEWORKS DE AGENTES IA (los más importantes)

### Rig — El más maduro y usado en producción
- **Repo:** github.com/0xPlaygrounds/rig
- **Crate:** `rig-core`
- **Licencia:** MIT
- **Estado:** Muy activo, usado en producción por St Jude, Coral Protocol, Dria, Nethermind, Neon
- **Qué hace:** Framework modular para apps LLM. Agentes, RAG, pipelines, tool calling, streaming, telemetría
- **Providers nativos:** OpenAI, Anthropic, Cohere, Gemini, DeepSeek, xAI, Ollama, Perplexity
- **Vector stores:** MongoDB, SQLite, Qdrant, Neo4j, LanceDB, in-memory
- **Features clave:**
  - Macro `#[tool]` para convertir funciones en herramientas de agente
  - Macro `#[derive(Embed)]` para embeddings
  - Pipelines composables (como LangChain pero type-safe)
  - Multi-turn streaming
  - Image generation
  - RAG con VectorStoreIndex trait
- **El SDK oficial de MCP (Rust) está construido sobre Rig**
- **Ideal para:** Tu caso — secretaries de GravityOS que llaman a Ollama

### ADK-Rust — Framework modular para agentes production-ready
- **Repo:** github.com/paultimothymooney/adk-rust (adk-rust.com)
- **Estado:** Activo
- **Qué hace:** Framework inspirado en Google ADK, LangChain, OpenAI SDK, Autogen — combinando lo mejor de cada uno
- **Features:**
  - Model-agnostic (Gemini, OpenAI, Anthropic con una línea)
  - Type-safe tools con macros
  - Modular y extensible
  - Blazingly fast (Rust puro)
- **Ideal para:** Quien quiera un equivalente a Google ADK pero en Rust

### AutoAgents — Multi-agent framework
- **Repo:** github.com/liquidos-ai/AutoAgents
- **Licencia:** MIT
- **Estado:** Activo
- **Qué hace:** Framework multi-agente con coordinación tipada
- **Features:**
  - ReAct executor y basic executor
  - Streaming responses y structured outputs
  - Derive macros para tools
  - **WASM runtime sandbox** para ejecutar tools de forma segura
  - Sliding window memory
  - Pub/sub tipado entre agentes
  - OpenTelemetry integrado
  - Deploy en cloud, edge, o **browser (WASM)**
- **Ideal para:** Orquestación multi-agente tipo CrewAI pero en Rust

### GraphBit — Rust core + Python wrapper (híbrido)
- **Web:** graphbit.ai
- **Qué hace:** Primer framework de agentes con core en Rust envuelto en Python
- **Features:**
  - Parallel orchestration engine (concurrencia real, no GIL)
  - Multi-LLM (OpenAI, Anthropic, Ollama)
  - Enterprise security
  - Mejor rendimiento que Langflow, AutoGen, Rasa en benchmarks
- **Ideal para:** Equipos que quieren rendimiento Rust pero API en Python

### graph-flow — Workflows multi-agente type-safe
- **Crate:** `graph-flow`
- **Qué hace:** Workflows multi-agente con grafos tipados en compile time
- **Features:**
  - Persistencia PostgreSQL o in-memory
  - Integración con Rig para LLM
  - Human-in-the-loop (interrupción y resumption)
  - FanOutTask (paralelismo)
  - Async nativo
- **Ideal para:** Workflows complejos con aprobaciones humanas

### Otros frameworks de agentes

| Crate | Descripción | Estado |
|-------|-------------|--------|
| **LangChain-rust** | Port de LangChain a Rust — chains, agents, memory | Activo |
| **Anchor-Chain** | Framework async con YAML o código para workflows LLM | Activo |
| **Swiftide** | RAG pipelines y agentes rápidos (de Bosun AI) | Activo |
| **Kwaak** | Múltiples agentes de código en paralelo sobre un codebase (usa Swiftide) | Activo |
| **Nerve** | Creación de agentes con YAML — define tools y flows, ejecuta multi-step | Activo |
| **CrustAGI** | Port de BabyAGI a Rust — task management con GPT | Experimental |
| **SmartGPT** | Framework modular para agentes tipo AutoGPT | Experimental |
| **AutoGPT-rs** | Auto-GPT en Rust | Experimental |
| **agentai** | Agentes simples con MCP, toolbox, structured output | Activo |
| **Anda** | Framework de agentes con ICP blockchain + TEE support | Activo |

---

## 2. SDKs DE MCP (Model Context Protocol)

| SDK | Crate | Descripción | Estado |
|-----|-------|-------------|--------|
| **rmcp (Oficial)** | `rmcp` | SDK oficial de Anthropic para MCP en Rust. Tokio async. Server + Client | **Oficial, muy activo** |
| **rust-mcp-sdk** | `rust-mcp-sdk` | Toolkit completo con macros, OAuth, elicitation. Protocol v2025-11-25 | Muy activo |
| **pmcp** | `pmcp` + `cargo-pmcp` | SDK production-grade, **16x más rápido que TypeScript SDK**, CLI para scaffolding, book de 27 capítulos | Activo |
| **rust-mcp-schema** | `rust-mcp-schema` | Schema tipado del protocolo MCP, auto-generado desde spec oficial | Activo |
| **mcpx** | `mcpx` | Implementación completa de MCP spec 2025-03-26 | Activo |
| **mcp-protocol-sdk** | `mcp-protocol-sdk` | SDK con múltiples transports | Activo |
| **prism-mcp-rs** | `prism-mcp-rs` | Enterprise-grade con 229+ tests, 39 ejemplos | Activo |

### Servidores MCP notables escritos en Rust:
- **hyper-mcp** — MCP server extensible con plugins WASM
- **video-transcriber-mcp-rs** — Transcripción de video con whisper.cpp
- **spreadsheet-mcp** — Análisis de spreadsheets eficiente en tokens
- **terminator** — Automatización de desktop con >95% success rate
- **containerd-mcp-server** — Operaciones de containers
- **rudof-mcp** — Validación RDF con ShEx/SHACL/SPARQL

---

## 3. CLIENTES LLM Y SDKs DE PROVIDERS

### Multi-provider (un crate, muchos backends)

| Crate | Providers soportados | Features |
|-------|---------------------|----------|
| **llm** (RLLM) | OpenAI, Anthropic, Ollama, DeepSeek, xAI, Phind, Google, Groq, Mistral, Cohere, HuggingFace, ElevenLabs | Multi-step chains, CLI tool, embeddings |
| **rust-genai** | OpenAI, Anthropic, xAI, Groq, Ollama + más | SDK multi-provider experimental |
| **allms** | OpenAI, Anthropic, Mistral + más | Interface unificada |
| **llmclient** | Gemini, OpenAI, Anthropic + más | Cliente unificado |

### Provider-específicos

| Crate | Provider | Notas |
|-------|----------|-------|
| **async-openai** | OpenAI | Fully async, el más completo |
| **openai-api-rs** | OpenAI | Alternativa community |
| **openai-safe** | OpenAI | Type-safe bindings |
| **az-openai-rs** | Azure OpenAI | Endpoints Azure |
| **anthropic-rs** | Anthropic (Claude) | Streaming, completions |
| **misanthropy** | Anthropic | Binding experimental |
| **cohere-rust** | Cohere | Generate, embed, classify |
| **ollama-rs** | Ollama (local) | **El más relevante para GravityOS** |
| **aws-sdk-bedrock** | AWS Bedrock | Foundation models via AWS |
| **hf-hub** | HuggingFace Hub | Descargar modelos/datasets |

### SDKs de Claude Code en Rust

| Crate | Descripción |
|-------|-------------|
| **cc-sdk** | SDK completo con 100% paridad con Python SDK. Streaming, hooks, MCP, auto CLI download |
| **claude-sdk-rs** | SDK async-first con CLI interactivo, analytics, token tracking |
| **claude-agent-sdk** | SDK para construir agentes con Claude Code. Hooks, sessions, permissioning |

---

## 4. MOTORES DE INFERENCIA

| Crate/Proyecto | Qué hace | GPU | Estado |
|---------------|----------|-----|--------|
| **Candle** (HuggingFace) | Framework ML minimalista. Transformers, Whisper, Stable Diffusion | CUDA, Metal | **Muy activo** |
| **Mistral.rs** | LLaMA/Mistral cuantizados. Apple Silicon, CUDA, CPU | Sí | **Muy activo** |
| **Burn** | Framework DL completo con autodiff, training, múltiples backends | CUDA, wgpu, Metal | **Muy activo** |
| **Rust-BERT** | Pipelines NLP (QA, summarization, NER) con Torch backend | Sí | Activo |
| **Ratchet** | Runtime ML con WebGPU para browser y native | WebGPU | En desarrollo |
| **Tract** (Sonos) | Inferencia neural self-contained. ONNX, TensorFlow | CPU | Activo |
| **Ort** | Bindings para ONNX Runtime (Microsoft) | CUDA, TensorRT | Activo |
| **WONNX** | Runtime ONNX 100% Rust con wgpu (WebGPU) | WebGPU | Activo |
| **tch-rs** | Bindings para PyTorch/LibTorch C++ | CUDA | Activo |
| **Kalosm** | Interface simple para modelos multimodales (language, audio, image) | Via Candle | Activo |
| **LlamaEdge** | Rust+WASM para LLMs en edge devices (GGUF) | WASM | Activo |
| **Edge Transformers** | ONNX Runtime wrapper tipo HuggingFace Optimum | CPU/GPU | Activo |
| **llama_cpp / llama-cpp-2** | Bindings safe para llama.cpp | CUDA | Activo |
| **CallM** | API alto nivel sobre Candle para LLMs locales | CPU/GPU | Activo |
| **Luminal** | Kernels GPU compile-time optimizados (experimental) | CUDA, Metal | Experimental |
| **DFDX** | Pure Rust DL con tensors dinámicos y autograd | GPU | Activo |

---

## 5. TOKENIZERS Y PROCESAMIENTO DE TEXTO

| Crate | Qué hace |
|-------|----------|
| **tokenizers** (HuggingFace) | BPE/WordPiece rápido para Transformers (GPT, BERT) |
| **tiktoken-rs** | Port de tiktoken de OpenAI (encodings GPT-4) |
| **rust-tokenizers** | Implementaciones nativas de BERT, SentencePiece |
| **text-splitter** | Chunking de texto para RAG/prompts |
| **nnsplit** | Segmentación neural de oraciones |
| **lingua-rs** | Detección de idioma (75+ idiomas) |
| **whatlang-rs** | Detección de idioma ligera |
| **NLPRule** | Corrección gramatical rule-based |
| **Vader Sentiment** | Análisis de sentimiento |
| **fastText** | Embeddings y clasificación de texto |

---

## 6. VECTOR STORES (para RAG)

| Crate | Backend | Notas |
|-------|---------|-------|
| **qdrant-client** | Qdrant | El más popular en Rust |
| **lancedb** | LanceDB | Columnar, rápido |
| **mongodb** | MongoDB Atlas Vector Search | Via rig companion crate |
| **sqlite-vec** | SQLite | Embebido, sin servidor |
| **tantivy** | Full-text search | Equivalente a Lucene en Rust |
| **llm-chain-hnsw** | HNSW in-memory | Para RAG con llm-chain |

---

## 7. HERRAMIENTAS DE CODING CON IA PARA RUST

### IDEs y Agentes de código que soportan Rust

| Herramienta | Tipo | Soporte Rust | Notas |
|-------------|------|-------------|-------|
| **Claude Code** | Terminal agent | Excelente | Rust's compiler errors ayudan al agente. SDK en Rust disponible |
| **Cursor** | IDE (VS Code fork) | Excelente | Cascade para edición multi-archivo |
| **Windsurf** | IDE (VS Code fork) | Bueno | Cascade agentic mode |
| **GitHub Copilot** | IDE plugin | Excelente | El más usado, deep GitHub integration |
| **Kiro** (AWS) | IDE AI-native | Bueno | Spec-driven development |
| **JetBrains Junie** | IDE agent | Bueno | Integrado en RustRover/CLion |
| **Codeium/Windsurf** | IDE/Plugin | Bueno | Free tier generoso |
| **TabNine** | IDE plugin | Bueno | Soporte local model training |
| **Sourcegraph Cody** | IDE plugin | Bueno | Cross-repo intelligence |
| **Goose** | Terminal agent (open-source) | Bueno | Extensible, local, automatiza tasks |

### Plugins de Claude Code específicos para Rust

| Plugin | Qué hace |
|--------|----------|
| **rust-lsp** | Plugin completo: rust-analyzer LSP + 16 hooks automáticos + clippy + cargo-audit |
| **claude-flow CLAUDE.md Rust** | Configuración para swarms de agentes Rust con batching paralelo |
| **Superpowers** | Bundle de competencias para SDLC completo |
| **TSK** | AI agent task manager con sandboxed Docker — múltiples agentes en paralelo |

---

## 8. PROMPT ENGINEERING Y ORCHESTRATION

| Crate | Qué hace |
|-------|----------|
| **llm-chain** | "Ultimate toolbox" — chain prompts, state, embeddings, utils |
| **LangChain-rust** | Port de LangChain — chains, agents, memory |
| **Anchor-Chain** | Workflows LLM async, statically-typed |
| **Outlines-core** | Structured generation — fuerza output a conformar schema |
| **AICI** (Microsoft) | Prompts como programas WASM |
| **RAG-Toolchain** | Pipeline RAG completo en Rust |

---

## 9. MLOps Y UTILIDADES

| Crate | Qué hace |
|-------|----------|
| **Dora** | Orquestación distribuida para ML pipelines |
| **cargo-audit** | Auditoría de seguridad de dependencias |
| **cargo-deny** | Checks de licencias, advisories, duplicados |
| **cargo-mutants** | Mutation testing |
| **cargo-semver-checks** | Verificación de semver |
| **cargo-geiger** | Detecta unsafe code |
| **cargo-bloat** | Análisis de tamaño de binario |

---

## 10. RELEVANCIA PARA GRAVITYOS

### ¿Por qué esto importa para tu proyecto?

**Opción actual (Python + Ollama):** Funciona bien para MVP. Python tiene más ecosistema maduro.

**Opción futura (Rust):** Para componentes de rendimiento crítico:

| Componente GravityOS | Crate Rust recomendado | Por qué |
|----------------------|----------------------|---------|
| Comunicación con Ollama | `ollama-rs` | Type-safe, async, rápido |
| MCP servers | `rmcp` (oficial) | El SDK oficial está en Rust |
| Risk classifier | Rust nativo | Rendimiento en hot path, sin GIL |
| Voice pipeline VAD | `Candle` + Silero ONNX via `Ort` | Baja latencia, streaming |
| Widget IPC | Rust + Unix sockets | Concurrencia sin overhead |
| Agente multi-model | `rig-core` | El más maduro, soporta Ollama |
| RAG local | `rig-core` + `sqlite-vec` o `tantivy` | Embebido, sin servidor externo |
| Tokenización | `tokenizers` (HuggingFace) | Estándar de industria |

### Ruta recomendada
1. **Phase 1 MVP:** Python (como está planificado) — más rápido de implementar
2. **Phase 2:** Reescribir hot paths en Rust (risk classifier, IPC, VAD)
3. **Phase 3:** Considerar `rig-core` + `ollama-rs` para el core de Zen
4. **Phase 4:** MCP servers en Rust con `rmcp` para el marketplace

---

*Documento compilado: 22 febrero 2026*
*Fuentes: crates.io, GitHub, HackMD Rust AI Ecosystem, JetBrains Rust Survey 2025, Shuttle benchmarks*
