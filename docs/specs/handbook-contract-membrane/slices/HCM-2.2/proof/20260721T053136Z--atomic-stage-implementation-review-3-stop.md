# HCM-2.2 atomic-stage implementation Review 3 stop

## Disposition

Fresh isolated Review 3 admitted the exact 199-path implementation subject
bound by dispatch
`20260721T050717Z--HCM-2-2--fresh-atomic-stage-implementation-review-3` and
subject fingerprint
`sha256:bb41f529990176f4554dcbdf4ed2e6354d710480065cdf0354c621613a0a924f`.
It returned `CHANGES_REQUIRED` with Required finding
`HCM-2.2-AR3-001`, **Lifecycle exact-byte binding remains coherently
rewritable**. The subject is rejected and the finding is accepted without
waiver.

## RED reproduction

The additive test
`coherent_result_witness_and_binding_rewrite_refuses_without_mutation`:

1. authored a candidate `1.2` and engine lifecycle-validation result `1.0`;
2. changed only the audit-only `validated_at_utc` value, preserving the
   semantic result fingerprint, result ID, result ref, and candidate identity;
3. wrote the changed exact bytes equally to the result and witness;
4. updated the binding's embedded JCS, document SHA-256, and byte length and
   recomputed its self-fingerprint; and
5. invoked author replay and promotion while retaining every forged file.

The focused command was:

```text
cargo test -p handbook-engine --test hcm_2_2_authority_repair coherent_result_witness_and_binding_rewrite_refuses_without_mutation -- --exact --nocapture
```

It failed RED because author replay returned the original candidate and
validation-result identities instead of refusing. That directly proves the
three mutually consistent files provide no independent expected exact-byte
authority.

## Authority analysis and stop

The authoritative candidate `1.2` fingerprint preimage carries exactly one
semantic lifecycle-validation-result ref. The authoritative result `1.0`
fingerprint excludes `validated_at_utc`, and its fixed ref grammar is derived
from that semantic fingerprint. Neither closed shape carries an independent
exact result-document hash or binding fingerprint. A fourth mutable sidecar
would repeat the same defect unless its expected identity were itself retained
by independent immutable authority; deriving that identity from any member of
the mutable set is circular.

Binding an exact-document digest into candidate/result authority therefore
requires a reviewed contract/schema/identity-graph change. That change is not
an implementation detail and is outside the review-clean implementation
authority selected by handoff
`20260720T225541Z--HCM-2-2--orchestration--atomic-stage-authority-repair-approved`.
The handoff also prohibits widening intent `1.2`, and both the handoff and the
explicit user instructions require stopping when identity becomes implicit or
cyclic or when risk expands beyond the review-clean plan.

GitNexus reconfirmed `evaluate_charter_intake` as `HIGH`: seven direct callers,
twelve impacted symbols, four modules, and two affected processes. New
uncommitted lifecycle-validation symbols remain unavailable in the clean-HEAD
index. No production symbol was edited after Review 3.

The authority worktree remains on
`486458acfe8373a977e595a4854ac786166e3e76` at branch
`feat/hcm-2-2-atomic-stage-authority-repair`; no commit was created. The
external comparison archive still hashes to
`29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e`, and the
original dirty implementation remains non-authoritative and unmodified by this
stop.

## Resume condition

Resume only from a separately authorized, fresh review-clean HCM-2.2 contract
repair that freezes an independently retained, acyclic exact-document digest
anchor and its schema, identity preimages, refs, persistence, migration,
recovery, and negative vectors. HCM-2.3 and later work remain unauthorized.
