# HCM-0.8 Planning Discovery Remediation

Recorded at: 2026-08-02T03:09:00Z

Review run: `/root/hcm08_authority_continuation_planning_review`
Dispatch:
`20260802T025342Z--HCM-0-8--post-clean-authority-continuation-planning-review`
Reviewed subject:
`sha256:2eddbc1215508f4ca60aaf09b771e72ede7e9d3b6fb1bfc1649c94706adab38c`
Verdict: `FINDINGS`

The reviewer was fresh, built-in, read-only, and changed no files. Exact
dispatch replay, ordinary validation, both baseline self-tests, strict UTF-8,
whitespace, and aggregate fingerprint replay passed.

## Consolidated disposition

### HCM08-PLN-DISC-001 — P1 — remediated

The grant now requires a typed, independently issued repository authority
artifact with issuer/owner role and identity, source task/thread/host/nonce,
issuance time, predecessor and identity bindings, authorized scope/symbol/risk,
and a separate exact CLEAN review dispatch. It must predate admission, the
issuer cannot be the admission reviewer or continuation executor, and every
stable field must match the grant. Self-issued, postdated, ownerless, issuer-
mismatched, and under-scoped cases are mandatory negatives.

### HCM08-PLN-DISC-002 — P2 — remediated

Admission findings cannot be repaired inside the continuation. Any P1/P2
terminates the immutable one-attempt grant and requires newly issued and
reviewed external authority at a true stop. This preserves the pre-edit gate
and prevents a circular remediation allowance.

### HCM08-PLN-DISC-003 — P2 — remediated

The grant now binds an immutable Git commit/tree baseline. Every continuation
and final handoff must reconcile the actual baseline-to-tip path delta and
complete GitNexus changed-symbol/risk observations against the path, symbol,
and risk ceilings. Omitted paths, out-of-ceiling symbols, and excess risk are
mandatory negatives.

### HCM08-PLN-DISC-004 — P2 — remediated

The exact committed HCM-3.2 five-dispatch prefix remains the immutable
predecessor. The positive fixture must add the qualifying authority artifact
and its separate CLEAN review, then construct both an active direct successor
handoff and a completed successor that consumes the grant. Reuse, second-
successor, summary mismatch, post-consumption dispatch, and predecessor-byte
mutation are mandatory negatives.

### HCM08-PLN-DISC-005 — P2 — remediated

The immutable `grant` is separated from per-dispatch `review_slot` and the
external `grant_fingerprint`. The fingerprint is `sha256:` plus lowercase
SHA-256 over UTF-8 RFC 8785/JCS canonical JSON for the complete grant followed
by one LF. A golden vector, stable-field mutations, and slot-invariance checks
are required. Each cycle admits exactly one dispatch, so bursts cannot escape
the fixed allowance.

## Remaining gate

No schema or validator implementation is permitted until a different fresh,
read-only reviewer closes all five finding IDs over the changed planning
subject. Any P1/P2 directly caused or unmasked by this remediation retains the
same planning lineage and does not restart discovery.
