---
name: rust-security-audit
description: Optional dependency and basic security checks (cargo audit / cargo deny / miri).
---

# Rust Security Audit (Optional)

Security is secondary here, but we keep a baseline.

## Steps

1) If available, run RustSec audit:

```bash
cargo audit
```

2) If available, run cargo-deny:

```bash
cargo deny check
```

3) If the code uses `unsafe`, consider Miri (nightly):

```bash
RUN_MIRI=1 ./scripts/verify.sh
```

4) For any finding:
- document in `DECISIONS.md` (accepted risk) or fix.

