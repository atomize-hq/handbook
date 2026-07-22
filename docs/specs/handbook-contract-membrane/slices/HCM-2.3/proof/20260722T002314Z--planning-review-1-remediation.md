# HCM-2.3 planning Review 1 remediation

> **Status:** all nine Review 1 Required findings are accepted and remediated
> in the proposed documentation subject. This record does not claim `CLEAN` or
> authorize implementation. A different fresh exact-subject reviewer must
> independently admit the remediated bytes.

## Review admission and result

Fresh isolated read-only Review 1 used
[`../../../handoffs/dispatches/20260721T234549Z--HCM-2-3--fresh-planning-review-1.json`](../../../handoffs/dispatches/20260721T234549Z--HCM-2-3--fresh-planning-review-1.json).
It replayed all nine raw path hashes, Python-ordinal order, and aggregate
`sha256:508b3521ddce807b7d56464895359e20f53a6ea56df7d777e434c2d12008ed40`,
then returned `CHANGES_REQUIRED`: zero Critical, nine Required, zero Optional,
and zero Nit findings. The reviewer remained read-only.

## Findings and parent disposition

| ID | Accepted finding | Bounded remediation |
|---|---|---|
| `HCM-2.3-PR1-001` | evaluation/candidate identity and validate/append order were impossible | froze evaluate -> finalized intake append -> zero-write candidate validate from committed intake -> independently reproduced candidate append -> promote; the CLI and real-binary sequence now follow it and test every inter-operation restart |
| `HCM-2.3-PR1-002` | old record versions were reused with new fields/values | allocated intake `1.1`, timestamp-free generic validation result `1.0`, candidate `1.4`, and promotion `1.2`; added one exact closed runtime schema and normative fingerprints; preserved every old version without fallback |
| `HCM-2.3-PR1-003` | validation-result authority was unspecified | froze its producer, timestamp-free shape, fingerprint/ref derivation, zero-write preview, candidate-append persistence as subordinate closure, and promotion-time reload/recomputation/currentness/refusal |
| `HCM-2.3-PR1-004` | operation context in key scope allowed drift to create a second mutation and conflicted with Phase 4 | removed context from key scope, made lookup/replay precede currentness, froze retained results/tombstones/recovery holds, and separated direct-CLI owner-domain replay from the future canonical outer Phase 4 ledger; the outer ledger derives and wraps an inner key without migration or reinterpretation |
| `HCM-2.3-PR1-005` | value/content/result objects made the canonical write sets ambiguous | classified them as non-independent subordinate closure; intake/candidate each retain exactly one semantic-record receipt, promotion retains exactly two authoritative receipts, and closure has no standalone receipt/list/read/authority/adoption/deletion |
| `HCM-2.3-PR1-006` | journal/inventory/recovery authority was incomplete | froze transaction-ID and file-name grammar, intent/verified/marker fields, deterministic no-follow inventory and limits, lock order, exact output sets, and a total table over every intake subset, eight candidate subsets, and four promotion subsets |
| `HCM-2.3-PR1-007` | `.handbook/profile-selection.json` was not executable/closed and list request semantics contradicted the blanket request rule | added a Draft 2020-12 closed schema and normative vector with every source-class field, both effects, typed mapping, intake-source field, fingerprint, negatives, and order equivalence; limited kind/instance requirement to instance-targeted operations |
| `HCM-2.3-PR1-008` | actual intake proof appeared to require a second seam movement | froze the only cell movement as the exact custom subset of `Artifact kind/schema registry`; registry-brief intake is that seam's compatibility dimension, while broader generic/custom intake and every other cell stay unmoved and receive exact diff assertions |
| `HCM-2.3-PR1-009` | the final-review instruction required a dispatch to hash itself | made the active dispatch transport outside its manifest; it may bind prior review artifacts, validates independently, and enters the commit only as an audit artifact after no reviewed subject byte changes |

No finding was waived. `HCM-2.3-PR1-004` uses a two-ledger boundary rather than
inventing the not-yet-authorized Phase 4 bootstrap descriptor. It satisfies the
finding's replay/currentness/retention defect while preserving the canonical
Phase 4 outer scope and the explicit prohibition on premature SDK/transport
authority. A different fresh reviewer must reject it if that separation is not
actually sufficient.

## Added authority artifacts

- [`../contracts/repository-profile-selection-1.0.0.schema.json`](../contracts/repository-profile-selection-1.0.0.schema.json)
  closes the fixed effect-binding record;
- [`../contracts/repository-profile-selection-vectors-v1.0.json`](../contracts/repository-profile-selection-vectors-v1.0.json)
  freezes mapping, fingerprint, order equivalence, and attacks;
- [`../contracts/generic-artifact-runtime-records-1.0.0.schema.json`](../contracts/generic-artifact-runtime-records-1.0.0.schema.json)
  closes all four additive record shapes;
- [`../contracts/generic-artifact-runtime-vectors-v1.0.json`](../contracts/generic-artifact-runtime-vectors-v1.0.json)
  freezes content, record, and canonical-YAML bytes and fingerprints; and
- [`../contracts/generic-artifact-runtime-contract-v1.0.md`](../contracts/generic-artifact-runtime-contract-v1.0.md)
  freezes sequence, closure authority, idempotency boundary, journals,
  inventory, total recovery, and compatibility proof.

These are planning contracts and vectors, not shipped definitions, production
assets, runtime fixtures, operation catalogs, or implementation.

## Independent machine replay

The parent used duplicate-rejecting JSON parsing for all four added JSON files,
validated both schemas with Draft 2020-12 schema checking, validated the
positive selection record with format checking, and validated all four runtime
records against the runtime schema. Result:

```text
json_files=4 schemas=2 selection_vectors=1 runtime_records=4
duplicate_keys=0 schema_validation=passed
```

An independent recursive sorted-key JCS/SHA-256 replay reproduced:

- selection normalized fingerprint
  `sha256:9f7feca616b30d6ddfa086ac1dc1bd8c4d6eeb23e10b093c183cec225358a636`;
- both value refs/fingerprints;
- normalized content fingerprint, persisted JCS+LF SHA-256, and 71-byte length;
- intake `1.1`, validation result `1.0`, candidate `1.4`, and promotion `1.2`
  subject fingerprints and derived IDs; and
- exact deterministic YAML SHA-256
  `sha256:698d2414185110592a993896e8239f4a324a41e6293a3f77b1ec7538436ab63d`
  and 67-byte UTF-8/LF length.

`git diff --check` passes after remediation. The complete link/fence/reference,
archive, handoff, preservation, scope, and change-detection wall must rerun over
the final exact subject before the different-fresh review dispatch.

## Re-review gate

Freeze all remediated packet, contract, decision, task, proof, and coupled
control-pack paths as a new sorted raw-byte manifest. The active Review 2
dispatch stays outside its own subject, contains no parent success conclusion,
and instructs a different fresh isolated read-only reviewer to re-audit the
entire subject plus every Review 1 disposition. Only a zero-finding `CLEAN`
result can advance to the primary planning commit.
