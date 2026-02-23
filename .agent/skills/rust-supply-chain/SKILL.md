---
name: rust-supply-chain
description: Prevent vulnerable or risky dependencies from entering the build.
---

<skill name="rust-supply-chain">

<description>Prevent vulnerable or risky dependencies from entering the build.</description>

<when_to_use>
- ALWAYS activate when `Cargo.toml` or dependency versions change.
- ALWAYS activate before adding any new crate.
- Complements `rust-verifier` — this handles dependency risk, verifier handles code quality.
</when_to_use>

<inherits from="rust-core" />

<critical_rules>
<rule id="1" level="NEVER">Add a new crate if the standard library provides equivalent functionality.</rule>
<rule id="2" level="ALWAYS">Use `default-features = false` and enable only the features you need.</rule>
<rule id="3" level="ALWAYS">Document every dependency decision in `docs/ai/DECISIONS.md`.</rule>
<rule id="4" level="NEVER">Ignore `cargo audit` findings — fix, replace, or document the risk.</rule>
</critical_rules>

<sections>

<section name="required-checks">
<content>
| Check | Command | Action on failure |
|-------|---------|-------------------|
| Known vulnerabilities | `cargo audit` | Update, replace, or document waiver |
| License &amp; ban policy | `cargo deny check` | Remove banned crate or request exception |
| Supply chain review | `cargo vet` (if configured) | Audit the crate or find a vetted alternative |
</content>
</section>

<section name="example">
<code_example language="toml">
# Cargo.toml — CORRECT
[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }

# WRONG — pulls in ALL features unnecessarily
# serde = "1"
</code_example>
</section>

<section name="documentation-requirements">
<content>
For every new dependency, record in DECISIONS.md:
- **Why this crate**: what problem it solves
- **Alternatives considered**: at least one and why rejected
- **License**: compatible with project license?
- **Feature flags**: which are enabled and why
- **Risk assessment**: maintenance status, unsafe count, transitive deps
</content>
</section>

<section name="red-flags">
<content>
NEVER accept without justification:
- Unmaintained crates (no commits in 12+ months)
- Many `unsafe` lines in transitive deps for a simple task
- Duplicate major versions of the same crate
- Network or crypto crates with low scrutiny
- Yanked versions in the dependency tree
</content>
</section>

<section name="cargo-hygiene">
<content>
- Gate optional functionality behind feature flags.
- In workspaces: `[workspace.dependencies]` for version inheritance.
- `#![warn(clippy::cargo)]` to catch manifest issues.
- `cargo update --dry-run` periodically to check for updates.
</content>
</section>

</sections>

<common_mistakes>
<mistake id="1">
<wrong>Adding `serde` with all features on</wrong>
<right>Use `default-features = false, features = ["derive"]`</right>
</mistake>
<mistake id="2">
<wrong>Not checking license compatibility</wrong>
<right>Always run `cargo deny check licenses` before merging</right>
</mistake>
<mistake id="3">
<wrong>Ignoring `cargo audit` because "it's a dev dependency"</wrong>
<right>Dev deps can still run arbitrary code during build</right>
</mistake>
</common_mistakes>

<verification_checkpoints>
<checkpoint id="1" command="cargo audit">0 vulnerabilities (or all have documented waivers in DECISIONS.md)</checkpoint>
<checkpoint id="2" command="cargo deny check">Passes for licenses and bans</checkpoint>
<checkpoint id="3" command="cat docs/ai/DECISIONS.md">Every new Cargo.toml entry has a corresponding decision entry</checkpoint>
<checkpoint id="4" command="cargo tree -d">No duplicate major versions (or all documented)</checkpoint>
</verification_checkpoints>

<scalability>
<level size="small" deps="&lt;10">Review each dependency manually — feasible</level>
<level size="medium" deps="10-50">Automate with `cargo deny` config; review only new additions</level>
<level size="large" deps="50+" workspace="true">Use `[workspace.dependencies]` for version control; automate audits in CI</level>
<level size="open-source">Add `cargo vet` for community-driven supply chain review</level>
</scalability>

<integration>
<related_skill name="rust-core" relationship="inherits" />
<related_skill name="rust-verifier" relationship="dependency-gate" />
</integration>

</skill>
