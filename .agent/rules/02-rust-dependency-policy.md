<rule id="02" name="rust-dependency-policy" priority="critical">

<purpose>Prevent unvetted, insecure, or unnecessary dependencies from entering the build.</purpose>

<requirements>
<req id="1" level="MUST">Standard library and existing Cargo.toml dependencies are allowed by default.</req>
<req id="2" level="MUST">Before adding a new crate, answer: 1) What capability is missing? 2) Why is this crate preferable vs alternatives? 3) What is the maintenance risk? 4) License compatibility? 5) What features to enable/disable?</req>
<req id="3" level="MUST">Run `cargo audit` (RustSec advisories), `cargo deny check` (licenses/bans/duplicates), and `cargo geiger` (unsafe footprint) for sensitive projects.</req>
<req id="4" level="SHOULD">Prefer `default-features = false` and enable only required features.</req>
<req id="5" level="SHOULD">Avoid pulling heavy ecosystems for trivial needs.</req>
<req id="6" level="MUST">Commit `Cargo.lock` for applications/binaries.</req>
<req id="7" level="SHOULD">Avoid loose version ranges for security-sensitive builds.</req>
<req id="8" level="MUST">Exceptions require explicit approval documented in `docs/ai/DECISIONS.md`.</req>
</requirements>

<examples>
<example type="correct">
```toml
[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }
```
With DECISIONS.md entry: "Added serde for config deserialization. derive-only, no default features."
</example>
<example type="violation">
Adding `serde = "1"` with all default features and no DECISIONS.md entry.
</example>
</examples>

<enforcement>Run `cargo audit`, `cargo deny check` before merge. Verify DECISIONS.md entry exists for each new dependency.</enforcement>

</rule>
