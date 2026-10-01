# HCM artifact-system seam — bounded repository-truth packet

Date: 2026-09-02
Baseline HEAD observed for this consultation: `18d7137db593fb76460094c7d3aaa422e65400eb`

This packet exists only to give an independent ChatGPT Pro consultation the
minimum current repository truth needed to review how the three proposed HCM
semantic decisions should compose with Handbook's artifact, projection, Flow,
Pipeline, SDK, and transport architecture. It is evidence, not implementation
authority or a request to preserve current V1 shapes.

## Review targets supplied separately and in full

1. `metric-gate-semantic-decision.md`
   - SHA-256: `457F8A3BF0A9151588669618FBB2382451F99962D0A19DA76BAD962752C3E2D2`
2. `posture-assurance-policy-semantic-decision.md`
   - SHA-256: `AC71F03BD3C15CC73B4BAA8365EF0A8F4A286B35C55C889F1CE3B06F79A82AA4`
3. `evaluation-authorization-capability-and-readiness-semantic-decision.md`
   - SHA-256: `815ACCDC2B7A476286F333E7626DCF5A664FF0740AC40EC137AD52E80AB69F74`

The repository and product are greenfield. Existing source is architectural
evidence and may be replaced when it conflicts with an approved semantic
model. There is no requirement for V1 migration, backward compatibility,
dual-read/write behavior, or legacy bridges absent a separately proven
external adopter.

## Current crate topology

The Cargo workspace contains:

| Directory | Package | Current responsibility relevant here |
| --- | --- | --- |
| `crates/engine` | `handbook-engine` | Canonical project authority, artifact/profile registries, Context Resolution, current private posture kernel, projections, snapshot/grounding, and posture mutation. |
| `crates/contracts` | `handbook-contracts` | Exact contract definitions, lifecycle, evaluation identity/run, evidence admission, claim verdicts, evaluation policies, and current gate/local-closeout composition. |
| `crates/dock` | `handbook-dock` | Dock protocol, exact manifest/run boundary, producer process invocation, and current adapter operating contract. |
| `crates/dock-json-schema` | `handbook-dock-json-schema` | Dock public JSON Schema generation and validation. |
| `crates/sdk` | `handbook-sdk` | Transport-free public composition over Engine, Contracts, Dock, Flow, and Pipeline owner operations. |
| `crates/cli` | `handbook-cli` | CLI parsing, rendering, streams, and exit behavior over SDK routes. |
| `crates/tauri` | `handbook-tauri` | Tauri transport/interface projection. |
| `crates/flow` | `handbook-flow` | Packet resolution from canonical project artifacts and grounding inputs. |
| `crates/pipeline` | `handbook-pipeline` | Pipeline capture/compile/handoff, mutation/read admission, and grounding inclusion. |

The required dependency direction remains Engine inward of Contracts and
owner crates inward of SDK/transports. Engine must not depend on Contracts to
issue or verify Context Resolution authority.

## Current artifact vocabulary and shipped profile

The current artifact-kind registry contains six named kinds:

| Kind id | User-facing concept | Supported roles in the current `1.1.0` kind |
| --- | --- | --- |
| `handbook.artifact-kind.project-authority` | Charter | `constitutional_authority` |
| `handbook.artifact-kind.project-context` | Project Context | `project_context` |
| `handbook.artifact-kind.environment-context` | Environment Context | `environment_context` |
| `handbook.artifact-kind.work-specification` | Work Specification | `atomic_action`, `coordination_horizon`, `delivery_unit`, `execution_envelope`, `implementation_unit` |
| `handbook.artifact-kind.decision-record` | Decision Record / ADR | none currently declared |
| `handbook.artifact-kind.risk-record` | Risk Record | none currently declared |

The shipped-root profile `handbook.profile.shipped-root@1.2.0` instantiates
only three root project artifacts:

- `project_authority` at `.handbook/project/charter.yaml`, always required;
- `project_context` at `.handbook/project/context.yaml`, always required; and
- `environment_context` at `.handbook/project/environment.yaml`, optional.

The same profile registers all six artifact kinds and their schemas. Its
`posture_evaluation_policy` is `null`; `projection_catalog_refs` is empty; and
all three root instances currently have empty `projection_definition_refs`
but one Markdown review `renderer_definition_ref` each.

The older `canonical_artifacts.rs` loader still contains a separate enum with
`Charter`, `ProjectContext`, `EnvironmentContext`, and `FeatureSpec`, while its
fixed descriptor array actually loads only Charter, Project Context, and
Feature Spec from older `.handbook/charter`, `.handbook/project_context`, and
`.handbook/feature_spec` paths. This overlaps but does not exactly match the
newer profile-based Project Authority / Project Context / Environment Context /
Work Specification / Decision Record / Risk Record vocabulary. Because the
product is greenfield, this is a taxonomy to reconcile, not a compatibility
surface to preserve automatically.

## Generic artifact machinery already present

`ArtifactInstanceDescriptor` already carries:

- exact kind and optional stable role;
- semantic capability references;
- canonical path and requiredness;
- exact artifact or capability dependencies and cardinality;
- lifecycle-policy and intake-definition references;
- renderer-definition references;
- projection-definition references;
- validation-overlay references; and
- extension data.

The kind and instance registries establish exact definition fingerprints and
reject conflicting identities. The current generic repository/operation layer
supports kind/instance listing, reading, validation, intake, candidate append,
candidate validation, promotion, mutation outcomes, and lineage.

The three proposed decisions do not say that every new semantic object must be
an `ArtifactInstanceDescriptor` or live in this generic authored-artifact
repository.

## Current canonical project artifacts and projections

The newer profile path treats the Charter as `project-authority` and exposes
typed canonical Charter content. Project Context and Environment Context each
have typed canonical schemas and Markdown review renderers.

Engine has owner-produced projections for selected canonical project facts,
including `CanonicalCharterProjection`,
`CanonicalProjectContextProjection`, and
`CanonicalEnvironmentContextProjection`. These projections deliberately hide
raw bytes and repository file identity from consumers.

The generic profile model supports projection definitions, but the shipped
profile does not currently configure a projection catalog or per-instance
projection definitions. Review Markdown renderers are configured instead.

Engine also contains a separate, currently private and `dead_code`-allowed
projection implementation with exact sources, currentness requests,
disclosure/authority behavior, lossiness, refusals, output content references,
and fingerprints. Snapshot/Grounding separately produces Resolution- and
disclosure-bounded projections. These facts do not establish that all ordinary
artifact reads currently pass through one universal projection engine.

Flow currently loads canonical project artifacts, validates them, renders
included artifacts, and may include them verbatim or as budget summaries in a
packet. It records omissions and budget decisions. Flow is a consumer of
canonical artifact truth and projections, not an owner of Charter, posture,
assurance, evidence, metric, or gate semantics.

Pipeline currently owns capture/compile/handoff and grounding inclusion. It
may eventually consume evaluation/gate dispositions for stage workflow, but a
semantic gate cannot mutate Pipeline or work state directly.

## Current posture and Context Resolution facts

The Charter schema and authoring surface already represent a baseline `1..=5`
and nine typed posture dimensions. The private `ProjectPostureKernel` derives
the effective vector and retains input/kernel fingerprints and a replay plan.
It does not yet produce the proposed exact public Resolved Posture Snapshot
with exact Posture Scope and complete policy-selection provenance.

The shipped profile has no posture evaluation policy. Free-form Charter
domain overrides, stance, shortcuts, and red lines are not currently an exact
executable assurance selector or typed exception authority.

Engine Context Resolution already owns typed envelopes, exact stacks and
dimensions, parent/constraint closure, mutation rules, escalation, private
authority admission/currentness, and posture-transition mutation authority.
The three decisions propose a stable Engine-produced Evaluation Authorization
Binding for Contracts to reference without reconstructing the private envelope.

Context Resolution authority, assurance strictness, producer capability,
viewer disclosure authority, and explanation density are distinct facts.

## Current Contracts, Dock, and SDK facts

`handbook-contracts` currently exposes V1 contract definitions, claims,
evaluation identity/run preparation, evidence admission, claim verdicts,
evaluation policy, and gate/local-closeout composition. Its current
`ScoreWeightV1` and gate composition are evidence, not compatibility
obligations for the proposed generic claim/metric/gate model.

The proposed Evaluation Coordination owner does not yet exist as the complete
Contracts-domain responsibility described by the new authorization/readiness
record. Current `profile_readiness` in the SDK concerns repository/profile
artifact readiness and must not be silently conflated with the proposed
evaluation-attempt Readiness Assessment.

The first-party Dock currently has an exact adapter rule around requested,
admitted-ceiling, and completed-actual Resolution. That rule is not universal
Engine authorization and does not prove host confinement. Future Dock changes
must retain exact Producer Selection, Prospective Evaluation Basis, attempt,
requested reach, actual reach, and provenance.

The SDK already routes Flow, Pipeline, Contracts, evidence/verdict/gate, Dock,
artifact, snapshot, Charter, Project Context, and posture-transition
operations. It is a public projection/composition layer and must not become an
alternate semantic owner. CLI and Tauri are transports over the same owner
results.

## Candidate classification of the new semantic objects

This is a question for review, not an approved answer:

| Candidate category | Likely members |
| --- | --- |
| Authored canonical project artifacts | Charter / Project Authority, Project Context, Environment Context, Work Specification, Decision Record, Risk Record |
| Owner-derived canonical records | Resolved Posture Snapshot, Resolved Assurance Requirements, Observation Plan, Evaluation Purpose, Evaluation Authorization Binding, Readiness Basis/Assessment, Prospective Evaluation Basis, canonical evidence/evidence set, final evaluation context, claim/metric/gate results, Enforcement Disposition |
| User-facing projections | Markdown review documents, packet sections, CLI/SDK/Tauri result views, canonical explanations at selected density |
| Transient operational objects | Requests, in-progress attempt state, host launch state, transport-local failures |

The review should test rather than assume this classification. In particular,
it must determine which owner-derived records, if any, belong in the generic
artifact registry/repository and which require dedicated owner repositories.

## Questions requiring guidance

1. Is the four-way distinction among authored project artifacts,
   owner-derived canonical records, projections, and operational objects the
   right top-level model?
2. Which proposed semantic records should use the generic artifact-kind /
   instance / lifecycle / lineage machinery, and which should remain in
   dedicated Engine or Contracts owner repositories?
3. Should the generic artifact registry gain a first-class derived-record
   category/capability, or would that wrongly collapse authored project truth
   with evaluation truth?
4. How should Project Context and Environment Context participate as declared
   policy, scope, plan, or readiness inputs without becoming ambient selectors
   or authorization facts?
5. How should Work Specification consume gate and downstream disposition
   results for local closeout and eventual parent promotion without embedding
   mutation authority in Contracts or the gate?
6. Are Decision Records and Risk Records merely authored inputs/consumers, or
   do the new decisions imply additional typed relationships to policies,
   requirements, observations, or downstream actions?
7. How should artifact-manifest freshness, policy/input currentness, Engine
   authorization currentness, evidence currentness, and operational reliance
   verification relate without becoming one overloaded status?
8. What should the projection architecture own? In particular, distinguish
   owner-produced canonical record projections, Context Resolution disclosure,
   Snapshot/Grounding Resolution, review Markdown rendering, and future
   explanation-density projection.
9. Does the current old `FeatureSpec` versus newer Work Specification taxonomy
   reveal a boundary that should be resolved before planning?
10. Does the existing SDK `profile_readiness` vocabulary create a harmful
    collision with evaluation readiness?
11. Which crate boundaries should remain, which require new subresponsibilities,
    and where would a new crate create more coupling rather than less?
12. What other architectural gaps, false equivalences, missing owner decisions,
    or premature assumptions become visible only when all three decisions are
    composed with this artifact-system truth?

## Constraints on the consultation

- Treat the product as greenfield.
- Do not request V1 migration, compatibility aliases, dual models, or legacy
  bridges without proving an independent architectural reason.
- Preserve Engine inward of Contracts and owner crates inward of SDK/transports.
- Do not let projections, SDK, CLI, Flow, Pipeline, Docks, or hosts acquire
  semantic ownership merely because they display or operationalize a result.
- Do not force every semantic object into the generic artifact repository.
- Do not invent concrete APIs, schemas, persistence technology, or task slices
  unless needed to expose a semantic contradiction.
- Separate semantic gaps that require another decision from implementation
  questions that belong in later task-level planning.

## Source integrity

The repository facts above were derived from the following current files:

- `docs/specs/handbook-contract-membrane/architecture-context.md`
  (`A8208183AC102B27FD7211845CADF635D1931B59CDCCBEB7A98A83ECD7DAAA88`)
- `crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml`
  (`5090E56E503C98DF20DF61A27EF946B239A0D0E9FB26D712427514A4697ED42F`)
- `crates/engine/src/canonical_artifacts.rs`
  (`7B56C04CFDC96628EA746763E4BC59BD38B727A4416DCBEB5343AC16548858BC`)
- `crates/engine/src/artifact_instance.rs`
  (`54181EA367F07C8AB9F1501DB2902C0C2BB3F20B6F5281BCA708E3B34E4412CB`)
- `crates/engine/src/profile_inspection.rs`
  (`A6EA334845D80238F9B4B1EC343354EE92A4058919A07EDB366A5C7C4F07C6D2`)
- `crates/engine/src/project_posture.rs`
  (`8B2C6048425DCD2A6A9365995CD886C1770F8B142C1A15D9A85EDE4829374BAB`)
- `crates/engine/src/projection.rs`
  (`A9EA3166C6908BF7D40FBA0316420F94CFC9F8FE0DD1E8F860893121ED0206D3`)
- `crates/flow/src/resolver.rs`
  (`4D8D017B24B3810461516146CD0C4E88C571E75E5F11D57CBDF7B4E4C3499936`)
- `crates/pipeline/src/lib.rs`
  (`633869BB267C62FC02C3D2582728A37D7D03375A80DFAA8F1E3F9EDABA6746FC`)
- `crates/contracts/src/lib.rs`
  (`BDC990BBD6E99EC066C099C41AB79A2CB8E621C583A4C0992FB76579E4532C63`)
- `crates/sdk/src/lib.rs`
  (`6FCB9C47E240637F0C7E9FFE47B91248585655D0C1816253143AF03398D821B1`)

GitNexus reports its index is 18 commits behind HEAD and keyword search is
degraded because the local FTS extension is unavailable. Exact symbol context
was still consulted for `ProjectPostureKernel` and `ProjectionOutput`; current
source was used as the controlling repository truth.
