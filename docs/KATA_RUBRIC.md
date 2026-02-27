<document name="kata-rubric" type="rubric">

<summary>Pass/fail criteria and scoring metrics for Rust training katas.</summary>

<checklists>

<checklist name="pass-criteria">
<item id="1" required="true">Tests green: `cd training/kata_suite && cargo test`</item>
<item id="2" required="true">Format: `cargo fmt -- --check` (or `./tools/verify.sh` from root)</item>
<item id="3" required="true">Lints: `cargo clippy -- -D warnings`</item>
</checklist>

<checklist name="fail-criteria">
<item id="1" required="true">Any compilation error.</item>
<item id="2" required="true">Any failing test.</item>
<item id="3" required="true">Any clippy warning (treated as error).</item>
<item id="4" required="true">Formatting differences in fmt check.</item>
</checklist>

</checklists>

<sections>

<section name="scoring">
<content>
Per kata:
- `iterations_to_green` = number of `cargo test` runs until green.
- `time_to_green` = minutes until green.
- `new_error_patterns` = new entries added to ERROR_PATTERNS.md.

Suite level:
- `kata_pass_rate` = green katas / 20.
</content>
</section>

</sections>

</document>
