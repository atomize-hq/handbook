# HCM-2.2 exact-result authority-repair planning proof

> **Status (2026-07-21): complete pre-review documentation-subject proof.**
> `CLEAN`, if obtained, is recorded only by the final immutable dispatch and
> parent handoff. This proof does not resume Rust, authorize HCM-2.3, or close
> the parent orchestration.

## Entry preservation

The rejected implementation worktree `C:\hcm22ar` was inventoried read-only at
exact HEAD `486458acfe8373a977e595a4854ac786166e3e76` on branch
`feat/hcm-2-2-atomic-stage-authority-repair` before this repair began.

| Preservation fact | Exact result |
|---|---|
| tracked paths | 1,464 |
| non-ignored untracked paths | 13 |
| ignored-untracked paths | 25,235 |
| total archived files | 26,712 |
| external archive | `C:\Users\spmcc\Documents\__Project_Code\_hcm22ar_preservation\486458acfe8373a977e595a4854ac786166e3e76-20260721T121830Z\hcm22ar-complete-worktree-snapshot.tar` (30,789,857,280 bytes) |
| deterministic manifest SHA-256 | `cbcb29fe7ef50184cbc2efcaa9489d07c99d0d4b51c7e92836e7e2cc3d1cf44a` |
| archive SHA-256 | `46dbed286dba9b3b6cedf7ef13340c43dcd56e068a5efa9af1deba0cabadd33d` |
| extracted-byte verification | all 26,712 tracked, ordinary-untracked, and ignored-untracked files byte-equal; source inventory stable during archive |
| clean repair worktree | `C:\hcm22ar-doc-repair` at the same exact HEAD on `codex/hcm-2-2-exact-result-authority-repair` |

No target file, index entry, staged state, branch HEAD, or rejected evidence byte
was reset, cleaned, staged, committed, discarded, or imported as implementation
authority. The three immutable implementation-review dispatches, original proof
wall, Review 1/2 remediation proofs, Review 3 stop proof, and original non-
authoritative research note were copied from the verified extraction byte-for-
byte. The original note remains exact in the complete archive; Review 1 later
required the documentation copy's rejected-worktree-only link to be replaced
by local proof evidence plus the preserved source path/hash.

## Review 3 admission and accepted finding

Fresh isolated Review 3 admitted the exact 199-path subject in
[`../../../handoffs/dispatches/20260721T050717Z--HCM-2-2--fresh-atomic-stage-implementation-review-3.json`](../../../handoffs/dispatches/20260721T050717Z--HCM-2-2--fresh-atomic-stage-implementation-review-3.json),
aggregate
`sha256:bb41f529990176f4554dcbdf4ed2e6354d710480065cdf0354c621613a0a924f`.
It returned Required finding `HCM-2.2-AR3-001`, accepted without waiver. The
coherent-rewrite RED changed only audit-only `validated_at_utc`, copied the new
bytes into result/witness, recomputed the mutable sidecar binding, and observed
author replay acceptance with unchanged semantic result and candidate `1.2`
identities. The full disposition is preserved in
[`20260721T053136Z--atomic-stage-implementation-review-3-stop.md`](20260721T053136Z--atomic-stage-implementation-review-3-stop.md).

## Selected repair

User-selected `HCM-2.2-ESC-003-D1` freezes candidate `1.3` as the independent
downstream exact-document anchor:

1. candidate-subject fingerprint stays complete and result-independent;
2. result `1.0` semantic identity still binds only that subject fingerprint;
3. exact persisted result JCS+LF digest/length enter one closed candidate
   binding;
4. final candidate identity includes the complete binding;
5. approval binds final candidate identity; and
6. promotion and intent `1.2` bind final candidate identity transitively.

The graph has a strict topological order and no back edge. Result self-hash and
mutable witness/binding/receipt/sidecar authority are prohibited. The result
`1.0` schema remains exactly 15,430 bytes with SHA-256
`1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`.
The intent `1.2` schema remains exactly 16,019 bytes with SHA-256
`c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`.

## Independently replayed exact identities

The vector transformation was replayed with sorted-key compact UTF-8 JCS and
independent SHA-256 calculations. Every prior published vector fingerprint was
first reproduced before candidate `1.3` was introduced.

| Vector | Subject | Semantic result | Exact result JCS+LF | Final candidate |
|---|---|---|---|---|
| create | `sha256:846ddf4906a093f26472cefc0e671587dd7da1e08bb781efd02cea96f3640281` | `sha256:e10efe2f45b808d48b3b399649c9e585727b56ea793a1948894d4265d032d9b8` | `sha256:c6696b0f9e8a8e3d1acf3fe916a3e18922869bf2cb9b2bee5457955a9488d538`, 4,334 bytes | `sha256:f2e8c3736c39c38e2f1b41bbbac7cdd839ee1fc954e77fe15e4909841cda4961` |
| amend | `sha256:8c87c62c7e95fafa12a307eadce3505d7974d97a80ea347f01fd0d1337b54747` | `sha256:08f67bbe6634d34aeaa07012353c50e73364079237036ab68413a8b0456a2aad` | `sha256:ce59629275937486506b3cc532bac5005a1f3ee93a7b0c4b277fc0d262b1fe0f`, 5,009 bytes | `sha256:723aa44c3a906511f6dc8c08984b99e89e581c5303e125b602059ab1bf0ffe99` |

The remediated runtime chain independently freezes fresh create approval
`sha256:ff88731faf9b327da39f9330dd71598ec22dd972f9126822b202db9d035e12f2`
and promotion
`sha256:8a1e5d73ac0163f4eb0acc53d39dbb4c7a027c5c2cae0cf3d80fd249733e1e63`.
The selected candidate-`1.3` amendment intent `1.2` is 7,591 bytes, with
fingerprint
`sha256:0df781964c952255ebc273e91753dca68a22562b7fd46198e6725524f94ba3f6`
and exact JCS+LF document SHA-256
`sha256:1fdddef8cd61f67c024d5f24fc0dd169f1aca2d87045efe1814f2ef54f374076`.

## Replay, orphan, migration, and recovery closure

The normative vectors freeze the author-lock recomputation order, discovery
selector pair, bounded no-follow complete inventory, and zero/one/many rows.
They classify result-without-candidate as an orphan, forbid adoption or cleanup,
and stop rather than invent automatic crash completion. Candidate `1.0`, `1.1`,
and `1.2` and their approvals remain immutable historical evidence; candidate
`1.3` reauthoring, result recomputation, and new approval are mandatory. Intent
`1.2` recovery reloads candidate `1.3` and verifies its exact result binding
before roll-forward.

The negative matrix explicitly covers coherent companion rewrite,
timestamp-only rewrite, forged second candidate, zero/one/many discovery,
orphan, missing result, wrong digest or LF-inclusive length, semantic/raw
crossing, unsafe entries, candidate `1.2` fallback, approval carry-forward, and
mutation or evidence deletion on refusal.

## GitNexus and scope

The baseline index at `486458ac` is current. Downstream implementation impact
is HIGH for `evaluate_charter_intake` (7 direct callers, 11 impacted symbols,
2 processes) and CRITICAL for generic `validate_record` (10 direct callers, 73
impacted symbols, 20 processes). This repair changes no indexed Rust symbol and
authorizes no implementation edit. A later implementation must refresh impact
before every symbol change and prefer the dedicated lifecycle-validation
service over expanding the generic validator.

## Review gate

Freeze the complete sorted path/raw-SHA-256 subject and submit the revised
identity, persistence, replay, orphan, migration, promotion, intent-recovery,
negative, task, status, decision, and proof surfaces to a different fresh
isolated read-only reviewer. Accept every finding without waiver, record each
remediation additively, and repeat with a different fresh reviewer until
`CLEAN`. Only then may the documentation authority commit and separate
handoff/ledger closeout commit be created. Stop before Rust and before HCM-2.3.
