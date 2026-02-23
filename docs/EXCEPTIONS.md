<document name="exceptions" type="policy">

<summary>Registry of approved rule bypasses. If it is not here, it does not exist.</summary>

<sections>

<section name="rules">
<content>
1. Every exception MUST include: unique ID, scope (file/module), justification, risk, human approver, expiration date.
2. If expired, the exception must be removed or renewed with new approval.
</content>
</section>

<section name="template">
<content>
- Exception ID: EXC-YYYY-MM-DD-XXX
- Scope:
- What is bypassed:
- Why:
- Risk:
- Approved by:
- Expires:
- Links: (PR, issue)
</content>
</section>

<section name="example">
<content>
- Exception ID: EXC-2026-02-23-001
- Scope: `src/parser.rs`
- What is bypassed: uso de `unwrap()` en ruta imposible según invariantes
- Why: micro-opt y simplificación; medido.
- Risk: `panic!` si invariante se rompe
- Approved by: @tech-lead
- Expires: 2026-03-23
- Links: PR #123
</content>
</section>

</sections>

</document>
