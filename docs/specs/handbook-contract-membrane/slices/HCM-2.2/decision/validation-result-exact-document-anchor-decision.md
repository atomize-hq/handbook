# HCM-2.2 validation-result exact-document anchor decision

**Decision ID:** `HCM-2.2-ESC-003-D1`  
**Status:** selected documentation authority subject; implementation remains
stopped pending complete-subject fresh review and parent closeout  
**Date:** 2026-07-21

## Decision

Select additive `handbook.artifact-candidate` version `1.3` as the independent
downstream exact-document anchor for
`handbook.lifecycle-validation-result` version `1.0`.

Candidate `1.3` keeps its complete fourteen-field candidate-subject fingerprint
independent of result identity and bytes. Its final fingerprint includes one
required closed `validation_result_binding` containing exactly:

- `validation_result_ref`;
- `validation_result_fingerprint`;
- `result_document_sha256`, over exact persisted result JCS+LF bytes; and
- `result_byte_length`, including the LF.

Result `1.0` remains unchanged and binds only the candidate-subject
fingerprint. Approval binds the final candidate `1.3` fingerprint. Promotion
and promotion-intent `1.2` bind the final candidate transitively. The result and
intent schemas remain unchanged.

## Identity graph

```text
candidate 1.3 subject
  -> candidate_subject_fingerprint
  -> result 1.0 semantic fingerprint/ref
  -> exact persisted result JCS+LF bytes/hash/length
  -> candidate 1.3 exact-result binding
  -> final candidate fingerprint/ID
  -> new approval
  -> promotion and intent 1.2
```

The result has no final-candidate or exact-binding edge, and the subject
fingerprint excludes the result binding. Every dependency moves forward, so the
graph is acyclic. No result self-hash or mutable witness, binding, receipt,
sidecar, or companion is independent authority.

## Replay and orphan decision

Under the author lock, independently recompute the candidate-subject
fingerprint and the expected semantic result fingerprint/ref from current
resolved authority, excluding audit-only `validated_at_utc`. Discover candidate
`1.3` records by the pair `(candidate-subject fingerprint, expected semantic
result ref/fingerprint)`.

- zero matches and no result is first authoring;
- exactly one match is replay only after final candidate identity and exact
  result digest/length equality pass;
- more than one match refuses and preserves all evidence;
- a result without a matching candidate is an orphan and refuses;
- a matching candidate without its result refuses; and
- unsafe, missing, rewritten, duplicate, mismatching, crossed, or malformed
  authority refuses before mutation.

An orphan is never adopted, overwritten, repaired, wrapped, or deleted
automatically. Automatic crash completion requires a separately reviewed
author-publication transaction and is not designed by this decision.

## Migration decision

Candidate `1.0`, `1.1`, and `1.2` records and approvals remain immutable
historical/checkpoint evidence. Selected-product authority requires candidate
`1.3` reauthoring, a newly computed result `1.0`, and new approvals. No dual
read, automatic upgrade, result-ref copy, approval carry-forward, selected-
product fallback, or historical rewrite is permitted.

## Rejected alternatives

- **Result self-hash:** rejected because excluding the field permits coherent
  recomputation and including it is circular.
- **Another mutable sidecar:** rejected because mutually consistent mutable
  companions do not select an expected audit-byte variant.
- **Make `validated_at_utc` semantic:** not selected because it turns audit time
  into constitutional identity and is a larger migration.
- **Automatic orphan adoption:** rejected because a result alone cannot prove
  intended candidate publication.

## Consequences and stop boundary

The rejected Review 3 implementation cannot resume as written. A later
implementation must replace candidate `1.2` selected-path behavior and the
unauthoritative witness/binding design with this candidate `1.3` contract,
begin RED with the complete negative matrix, and receive a new complete-subject
implementation review. This decision authorizes no Rust edit and no HCM-2.3
work.

Normative detail lives in
[`../SPEC.md`](../SPEC.md),
[`../contracts/authority-repair-runtime-vectors-v1.0.json`](../contracts/authority-repair-runtime-vectors-v1.0.json),
and
[`../contracts/runtime-record-fingerprint-vectors-v1.0.json`](../contracts/runtime-record-fingerprint-vectors-v1.0.json).
The prior
[`../research/validation-result-exact-document-anchor-options.md`](../research/validation-result-exact-document-anchor-options.md)
remains non-authoritative decision input.
