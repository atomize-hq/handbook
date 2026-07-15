# Semantic Model

## Status

The HCM-0.2 scope in this document is frozen design authority: stable semantics and configurable language, instance-profile artifact/vocabulary composition, schema registration, artifact kinds and instances, intake and promotion, the constitutional root, Charter intake, the project-posture owner boundary, and vocabulary semantics. HCM-0.3 additionally freezes Context Resolution, deterministic Projection, memory promotion, Snapshot Memory, delta/drift, redaction/retention, and snapshot-grounding semantics. HCM-0.4 freezes adapter preservation across the SDK, CLI, Tauri, and the distinct transitional/permanent Substrate boundaries. The exact field contracts and validation/defaulting matrices live in `05-contracts-schemas-and-gates.md`.

These freezes are not implementation proof. All runtime gates remain open, and the shipped artifact/default-instance set plus shipped Resolution labels/default policy remain explicitly unresolved rather than being selected by examples.

## Purpose

This file defines the conceptual model that profiles, artifacts, vocabulary, Context Resolution, projections, memory, contracts, and adapters must share.

The system must be customizable without reducing every concept to an arbitrary string.

## Stable semantics and configurable language

Handbook maintains stable internal semantic identifiers. A user profile controls display language, local work-unit names, intentional role conflation, and artifact presentation.

Example stable roles:

```text
constitutional_authority
project_context
environment_inventory
delivery_unit
coordination_horizon
implementation_unit
execution_envelope
atomic_action
contract
claim
evidence
verdict
gate
```

A user may render these as Charter, System Brief, Feature, Phase, Slice, Packet, Task, or any other terms.

Stable identifiers exist for validation and translation. They are not a language-policing mechanism.

Handbook keeps two explicit semantic namespaces:

- the versioned **stable-role registry** identifies typed places in the system and workflow, such as `constitutional_authority`, `project_context`, or `execution_envelope`;
- the versioned **semantic-capability registry** identifies behavior/conformance contracts, such as `constitutional_root`.

Profiles may select labels, aliases, and explicit structural absorptions only for stable roles. They cannot use a capability ID as a vocabulary role, create an undeclared role/capability, change machine meaning, or rename a schema ID, kind ID, operation ID, or CLI command. A kind separately advertises supported role refs and capability contracts; an instance separately selects `role_ref` and zero or more `capability_refs`.

The stable-role registry is not ambient. Every profile, kind, and vocabulary record pins the same exact registry ref/fingerprint pair, and resolved-profile identity includes that pair. The registry owns stable role IDs, canonical display labels, and role categories; vocabulary owns only profile-selected labels, aliases, and allowed structural absorption. A changed registry fingerprint is a changed semantic input and cannot silently alter default labels or role validation.

Every versioned HCM-0.2 definition ref is mechanically derived as its declared namespaced identity plus `@` plus its declared SemVer, and every referenced definition has a recomputable semantic fingerprint producer. Bare mutable refs, reconstructed aliases, and definitions with no fingerprint closure fail closed.

## Instance profile

An instance profile defines the Handbook semantics for one repository or project.

It contains:

- profile identity, profile version, and schema version;
- explicit parent profile reference, if any;
- one exact stable-role registry ref/fingerprint pair;
- schema-registry and artifact-kind sources;
- a complete artifact-instance registry for the resolved profile;
- one vocabulary profile;
- Resolution stack;
- projection definitions;
- validator/dock requirements;
- posture-evaluation policy ref/fingerprint pair or explicit null;
- optional adapter overlays and declared extensions.

Recommended first-version precedence:

```text
shipped root profile
  -> selected named profile layer
  -> repository profile layer
  -> exact profile selection at invocation
```

Ambient, silently discovered local overrides are out of scope initially.

Each layer is an explicit versioned record with at most one exact parent. For every layerable field, omission inherits the parent's complete value and presence replaces the complete value; an explicit empty list/map clears it. There is no implicit append, key merge, deletion/tombstone, last-write-wins conflict repair, or multi-parent merge in v1. A replacement must independently validate after resolution, so a layer cannot clear the constitutional root or weaken its floors.

Repository configuration is a profile layer, not an untracked mutation of the shipped profile. Invocation may select one exact resolved profile and supply operation-specific request fields, but v1 has no invocation-time profile-field override. Missing identity, authority, schema, kind, instance, path, requiredness, or constitutional-root decisions never infer from filenames, current enums, templates, or prose.

Profile resolution is deterministic. It emits the ordered source-profile refs/fingerprints, the source layer chosen for every replace-whole field, fully resolved schema/kind/instance/vocabulary values, diagnostics, and a fingerprint over normalized semantic content and exact definition fingerprints. Timestamps, local absolute paths, and invocation request fields are not profile-fingerprint inputs.

## Schema registry

The schema registry maps an exact stable content-schema ID and semantic version to one repository-relative JSON Schema document, its Draft 2020-12 meta-schema, media type, and SHA-256 document fingerprint.

- `SchemaRegistryEntry` owns safe schema resolution metadata; the schema document owns structural content rules.
- `ArtifactKindDefinition` refers to a registry entry by exact ID and version. It does not embed an ambient path or fetch a schema on demand.
- The tuple `(content_schema_id, content_schema_version)` is unique in a resolved profile. Conflicting documents or fingerprints for the same tuple fail closed.
- First-version compatibility is exact. Version ranges, implicit latest selection, remote refs, network fetching, executable hooks, symlink escapes, and unversioned ambient discovery are refused.
- Schema validation does not grant semantic or constitutional authority. Handbook-owned semantic validators, intake coverage, approvals, and external dock evidence remain separate layers.

## Artifact-kind and instance registries

The registry replaces the fixed assumption that every repository has exactly the same canonical documents. It distinguishes reusable **artifact kinds** from configured **artifact instances**.

### `ArtifactKindDefinition`

An artifact kind defines the reusable shape and behavior of a class of canonical artifact. It identifies at least:

- stable kind ID and kind version plus a derived definition fingerprint;
- exact canonical content-schema reference;
- zero or more supported stable-role refs;
- zero or more versioned semantic capability contracts and schema bindings;
- structural and semantic-validation profile references;
- fixed deterministic renderer-definition references;
- future generic Projection-definition references, which remain empty until their later contract is frozen;
- lifecycle and review-trigger semantics;
- required cross-artifact capabilities bound to exact capability-contract refs and explicit cardinality;
- declared extensions and exact-version compatibility posture.

The kind definition does not contain a repository-specific path, user-facing instance label, or requiredness decision.

A capability name alone is not conformance. Each advertised capability cites a versioned capability contract and binds its required semantic fields to JSON Pointers in the canonical content schema. Meta-validation proves that the pointers exist with compatible shapes; Handbook semantic validation proves the cross-field and authority invariants. A kind may advertise at most one contract version for a given capability ID, so an instance's selected capability ID resolves unambiguously through that kind.

### `ArtifactInstanceDescriptor`

An artifact instance binds one kind into a repository/project profile. It identifies at least:

- stable artifact instance ID;
- exact artifact-kind reference;
- selected `role_ref`, or explicit null when the instance has no stable role;
- explicit selected `capability_refs`, possibly empty;
- user-facing label;
- one concrete normalized canonical repo-relative path in the resolved profile;
- explicit `always`, `conditional`, or `optional` requiredness plus a condition reference only for `conditional`;
- typed dependencies on exact instance IDs or exact semantic-capability contract refs, including required cardinality;
- lifecycle policy reference, while mutable lifecycle/lock state remains canonical artifact state;
- explicit intake definition reference or null;
- explicit fixed-renderer and future Projection-definition selections;
- repository-specific validation overlays allowed by the kind;
- declared extensions.

One kind may back multiple instances. A profile selects instances; it does not create new hard-coded product types.

The authored descriptor does not inherit intake/rendering selections implicitly from a kind. It records an exact intake definition that targets the selected kind, explicit null when the kind/capability permits no intake, and explicit renderer/Projection lists. Intake compatibility is owned in one direction by `ArtifactIntakeDefinition.artifact_kind_ref`; kinds do not list or fingerprint intake definitions. This keeps the definition graph acyclic and makes the fully resolved profile replayable.

V1 dependency cardinality is intentionally small: `exactly_one` requires one distinct resolved provider and `at_least_one` requires one or more. Instance-ID dependencies allow only `exactly_one` because instance IDs are unique. Capability dependencies cite both the stable capability ID and its exact contract ref; resolution returns every matching provider instance in stable instance-ID order and never selects a source-order winner. Zero providers, too many providers for `exactly_one`, a contract-version mismatch, or duplicate capability declarations within one kind fail closed.

### Constitutional root

Every valid resolved profile contains exactly one instance selected for the `constitutional_root` semantic capability. That instance is `always` required. Its display name and concrete canonical path are profile-defined; the shipped default binding may remain Charter.

This is a semantic invariant, not a literal filename requirement.

A kind may satisfy the constitutional-root capability only when its declared capability contract and schema bindings cover policy authority, decision authority, exception/waiver authority, engineering-posture floors and red lines, and review/reassessment triggers. The instance must select `capability_refs: [constitutional_root]`; its separately selected stable role is normally `constitutional_authority`. Merely assigning that role, the label `Charter`, a familiar filename, or requiredness is insufficient.

No profile overlay, condition, waiver, vocabulary mapping, or adapter may remove or multiply the constitutional-root capability. Changing which instance selects it is an explicit reviewed profile change and must preserve one valid root throughout atomic promotion.

### Shipped defaults require a decision session

Handbook will ship an opinionated, versioned collection of artifact kinds and a default profile selecting some instances. The actual default set is intentionally unresolved in this control pack.

Before the set is frozen, Phase 0 must produce:

1. focused research into common durable project-governance/context artifacts and their failure modes;
2. a comparison of minimal, standard, and fuller candidate sets;
3. a user brainstorming/decision session covering purpose, overlap, lifecycle, requiredness, and projection needs;
4. an explicit approved decision identifying the shipped kinds, default instances, and which are required or optional.

Names such as Charter, Project Context, Environment Inventory, Technology Stack, decision policy, or risk register are candidate examples only. Their presence in this pack is not approval of the final shipped set.

### Custom artifacts

Repositories and selected profiles may define artifact kinds beyond the shipped collection.

- First-party artifacts may receive specialized deterministic authoring and rendering.
- Custom kinds define repository-local schemas with stable IDs/versions and pass the same kind-definition meta-validation as shipped kinds.
- Custom artifacts receive generic structural validation, lifecycle, intake, and contract behavior first.
- Custom kinds may provide an intake coverage contract without adding Handbook code. Generic configured custom-kind Projections wait for the Phase 3 deterministic Projection engine.
- A custom artifact does not require a new Rust enum variant merely to exist.
- Custom kinds never generate or rename CLI commands; stable generic operations select kind/instance IDs.
- Remote schema fetching, executable schema hooks, and ambient unversioned overrides are out of scope initially.

Structural JSON Schema validates YAML after parsing. Cross-field/cross-artifact semantic rules remain separate, and external domain validators continue to integrate through docks rather than executable code embedded in schemas.

Fixed first-party renderer definitions are not Projections. They accept validated canonical truth and declared deterministic rendering inputs only. Generic configured custom-kind views and any Resolution-aware view remain unavailable until the Phase 3 Projection contract is implemented.

## Artifact intake semantics

An `ArtifactIntakeDefinition` is a versioned coverage contract for obtaining the information needed to produce one artifact kind. It is not a fixed terminal questionnaire and is not itself canonical project truth.

It defines:

- coverage items and their mapping into the canonical artifact schema;
- conditional branches and applicability rules;
- which values may be inferred from repository evidence;
- which values require an authorized user declaration;
- evidence, freshness, confidence, and sensitivity requirements;
- specificity, completeness, contradiction, and known-unknown rules;
- approval requirements and promotion gates;
- optional question wording/prompt guidance for agent-facing projections;
- reassessment trigger refs with the exact affected coverage IDs each trigger reopens.

Each coverage item also declares an authority class (`observational`, `rationale`, or `normative`) and the allowed source kinds for that class. Only an explicit coverage-level deterministic default may create a defaulted value. A default records its source and authority effect; it cannot satisfy a user-declaration or approval requirement and cannot silently create constitutional policy.

Three first-version acquisition modes share the same definition and output schema:

- `guided_adaptive` — recommended; the skill-directed agent inspects repository evidence and asks only unresolved, normative, high-impact, contradictory, or low-confidence questions;
- `express` — asks a smaller high-value subset, then exposes inferred/defaulted fields and known unknowns for review;
- `agent_assisted` — prepares the fullest evidence-backed candidate possible and asks only blocking or authority-required questions.

Mode changes the acquisition path, not the canonical schema or semantic quality bar. Missing coverage remains explicit and cannot be hidden by choosing a shorter mode.

Coverage evaluation uses typed states: `satisfied`, `unknown`, `contradicted`, `waived`, `not_applicable`, or `blocked`. `not_applicable` requires its applicability proof; `waived` requires declared waiver authority and does not equal satisfied; unresolved required normative coverage blocks promotion.

### Intake records, candidates, and promotion

Keep three authorities distinct:

| Record | Role | Authority |
|---|---|---|
| `ArtifactIntakeRecord` | Immutable audit of questions, declarations, inferences, evidence, confidence, gaps, and evaluation | descriptive provenance only |
| `ArtifactCandidate` | Normalized artifact-shaped proposal with source mapping and unresolved issues | candidate; never canonical |
| canonical artifact YAML | Approved project truth used by projections/contracts | authoritative at its declared semantic role |

The LLM agent running the Handbook skill conducts the interview and uses CLI/SDK operations to inspect coverage, submit observations/declarations, validate the candidate, and request promotion. Handbook supplies deterministic schemas, evaluation, and promotion rules; it does not wrap an internal question-by-question UI or perform an untracked nested model call.

The intake record is finalized after acquisition and coverage evaluation. It never receives forward links to later records. A candidate points back to its intake record; an approval points to the immutable candidate/fingerprint; and a promotion points to the candidate plus approvals. An append-only derived index may expose forward navigation, but it is not authority and never changes source-record fingerprints.

Promotion is a compare-and-write transition. Handbook re-resolves the exact profile/kind/schema/intake versions, verifies the candidate fingerprint and current target state, reruns structural/semantic/coverage checks, validates approval authority, writes canonical YAML atomically, and emits an immutable promotion record. A stale candidate, changed target, missing approval, unresolved required coverage, or unknown required semantic fails closed.

### Charter intake

The historical Charter questionnaire becomes the first rich `ArtifactIntakeDefinition`: `CharterIntakeDefinition`.

Its coverage domains include project identity/shape, delivery constraints, operational reality, baseline posture, default delivery implications, risk domains, engineering-posture dimensions, exceptions/governance, engineering debt, and decision-record policy. The exact wording and branching may change, but lost coverage requires an explicit design decision rather than accidental omission.

Each normalized answer maps to one of:

- canonical Charter policy or governance;
- Charter rationale;
- a referenced, freshness-qualified observation used to justify a decision;
- an unresolved or explicitly waived intake gap.

Canonical Charter YAML is the constitutional authority. Before Phase 3, its Markdown and other fixed human-review outputs are renderer-derived views. Phase-3 Resolution-aware Tauri, packet, and agent-context outputs are Projections. The intake record remains available for audit and targeted reassessment without becoming a second editable Charter.

## Project posture kernel

`ProjectPostureKernel` is a deterministic, fingerprinted resolved view rather than an additional canonical artifact by default.

It resolves:

- canonical Charter posture, red lines, exception policy, and decision authority;
- approved domain/instance overrides;
- applicable project conditions and contract state;
- current evidence and snapshot references where freshness matters;
- effective engineering-posture levels, floors, triggers, shortcuts, and proof obligations.

The Charter is its primary constitutional authority. Observations may change the kernel's applicability calculation, but do not rewrite Charter policy.

The kernel fingerprint covers normalized semantic inputs and resolved output, including the exact profile ref and `resolved_profile_fingerprint`, a typed ref/fingerprint pair for every Charter, override, condition, contract, evidence, and snapshot input, and an explicit content-addressed freshness-evaluation basis whenever time affects applicability. Bare mutable refs are never sufficient replay inputs. It excludes the presentation-only resolution timestamp. Repeating resolution against the same input pairs and freshness basis produces the same fingerprints; changed bytes behind the same ref must change the cited fingerprint or cause refusal, and crossing an expiry boundary creates a new basis/fingerprint and may change the kernel honestly.

A `PostureRecommendation` may be derived when snapshots, contracts, evidence, or lifecycle events cross an approved trigger. In v1 each recommendation contains exactly one global engineering-dimension transition plus one causal/applicability scope ref, the exact kernel, policy, trigger-or-rule, evidence, and snapshot-delta fingerprints, and one confidence, urgency, approval requirement, and suggested-action set. Scope explains where the evidence arose; it is not a second posture-state coordinate and does not select a scoped authority path. A causal event that warrants two dimension changes produces two independently fingerprinted recommendations. A recommendation is not an enacted posture change.

Only an authorized, reviewed `PostureTransition` may update canonical policy. V1 transitions target the one canonical constitutional-root authority only; approved domain/instance overrides may influence resolution and recommendations but remain read-only inputs until a later typed override-mutation contract defines their own schema bindings, reassessment, and atomic-write rules. Raising posture may react to a single hard trigger; lowering posture requires sustained evidence across a configured window and cannot cross approved floors or red lines. This hysteresis prevents policy thrashing.

An approved `PostureEvaluationPolicy` defines hard triggers, accumulated-signal thresholds/windows, cooldown/revisit rules, recommendation recipients, and acknowledgment/escalation behavior. Handbook core emits a typed recommendation/event; CLI, Tauri, Substrate, or another adapter decides how to present or deliver the notification without becoming posture authority.

Use namespaced terms such as `EngineeringPostureDimension` and `PostureLevel`. These are distinct from the six `ContextResolution` dimensions: posture describes engineering rigor and guardrails, while Context Resolution describes the granularity/authority envelope of a view or run.

## Vocabulary model

Vocabulary is organized by semantic axes, not a flat synonym map.

Recommended axes:

- scope hierarchy;
- delivery units;
- execution units;
- cadence/timebox;
- lifecycle/governance;
- evidence/verification;
- organizational roles.

### Intentional lexical conflation

A user may assign the same displayed term to distinct semantic roles.

Example:

```yaml
vocabulary:
  labels:
    delivery_unit: feature
    coordination_horizon: feature
```

The internal distinction remains available to validators and adapters.

### Intentional structural conflation

A user may genuinely treat multiple roles as one workflow unit.

Example:

```yaml
work_units:
  implementation:
    id: task
    absorbs:
      - implementation_unit
      - execution_envelope
      - atomic_action
```

The profile makes the collapse explicit so adapters can report whether the target system can preserve it.

Vocabulary defaults are deterministic: a missing label uses the stable-role registry's canonical display label; aliases and absorptions default to empty. Duplicate display labels are allowed, but typed context must still resolve exactly one stable role. Absorptions are directed, acyclic, cannot absorb the `constitutional_authority` role, cannot mention a capability ID, and cannot erase an authority boundary or change a schema/kind/operation identifier. An adapter that cannot preserve a declared structural absorption must refuse or report a typed loss; it may not silently flatten the workflow.

## Context Resolution

Use namespaced implementation terminology such as:

- `ContextResolution`;
- `ContextResolutionEnvelope`;
- `ResolutionProjection`;
- `ResolutionEscalationRequest` and `ResolutionEscalationDisposition`;
- `ResolutionPromotionRequest` and `ResolutionPromotionDisposition`.

Do not use an unqualified `Resolution` type in code because route, path, selector, and dependency resolution already have different meanings.

### Definition

Context Resolution is the declared operating granularity of an artifact view, agent run, or workflow stage, together with the context, authority, memory, tooling, and validation envelope appropriate to that granularity.

### Six dimensions

Every envelope can independently constrain:

1. **Scope horizon** — how much of the system is in view.
2. **Detail resolution** — how granular the visible information is.
3. **Temporal horizon** — immediate action versus long-range outcome.
4. **Authority horizon** — which truth may be modified.
5. **Memory horizon** — where observations may be persisted.
6. **Validation horizon** — what level of completion may be claimed.

These dimensions must not be collapsed into a token budget or one numeric level.

### Ordered stack

The first version uses one exact, versioned `ContextResolutionStackDefinition` rather than an arbitrary graph. Its definition ref is derived from its declared ID and SemVer, and its fingerprint follows the uniform exact-definition rules in `05-contracts-schemas-and-gates.md`.

The shipped default may resemble:

```text
strategic
  -> coordination
  -> execution
  -> operation
```

Profiles may rename, add, remove, or intentionally collapse levels. Work-unit terms and resolution levels are related but not identical.

The current `L0`–`L3` work-level mechanism is a useful precursor, not the final semantic model, because it mixes scope levels with workflow phases such as merge/quality gate.

The stack definition owns:

- one linear ordered list of stable level IDs and profile-selected display labels;
- one ordered value domain for each of the six dimensions;
- a complete six-dimension default for every level;
- exact matcher, escalation-policy, and memory-promotion-policy refs/fingerprints;
- its deterministic `definition_fingerprint`.

For every dimension, rank zero is the least disclosure, reach, durability, or claim authority and larger ranks grant more. A child envelope may remain equal to or narrow any parent rank; it may not increase one without a typed escalation approved by authority outside the child. A profile may name the values differently, but it cannot reverse this comparison rule or collapse two values that have different authority effects.

The ordered level list supplies named defaults and navigation. It is not a seventh aggregate authority score. Two envelopes at the same active level may still differ in one or more dimensions, and every comparison is dimension-by-dimension.

## Resolution envelope

`ContextResolutionEnvelope` is a fully materialized immutable execution/view constraint. It pins the exact resolved profile and stack, records all six dimension values, and has at most one parent envelope. Additional upstream contracts are exact constraint inputs, not merge parents.

Conceptual shape:

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
envelope_id: envelope.capture-screenshot
resolved_profile:
  ref: handbook.profile.example@1.0.0
  fingerprint: sha256:...
resolution_stack:
  ref: handbook.context-resolution.example@1.0.0
  fingerprint: sha256:...
active_level_id: execution
objective_ref: work.task.capture-screenshot
dimensions:
  scope_horizon: assigned_unit
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: local_write
  memory_horizon: execution
  validation_horizon: unit_closeout
parent_envelope: null
constraint_inputs:
  - ref: charter.security@7
    fingerprint: sha256:...
mutation_rules:
  - rule_id: allow_browser_paths
    effect: allow
    target_kind: repository_path
    selector: crates/browser/**
  - rule_id: deny_canonical_policy
    effect: deny
    target_kind: repository_path
    selector: docs/canon/**
escalation_triggers:
  - trigger_id: parent_contract_conflict
    policy_ref: handbook.resolution-trigger.parent-contract-conflict@1.0.0
    policy_fingerprint: sha256:...
envelope_fingerprint: sha256:...
```

Root envelopes declare every field and have no parent. A child repeats the complete resolved state and cites its exact parent ref/fingerprint. Its effective mutation set is the intersection of parent and child allows minus the union of parent and child denies; a valid selector overlap always resolves to deny. Mutation selectors use the exact matcher contract pinned by the stack definition. Unknown target kinds, malformed/unresolvable selectors, matcher evaluation that cannot determine a stable match set, stale parent fingerprints, or any dimension increase fail closed.

`envelope_fingerprint` covers the normalized record, exact profile/stack/parent/constraint fingerprints, and ordered mutation/escalation semantics, excluding only its own field. Invocation cannot silently rewrite the selected profile or stack. Higher-horizon constraints constrain lower-horizon work. Lower-horizon observations do not rewrite higher-authority truth without a separately authorized promotion.

### Escalation and promotion

`ResolutionEscalationRequest` asks for a dimension increase, missing source, or broader mutation authority. It binds the current envelope, proposed complete envelope, exact trigger, evidence, and requested authority. A separate append-only `ResolutionEscalationDisposition` binds that exact request and records one terminal approval, refusal, or supersession. Neither record mutates in place, and creating the request does not grant the requested authority; only the named authority may approve a replacement envelope.

`ResolutionPromotionRequest` proposes moving a reviewed conclusion into a more durable memory horizon and binds exact source observations/projections, target semantic-memory record, requested authority, and compare-and-write basis. A separate append-only `ResolutionPromotionDisposition` records exactly one applied, refused, or stale outcome with its decision, validation evidence, approving authority, and result when applied. Prior request/disposition bytes remain immutable. Promotion never turns a snapshot or Projection into canonical artifact/contract authority, and artifact/intake/posture/lifecycle mutations continue to use their own contracts.

## Projection semantics

Terminology in this pack is strict:

- A **renderer-derived view** is a fixed deterministic pre-Phase-3 human-review output produced by a first-party renderer from approved canonical truth. It accepts no Context Resolution input and is outside the capitalized `Projection` request/result/provenance contract.
- A capitalized **Projection** is a Phase-3 generic Resolution-aware derived view from exact fingerprinted structured sources for a declared requester, surface, vocabulary profile, and Resolution envelope. Sources may be canonical artifacts or immutable semantic/observation records such as snapshots and deltas; Projection preserves each source's authority class and never upgrades descriptive evidence into canonical or peer truth.

Generic configured custom-kind Projections and every Resolution-aware view begin only after the Context Resolution kernel and deterministic Projection engine land in `HCM-3.2` and `HCM-3.3`. Phase 2 introduces no precursor engine and no separate Projection provenance subset.

Every capitalized Projection uses one exact `ProjectionDefinition`. The definition declares compatible source record/kind/capability selectors, allowed surfaces, one or both deterministic operations, a target schema, mandatory currentness requirements (`none` with an empty family set or exact-revision checking with complete family/adapter/slot bindings), one exact fingerprinted metadata-only disclosure policy, one exact fingerprinted built-in metadata-only support evaluator, field/claim rules with complete six-dimension minimum Resolution and registered disclosure classification, and exact built-in derivation refs/fingerprints. Exact profile/definition/source validation is the sole owner of semantic-capability compatibility: it resolves capability contracts and bindings for every selector before a request can produce per-rule evaluations, and missing, stale, or incompatible capability semantics refuse the request rather than becoming `unsupported`. Before reading protected payload bytes, the engine compares the request envelope with each applicable rule minimum, maps any exact upstream redaction disposition, evaluates the bound fail-closed disclosure policy, and asks only the bound evaluator for source-kind, source/target-schema, pointer, and derivation-I/O support. Missing/stale/incompatible evaluator or schema-registry state refuses before a result; evaluator or schema-registry semantic drift changes the required definition/evaluation/result fingerprint closure. Repository-defined projection records may compose declarative selectors and allowlisted Handbook derivations; they cannot load executable hooks, remote code, model prompts, transport-owned business rules, content-sniffing disclosure/support rules, or extension-supplied required currentness/disclosure behavior.

### Reveal

Expose fields or referenced records already present in the exact bound source.

- deterministic;
- non-inventive;
- preserves whether the source is canonical authority, reviewed semantic memory, or descriptive observation/evidence.

### Derive

Compute a deterministic aggregate, selection, normalization, or presentation from fields in exact bound sources.

- deterministic;
- provenance-bearing;
- may be lossy as a view;
- cannot destroy, replace, or upgrade any source's authority class.

### Synthesize

Generate new interpretation, prose, decomposition, or candidate detail through an agent/model.

- not part of the initial projection engine;
- candidate-only;
- requires explicit provenance and a review/promotion gate;
- if Handbook owns the use case later, it must invoke `unified-agent-api` programmatically rather than through prompt-shaped shell behavior.

## Collapse and expansion

User-facing mental model:

- **collapse** projects broader exact source content into a smaller working set while retaining its authority class;
- **expand** requests broader horizon or finer source detail when the current envelope is insufficient.

Implementation-level operations should remain explicit:

- project;
- reveal;
- derive;
- escalate;
- aggregate;
- promote.

Expansion cannot invent detail absent from the exact bound sources. Missing detail either remains missing, triggers escalation, or enters a candidate synthesis workflow. If the source is canonical, only its owning canonical mutation contract can add canonical detail; if it is an observation or semantic record, expansion does not promote it into canonical truth.

Collapse is a Projection whose target envelope is equal to or narrower than the source/requester envelope on every dimension. Expansion is not a Projection result: it is either a new authorized request using an already valid broader envelope or a `ResolutionEscalationRequest`. A Projection engine never widens its own authority in order to satisfy a request.

## Projection provenance

Every capitalized Projection records:

- exact source ref/fingerprint pairs, never parallel unpaired lists;
- exact resolved-profile, vocabulary, Projection-definition, and envelope ref/fingerprint pairs;
- requester purpose and output surface;
- operation and target schema;
- definition-owned currentness requirements, request expected-revision basis, and result validation evidence, including selector/adapter/slot closure when exact checking applies;
- included source/output paths and claim refs;
- typed omissions with source path/claim, reason, and proof effect;
- computed lossiness classification;
- exact deterministic derivation refs plus input/output fingerprints;
- output and complete result fingerprints;
- `authority_effect: none`.

Initial omission reasons are `out_of_resolution`, `redacted`, `unavailable`, and `unsupported`. Every definition rule is accounted as included, omitted, or `not_applicable`. In v1, a rule is applicable exactly when its declared `operation` equals the request operation; operation mismatch is the only `not_applicable` reason. Lossiness precedence is `redacted` over `partial` over `collapsed` over `lossless`: any redaction yields `redacted`; otherwise unavailable/unsupported yields `partial`; otherwise out-of-Resolution omission or an included lossy derivation yields `collapsed`; otherwise the result is `lossless`. An omitted `required_for_result` rule carries `not_observed` even when it names no claim; any omitted rule with claim refs carries `not_observed` for each named claim; only an omitted non-required claimless rule carries `none`. The result is still emitted with the target absent under an omission-compatible target schema, but no `not_observed` requirement or claim can contribute a passing verdict. A Projection may be evidence input, but it does not mutate its source or become a second editable authority.

## Memory semantics

Memory is Resolution-tagged rather than flat. The model distinguishes **memory horizon** from **memory record class**.

### Memory horizons

Horizons describe where memory belongs and how much authority it may carry:

- strategic memory: rare, high-authority, explicitly reviewed;
- coordination memory: contracts, decisions, feature/seam truth;
- execution memory: task outcomes, proof, local tradeoffs;
- operation memory: transient observations and command output.

### Memory record classes

Record classes describe what kind of memory a record contains:

- **semantic memory** — reviewed conclusions, decisions, invariants, and promoted knowledge;
- **Snapshot Memory** — immutable point-in-time observations of selected project/world state;
- **transition memory** — handoffs, escalations, dispatches, and intended next action;
- **operational memory** — transient observations and working state that have not been promoted.

A Snapshot Memory record can exist at any memory horizon. It is not a fifth horizon after operation memory.

An observation is not durable higher-horizon memory merely because an agent emitted it.

Promotion requires:

- a target memory horizon;
- source evidence references;
- validation appropriate to the target horizon;
- explicit authority to update that truth;
- a durable promotion record.

Promotion is compare-and-write against the target memory authority. Stale source fingerprints, missing validation for the target horizon, or an approver outside the target authority fail closed. The promoted record is new reviewed semantic memory with its own identity; the source observation, delta, or Projection remains immutable evidence.

## Snapshot Memory

### Definition

Snapshot Memory is an immutable, deterministic, provenance-bearing observation of selected repository, artifact, workflow, contract, evidence, and session state at a declared point in time and Context Resolution.

Recommended internal names include:

- `ContextMemorySnapshot`;
- `SnapshotCapturePolicy`;
- `SnapshotDelta`;
- `SnapshotProjectionRequest`;
- `SnapshotProjectionResult`.

Avoid an unqualified `Snapshot` type because tests, pipeline routing, capture rollback, and other subsystems already use snapshot terminology for unrelated purposes.

### Authority rule

A snapshot records what was observed. It does not define:

- what must be true — contracts and canonical artifacts own that;
- what should happen next — handoffs and dispatches own that;
- whether a claim passed — evidence/verdict/gate semantics own that.

A snapshot may be submitted as evidence or grounding context. It never becomes a peer truth authority.

### Snapshot contents

A capture policy selects fields from several state families.

`SnapshotCapturePolicy` is an exact versioned definition. It pins every allowed memory horizon, source adapter, multi-source family slot/composite-revision rule, static bounded-window selection rule, comparison contract, bounded-skew rule, redaction policy, retention policy, predecessor rule, and drift-rule catalog by ref/fingerprint and carries a derived policy fingerprint. Live source revision and exclusive cursor are capture inputs recorded in the snapshot, not policy-definition fields. It cannot discover ambient state families, widen a source adapter, or select a more durable memory horizon at capture time. Every allowed horizon/trigger/record-class combination must resolve exactly one retention rule.

#### Capture identity

- snapshot ID and schema version;
- capture trigger and capture-policy ref/fingerprint;
- capture start/end time;
- producing Handbook/SDK version;
- repository/workspace identity;
- exact resolved-profile and active-envelope refs/fingerprints;
- previous snapshot ref/record-fingerprint pair;
- state and record fingerprints.

#### Git and worktree state

- repository identity;
- current branch and worktree identity;
- HEAD and upstream/tracking state;
- merge/rebase/cherry-pick state where relevant;
- staged, modified, deleted, and untracked path inventories;
- diff statistics and fingerprints;
- references to separately stored patch/full-diff evidence where policy permits;
- dirty-state fingerprint.

Large diffs are referenced rather than embedded.

#### Handbook state

- selected instance profile, artifact-kind registry, vocabulary profile, and Resolution-stack fingerprints;
- canonical artifact inventory, lifecycle states, and content fingerprints;
- active intake/candidate refs, unresolved coverage, and pending approvals without embedding sensitive interview transcripts;
- resolved project-posture kernel fingerprint and pending recommendation/transition refs;
- active contract IDs/statuses;
- unresolved required claims, blockers, and promotion obligations;
- active dock/capability versions;
- latest relevant verdict and gate refs.

#### Work and planning state

- active phase, slice, packet, task, or configured work-unit roles;
- bounded recent-completion window;
- current in-flight work;
- bounded expected/queued-next window;
- child packets and decompositions;
- deferred or blocked work;
- expected acceptance and proof gates;
- actual completed proof gates.

Recent/next windows record their selection policy, ordering source, cursor, and requested count. “Last N” is not deterministic unless the source ledger and ordering rule are explicit.

#### Session and transition state

- current/prior top-level orchestration refs;
- latest applicable true-stop handoff and orchestration decision;
- active internal dispatches and parent reconciliation status;
- proof-relevant delegated-run identities, statuses, and verdicts;
- current authority boundary;
- unresolved escalations;
- next top-level resume or human-interaction condition, when one exists.

#### Evidence and environment state

- recent validation/evidence refs and normalized statuses;
- proof classifications;
- relevant non-secret toolchain/runtime versions;
- flaky/failed/incomplete proof obligations;
- declared exclusions and redactions.

### Strategic capture hooks

Capture policies may activate at:

- top-level orchestration onboarding and true-stop closeout;
- internal dispatch creation, delegated-run start/result, and parent reconciliation;
- phase, slice, packet, or task open/close;
- child-packet decomposition;
- blocker or escalation creation/resolution;
- canonical artifact create/update/lock/promotion;
- intake start/close, candidate validation/promotion/refusal, and targeted reassessment;
- project-posture recommendation, acknowledgment, approval, or transition;
- contract review-ready/lock/activate/close/deprecate transitions;
- evidence, verdict, or gate completion;
- commit, branch/worktree change, merge, publish, or released-consumer proof.

Do not snapshot every command by default. Command streams are operational logs. Snapshot Memory captures strategically meaningful state boundaries.

### Paired top-level orchestration snapshots

The primary onboarding pattern is:

```text
prior top-level orchestration end snapshot
  -> resumed top-level orchestration start snapshot
  -> deterministic SnapshotDelta
  -> Resolution-aware grounding projection
```

This makes a stale handoff, unexpected git change, queue reorder, new blocker, or altered contract state visible before the next session acts.

Every boundary snapshot records a stable boundary-stream ref and a unique, strictly increasing sequence within its repository/workspace/stream. A concurrent sequence-allocation collision retries or refuses; it cannot introduce a tie-breaker. The exact predecessor rule in its capture policy defines legal trigger transitions and selects the greatest eligible earlier sequence in the same repository, workspace, and stream. A previous-snapshot link cites that record fingerprint and sequence. Self, future, duplicate-sequence, cyclic, skipped-eligible, wrong-stream/workspace, or wrong-trigger links fail closed.

### Capture consistency

Git, artifacts, work ledgers, contracts, and evidence may change during capture. Every snapshot records:

- capture start/end time;
- relevant pre/post revisions or fingerprints;
- consistency mode: `stable`, `bounded`, or `unstable`;
- retry/refusal outcome;
- surfaces not captured atomically.

Each selected state family also records its exact source-adapter ref/fingerprint, pre/post revision pair, the exact immutable revision used to produce its payload, normalized payload fingerprint, static-window plus capture-time source/cursor inputs where applicable, and capture disposition. When one family adapter reads multiple source slots, its family revision is a deterministic composite over every declared slot, and the snapshot also records pre/captured/post revisions per slot. `stable` requires equal pre/captured/post revisions for the family composite and every slot. `bounded` requires an exact immutable captured revision for each separately read payload/slot plus an exact policy-selected bound-rule/evaluation record proving that cross-source skew stayed within the bound. Merely finishing a read while sources changed is not bounded consistency.

Top-level consistency is derived, never caller-selected. With no selected-family exclusions, all-stable families yield `stable`; a mix of stable/bounded families with every exact bound evaluation passing yields `bounded`. Any unstable observed family or any whole selected-family exclusion (`unavailable`, `unsupported`, `redacted`, or `unstable`) yields top-level `unstable` plus diagnostic-only admissibility, or no record when the policy says refuse. Field-level redaction inside an observed family is instead recorded by redaction dispositions and does not itself change source consistency. If authoritative revisions change outside the permitted bound, the system retries under the exact policy or emits/refuses according to this table. An unstable snapshot cannot ground promotion, closeout, a hard gate, or a later delta that claims stable comparison.

### Fingerprints

Use two distinct fingerprints:

- **state fingerprint** — normalized observed state, excluding volatile capture metadata such as timestamp;
- **record fingerprint** — the complete immutable snapshot record, including trigger, time, provenance, and redaction declarations.

Two snapshots taken at different times may share a state fingerprint while remaining distinct records.

The state fingerprint covers schema identity, repository/workspace identity, exact capture-policy/profile/envelope fingerprints, every selected state-family composite and per-slot pre/captured/post revision, bound evaluation, window capture input and normalized payload, ordered exclusions, and redaction outcomes. It excludes snapshot ID, boundary-stream ref/sequence, capture timestamps/trigger, previous-snapshot link, record fingerprint, and other record-only provenance. The record fingerprint covers the normalized complete record except itself. Therefore equal selected state under the same semantic policies may retain one state fingerprint across later boundary sequences or different transition streams while producing distinct record fingerprints. Map keys, repository-relative paths, unordered semantic sets, and evidence refs use contract-defined canonical ordering.

### Snapshot deltas

`SnapshotDelta` is a deterministic derived artifact. It compares two compatible snapshots without mutating either.

Compatibility is fail-closed. Both inputs must cite exact record fingerprints, the same repository/workspace identity, compatible schema versions, and the same exact comparison-contract ref/fingerprint. The comparison contract declares comparable policy/state-family versions and path/record identity rules. A delta names common compared families plus every family excluded because it was absent, redacted, unstable, or incompatible; it never silently treats missing data as unchanged.

Useful delta families include:

- planned work completed/not completed;
- unplanned work introduced;
- child packets created;
- queue reorder or scope expansion;
- files changed inside/outside predicted scope;
- artifact/contract lifecycle transitions;
- intake coverage/approval changes and posture recommendation/transition changes;
- proof gates gained, lost, or still missing;
- dirty state introduced/resolved;
- blockers/escalations added or cleared;
- handoff prediction versus actual resumed top-level work;
- internal dispatch objective versus delegated result and parent disposition;
- repeated rework or failure patterns.

### Drift semantics

Deterministic policy may classify deltas as:

- `expected_progress`;
- `justified_divergence`;
- `unexplained_drift`;
- `scope_expansion`;
- `execution_inefficiency_signal`;
- `planning_inaccuracy_signal`;
- `proof_drift`;
- `semantic_drift`;
- `stale_handoff`.

Divergence is not automatically failure. A durable handoff, escalation, decision, or child-packet record may explain and authorize the change. The delta records the signal and justification refs; it does not invent intent.

Every normalized change identifies family, stable path/record key, change kind, and before/after value fingerprints. The delta binds the exact drift-rule catalog resolved from the endpoint policies or explicitly admitted by their comparison contract. It evaluates every catalog rule exactly once in catalog order as matched, not matched, or not applicable; every matched evaluation produces exactly one unique signal and every signal maps back to exactly one matched catalog rule. Missing, duplicate, stale, refused, or caller-skipped evaluation fails closed. Every signal cites its stable ID, exact rule ref/fingerprint, matching changes, evidence refs, and optional durable justification refs. No free-form explanation changes deterministic classification. `delta_fingerprint` covers both exact snapshot inputs, the comparison contract, drift catalog, compared/excluded families, normalized changes, complete rule evaluations, signals, and justification refs except itself.

### Resolution-aware snapshot projection

A snapshot may be comprehensive, but an agent must not automatically receive all of it.

Snapshot projection is a specialized capitalized Projection. It uses the same generic `sources`, `resolution_envelope`, exact `ProjectionDefinition`, request/result provenance, included/omissions/not-applicable rule accounting, lossiness, fingerprint, and `authority_effect: none` fields as every other Projection. Its definition declares an exactly-one snapshot selector and may declare a separate exactly-one compatible-delta selector; if the selector exists the request must satisfy it rather than treating that selector as optional. It applies the target Context Resolution envelope to select:

- relevant work and queue state;
- applicable contracts and constraints;
- changed paths and diff summaries appropriate to authority;
- unresolved blockers/escalations;
- proof and promotion obligations;
- recent history needed by the requested horizon.

An execution agent may receive task-local changes and next actions. A strategic session may receive phase drift, cumulative proof debt, queue churn, and unresolved architecture decisions.

Projection provenance records included/omitted snapshot fields and exact source snapshot/delta record fingerprints. The snapshot-grounding definition's mandatory currentness field declares the exact required family, source-selector, adapter, source-slot set, and `captured_revision` basis. Its disclosure contract applies the same per-rule minimum Resolution and exact metadata-only policy as generic Projection. A snapshot/delta redaction disposition records an exact original JSON Pointer/subtree and action-typed optional retained pointer outside that subtree. A request for the original/subtree deterministically yields a Projection `redacted` omission before payload access, never `unavailable`; the exact retained transformed/fingerprint field is evaluated independently even when it shares earlier path segments. A caller that needs a fresh capture completes it before constructing the Projection request, then binds that new snapshot (and any compatible delta) as the request's exact sources. The request expected values are copied from that exact snapshot's captured family composite/per-slot revisions; result observations must equal those same values, not merely agree with unrelated caller input. Unfiltered delta signals require currentness coverage for every compared family, while unchecked-family signals must be filtered/omitted with proof effects. A placeholder or pre-capture source, omitted/extra/duplicate family, selector/adapter/slot substitution, captured-value mismatch, missing check, stale adapter, or unchecked delta-signal family produces a typed refusal and no Projection result. Comprehensive capture never implies comprehensive disclosure.

### Handoff and snapshot relationship

Keep the record roles separate:

| Record | Question answered |
|---|---|
| Contract/canonical artifact | What must be true? |
| Snapshot Memory | What was observed at this boundary? |
| Handoff | Why did top-level orchestration stop, what happened, and how may it resume? |
| Dispatch | What exact internal delegated run, top-level resume, or human interaction is authorized? |
| Evidence/verdict/gate | What supports or decides a contract claim? |
| SnapshotDelta | What changed between observations? |

Handoffs reference top-level start/end snapshots and deltas rather than copying their contents. Internal delegated-run results may reference narrower snapshots or evidence, but they return to the active parent and do not become global handoffs merely because a subagent turn ended.

A proof-relevant delegated-run result identifies its parent orchestration, immutable dispatch and fingerprint, role, built-in agent identity/status, required skills, reviewed/produced subject fingerprint, review round, predecessor/remediation lineage, verdict/finding refs, and parent disposition. This is execution evidence, not a new authority class.

### Security, redaction, and retention

Snapshot capture must not include by default:

- secret values or credential material;
- unrestricted environment variables;
- `.env` or secret-file contents;
- raw command arguments that may contain secrets;
- unrestricted command output;
- large/full diffs when a fingerprint and evidence reference suffice.

Every snapshot records the redaction/capture policy and excluded surfaces.

Redaction is driven by one exact `SnapshotRedactionPolicy`. V1 fixes `fail_closed: true` and unmatched-surface action `omit`; both enter the policy fingerprint. Rules match typed surfaces and choose `omit`, `fingerprint_only`, `artifact_ref_only`, or `redacted_summary`. When rules overlap, identical actions remain valid and `omit` wins over any other action; two or more distinct non-omit actions are incomparable and the capture refuses instead of guessing. Unknown, unclassifiable, matcher-failed, and known-but-unmatched input is omitted. Explicit deny-floor rules cover secret values, unrestricted environment variables, `.env`/secret-file contents, raw command arguments/output, and unrestricted full diffs, and no invocation flag can weaken them.

Retention is driven by one exact `SnapshotRetentionPolicy` keyed by the complete tuple of memory horizon, trigger, and record class. Contract/milestone snapshots may be durable indefinitely; high-frequency session or operation snapshots may use content-addressed deduplication, retention windows, or reviewed compaction. Deduplication may share immutable payload storage but preserves distinct record identities. Compaction creates a new reviewed aggregate with source refs/fingerprints and never rewrites retained records or removes a record still referenced by a handoff, evidence, gate, legal hold, or active retention floor.

## Resolution-aware validation

A projection can prove only what it exposes and observes.

- Included and observed required claims may pass.
- Omitted or unavailable claims remain `not_observed` unless another evidence source covers them.
- A local task can close without claiming that its parent feature or phase is complete.
- A gate must distinguish local completion from promotion readiness.
- An external dock declares the Resolution envelope under which it collected evidence.

## Adapter implications

`handbook-sdk` presents transport-neutral typed use cases over the semantic owners. CLI, Tauri, and Substrate adapters preserve the selected operation definition, negotiated API context, request fingerprint, schema versions, exact semantic refs/fingerprints, Resolution envelope, provenance, omissions, idempotency result, conditional write set/receipts, and typed outcome; they may change invocation mechanics and product wording but not domain meaning. A distinct immutable bootstrap descriptor per API major is the discovery root; snapshot-bound paged operation/profile/schema/governed-record catalogs, exact record and vocabulary/Resolution/Projection reads, and applicable-snapshot selection follow it, and adapters never infer shipped defaults or depend on retained process-local state.

Future workflow adapters map:

- stable semantic roles;
- vocabulary labels and aliases;
- structural conflations;
- artifact kinds;
- Resolution placement;
- projection expectations;
- lifecycle and gate semantics.

An adapter translates between systems. It does not mutate Handbook canonical truth merely to imitate another tool's terminology.

No adapter may infer an operation from CLI help/prose, convert a blocked/refused outcome into an untyped string, silently widen a Resolution envelope, or replace a canonical semantic ref with a display label. The transitional Substrate CLI bridge and permanent crates.io Rust boundary are separate adapters and require separate proof; success of either one does not prove the other.
