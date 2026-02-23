<rule id="05" name="rust-quality-bar" priority="high">

<purpose>Enforce minimum code quality standards: no warnings, no unsafe without justification, no panics in production code.</purpose>

<requirements>
<req id="1" level="MUST">Treat warnings as errors in CI (`clippy` runs with `-D warnings`).</req>
<req id="2" level="MUST">Keep the code formatted (`cargo fmt` check must pass).</req>
<req id="3" level="MUST NOT">Introduce `unsafe` without: a written invariant in comments, a test that would fail if the invariant breaks, and an entry in `DECISIONS.md`.</req>
<req id="4" level="MUST NOT">Use `unwrap()` / `expect()` in production code. Allowed only with a documented exception in `EXCEPTIONS.md`.</req>
<req id="5" level="MUST NOT">Index into collections with `[]` when input is not proven in-bounds. Prefer `.get()` + explicit handling.</req>
</requirements>

<examples>
<example type="correct">
Using `data.get(idx).ok_or(AppError::OutOfBounds)` instead of `data[idx]`.
</example>
<example type="violation">
Using `data[user_input]` where `user_input` comes from untrusted source.
</example>
</examples>

<enforcement>`cargo clippy -- -D warnings` and `cargo fmt --check` must exit 0. Manual review for unsafe/unwrap usage.</enforcement>

</rule>
