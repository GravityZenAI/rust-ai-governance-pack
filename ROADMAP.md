# Roadmap — rust-ai-governance-pack

This document outlines the planned evolution of the governance pack.

---

## Short Term (v1.1 – v1.3)

### v1.1 — Expanded Katas
- Additional katas for memory safety patterns (arena allocation, pin projections)
- Async/await governance katas (cancellation safety, structured concurrency)
- Error handling katas (thiserror vs anyhow, custom error hierarchies)
- Target: 30 katas total (currently 20)

### v1.2 — CI Integration Template
- GitHub Actions reusable workflow that teams can import
- Runs all 7 verification gates as CI checks
- SARIF output for GitHub Code Scanning integration
- Badge generation for README (governance score)

### v1.3 — Documentation Expansion
- Video walkthrough of the governance workflow
- Decision tree: "Which rule applies to my case?"
- FAQ based on community questions
- Architecture Decision Records (ADR) for each rule

---

## Medium Term (v2.0 – v2.1)

### v2.0 — Multi-Language Support
- Extend governance framework beyond Rust:
  - Go module (similar rules adapted for Go idioms)
  - Python module (type hints, safety patterns, dependency governance)
- Shared verification gate interface across languages
- Language-specific katas for each module

### v2.1 — Formal Verification Integration
- Integration with Prusti (Rust formal verifier)
- Integration with Kani (model checker for Rust)
- Verification gates that use formal proofs, not just tests
- "Formally verified" badge for code that passes all proofs

---

## Long Term (v3.0+)

### v3.0 — AI Agent Governance Runtime
- Runtime library that AI agents import to self-govern
- Real-time code analysis before commit
- Integration with OpenClaw, LangChain, CrewAI agent loops
- Telemetry: track which rules are triggered most often

### v3.1 — Community Governance
- Plugin system for custom rules and skills
- Community-contributed katas with review process
- Governance pack marketplace
- Benchmarking: compare governance scores across projects

### v3.2 — OWASP ASI Mapping
- Formal mapping of each rule and verification gate to OWASP ASI categories
- Combined score when used with AI-Bastion (infrastructure) and Guardian (perimeter)
- Compliance report generation for enterprise audits

---

## How to Contribute

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Feature requests and roadmap suggestions welcome via [GitHub Issues](https://github.com/GravityZenAI/rust-ai-governance-pack/issues).
