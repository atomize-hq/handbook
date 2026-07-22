# HCM-2.3 Planning Review 2 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_2`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 15 paths, aggregate
  `sha256:f675b1ce97c8f7f316f722f05152129e6ab0fe0e52510ae36ffa10039c9eacea`
- findings: zero Critical, seven Required, zero Optional, zero Nit
- disposition: every Required finding was accepted; none was waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| `HCM-2.3-R2-001` | generic intake `1.1` collided with immutable HCM-2.2 Charter `1.1` | allocated generic intake `1.2`, preserved `1.0`/`1.1`, changed the closed schema and every current packet reference, and recomputed the intake, validation, candidate, and promotion identity chain; the Review 1 remediation remains immutable historical evidence and is superseded on this point by this record |
| `HCM-2.3-R2-002` | the positive selection record omitted the shipped parent closure and semantic duplicate rules | bound both profiles, the stable-role registry, all 17 shipped schemas plus the custom schema, all seven shipped kinds plus the custom kind, the constitutional capability, two validators, condition, shipped vocabulary/Context Resolution, all three policies, Charter intake plus custom intake, and exact replacement/count expectations; added same-ref/different-source, cross-class, incomplete-source, and incomplete-replacement refusals and recomputed the normalized fingerprint |
| `HCM-2.3-R2-003` | Phase 4 derived a different owner-domain key | froze one transport-independent owner-domain key/request preimage; future outer transport state passes and binds the same key/request pair and cannot derive a second inner namespace; required both cross-era orderings, conflict, tombstone, and every inter-ledger crash gap |
| `HCM-2.3-R2-004` | operation context/evaluation, domain request/result/ledger, journal, marker, evidence, and receipt mapping were prose-only or incompatible | added one closed Draft 2020-12 control-record schema and normative fingerprint vectors for 11 positive records plus negative/state rules; HCM-2.3 local results now report authoritative outputs and separately typed internal evidence, while Phase 4 alone constructs the canonical six-field `WriteReceipt` with one receipt per authoritative output |
| `HCM-2.3-R2-005` | recovery omitted complete-installed/lost-staging and journal lifecycle states | admitted `installed_complete` verification only for the exact complete installed set, covered marker-backed replay with absent/corrupt staging, rejected marker plus incomplete installed output, froze the total post-marker `E/D/L/Q` prefix matrix, and retained bounded `.committed` journals without cleanup/compaction or deletion promises |
| `HCM-2.3-R2-006` | the one-cell ceiling would leave real generic intake classified stale | defined one atomic maximum classification change set with exactly two subset cells: exact registry-brief kind/schema registry and exact registry-brief intake coverage each may move `TargetOnly -> RealPathAdopted`; the Charter cell, broader generic intake, and every other cell remain fixed |
| `HCM-2.3-R2-007` | Review 2 did not subject-bind the prior Review 1 dispatch | the next manifest must include both prior dispatches, both prior remediation/proof artifacts, this remediation, every packet/control correction, and exclude only its own new active dispatch; primary-commit inventory minus review subject must equal that single active dispatch |

## Closed machine authority added

The repaired packet now has six duplicate-safe JSON artifacts, three Draft
2020-12 schemas, four runtime semantic-record positives, eleven control-record
positives, and one complete repository-selection positive. The current oracle
replays:

- selection fingerprint
  `sha256:d9c4e5c25c07b355f26de7dc6e4e53c5f98fe45f21185085da470db77b93e050`;
- generic intake `1.2`
  `sha256:34c78ba4975b2abbb629cdf0b3652384063216887ee4081b30e69df21786dab1`;
- validation result
  `sha256:9f36a05e99e6c2b006b7c63f34e2cddb45cc55caa328b46f627464030747af88`;
- candidate `1.4`
  `sha256:205e804019566fd44e853a9528382a9d21cb50aa0b691b07c8c1719b9bb697bf`;
- promotion `1.2`
  `sha256:0262025ec5447a2a5328f357d0a239d55ecf5ae15751ab4cfd252a0eaad0dcc7`;
- all eleven control-record terminal fingerprints; and
- the unchanged 67-byte canonical YAML digest
  `sha256:698d2414185110592a993896e8239f4a324a41e6293a3f77b1ec7538436ab63d`.

An independent local validation pass reported: `PASS 6 JSON duplicate parse; 3
Draft2020-12 schemas; 16 fingerprint vectors`. Runtime semantic negatives now
also reject duplicate candidate field-source identities and duplicate promotion
definition identities even when JSON object inequality would satisfy
`uniqueItems`.

## Scope and next review

The remediation changed only the HCM-2.3 planning packet. It did not edit Rust,
Cargo, runtime tests, production assets, HCM-2.2 authority, handoff ledger, or a
preservation location. `git diff --check` passed after remediation. The repaired
complete documentation subject requires a different fresh isolated read-only
reviewer and remains unauthorized for implementation or commit until that
subject is CLEAN.
