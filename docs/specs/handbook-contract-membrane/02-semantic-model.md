# Semantic Model

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

## Instance profile

An instance profile defines the Handbook semantics for one repository or project.

It contains:

- profile identity and schema version;
- artifact-kind sources and artifact-instance registry;
- vocabulary profile;
- Resolution stack;
- projection definitions;
- validator/dock requirements;
- shipped defaults and explicit repository overrides;
- optional adapter overlays.

Recommended first-version precedence:

```text
shipped default profile
  -> selected named profile
  -> repository profile
  -> explicit invocation overrides
```

Ambient, silently discovered local overrides are out of scope initially.

## Artifact-kind and instance registries

The registry replaces the fixed assumption that every repository has exactly the same canonical documents. It distinguishes reusable **artifact kinds** from configured **artifact instances**.

### `ArtifactKindDefinition`

An artifact kind defines the reusable shape and behavior of a class of canonical artifact. It identifies at least:

- stable kind ID, schema version, and definition fingerprint;
- canonical content schema reference;
- zero or more compatible semantic roles/capabilities;
- structural and semantic-validation profiles;
- optional `ArtifactIntakeDefinition`;
- supported deterministic projections;
- lifecycle and review-trigger semantics;
- required cross-artifact capabilities;
- extension and compatibility rules.

The kind definition does not contain a repository-specific path, user-facing instance label, or requiredness decision.

### `ArtifactInstanceDescriptor`

An artifact instance binds one kind into a repository/project profile. It identifies at least:

- stable artifact instance ID;
- artifact-kind reference and exact version/range policy;
- selected semantic role when the kind supports more than one;
- user-facing label;
- canonical repo-relative path or path template;
- requiredness rule and applicable project conditions;
- dependencies on other instance IDs or semantic capabilities;
- lifecycle/lock state or policy reference;
- selected intake and projection definitions;
- repository-specific validation overlays allowed by the kind.

One kind may back multiple instances. A profile selects instances; it does not create new hard-coded product types.

### Constitutional root

The shipped model retains one required constitutional-root semantic role. Its display name and path may be profile-defined; the shipped default binding may remain Charter.

This is a semantic invariant, not a literal filename requirement.

A custom kind may satisfy the constitutional-root role only when its declared capabilities and schema meet the constitutional contract. Merely assigning the label `Charter` is insufficient.

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
- Custom artifacts receive generic structural validation, lifecycle, projection, and contract behavior first.
- Custom kinds may provide an intake coverage contract and deterministic projections without adding Handbook code.
- A custom artifact does not require a new Rust enum variant merely to exist.
- Custom kinds never generate or rename CLI commands; stable generic operations select kind/instance IDs.
- Remote schema fetching, executable schema hooks, and ambient unversioned overrides are out of scope initially.

Structural JSON Schema validates YAML after parsing. Cross-field/cross-artifact semantic rules remain separate, and external domain validators continue to integrate through docks rather than executable code embedded in schemas.

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
- reassessment triggers for affected coverage items.

Three first-version acquisition modes share the same definition and output schema:

- `guided_adaptive` — recommended; the skill-directed agent inspects repository evidence and asks only unresolved, normative, high-impact, contradictory, or low-confidence questions;
- `express` — asks a smaller high-value subset, then exposes inferred/defaulted fields and known unknowns for review;
- `agent_assisted` — prepares the fullest evidence-backed candidate possible and asks only blocking or authority-required questions.

Mode changes the acquisition path, not the canonical schema or semantic quality bar. Missing coverage remains explicit and cannot be hidden by choosing a shorter mode.

### Intake records, candidates, and promotion

Keep three authorities distinct:

| Record | Role | Authority |
|---|---|---|
| `ArtifactIntakeRecord` | Immutable audit of questions, declarations, inferences, evidence, confidence, gaps, and evaluation | descriptive provenance only |
| `ArtifactCandidate` | Normalized artifact-shaped proposal with source mapping and unresolved issues | candidate; never canonical |
| canonical artifact YAML | Approved project truth used by projections/contracts | authoritative at its declared semantic role |

The LLM agent running the Handbook skill conducts the interview and uses CLI/SDK operations to inspect coverage, submit observations/declarations, validate the candidate, and request promotion. Handbook supplies deterministic schemas, evaluation, and promotion rules; it does not wrap an internal question-by-question UI or perform an untracked nested model call.

### Charter intake

The historical Charter questionnaire becomes the first rich `ArtifactIntakeDefinition`: `CharterIntakeDefinition`.

Its coverage domains include project identity/shape, delivery constraints, operational reality, baseline posture, default delivery implications, risk domains, engineering-posture dimensions, exceptions/governance, engineering debt, and decision-record policy. The exact wording and branching may change, but lost coverage requires an explicit design decision rather than accidental omission.

Each normalized answer maps to one of:

- canonical Charter policy or governance;
- Charter rationale;
- a referenced, freshness-qualified observation used to justify a decision;
- an unresolved or explicitly waived intake gap.

Canonical Charter YAML is the constitutional authority. Its Markdown document, Tauri view, packets, and agent contexts are projections. The intake record remains available for audit and targeted reassessment without becoming a second editable Charter.

## Project posture kernel

`ProjectPostureKernel` is a deterministic, fingerprinted resolved view rather than an additional canonical artifact by default.

It resolves:

- canonical Charter posture, red lines, exception policy, and decision authority;
- approved domain/instance overrides;
- applicable project conditions and contract state;
- current evidence and snapshot references where freshness matters;
- effective engineering-posture levels, floors, triggers, shortcuts, and proof obligations.

The Charter is its primary constitutional authority. Observations may change the kernel's applicability calculation, but do not rewrite Charter policy.

A `PostureRecommendation` may be derived when snapshots, contracts, evidence, or lifecycle events cross an approved trigger. It records affected dimensions/scope, proposed transition, triggering observations, evidence refs, confidence, urgency, approval requirement, and suggested actions. A recommendation is not an enacted posture change.

Only an authorized, reviewed `PostureTransition` may update canonical policy or approved overrides. Raising posture may react to a single hard trigger; lowering posture requires sustained evidence across a configured window and cannot cross approved floors or red lines. This hysteresis prevents policy thrashing.

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

## Context Resolution

Use namespaced implementation terminology such as:

- `ContextResolution`;
- `ContextResolutionEnvelope`;
- `ResolutionProjection`;
- `ResolutionEscalation`;
- `ResolutionPromotion`.

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

The first version uses a configurable ordered stack rather than an arbitrary graph.

The shipped default may resemble:

```text
strategic
  -> coordination
  -> execution
  -> operation
```

Profiles may rename, add, remove, or intentionally collapse levels. Work-unit terms and resolution levels are related but not identical.

The current `L0`–`L3` work-level mechanism is a useful precursor, not the final semantic model, because it mixes scope levels with workflow phases such as merge/quality gate.

## Resolution envelope

Conceptual shape:

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
active_level: execution
goal_ref: work.task.capture-screenshot
dimensions:
  scope_horizon: assigned_task
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: task_local
  memory_horizon: execution_record
  validation_horizon: task_closeout
parent_refs:
  - feature.browser-evidence
inherited_constraint_refs:
  - charter.security
  - feature.browser-evidence.acceptance
allowed_mutations:
  - crates/browser/**
forbidden_mutations:
  - docs/canon/**
escalation_triggers:
  - parent_contract_conflict
  - architectural_decision_required
  - insufficient_acceptance_criteria
```

Higher-horizon constraints constrain lower-horizon work. Lower-horizon observations do not rewrite higher-authority truth without promotion.

## Projection semantics

A projection is a derived view from canonical structured truth for a declared requester, surface, vocabulary profile, and Resolution envelope.

### Reveal

Expose canonical fields or referenced child artifacts already present.

- deterministic;
- non-inventive;
- canonical-source preserving.

### Derive

Compute a deterministic aggregate, selection, normalization, or presentation from canonical fields.

- deterministic;
- provenance-bearing;
- may be lossy as a view;
- cannot destroy or replace source truth.

### Synthesize

Generate new interpretation, prose, decomposition, or candidate detail through an agent/model.

- not part of the initial projection engine;
- candidate-only;
- requires explicit provenance and a review/promotion gate;
- if Handbook owns the use case later, it must invoke `unified-agent-api` programmatically rather than through prompt-shaped shell behavior.

## Collapse and expansion

User-facing mental model:

- **collapse** projects broader canonical meaning into a smaller working set;
- **expand** requests broader horizon or finer canonical detail when the current envelope is insufficient.

Implementation-level operations should remain explicit:

- project;
- reveal;
- derive;
- escalate;
- aggregate;
- promote.

Expansion cannot invent detail absent from canonical truth. Missing detail either remains missing, triggers escalation, or enters a candidate synthesis workflow.

## Projection provenance

Every projection should record:

- source artifact references and fingerprints;
- profile and vocabulary versions;
- projection definition/version;
- requested Resolution envelope;
- fields or claims included;
- fields or claims omitted;
- lossiness classification;
- deterministic derivation identifiers;
- synthesis provenance when applicable;
- promotion eligibility.

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

#### Capture identity

- snapshot ID and schema version;
- capture trigger and capture-policy ID/version;
- capture start/end time;
- producing Handbook/SDK version;
- repository/workspace identity;
- active Context Resolution;
- previous snapshot reference;
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

- current/previous session refs;
- latest handoff and orchestration decision;
- active dispatch;
- current authority boundary;
- unresolved escalations;
- next recommended session type.

#### Evidence and environment state

- recent validation/evidence refs and normalized statuses;
- proof classifications;
- relevant non-secret toolchain/runtime versions;
- flaky/failed/incomplete proof obligations;
- declared exclusions and redactions.

### Strategic capture hooks

Capture policies may activate at:

- session onboarding and closeout;
- orchestration dispatch creation/consumption;
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

### Paired session snapshots

The primary onboarding pattern is:

```text
prior session end snapshot
  -> new session start snapshot
  -> deterministic SnapshotDelta
  -> Resolution-aware grounding projection
```

This makes a stale handoff, unexpected git change, queue reorder, new blocker, or altered contract state visible before the next session acts.

### Capture consistency

Git, artifacts, work ledgers, contracts, and evidence may change during capture. Every snapshot records:

- capture start/end time;
- relevant pre/post revisions or fingerprints;
- consistency mode: `stable`, `bounded`, or `unstable`;
- retry/refusal outcome;
- surfaces not captured atomically.

If authoritative revisions change during capture, the system retries under a bounded policy or emits an `unstable` snapshot that cannot ground promotion or closeout decisions.

### Fingerprints

Use two distinct fingerprints:

- **state fingerprint** — normalized observed state, excluding volatile capture metadata such as timestamp;
- **record fingerprint** — the complete immutable snapshot record, including trigger, time, provenance, and redaction declarations.

Two snapshots taken at different times may share a state fingerprint while remaining distinct records.

### Snapshot deltas

`SnapshotDelta` is a deterministic derived artifact. It compares two compatible snapshots without mutating either.

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
- handoff prediction versus actual next-session work;
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

### Resolution-aware snapshot projection

A snapshot may be comprehensive, but an agent must not automatically receive all of it.

Snapshot projection applies the target Context Resolution envelope to select:

- relevant work and queue state;
- applicable contracts and constraints;
- changed paths and diff summaries appropriate to authority;
- unresolved blockers/escalations;
- proof and promotion obligations;
- recent history needed by the requested horizon.

An execution agent may receive task-local changes and next actions. A strategic session may receive phase drift, cumulative proof debt, queue churn, and unresolved architecture decisions.

Projection provenance records included/omitted snapshot fields and the source snapshot fingerprint. Comprehensive capture never implies comprehensive disclosure.

### Handoff and snapshot relationship

Keep the record roles separate:

| Record | Question answered |
|---|---|
| Contract/canonical artifact | What must be true? |
| Snapshot Memory | What was observed at this boundary? |
| Handoff | What happened, and how should work transition? |
| Dispatch | What exactly may the next session do? |
| Evidence/verdict/gate | What supports or decides a contract claim? |
| SnapshotDelta | What changed between observations? |

Handoffs reference start/end snapshots and deltas rather than copying their contents.

### Security, redaction, and retention

Snapshot capture must not include by default:

- secret values or credential material;
- unrestricted environment variables;
- `.env` or secret-file contents;
- raw command arguments that may contain secrets;
- unrestricted command output;
- large/full diffs when a fingerprint and evidence reference suffice.

Every snapshot records the redaction/capture policy and excluded surfaces.

Retention is policy-driven by horizon and trigger. Contract/milestone snapshots may be durable indefinitely; high-frequency session or operation snapshots may use content-addressed deduplication, retention windows, or reviewed compaction without rewriting retained immutable records.

## Resolution-aware validation

A projection can prove only what it exposes and observes.

- Included and observed required claims may pass.
- Omitted or unavailable claims remain `not_observed` unless another evidence source covers them.
- A local task can close without claiming that its parent feature or phase is complete.
- A gate must distinguish local completion from promotion readiness.
- An external dock declares the Resolution envelope under which it collected evidence.

## Adapter implications

Future workflow adapters map:

- stable semantic roles;
- vocabulary labels and aliases;
- structural conflations;
- artifact kinds;
- Resolution placement;
- projection expectations;
- lifecycle and gate semantics.

An adapter translates between systems. It does not mutate Handbook canonical truth merely to imitate another tool's terminology.
