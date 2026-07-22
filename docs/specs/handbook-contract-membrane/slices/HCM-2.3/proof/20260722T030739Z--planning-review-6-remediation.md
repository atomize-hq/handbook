# HCM-2.3 Planning Review 6 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_6`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 26 paths, aggregate
  `sha256:154205bcbd7e38f266ceed1c941e9ddd5613d8c7d763a63d503f62cd48d9b2d7`
- findings: zero Critical, one Required, zero Optional, zero Nit
- disposition: the Required finding was accepted; it was not waived

## Exact disposition

| Finding | Accepted defect | Remediation |
|---|---|---|
| `HCM-2.3-PR6-001` | established refusals were named as retained outcomes, but establishment, closed refusal bytes, journal transitions, evidence, crash recovery, replay, and tombstone behavior were not frozen | added the exact pre-establishment/established matrix; added a closed operation-discriminated `establishedRefusal`; extended intent, verified stage, marker, evidence, and result records with outcome/refusal bindings; required non-null evidence for refused results; froze zero artifact outputs and the only legal crash prefixes; added a complete positive refused-promotion chain, retained replay, active/retained/tombstone ledger entries, and 11 focused refusal attacks; aligned SPEC, implementation plan, and todo |

## Normative refusal lineage

The positive proof uses a second `artifact.candidate.promote` request whose null
expected-current value conflicts with the locked canonical YAML fingerprint.
Its exact transaction ID is
`artifact_candidate_promote_af1b9d68d5a96e43264f40cd70a18c4e3e539950468ffb3e17ef2204ff7d4901`.
The closed refusal is `stale_current_artifact` / `currentness`, with null
expected fingerprint and observed canonical fingerprint
`sha256:698d2414185110592a993896e8239f4a324a41e6293a3f77b1ec7538436ab63d`.

The terminal lineage reproduces as:

- request:
  `sha256:7dff956440b77ffcab2ecd55c4d59aa72e23e9ce3b35ac5b7b091f9fb3d12fc9`;
- outputless refusal intent:
  `sha256:329d38ed473f8de47c98daffa26ba200c35051f542d437eeb1d63b53b7532e8b`;
- refusal-decision verification:
  `sha256:ba226d06355bc618a46c72bfb83b1ba0d84ecb2bfe5d8502dfb2654c54b0267b`;
- refused marker:
  `sha256:ecd62ece4f4eaeaf423a1423943ca84313c013bd1ba319080cb2275eecb8de62`;
- internal evidence:
  `sha256:dd85621bc3048964c5b81eeec5c42913559324b6ac5ca99bf6dd9470edf2e304`;
- refused result and its exact replay:
  `sha256:95e70b06746bf162f4e05b853e37adb0a5aee7ef798fb08a2b8ff15757cbda85`;
- active, retained-result, and tombstone ledger entries:
  `sha256:5d2ab3784b4d069a4fdbf5fae24e50d4428f1591c5d3385c32eb72992cd54c3a`,
  `sha256:6c6ebe82f04de91ec74c9e0229bdc81fd67253327333a7dc3652df52a1023cf6`,
  and
  `sha256:03ebf8cc1e96b2de3b3b15b650b5f35ffefdf2fd50173f1023e351675b058f4d`.

All existing successful chains now carry explicit `commit`/`committed` plus a
null refusal. Their downstream intent, stage, marker, evidence, result, and
ledger fingerprints were regenerated; runtime semantic-record and persisted-
byte identities did not change.

## Recovery and refusal authority

For a new key, repository identity, exact operation context, and deterministic
operation evaluation complete before establishment. A pre-establishment control
failure writes no hold, journal, evidence, result, ledger, or artifact byte.
Only a complete commit plan or one closed established refusal may atomically
create the matching active hold and intent.

An established refusal repeats its complete refusal and exact operation,
transaction, request, context, and fingerprint chain through verified decision,
marker, evidence, and result. Its legal durable states are only the ordered
prefixes ending in retained-result ledger and committed rename. It never writes
an artifact output. Exact retained replay is byte-identical; same-key different
request conflicts; an exact tombstone request expires; a different tombstone
request conflicts. Missing-middle, output-bearing, evidence-less, mismatched,
duplicate, or non-prefix state refuses without mutation.

## Verification replay

Independent duplicate-safe Draft 2020-12 and semantic replay reported:

- all 39 control positive records validate against the closed control schema;
- all 39 terminal fingerprints reproduce under RFC 8785/SHA-256 exclusion;
- all success and refused transaction IDs reproduce the operation-discriminated
  preimage;
- the refused result has exact non-null evidence and zero authoritative outputs;
- the 40 negative vectors include every required refusal establishment,
  binding, crash, replay, and tombstone attack; and
- JSON parsing and `git diff --check` pass.

## Scope and next review

The remediation changed only the HCM-2.3 control schema/vector, runtime
contract, SPEC, plan, todo, and this proof record. It did not edit Rust, Cargo,
runtime tests, production assets, HCM-2.2 authority, a handoff ledger, or a
preservation location. The repaired complete documentation subject requires a
different fresh isolated read-only reviewer and remains unauthorized for
implementation or commit until that complete subject is CLEAN.
