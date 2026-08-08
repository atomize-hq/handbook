# HCM-3.6 — Project posture and atomic constitutional authority

**Packet ID:** `HCM-3.6-ATOMIC-AUTHORITY-REPLANNING`

**Status:** implementation-ready documentation authority; no product
implementation is authorized

**Integrated outcome:** `hcm-3-6-atomic-authority-replanning`

## Authority and supersession

The reviewed selector for this current packet is
[`decision/20260808T041700Z--hcm-3-6-atomic-authority-replanning-selector.md`](decision/20260808T041700Z--hcm-3-6-atomic-authority-replanning-selector.md).
It resolves only the atomic-authority gap named by the immutable P0 true stop
`HCM36-P3-ATOMIC-TRANSITION-PRIMITIVE`. The prior planning/P0 selectors,
primary commit `cab94769bcc14049ecd42378cf34462079fae386`, CLEAN review, and
authority-boundary handoff remain historical evidence and are not reopened.

This packet is documentation and read-only source analysis only. It decides a
complete future private implementation contract. Rust, product tests, fixtures,
definition/schema assets, dependencies, configuration, public APIs, adapters,
transports, HCM-5, release, protected paths, and remote work remain
unauthorized until a new implementation selector admits the entire source
ceiling and proof wall.

## Invariants

1. Canonical `.handbook/project/charter.yaml` is the only editable
   constitutional authority. `ProjectPostureKernel` is derived, never another
   writable artifact.
2. The nine engineering-posture dimensions are global and remain distinct from
   all six Context Resolution dimensions. `causal_scope_ref` is evidence
   metadata only.
3. A recommendation is immutable and advisory. Only one authorized
   `PostureTransition` may cause one mapped Charter leaf change.
4. Candidate promotion and posture transition are different semantic records
   but participate in one canonical-authority head chain.
5. Current reads, lifecycle events, recovery, and later promotion must resolve
   the same terminal Charter bytes and lifecycle head.
6. HCM-5 remains the only contract-gate verdict and parent-promotion owner.

## Private owner and call boundary

`handbook-engine::project_posture` is the sole posture semantic owner and stays
a private module. It owns exact input validation, kernel normalization and
fingerprints, policy evaluation, recommendation identity, physical mapping,
transition admission, and resulting-kernel replay. The existing private
Charter authority transaction remains the physical write owner.

```text
exact Charter/profile/condition/contract/evidence/snapshot pairs
  + explicit FreshnessEvaluationBasis
  -> resolve_project_posture_kernel
  -> ProjectPostureKernel
  -> evaluate_posture_recommendation
  -> NoRecommendation | Recommended | Refused

reviewed recommendation + exact approval + current Charter/lifecycle head
  + mapped reassessment proof
  -> apply_posture_transition
  -> private Charter authority transaction
  -> canonical Charter + PostureTransition + lifecycle rebase
  -> re-resolved ProjectPostureKernel
```

Flow, pipeline, SDK, CLI, Tauri, Substrate, delivery adapters, HCM-3.4 private
Snapshot types, HCM-3.5 views, and HCM-5 do not become posture owners. Future
adapters may receive typed notification/acknowledgement intent only under
separate authority.

## Exact inputs, kernel, recommendation, and refusal

Every semantic source is one exact ref/fingerprint pair. The selected profile
also binds `resolved_profile_fingerprint`; every freshness-qualified input binds
one explicit immutable `FreshnessEvaluationBasis`. Missing, mutable-only,
ambiguous, stale, changed-behind-ref, unresolved-required-condition, unknown
freshness, or insufficient-evidence inputs refuse fail-closed.

The kernel carries the constitutional pair, selected-profile pair, applicable
override/condition/contract/evidence/snapshot pairs, optional freshness pair,
and all nine derived global dimensions with effective level, floor, red line,
trigger, shortcut, and proof refs. Semantic lists without order meaning sort
byte-lexically. Presentation IDs, recommendation refs, and resolution
timestamps do not enter `input_fingerprint` or `kernel_fingerprint`.

Hard triggers and policy-local accumulated rules remain distinct. One
recommendation binds one kernel/policy pair, exactly one dimension transition,
one causal scope, one mutually exclusive trigger source, exact evidence and
freshness, hysteresis, approval, notification semantics, suggested actions,
and `recommendation_only`. An event affecting two dimensions produces two
recommendations. Lowering requires the configured sustained window and
cooldown and cannot cross a floor or red line.

Expected semantic negatives return a closed `NoRecommendation` or `Refused`
outcome. Repository, I/O, durability, and atomic-integrity failures are
operation errors. Neither outcome can mutate Charter, create a gate verdict,
or promote a parent.

## Physical one-dimension mutation

The exact canonical `1.1` Charter map is fixed:

```text
0 speed_vs_quality
1 type_safety_static_analysis
2 testing_rigor
3 scalability_performance
4 reliability_operability
5 security_privacy
6 observability
7 dx_tooling_automation
8 ux_polish_api_usability
```

For index `i`, the only admitted leaf is
`/engineering_posture/dimensions/i/level_override`. The engine verifies the
stored `dimension_id` at `i`; a keyed pseudo-path is invalid. The expected
physical value is `null` or integer `1..=5`. With unchanged baseline `B`, the
expected effective level is `old_override.unwrap_or(B)`. Proposed effective
level `P` serializes as `null` when `P == B`, otherwise integer `P`.

The transition cannot target `posture.baseline_level`: that could alter every
dimension whose override is null. It cannot change dimension order, ID,
stance, trigger, shortcut, red-line, domain-override, another dimension, or an
approved override source. A same-effective proposal is a no-op refusal, even
if it could remove a redundant stored override.

Admission binds the complete current Charter bytes, byte length and SHA-256,
the current canonical fingerprint, expected physical value, unchanged
baseline, expected effective level, and proposed physical/effective levels.
The engine parses and deterministically serializes the entire typed Charter and
requires a deep diff of exactly that one leaf. Precommit CAS then byte-compares
the live Charter with the retained old bytes; fingerprint-only or substring
matching is insufficient.

## Immutable PostureTransition identity

The three private records use the exact closed grammar in `05`. A
`Fingerprint` is `sha256:` plus 64 lowercase hex characters; `Pair` is exactly
`{ref, fingerprint}`; `CanonicalDocument` is exactly `{ref, fingerprint,
document_sha256, byte_length}`; and `OutputRecord` is exactly `{ref,
fingerprint, document_sha256, byte_length}`. Canonical length is
`1..=1_048_576`, record length is `1..=262_144`, and the canonical fingerprint
equals its document SHA-256. IDs/refs/digests/enums/times are JSON strings,
lengths are JSON `u64`, indexes/levels/floors are JSON `u8`, and floats or
numeric strings refuse. Generic refs are bounded `1..=512` safe UTF-8
bytes. Every object denies unknown and missing fields, every array is non-null,
every semantically unordered `Pair` array is unique and strictly sorted by the
lexicographic tuple (`ref` UTF-8 bytes, `fingerprint` UTF-8 bytes), and every
other semantically unordered ref array is unique and strictly sorted by element
UTF-8 bytes. Ref-only and fingerprint-only `Pair` ordering are invalid.
`extensions` is exactly `{}`. The only nullable fields are the two
`Change` stored values and `kernel_replay.freshness_basis`; stored values are a
level or JSON null, and freshness is a complete `Pair` or JSON null.

`AuthorityHead` has exactly `kind`, `source`, `canonical`,
`lifecycle_transition`, and `promotion_ancestor`. Kind is `promotion` or
`posture_transition`; all latter fields are complete pairs/documents. Content
refs use exact `<64-lower-hex>` basenames equal to their fingerprint suffixes.
Posture is never genesis, so no head field is nullable.

`Change` has exactly `dimension_id`, `dimension_index`, `authority_path`,
`operation`, `baseline_level`, `expected_stored_value`,
`expected_effective_level`, `proposed_stored_value`, and
`proposed_effective_level`. Index is `0..=8`; levels are `1..=5`; stored values
are a level or JSON null; operation is only `replace`; ID/index/path select one
fixed-map row; stored/effective algebra uses the unchanged baseline; and old
and new effective levels differ.

`Reassessment` is exactly `{intake_definition: Pair,
affected_coverage_ids: ["engineering_posture.dimensions"],
validation_result_inputs: [Pair]}`. Validation and approval arrays each have
`1..=16` unique pairs sorted by `(ref, fingerprint)`. Intake definition is the
exact pair `handbook.intake.charter@1.0.0` /
`sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c`.
`KernelReplay` has exactly
the following fields:

```text
constitutional_artifact_ref, source_authority_fingerprint,
resulting_authority_fingerprint, source_input_fingerprint,
resulting_input_fingerprint, profile_input,
override_inputs, condition_inputs, contract_inputs, evidence_inputs,
snapshot_inputs, freshness_basis, dimensions,
applicable_scope_refs, omitted_condition_refs, unresolved_condition_refs
```

The constitutional ref is `.handbook/project/charter.yaml`; `profile_input`
and every `*_inputs` member are `Pair`s; pair/ref lists are `0..=256` and
unique, with `Pair` lists sorted by the exact tuple above and string ref lists
sorted by element UTF-8 bytes. `dimensions` has exactly the fixed nine-ID
order. Each closed dimension has exactly `dimension_id`,
`source_effective_level`, `resulting_effective_level`, `floor`,
`red_line_refs`, `trigger_refs`, `allowed_shortcut_refs`, and
`proof_obligation_refs`; levels are `1..=5` and ref lists are
bounded/sorted/unique. Only the selected dimension may differ, and its two
levels equal `Change`. The resulting input fingerprint is derived by
substituting the resulting constitutional pair into the normalized source
closure. Raw Snapshot/delta payload, ambient time/lookup, recommendation refs,
and a second authority are forbidden.

The exact `handbook.posture-transition` / `1.0` top level is:

```text
schema_id, schema_version, transition_id,
recommendation: Pair, source_kernel: Pair, evaluation_policy: Pair,
target_authority_ref, target_authority_class,
prior_authority_head: AuthorityHead,
expected_canonical: CanonicalDocument,
change: Change,
approval_inputs: [Pair], authorized_by_ref,
reassessment: Reassessment,
kernel_replay: KernelReplay,
resulting_canonical: CanonicalDocument,
resulting_kernel: Pair,
effective_at_utc, extensions, transition_fingerprint
```

Target ref/class are `.handbook/project/charter.yaml` /
`constitutional_root`; actor ref is `1..=256` safe bytes; and audit time is
canonical UTC seconds. `transition_fingerprint` is lowercase SHA-256 over RFC
8785 JCS excluding only `transition_id`, `transition_fingerprint`, and
`effective_at_utc`. ID/ref derive as `posture-transition_<hex>` /
`posture-transitions/posture-transition_<hex>.json`; persisted bytes are exact
JCS plus one LF and are at most 262,144 bytes.

The semantic record includes the prior lifecycle pair but excludes every
resulting lifecycle ref, fingerprint, hash, and length. Construction is
strictly acyclic:

```text
PostureTransition semantic closure
  -> PostureTransition fingerprint/ref/exact JCS+LF bytes
  -> lifecycle-transition 1.1 semantic closure binding that posture pair
  -> lifecycle fingerprint/ref/exact JCS+LF bytes
  -> posture intent binding exact Charter + both exact record outputs
  -> exact stages/markers -> committed merged edge
```

Lifecycle identity can depend on posture identity; posture identity never
depends on lifecycle identity. The intent independently binds the selected
document SHA-256/length, so changing an excluded audit timestamp is an exact-
byte mismatch even though the semantic ID is stable.

## One canonical-authority head

The private current-head resolver merges two immutable edge kinds:

- promotion journal `handbook.charter-promotion-transaction-intent` / `1.2`;
- posture journal `handbook.charter-posture-transaction-intent` / `1.0`.

Each committed edge binds an optional/prior basis fingerprint, resulting
canonical fingerprint/document hash/length, semantic source record pair, and
resulting lifecycle-head pair. The chain has one promotion genesis. Posture
cannot create initial Charter authority. Across both partitions, each basis
has exactly one successor; every committed edge is reachable; cycles, forks,
duplicate successors, disconnected histories, unsafe terminals, and histories
beyond 4,096 edges refuse.

The existing public committed-Charter read shape is preserved. It returns the
terminal Charter bytes/fingerprint, the most recent promotion ancestor in its
existing `promotion_ref`, and the terminal lifecycle head in its existing
`lifecycle_transition_ref`. The internal head also carries source kind and
source ref/fingerprint. No public enum/field/export or Flow edit is required.

A later candidate promotion may use a posture result fingerprint as its
expected current basis. Its unchanged promotion record remains the next
semantic edge. Promotion-only terminal validation still proves every promotion
journal/output; only cross-edge head selection becomes heterogeneous.

## Lifecycle compatibility

Existing lifecycle-transition `1.0` records remain immutable and continue to
represent promotion/event results. A posture commit adds one private
`handbook.lifecycle-transition` / `1.1` record in the same existing
content-addressed partition. Its exact closed posture-rebase top level is:

```text
schema_id, schema_version, transition_id, transition_kind,
lifecycle_policy: Pair, target_instance_id,
prior_transition: Pair,
prior_state, prior_state_fingerprint,
new_observation_refs, active_observation_refs,
result_state, result_state_fingerprint,
prior_canonical_fingerprint, resulting_canonical_fingerprint,
authority_transition: Pair,
reassessment: Reassessment,
transitioned_at_utc, transition_fingerprint, extensions
```

Constants are schema `handbook.lifecycle-transition` / `1.1`, kind
`posture_transition`, target `project_authority`, both states `current`, empty
new/active observations, exactly one affected coverage ID
`engineering_posture.dimensions`, and empty extensions. Lifecycle policy is
the exact pair `handbook.lifecycle.constitutional-review-lock@1.0.0` /
`sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3`.
Both state fingerprints equal the canonical fingerprint of lifecycle state
`current`. Prior transition
uses the lifecycle content-ref grammar and may name an admitted `1.0` or `1.1`
record. Prior/result canonical fingerprints equal the PostureTransition
expected/result bindings; `authority_transition` equals the already completed
PostureTransition pair; reassessment is byte-for-byte equal; and
`transitioned_at_utc == effective_at_utc`. `transition_fingerprint` is SHA-256
over RFC 8785 JCS excluding only ID, fingerprint, and audit time; ID/ref derive
as `lifecycle-transition_<hex>` /
`lifecycle-transitions/lifecycle-transition_<hex>.json`. Persisted bytes are
exact JCS+LF and at most 262,144 bytes. Missing, extra, unknown-enum, nonempty
observation, bad-bound, null, reordered, or substituted nested data refuses.

Posture admission requires lifecycle state `current` with no active
observations, so it cannot clear unrelated review or reassessment work. The
mapped coverage is re-evaluated against the proposed Charter before commit and
recorded by the rebase. The lifecycle loader privately discriminates v1.0 and
v1.1; the public lineage-record enum does not gain a variant.

After posture, lifecycle observation/event recording chains from the v1.1
head. A later promotion may cite that same head in its validation result and
then emit its unchanged v1.0 promotion lifecycle transition. No observation,
event, promotion, or recovery path may independently select another Charter.

## Atomic journal and recovery

The posture transaction reuses the existing lock order
`promotion.lock -> registry.lock -> lifecycle.lock`; the historical lock name
does not grant promotion-only authority. Its journal root is
`.handbook/state/transactions/posture-transitions/` with create-new
`<transaction_id>.pending`, `.committed`, and `.rolled-back` terminals.

The `handbook.charter-posture-transaction-intent` / `1.0` record is closed,
bounded exact JCS+LF. Its exact top-level fields are:

```text
schema_id, schema_version, transaction_id, mutation_mode,
basis_head: AuthorityHead,
expected_canonical: CanonicalDocument,
change: Change,
authority_inputs {
  recommendation: Pair, source_kernel: Pair, evaluation_policy: Pair,
  approval_inputs: [Pair], authorized_by_ref,
  reassessment: Reassessment
},
outputs {
  canonical: CanonicalDocument,
  posture_transition: OutputRecord,
  lifecycle_transition: OutputRecord
},
kernel_replay: KernelReplay,
recovery { old_canonical_status, old_canonical: CanonicalDocument },
intent_fingerprint
```

`mutation_mode` is only `single_dimension_level_override_replace`.
`old_canonical_status` is only `present`; posture cannot create genesis, so no
basis/recovery value is nullable. Transaction ID is internally allocated as
`posture-transaction_` plus exactly 32 lowercase hex characters from 128 random
bits, is collision-checked across all three suffixes, and is not caller input.
`intent_fingerprint` is SHA-256 over RFC 8785 JCS of every field except itself,
including transaction ID. Intent bytes are exact JCS+LF and at most 262,144
bytes.

Before a pending journal is created and again on every recovery pass, equality
is total: intent `basis_head` equals PostureTransition `prior_authority_head`,
intent `expected_canonical` equals PostureTransition `expected_canonical`, and
the complete JSON values satisfy `intent.basis_head.canonical ==
intent.expected_canonical == PostureTransition.prior_authority_head.canonical
== PostureTransition.expected_canonical == intent.recovery.old_canonical`.
Intent change/authority/reassessment/replay values equal the PostureTransition;
intent canonical output equals the PostureTransition result; posture output
pair derives from that record and its hash/length equal selected bytes;
lifecycle prior head/canonical, authority pair, reassessment, result canonical,
and audit time equal their named posture/basis values; and lifecycle output
pair/hash/length derive from its selected bytes. The committed merged edge is
exactly basis canonical to output canonical, uses the posture pair as source,
the lifecycle pair as resulting lifecycle head, and retains the basis promotion
ancestor.

The transaction root admits only the three transaction suffixes and the fixed
private `.intent-staging` / `.output-staging` directories. Intent scratch is
`<32-lower-hex>.intent`; output scratch is `<32-lower-hex>.canonical`,
`.posture-transition`, or `.lifecycle-transition`. Its independent random
token is 128 bits. Scratch may be partial after a crash but is never a recovery
or head input; only exact verified bytes enter pending by no-replace rename,
and cleanup removes only an admitted safe regular scratch file. Unknown or
unsafe root/scratch entries are preserved and refused.

The exact pending-name allowlist is `intent.json`, `canonical.old`,
`canonical.new`, `posture-transition.new`, `lifecycle-transition.new`, and the
`.tmp`/published forms of `prepared`, `canonical-installed`,
`records-installed`, `committed`, and `rolled-back`. Unknown entries,
non-regular files, symlinks/reparse points, or competing terminal suffixes
refuse. `intent.json`, old canonical, and each stage are exact intent-bound
bytes. Intent/snapshot are durable first; stages admit only the prefix order
canonical -> posture -> lifecycle. Every write is fsynced, closed, reopened,
byte-verified, and rename-no-replace published.

A marker is exactly 72 ASCII bytes:
`sha256:<lowercase SHA-256 of exact intent.json bytes>\n`. A `.tmp` marker may
be absent or an exact prefix; a published marker is the full string; temp plus
published or any non-prefix byte refuses. Marker causality is exactly
`prepared -> canonical-installed -> records-installed -> committed`;
`rolled-back` is mutually exclusive with forward progress after prepared.
Prepared requires all three exact stages. Canonical install consumes its stage
and exact-rereads the target. Records-installed requires both exact final
records and consumed record stages. Committed requires the complete forward
chain and all equalities.

A committed terminal contains exactly `intent.json`, `canonical.old`,
`prepared`, `canonical-installed`, `records-installed`, and `committed`. A
rolled-back terminal contains exactly `intent.json`, `canonical.old`, and
`rolled-back`. Terminal publication is no-replace; no temp, stage, unknown, or
opposite-terminal name remains.

No semantic refusal creates a pending journal or changes canonical/final record
partitions. After `prepared`, readers recover pending work before current-head
selection:

| Durable observation | Required recovery |
| --- | --- |
| current canonical is exact basis; no transaction final or forward marker at/after canonical-installed; stage/prepared state is an admitted prefix | remove only exact owned stages, publish exact rolled-back marker, and finalize `.rolled-back` |
| current canonical is exact result; each record is available as an exact stage or exact final | install missing finals, replay missing causal markers, and finalize `.committed` once |
| committed marker or terminal exists | require exact result canonical, both exact finals, exact forward markers/name set, cross-record equality, and kernel replay |
| rolled-back marker or terminal exists | require exact basis canonical, no transaction-owned final, and exact rolled-back name set |
| missing required byte; unknown/extra/unsafe name; stage/marker order violation; ref/fingerprint/hash/length substitution; fork; or any other state | preserve every byte and refuse durability; never guess, overwrite, delete evidence, or claim success |

The result kernel is computed before staging from the proposed Charter and a
bounded normalized replay closure containing exact pairs and derived posture
inputs, never raw Snapshot/delta payload. Recovery recomputes it from the
journal closure and exact current/staged Charter. Success is returned only when
the committed PostureTransition resulting pair and a live resolver replay are
equal. Intent-bound markers transitively bind both final documents, and the
merged edge independently binds their semantic pairs without adding the
lifecycle pair to the PostureTransition fingerprint.

## Internal schemas and public-surface decision

The three new identities are private durable record schemas implemented by
closed Rust decode/validation and exact-byte tests. They are not package
definitions, public JSON Schema documents, SDK/transport records, or semantic
capabilities:

| Identity | Partition | Public effect |
| --- | --- | --- |
| `handbook.posture-transition` / `1.0` | `.handbook/state/posture-transitions/` | none |
| `handbook.lifecycle-transition` / `1.1` | existing `.handbook/state/lifecycle-transitions/` | none; v1.0 remains admitted |
| `handbook.charter-posture-transaction-intent` / `1.0` | `.handbook/state/transactions/posture-transitions/` | none |

No new `definitions/schemas` file, `LineageRecordClassV1` variant, Cargo
dependency, operation, DTO, public method/type/module, CLI, SDK, Tauri,
Substrate, Flow, or HCM-5 surface is selected. Discovery of such a need is a
hard authority stop.

## Private test seam and proof

Tests live beneath `crates/engine/src` as `#[cfg(test)]` child modules of the
private semantic and transaction owners. The packet intentionally rejects an
external `crates/engine/tests/hcm_3_6_project_posture.rs`: accessing the private
module from that integration crate would require an unauthorized public export.

The exact future wall is
[`proof/20260808T041700Z--hcm-3-6-atomic-authority-proof-matrix.md`](proof/20260808T041700Z--hcm-3-6-atomic-authority-proof-matrix.md).
It includes all nine mappings, baseline/override algebra, deep-diff and CAS
negatives, record fingerprints, heterogeneous-chain faults, lifecycle v1.0/v1.1
alternation, later promotion, lifecycle events, crash recovery, resulting
kernel replay, privacy, existing HCM-2.2 consumers, Context Resolution
separation, and HCM-5 non-authority.

The named compatibility commands include at minimum:

```text
cargo test -p handbook-engine --test hcm_2_2_transaction_promotion
cargo test -p handbook-engine --test hcm_2_2_promotion_workflow
cargo test -p handbook-engine --test hcm_2_2_lifecycle_store
cargo test -p handbook-engine --test hcm_2_2_approval_use
cargo test -p handbook-engine --test hcm_2_2_authority_repair
cargo test -p handbook-engine --test hcm_2_4_charter_profile_compatibility
cargo test -p handbook-engine --all-features
cargo test -p handbook-flow --all-features
cargo test --workspace --all-targets --all-features
```

The new crate-local test modules must also run directly by exact test
names/filters so no focused mapping or recovery case can hide inside the broad
wall.

## Stops

Stop for a new public/schema-asset/dependency/configuration/transport surface;
an edit outside the exact future source ceiling; a lifecycle model that clears
unrelated work; a head model that fabricates a promotion; raw Snapshot/delta
payload access; ambient time; partial P1/P2 selection; missing HIGH/CRITICAL
acceptance; any unresolved P1/P2; protected-path drift; or any remote action.
