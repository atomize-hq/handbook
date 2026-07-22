# HCM-2.3 Planning Review 3 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_3`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 20 paths, aggregate
  `sha256:7955e55b62ee2dce83f5488f6a855f3795d2a3d2be276b59e30e2f27c1b41e0a`
- findings: zero Critical, four Required, zero Optional, one Nit
- disposition: every Required finding and the Nit were accepted; none was
  waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| `HCM-2.3-R3-001` | repository-selection array order was described as non-semantic, but the stated raw-record fingerprint changed when source arrays were reordered | froze pre-fingerprint normalization: sort every `*_sources` array by UTF-8 `exact_ref` bytes, sort `allowed_schema_roots` by UTF-8 bytes, leave every other value unchanged, then fingerprint the complete normalized record with RFC 8785/SHA-256; added a concrete fully reversed equivalence record; both records reproduce `sha256:3a5c945e68c29fa765bc4568ff4eae1eacbd84a9f90a71a00a56bb5d04cc82c9`; recomputed the operation-context and dependent control-record fingerprints |
| `HCM-2.3-R3-002` | the intake transaction persisted only its semantic record and used illustrative bytes | made the exact intake output tuple title value, summary value, intake record; the value scalar JCS+LF bytes are respectively 23 bytes / `sha256:49f5c34d7b957af39cefd0769b451e96114517a3938c90976b6fb280b1627471` and 28 bytes / `sha256:64e46d8559a68c5ee0747cd42dc662fa1f6a1ec3a774271e90e434cf4b571d89`; the complete intake-record JCS+LF bytes are 2,052 bytes / `sha256:f0acd798dbe8f2c92b31ac7cbf6a8a43e3d181d2a6e61f85fcb91218149adcf3`; regenerated intent, verified stage, marker, evidence, result, and ledger fingerprints from those exact outputs |
| `HCM-2.3-R3-003` | the control schema did not close operation/output mappings or exact ref/path grammar, and had no candidate/promotion positive chains | added operation-discriminated request, transaction-family, intent tuple, staged tuple, marker, evidence, result, and ledger constraints for all three operations; added concrete candidate and promotion chains, bringing the control-positive count to 25; tightened exact-ref and safe local-path grammars; added attack vectors for ref casing/hyphen grammar, unsafe path segments, cardinality, authority class, transaction family, and atomic group |
| `HCM-2.3-R3-004` | HCM-2.3 required execution tests against a Phase 4 outer state model that this slice neither owns nor defines | limited HCM-2.3 proof to owner-side derivation and replay invariants, including rejection of transport metadata from the owner preimage; explicitly deferred outer-ledger transition orderings and inter-ledger crash execution fixtures to Phase 4; preserved the rule that Phase 4 may pass but never reinterpret the one owner key/request pair |
| Nit | the runtime schema's internal definition name still said intake v1.1 | renamed the internal `$defs` key and reference to `intakeRecordV12`; wire schema identity remains the already-selected `1.2` |

The immutable Review 2 remediation remains historical evidence. Its prior raw
selection fingerprint and its requirement for HCM-2.3 cross-era/inter-ledger
execution tests are superseded by this remediation and by the current normative
packet; no immutable review artifact was rewritten.

## Verification replay

An independent duplicate-safe validation replay reported:

- two repository-selection records schema-valid and normalization-equivalent;
- four runtime semantic records schema-valid with exact terminal identities;
- 25 control records schema-valid with every terminal fingerprint replayed;
- all intake, candidate, and promotion transaction IDs reproduced from the
  closed preimage;
- every intent/stage byte tuple matched exact runtime persisted bytes;
- every marker/evidence/result/retained-ledger dependency agreed exactly; and
- seven concrete mutated records were rejected for uppercase exact ref,
  doubled-hyphen exact ref, empty path segment, operation cardinality,
  operation authority class, transaction family, or atomic group.

## Scope and next review

The remediation remains documentation-only. It did not edit Rust, Cargo,
runtime tests, production assets, HCM-2.2 authority, a handoff ledger, or a
preservation location. The repaired complete documentation subject requires a
different fresh isolated read-only reviewer and remains unauthorized for
implementation or commit until that complete subject is CLEAN.
