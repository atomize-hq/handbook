# HCM-2.2 Planning Review 4 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_4`
- Dispatch:
  `20260719T150951Z--HCM-2-2--fresh-planning-review-4.json`
- Reviewed subject: 23 paths,
  `sha256:4570fc832b38ca977e2490161885c2ebb8520a1c865a434028b63acddc050149`
- Result: `CHANGES_REQUIRED`
- Required findings: 7

The parent accepted all seven findings. Review 4 authorizes no commit.

## Exact remediation

1. **Acyclic registry identity.** Registry state is now immutable and contains
   no prior-state or transition-head field. Transition records point from the
   prior state/head to the result state; result state never points back. Current
   authority is the committed `(state fingerprint, head fingerprint)` pair.
   The two exact schemas and closure vectors freeze that graph.
2. **Atomic registry authority.** Bootstrap/add/revoke/mapping update now share
   one registry lock, intent, staged state+transition, committed marker, reader
   rule, total recovery behavior, and invisible-orphan treatment. No mutable
   registry file or final-record deletion remains.
3. **Fixed initial quorum.** Bootstrap requires a non-empty exact ordered
   `initial_charter_quorum`; the bootstrap credential must cover all pairs and
   later registry states must preserve them. Initial promotion requires one
   fresh approval per pair and no caller-selected subset.
4. **Byte-closed authenticator protocol.** The selected protocol is native
   `fido2-ctap2.1-native` with RP ID `handbook.local`, exact JCS/clientDataHash/
   signed-preimage rules, registration and assertion schemas, UP/UV/counter
   checks, and wrong-RP/preimage negatives. The assurance claim explicitly
   makes no hardware-proven or real-world identity claim because the admitted
   `none`/self attestation policy has no shipped manufacturer roots.
5. **Per-record timestamp semantics.** The blanket wall-clock exclusion was
   removed. A class matrix and
   `runtime-record-fingerprint-vectors-v1.0.json` freeze own-ID/fingerprint
   removal, exact audit-only exclusions, semantic timestamps, expected
   fingerprints, and derived IDs for twelve runtime record classes.
6. **Ownership-free promotion recovery.** Promotion no longer records or
   infers `preexisting_equal` ownership. Rollback restores canonical truth but
   never deletes a final content-addressed record; uncommitted records remain
   invisible through journal-derived indexes. Optional unreachable-record GC
   is explicitly non-authoritative and out of scope.
7. **Durable standalone lifecycle events.** Observation/transition writes now
   have an exact lifecycle lock, intent, staged records, committed marker,
   reader rule, total recovery/replay/orphan behavior, crash/concurrency proof,
   and fixed promotion-then-lifecycle lock ordering. Promotion clearance still
   commits canonical, promotion, and lifecycle authority through one promotion
   journal.

## Exact machine-contract delta

- Added
  `contracts/authenticator-registration-1.0.0.schema.json`.
- Replaced the planned registry-state, registry-transition, and assertion
  schema documents with their remediated exact bytes.
- Added `contracts/runtime-record-fingerprint-vectors-v1.0.json`.
- Regenerated six schema-registry entries, eleven definition/profile closure
  vectors, and eleven literal RFC 8785 JCS lines.
- Added the authenticator-registration source to the complete selected-profile
  schema source list and the approval-policy dependency closure.

## Required replay before Review 5

The parent must independently validate all exact schema raw bytes, schema-entry
fingerprints, JCS line lengths/hashes, definition closure, profile source-list
equality, intake fingerprint/raw bytes, runtime-record vectors, Markdown links,
task state, diff whitespace, all three handoff-validator modes, retained
profile-schema tests, and the focused current baselines. It must then create a
new immutable complete-subject dispatch and use a different fresh read-only
reviewer. Only a later `CLEAN` result over unchanged bytes can authorize the
primary planning commit.
