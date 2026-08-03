# HCM-3.2 Context Resolution Kernel

Status: fresh operator-authorized restart; selector review required before RED/Rust
Phase: HCM-3
Slice: HCM-3.2
Parent orchestration: `20260802T232751Z--HCM-3-2--context-resolution-kernel-fresh`
Integrated outcome: `hcm-3.2-context-resolution-kernel-fresh-whole-slice`

The operator course correction starts a genuinely fresh HCM-3.2 parent at
commit `a1aef5a5fc8a4471cee6044e16bd751ae512e216`, tree
`17dbde4d8f12127d7ddd7ac0ed8997c5f33789c8`, with no handoff selector. The
stopped HCM-3.2 parent and its findings remain immutable historical evidence;
this parent does not supersede, directly succeed, or reuse any of its parent,
outcome, packet, finding, cycle, or budget identities.

## Objective

Land the engine-owned Context Resolution kernel defined by the exact HCM-3.2
row. The kernel must resolve configurable linear stacks, materialize immutable
six-dimension envelopes, enforce parent/child narrowing and fail-closed mutation
authority, and expose typed memory, validation, escalation, and promotion
boundaries with deterministic fingerprints.

This slice closes only `PG-RES-01` and the Context Resolution subset of
`PG-PROFILE-01`. It does not begin capitalized Projection, Snapshot Memory,
pipeline/packet adoption, project posture, SDK, CLI/Tauri/transport, release,
publication, or HCM-3.3+ work.

## Authority and dependencies

The implementation is bounded by:

- `04-phase-slice-map.md`, exact HCM-3.2 row;
- `01-target-architecture.md`, Handbook ownership, crate ownership, invariants
  3-7, 12-15, 20-21, and Phase 3 non-goals;
- `02-semantic-model.md`, Context Resolution definition, six dimensions,
  ordered stack, envelope, escalation, and promotion;
- `03-seam-crosswalk.md`, Pipeline work levels and Context Resolution definition
  stack rows;
- `05-contracts-schemas-and-gates.md`, exact stack, envelope, escalation, and
  memory-promotion contracts;
- `06-proof-and-regression-ledger.md`, `PR-005`, `PG-PROFILE-01`, `PG-RES-01`,
  and regression rules 29, 42, 48, 49, and 81-92;
- completed HCM-3.1 handoff
  `20260801T141509Z--HCM-3-1--orchestration--vocabulary-resolution-completed`
  as predecessor context only.

HCM-3.1 vocabulary/stable-role identity and the shipped profile closure are
immutable dependencies. The user selection of HCM-3.2 is the slice authority;
the predecessor handoff neither widens nor narrows it.

## Selected authority admission

The product authority selected Option 2. HCM-3.2 adds one narrow closed
`handbook.context-resolution-authority-binding` record and one admission
resolver in the Context Resolution module. The binding is not a shipped
definition, schema-catalog family, generic token framework, or cross-product
authority service. It is a profile-selected canonical generic artifact whose
current bytes are read only through `ArtifactRepositoryAuthorityGuardV1` and
`GenericArtifactLineageStoreV1` recovery/currentness, using the existing HCM-2.3
artifact mutation owner. The generic store changes only in the two selector-
named methods needed to recognize and route the committed `registry` family
without parsing it as a generic-artifact intent; every other primitive remains
unchanged.

The resolver pins the caller-supplied expected artifact-byte fingerprint and
derives a second RFC-8785/SHA-256 semantic binding fingerprint. The closed
record pins repository identity, selected profile and stack, exact committed
approver-registry state/head, eleven authority mappings (`root_creation`,
`parent`, `requested`, `approving`, `decision`, `evidence`,
`trigger_condition`, `constraint`, `source`, `target`, `target_memory`), and
the four exact candidate mappings. Owner lookup selects only active,
non-exhausted committed credentials that cover the exact
`ApproverAuthorityPairV1`. The resolver uses the existing native authenticator
port, request encoder, ES256 assertion verifier, RP/user-verification checks,
and counter rule. Its challenge covers the repository, registry head, binding
byte/semantic fingerprints, authority use, and exact subject ref/fingerprint.

Admission is an opaque HCM-3.2 value with private fields. Every uncached
challenge includes a fresh 32-byte nonce allocated through the engine's
existing `getrandom` dependency; entropy failure refuses before authenticator
use. Byte-identical reuse of the same use/ref/fingerprint inside one resolver
returns the cached exact admission; reuse of a use/ref with changed bytes
refuses. Across resolver instances, a new nonce makes a captured assertion
invalid even when the authenticator's permitted counter behavior is zero/zero.
Nonzero counters must also increase within one resolver lifetime. The resolver
does not persist, enroll, rotate, or mutate credentials. Envelope and
transition constructors consume admissions for the exact subject they
validate; caller coherence alone grants nothing.

The selected authority also accepts the live HIGH impact on
`ContextResolutionStackDefinition` and CRITICAL impact on exactly
`AuthoredStack::resolve` and
`ContextResolutionStackDefinition::load_bytes`, subject to the reviewed
selector and complete proportional proof wall. No other existing public or
HIGH/CRITICAL symbol may be edited.

## Frozen owner and identities

`handbook-engine` owns the pure Context Resolution model, validation,
fingerprinting, matcher evaluation, and append-only transition admission.
`handbook-flow`, `handbook-pipeline`, `handbook-sdk`, and transports receive no
new behavior in this slice.

The following admitted identities remain byte-exact:

- `handbook.context-resolution.shipped-root@1.0.0`;
- `handbook.mutation-matcher.core@1.0.0`;
- `handbook.resolution-escalation.core@1.0.0`;
- `handbook.memory-promotion.core@1.0.0`;
- the HCM-3.1 selected profile, stable-role registry, and vocabulary identities.

The shipped definition bytes and fingerprints do not change. HCM-3.2
generalizes stack validation around that admitted shipped instance and adds
runtime application; it does not silently create a new shipped default.

## Configurable ordered stack

`ContextResolutionStackDefinition` continues to bind one exact definition ref,
definition fingerprint, and the three exact core policy fingerprints. Its
runtime-resolved representation additionally retains:

- one non-empty broad-to-narrow linear list of unique stable level IDs and
  display labels;
- exactly six non-empty ordered value domains;
- unique value IDs and contiguous ranks beginning at zero in every domain;
- one complete six-dimension default per level;
- defaults that preserve or narrow every dimension from each broader level to
  its adjacent narrower level.

Profiles may vary level count, IDs, labels, domain values, and equal adjacent
defaults. They may not reverse rank meaning, omit a dimension, create an
arbitrary graph, select a different policy at invocation time, or hide changed
semantic bytes behind an unchanged ref/fingerprint.

## Six-dimension envelope

The public kernel uses namespaced types. It does not introduce an unqualified
`Resolution` type.

An envelope input declares:

- stable envelope ID and objective ref;
- the exact selected resolved-profile ref/fingerprint;
- the exact selected stack ref/fingerprint;
- active level ID;
- all six explicit dimension value IDs;
- zero or one exact parent envelope ref/fingerprint;
- deterministically ordered exact constraint ref/fingerprint pairs;
- ordered mutation rules;
- ordered exact escalation-trigger bindings.

The resolver receives the selected `ResolvedInstanceProfile` and, for a child,
the actual parent envelope. Omission never inherits later. Root inputs have no
parent. Child inputs must cite the supplied parent exactly, use the same profile
and stack, and preserve or narrow every dimension. A stale/missing parent,
profile or stack mismatch, unknown value, or rank increase refuses without a
partial envelope and returns a typed escalation candidate where the exact core
policy defines one.

`envelope_fingerprint` is RFC-8785/SHA-256 through the existing uniform
fingerprint producer over the complete normalized envelope excluding only its
own fingerprint. Presentation time, filesystem state, and invocation order are
not inputs.

## Mutation semantics

The exact core matcher supports only `repository_path` and the shipped
`normalized_repo_relative_glob_v1` grammar. Validation is ASCII, case-sensitive,
bounded, separator-normalized, and rejects absolute paths, backslashes, empty,
dot/dot-dot, URI, NUL, malformed wildcard, and unresolvable selectors.

Root effective mutation is local allow minus local deny. Child effective
mutation is parent effective allow intersected with child local allow, minus
the union of parent and child denies. No allow rule means no grant. Any valid
allow/deny overlap resolves deny. Unknown target kind, malformed target,
matcher failure, or an allow-containment relation that cannot be proven is an
indeterminate refusal, never an allow.

A child allow must be provably contained by a parent effective allow under the
closed grammar. Identical selectors, literal narrowing, segment wildcard
narrowing, and terminal-recursive-prefix narrowing are supported. Any possible
expansion or relation the kernel cannot prove refuses with
`mutation_allow_expansion`; it does not weaken the parent or partially apply
the child.

Mutation evaluation is pure. It normalizes and evaluates a declared target; it
does not read the filesystem, follow symlinks, or infer ambient authority.

## Memory and validation semantics

Memory and validation checks resolve requested value IDs through the same
selected stack domains:

- a requested memory rank at or below the envelope rank is authorized;
- a higher memory rank returns a typed promotion-required result and grants no
  write;
- a validation claim at or below the envelope rank is authorized;
- a higher validation rank returns `not_authorized`, never passed;
- unknown values and stale stack/profile identity refuse.

Memory promotion writes only new reviewed semantic-memory state through the
exact core request/disposition contract. It cannot mutate an artifact,
contract, posture policy, Snapshot, Projection, or gate.

## Escalation and promotion records

Requests and terminal dispositions are immutable separately fingerprinted
records. Constructors validate exact request bindings, same-profile/stack
proposal relations, non-empty source/evidence requirements where required,
outcome-specific fields, and compare-and-write bases.

The in-memory admission registry is an exact semantic validator, not a durable
store. It admits at most one terminal disposition per exact request, rejects a
changed record under a reused ID, rejects stale request fingerprints, and
preserves prior request/disposition bytes. Creating an escalation request never
grants authority. Only `approved` may cite an authorized replacement envelope;
only `applied` memory promotion may cite a new semantic-memory result.

Durable storage, operations, SDK exposure, and adapter delivery are later
slices.

## L0-L3 compatibility posture

The existing pipeline `work_level` and scoped-block filter remain a useful
precursor. HCM-3.2 neither maps `L0`-`L3` into public kernel identifiers nor
edits pipeline stages/rules, because HCM-3.5 owns Resolution-aware pipeline
adoption. The current pipeline compile/filter positive and negative targets
must replay unchanged. This is an intentional preservation boundary: no useful
behavior is removed, and no legacy mixed taxonomy is frozen into the new
kernel. The HCM-3.5 selector must later choose the actual migration/removal.

## Public Rust surface

The selector permits exactly these 24 new namespaced engine exports:

- `ContextResolutionExactBinding`, `ContextResolutionAuthorityUse`,
  `ContextResolutionAuthorityAdmission`, and
  `ContextResolutionAuthorityAdmissionResolver`;
- `ContextResolutionDimensions`, `ContextResolutionEnvelopeInput`, and
  `ContextResolutionEnvelope`;
- `ContextResolutionMutationEffect`, `ContextResolutionMutationRule`, and
  `ContextResolutionMutationDecision`;
- `ContextResolutionMemoryDecision` and
  `ContextResolutionValidationDecision`;
- `ContextResolutionEscalationCandidate`,
  `ContextResolutionEscalationRequest`,
  `ContextResolutionEscalationOutcome`, and
  `ContextResolutionEscalationDisposition`;
- `ContextResolutionSemanticMemoryTarget`,
  `ContextResolutionSemanticMemoryRecord`,
  `ContextResolutionPromotionRequest`, `ContextResolutionPromotionOutcome`,
  and `ContextResolutionPromotionDisposition`;
- `ContextResolutionTransitionRegistry`;
- `ContextResolutionKernelError` and `ContextResolutionKernelErrorKind`.

All fields that would permit bypassing validation remain private. Constructors
and accessors are typed and deterministic. No `serde_json::Value` dispatcher,
ambient latest/range lookup, new crate, dependency, feature, version bump, or
transport surface is authorized. Public visibility is bounded to the owner
crate and is not a publication or downstream-adoption claim.

The exact public symbols and callable signatures are frozen in the selected
decision record. All public fields remain private except existing external
primitive fields. No implementation may add a public symbol or change a frozen
signature without amending and independently reviewing the selector first.

## Proof obligations

Focused RED/GREEN proof must cover:

1. shipped stack/profile/fingerprint replay and one configurable non-shipped
   linear stack with all six dimensions;
2. invalid/duplicate/missing domains, non-contiguous ranks, missing defaults,
   and adjacent rank increase refusal;
3. root resolution, deterministic replay, all six independent values, same
   active level with different dimensions, and profile/stack/parent mismatch;
4. parent equality/narrowing and every single-dimension increase refusal;
5. valid literal/glob/recursive matching, parent/child allow intersection,
   deny-on-overlap, expansion refusal, malformed selector/target, unknown kind,
   and indeterminate containment refusal;
6. memory same/narrow authorization, higher-rank promotion-required, validation
   same/narrow authorization, and higher-rank not-authorized;
7. escalation request no-effect, approval/refusal/supersession cardinality,
   promotion applied/refused/stale cardinality, stale compare-and-write, and
   forbidden target-authority refusal;
8. unchanged HCM-3.1 exact definition/profile replay and unchanged pipeline
   L0-L3 scoped filtering;
9. no Projection, Snapshot, flow, pipeline, compiler, CLI, SDK, transport,
   dependency, unsafe, or shipped-definition delta.
10. real-path authority admission for the exact canonical artifact and generic
    recovery/currentness path; repository/profile/stack/registry-head mismatch;
    raw/semantic fingerprint mismatch; missing/extra/malformed mappings;
    zero/multiple/inactive/exhausted/wrong-pair credentials; authenticator
    refusal/unavailability; RP/UV/credential/signature/counter failure; fresh
    nonce behavior; captured assertion refusal across fresh resolvers for both
    zero and nonzero counters; exact replay and changed-subject refusal; and
    the exact subject mapping for all eleven authority uses;
11. typed promotion proof that the actual source envelope admits every source,
    the target horizon is strictly higher in the supplied stack, only a typed
    semantic-memory target/result can enter the registry, and applied/refused/
    stale compare-and-write outcomes cannot grant artifact/contract/posture or
    HCM-3.4 durable-memory authority.

The proportional proof wall is:

- focused engine stack/policy/profile/kernel tests;
- focused pipeline scoped-work-level regression targets without pipeline edits;
- relevant engine and pipeline crate tests;
- `cargo test --workspace --all-targets --all-features`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo fmt --all -- --check` and `git diff --check`;
- ordinary handoff validation plus both orchestration self-tests;
- GitNexus impact before existing-symbol edits and scoped plus compare-to-main
  detection before the primary commit, with FTS/comparison unavailability
  recorded honestly;
- exact path/scope, immutable-definition, remote-baseline, and protected-root
  non-access/non-mutation audits.

## Production, test, and control ceilings

Production edits are limited to:

- `crates/engine/src/context_resolution_registry.rs`;
- new `crates/engine/src/context_resolution.rs`;
- `crates/engine/src/artifact_lineage_store.rs`, only
  `GenericArtifactLineageStoreV1::validate_inventory` and conditionally
  `GenericArtifactLineageStoreV1::require_journal_authority`;
- `crates/engine/src/lib.rs` only for the exact namespaced exports above.

Test/fixture edits are limited to:

- `crates/engine/tests/context_resolution_stack.rs`;
- new `crates/engine/tests/context_resolution_kernel.rs`;
- new `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

Existing pipeline L0-L3 regression targets are execution-only proof and are not
an edit allowance.

Control-plane edits are limited to the HCM-3.2 SPEC, plan, todo, selector,
proof records, exact affected HCM 00-06 rows after earned proof, current v1.4
dispatches, one parent handoff, the deterministic ledger, and `09` only for a
fresh validated P3/P4.

Ceilings: at most 4 production paths, 3 test/fixture path families, 80 changed
named production declarations under the selector counting rule, and 2,000
hand-written production lines. Risk is limited to
`ContextResolutionStackDefinition` at HIGH; `AuthoredStack::resolve`,
`ContextResolutionStackDefinition::load_bytes`, and
`GenericArtifactLineageStoreV1::validate_inventory` at CRITICAL; the
conditionally necessary `GenericArtifactLineageStoreV1::require_journal_authority`
only while fresh impact remains LOW; and new symbols in the same engine subsystem.
Exceeding a ceiling requires reviewed same-slice decomposition or an authority
stop; dependency/unsafe/schema-catalog/transport expansion requires user
authority.

## Review and closeout

The parent freezes one outcome registry before review. Planning,
implementation, proof, and final-closeout stages follow the live v1.4 causal
cadence. Every review dispatch is immutable, schema-valid, manifest-replayed,
fresh, read-only, and built-in. Valid P1/P2 findings receive one consolidated
remediation and different-fresh closure, with only the two permitted directly
causal supplementals. No cycle follows CLEAN.

Completion requires no unresolved P1/P2, full proof, scoped and compare-to-main
change detection, one reviewed primary commit or reviewed stack, a separate
mechanical v1.4 handoff/ledger closeout commit, and atomic expected-old update
of the dedicated local integration ref. No push occurs.

## Stop conditions

Stop non-completed if work requires HCM-3.3+, pipeline/flow adoption, changed
shipped definition/profile/vocabulary bytes, a new dependency/crate/schema or
transport, unsafe/native/platform machinery, more than the reviewed ceilings,
unresolved HIGH/CRITICAL impact, unavailable mandatory delegation, exhausted
causal budget, or broader human/product authority.

The prior authority stop was released by the exact same-slice resumption
binding. RED/Rust remains prohibited until the amended selector receives one
fresh schema-valid implementation-stage independent CLEAN review. A finding or
validator refusal keeps the stop active until resolved within the causal
budget.
