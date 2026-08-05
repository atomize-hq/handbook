# HCM-3.2 Context Resolution finalization selector

Status: fresh-parent planning subject. Independent v1.4 selector review must
return CLEAN before any Rust, test, or fixture edit.

## Fresh authority and additive lineage

- parent orchestration:
  `20260805T014559Z--HCM-3-2--context-resolution-finalization`;
- integrated outcome:
  `hcm-3.2-context-resolution-finalization-whole-slice`;
- expected base: `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`;
- expected tree: `d51327c442190edcf1bf56423db54fd2522c5e6f`;
- publication: local-only expected-old compare-and-swap to
  `refs/heads/orchestration/handbook-hcm-3-2-fresh-review-20260805`; never
  push.

This decision is additive. The complete product semantics, compatibility
tuple, JCS declaration, 53-leaf payload, fingerprint domains, public surface,
and proof ceilings in
`decision/2026-08-01-context-resolution-kernel-selector.md` remain the
product contract except where this selector strengthens finalization and
finding-closure requirements. Handoff
`20260804T193800Z--HCM-3-2--orchestration--implementation-review-budget-exhausted`
is immutable source context only. It is not a same-parent continuation and is
never named in `supersedes`.

The quarantined checkpoint
`ffaa853fe91625244040d103a4d04bbfff214b1b` / tree
`e22b3a9e0a2321f311a7698a2e977dc9ec191791`, its invalid/blocked continuation
evidence, and the thirteen-path dirty worktree at
`C:\Users\spmcc\.codex\worktrees\9586\handbook` are read-only recovery input.
They are not authority, reviewed state, a baseline, or publishable history.
Recovery is selective and every admitted byte must pass this fresh selector,
proof wall, and independent review.

HCM-0.11 is withdrawn. No validator, handoff/dispatch schema or template,
orchestration semantic, skill, or other repository-tooling edit is selected.
Existing validators are execution-only gates.

## Frozen outcome registry and cadence

The exact registry preimage is this canonical JSON plus one LF:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/2026-08-05-context-resolution-finalization-selector.md","integrated_outcome_id":"hcm-3.2-context-resolution-finalization-whole-slice","packet_ids":["HCM-3.2-FINAL-P1-selector-and-recovery","HCM-3.2-FINAL-P2-product-remediation","HCM-3.2-FINAL-P3-proof-and-review","HCM-3.2-FINAL-P4-closeout"]}]
```

- registry fingerprint:
  `sha256:34ddf53e78a819f92d3adf6520883300b77a60a03af18e5d63d040f421a2ac0a`;
- causal budget:
  `sha256:965504ad2e1bde129646aab608915599ff3025c11f719b9b7e002cb8426f1882`.

Exactly these packet IDs are registered:

1. `HCM-3.2-FINAL-P1-selector-and-recovery`;
2. `HCM-3.2-FINAL-P2-product-remediation`;
3. `HCM-3.2-FINAL-P3-proof-and-review`;
4. `HCM-3.2-FINAL-P4-closeout`.

Planning/selector and implementation each receive one fresh default v1.4
cadence: one complete-subject discovery review or same-fingerprint burst, one
consolidated remediation, one different-fresh closure, and at most two
immediately causal supplementals. A supplemental may address only P1/P2
caused or unmasked by the preceding repair inside unchanged scope, authority,
and risk. No cycle follows CLEAN, no general discovery is reopened during
closure, and no identity rename resets the budget.

## Product objective and owner contracts

`handbook-engine` completes the bounded HCM-3.2 Context Resolution kernel and
private JCS authority capsule through the real generic owner pipeline:

```text
intake -> candidate -> HCM semantic validation -> authenticated registry
publication -> generic promotion -> committed lineage -> current authority
witness -> resolver/envelope/transition consumption
```

Only an authentic committed binding may persist, recover, replace, resolve, or
authorize consumption. Publisher credentials, direct predecessor registry
transition, capsule/payload/semantic fingerprints, generic candidate and
promotion records, exact predecessor quartet, repository/profile/stack
identity, and the live current registry mapping all verify fail closed.

Historical committed proof establishes only that an older authority was
legitimately committed. It cannot construct an admission, resolver, envelope,
cache entry, clone, request, disposition, or current witness. Promotion,
resolution, and every authority-bearing consumption require the current live
registry and generic-lineage witnesses.

The prior selector's complete ordered-stack, six-dimension, inheritance,
mutation, memory, validation, escalation, promotion, public-signature,
compatibility-tuple, JCS, bound, and fingerprint contracts remain exact.
HCM-3.1 vocabulary/profile/definition identities remain byte-identical.

## Exact production, test, and symbol boundary

The absolute production ceiling remains:

- `crates/engine/src/artifact_lineage_store.rs`;
- `crates/engine/src/artifact_mutation.rs`;
- `crates/engine/src/context_resolution.rs`;
- `crates/engine/src/context_resolution_registry.rs` only if fresh selector
  review proves it required;
- `crates/engine/src/lib.rs` only for the already-reviewed private HCM-3.2
  surface.

The planned production subject is exactly the first three paths. The registry
and library paths remain byte-identical. A sixth production path stops before
edit.

Selected existing symbol seams are:

- `validate_persisted_output_authority`;
- `validate_request_subject`;
- `GenericArtifactLineageStoreV1::recover_pending`;
- `GenericArtifactLineageStoreV1::verify_committed`;
- `ArtifactMutationServiceV1::promote`;
- `candidate_preview`;
- `build_promotion_plan`;
- `parse_promotion`;
- `PromotionDocumentV1`;
- `GenericArtifactLineageStoreV1::require_journal_authority`;
- `ContextResolutionEnvelope` and its existing resolution, mutation, memory,
  validation, candidate, escalation, and promotion-consuming methods.

The selected private HCM helpers and values are limited to capsule/JCS
validation; publisher verification; predecessor validation; current,
historical, pending, and corrupt proof states; current-witness loading;
registry-head gap detection; HCM quarantine validation/write/completion; and
committed current/historical lineage reads. No public signature changes.

Every existing function, method, class, or struct body receives live upstream
GitNexus impact analysis before edit. The previously accepted CRITICAL ceiling
is limited to the four lineage/authority functions named by the prior
selector. Any other HIGH/CRITICAL result is a stop before edit. The hand-written
ceilings remain 2,400 production lines, 2,400 focused test/fixture lines, 80
changed named production declarations, and 18 material test functions.

Test edits are limited to:

- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/hcm_2_3_generic_lineage.rs`;
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

No public API, dependency, crate, Cargo/version, unsafe/native/platform,
transport, generic framework, shipped definition/profile/vocabulary identity,
or second artifact family is selected.

## Transition state machine and proof modes

The only proof states are:

1. `CandidateValidated`: durable generic candidate evidence; non-capability;
2. `PublicationAuthorizedPendingCommit`: live H2 authorizes exact output but
   generic T2 is not committed; non-capability;
3. `HistoricalCommittedProof`: exact displaced bytes plus one unique committed
   direct successor verify history only; non-capability and non-cloneable;
4. `CurrentAuthorityWitness`: exact canonical bytes, committed generic
   lineage, direct live registry successor/mapping, publisher credential,
   predecessor, repository/profile/stack, and every fingerprint verify now;
5. `CorruptOrAmbiguous`: quarantine/refusal only.

State is derived from durable journal, canonical bytes, and live registry;
callers choose no historical/current mode. H0/candidate/H1/T1 and
H1/candidate/H2/T2 sequences are exact. After H2 and before T2, neither old nor
new authority is operational. A failed promotion preserves the previous
artifact as authentic history but never reactivates it as current authority.

Replacement preserves the exact predecessor quartet
`outer_artifact_ref`, `outer_artifact_fingerprint`,
`payload_byte_fingerprint`, and `semantic_binding_fingerprint`. Root uses four
literal nulls; replacement uses four exact strings; mixed-null or rebound
quartets refuse.

## Mandatory carried product findings

These stable IDs remain mandatory requirements and adversarial review targets;
they are not renamed, waived, or transferred as v1.4 finding ownership:

- `HCM32-JCS-IMPL-DISC-002` P1: `ContextResolutionEnvelope` and every
  clone must retain the live `CurrentAuthorityWitness`, not only an authority
  JSON value. Every authority-bearing operation revalidates that witness and
  re-runs the complete retained publisher challenge -> assertion -> decoded
  response chain through the real cryptographic verifier: signature, client
  data, RP-ID, flags, counter, credential/use-head, direct registry mapping,
  committed generic lineage, and current capsule bytes all verify together.
  Staleness makes mutation `Indeterminate` and makes memory, validation,
  child resolution, candidate/escalation construction, promotion, and
  transition admission fail with the existing stale/refusal types.
  Observational binding/dimension accessors confer no authority.
- `HCM32-JCS-IMPL-DISC-004` P2: every selected recovery phase and the complete
  selected reason/cardinality matrix must be implemented and proved. The
  matrix covers zero, one, and multiple candidate closures; installed-before-
  marker and every pending/staging/install/marker/evidence/result/ledger/
  rename durability boundary; exact retry/convergence; and closed refusal or
  quarantine for missing, conflicting, invalid, or wrong-predecessor state.
- `HCM32-JCS-IMPL-DISC-005` P2: quarantine validation covers every selected
  phase, rejects malformed or null optional values, validates the deterministic
  idempotency-key structure, and enforces exact transition/outer/candidate/
  result identity. Completion must match the open record's transition, outer
  fingerprint, candidate ref, candidate fingerprint, idempotency key, and
  committed T2 result. A different otherwise-valid candidate for the same
  transition cannot close or replace the quarantine record.
- `HCM32-JCS-IMPL-DISC-006` P2: proof and canonical documentation must state
  only observed currentness, recovery compatibility, quarantine identity, and
  command results. Planned, unavailable, or review-failed evidence is never
  GREEN.

The earlier repairs remain required and regression-proved:

- `HCM32-JCS-IMPL-DISC-001`: hash-consistent signature, client data,
  RP-ID, flags, counter, and challenge substitutions reach and fail the real
  cryptographic verifier rather than an earlier byte-equality guard;
- `HCM32-JCS-IMPL-DISC-003`: retry availability requires the exact authentic
  direct registry transition and one-mapping state delta, including current
  credential/use-head and capsule predecessor closure.

In addition to those stable finding scopes, markerless recovery for generic
non-HCM transactions remains exactly fail-closed. HCM registry-head recovery
is selected only after exact HCM kind/instance/operation/outer mapping and
candidate closure verification; it never broadens generic recovery. Findings
004 and 005 close only after the complete P3 recovery/quarantine subject and
matrix are implemented, proved, and independently reviewed.

## Fail-closed negative matrix

The complete implementation/proof subject covers at least:

| Boundary | Required refusal or convergence |
|---|---|
| malformed wrapper/declaration/payload, duplicate keys, BOM, trailing bytes, wrong tuple/version, limit overflow | refuse before authority or expensive semantic work |
| cross-domain fingerprint, repository/profile/stack, predecessor, candidate, transition, assertion, response, mapping, or credential substitution | refuse with no promotion/current witness |
| direct canonical seed, generic eligibility alone, marker-only or incomplete journal, restored old bytes | never create current authority |
| registry head advance, mapping loss, credential revocation, stale cache/admission/envelope clone, hash-consistent retained assertion/response substitution | every authority-bearing use re-runs the combined retained cryptographic/currentness proof and refuses or returns indeterminate as frozen above |
| failed promotion | previous canonical authority remains historical only; no stale operational fallback |
| H2/T2 crash before intent, pending, staging, installation, marker, evidence, result, ledger, or rename durability | exact retry/convergence or bounded transition-keyed HCM quarantine |
| missing, multiple, invalid, wrong-predecessor, or conflicting candidate/journal | closed reason/cardinality and non-capability quarantine |
| generic non-HCM markerless transaction | unchanged fail-closed generic refusal |
| quarantine completion using a different candidate | refuse and preserve the original open record bytes |
| O1 -> O2 -> O3 displacement | older exact bytes remain historically verifiable; latest committed current binding alone is consumable |
| concurrent retry | one deterministic serialized result or exact conflict; no ambiguous authority |

Clone/cache reconsumption, envelope mutation/memory/validation/escalation,
transition registry admission, and semantic-memory promotion are included, not
inferred from resolver-construction proof. Combined adversarial tests mutate
retained challenge/assertion/response fields after cloning and exercise every
one of those downstream consumption paths through the real verifier.

## Compatibility and deferred boundary

The existing public Context Resolution signatures, generic non-HCM intake,
candidate, promotion, journal, recovery, and currentness behavior remain
compatible. HCM-3.1 vocabulary and exact shipped profile/stack definitions,
and useful L0-L3 precursor behavior, replay unchanged. No flow/pipeline,
Projection, Snapshot, posture, SDK, CLI, Tauri, Substrate, HTTP, dock, or
transport adoption is selected.

The staged H2 installation-authorization / H3 operational-activation design in
`decision/2026-08-04-deferred-continuous-availability.md` remains future-only.
HCM-3.2 accepts the explicit fail-closed no-authority interval and does not
implement continuous availability.

## Proof wall and review

Before implementation discovery review, converge the real intake -> candidate
-> authenticated promotion -> committed recovery -> replacement ->
resolution/consumption path and recursive fixture/consumer manifest. Focused
proof covers every negative row above and the complete ordered-stack kernel.
Run:

- focused Context Resolution and generic-lineage tests;
- exact definition/profile/vocabulary fixture replay;
- all `handbook-engine` tests;
- workspace check and tests as applicable;
- strict workspace all-target/all-feature Clippy;
- formatting and `git diff --check`;
- ordinary handoff validation and both self-tests;
- GitNexus scoped and compare-to-main change detection before primary commit;
- remote feature-branch baseline and protected-path re-observation.

Closeout is also review-bearing. Before the sole mechanical closeout commit,
a fresh discovery reviewer checks the exact completed handoff/ledger draft,
primary-tip binding, validator evidence, expected two-commit topology, CAS
preconditions, and protected-path observations. Consolidate any findings and
obtain different-fresh CLEAN on the repaired exact draft. After the mechanical
commit, a fresh fingerprint-bound verification confirms the committed tree and
two-commit topology before local CAS; any P1/P2 is a non-completed stop rather
than a ref publication.

GitNexus FTS/comparison unavailability is recorded as unavailable, never
GREEN. Complete implementation discovery may use one same-fingerprint burst
with distinct security, state-machine/recovery, and compatibility/regression
lenses. Findings consolidate before one repair. A different fresh closure
reviews the changed subject and affected complete paths. No unresolved P1/P2
may reach proof completion, primary commit, or publication.

## Completion and stop conditions

Completion requires selector CLEAN, implementation/proof CLEAN, complete
real-path and negative proof, scoped/compare change detection, one reviewed
primary commit or stack, a separate mechanical v1.4 handoff/ledger commit, and
expected-old local ref CAS while the remote feature baseline and protected
paths remain unchanged.

Stop before edit or completion for a sixth production path, unexpected
HIGH/CRITICAL symbol, public API, dependency/crate, unsafe/native/platform or
transport surface, shipped identity change, generic ownership expansion,
H2/H3 implementation, HCM-3.3+, causal-budget exhaustion, mandatory delegation
failure, validator/tooling change, or unresolved P1/P2.
