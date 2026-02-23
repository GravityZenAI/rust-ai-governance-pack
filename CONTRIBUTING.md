<document name="contributing" type="guide">

<summary>How to contribute to the Rust AI Governance Pack: issues, PRs, guidelines, and code of conduct.</summary>

<sections>

<section name="reporting-issues">
<content>
- Open an issue describing the problem or improvement.
- Include your AI agent (Claude, Copilot, Cursor, etc.) and Rust version.
- Provide minimal reproduction steps if applicable.
</content>
</section>

<section name="submitting-changes">
<content>
1. Fork this repository.
2. Create a feature branch (`git checkout -b feature/my-improvement`).
3. Make your changes.
4. Ensure `tools/verify.sh` passes in your own projects.
5. Submit a Pull Request with a clear description.
</content>
</section>

<section name="what-we-want">
<content>
**New Skills** (`.agent/skills/`):
- Async/await patterns
- WebAssembly compilation
- Embedded Rust
- Cross-compilation
- Performance optimization

**Improvements to Existing Content**:
- Better error pattern documentation
- Real-world case studies
- Integration guides for specific AI agents
- Translations

**New Verification Gates**:
- Additional `cargo` tools integration
- Custom lint rules
- Benchmark gates
</content>
</section>

<section name="guidelines">
<content>
- Keep rules concise — agents work better with short, clear instructions.
- Skills should be loaded on-demand, not always — respect context window limits.
- Every rule must be verifiable (no subjective guidelines).
- Test your changes with at least 2 different AI coding agents.
</content>
</section>

<section name="code-of-conduct">
<content>
Be respectful, constructive, and evidence-based — just like the pack itself.
</content>
</section>

<section name="license">
<content>
By contributing, you agree that your contributions will be licensed under Apache-2.0.
</content>
</section>

</sections>

</document>
