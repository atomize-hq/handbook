# Phase and Slice Map

## Status

This is the provisional program decomposition. Phase 0 documentation/design slices may proceed after control-pack review. No Rust implementation slice is authorized until its own packet is created and approved.

## Sequencing rule

The program proceeds from semantic authority to representation, projection, transport, verification, and consumers:

```text
control-pack/design authority
  -> artifact-kind/intake semantics + researched default set
  -> profile and artifact-instance semantics
  -> canonical YAML truth
  -> vocabulary + Context Resolution + Snapshot Memory + posture + projections
  -> SDK + complete machine transports
  -> contract membrane + docks
  -> Substrate/Tauri/adapters
```

Do not begin with the CLI, Tauri UI, external docks, or Substrate integration before their underlying typed semantics exist.

## Top-level slice-runner rule

Every authorized slice runs under one top-level orchestrator selected by explicit `PHASE_ID`, `SLICE_ID`, and optional packet. An optional handoff is resume context for that scope; it does not select the slice.

The parent owns preflight, selective context assembly, specification/plan repair, implementation or documentation, verification, fresh review, remediation, re-review, proof-wall closeout, control-pack updates, and commit. Bounded child work may be delegated through immutable dispatch envelopes to fresh built-in `default` subagents, but the parent waits for their structured results and remains active.

Creating a child packet or internal dispatch does not complete the parent slice. A new top-level handoff/task is justified only by slice completion, required human interaction, external blockage, broader authority, context/runtime exhaustion, or unavailable mandatory delegation.

## Phase 0 — Architecture and contract freeze

**Purpose:** turn the current idea lineage into implementation-grade target authority without changing Rust.

| Slice | Objective | Primary outputs | Exit gate |
|---|---|---|---|
| `HCM-0.1` | Establish the context-engineering control pack | `00`–`08`, handoff schema/template, active-doc pointers | historical container/bootstrap scope is preserved; its recorded clean review remains evidence for the semantic content it checked, but its user-routed per-round handoff choreography is superseded by `HCM-0.8` |
| `HCM-0.2` | Freeze artifact-kind/instance, schema-registry, intake, instance-profile, vocabulary, and constitutional-root semantics | approved `02` and exact schema sections in `05`; `ArtifactKindDefinition`, `ArtifactInstanceDescriptor`, `ArtifactIntakeDefinition`, Charter intake/promotion, and posture-kernel decisions | every semantic field has an owner, defaulting rule, validation rule, authority boundary, and explicit non-goal; examples are not treated as shipped defaults |
| `HCM-0.3` | Freeze Context Resolution, Snapshot Memory, and deterministic projection contracts | exact envelope, snapshot/capture/delta/projection, omission, provenance, consistency, redaction, and promotion contracts | reveal/derive/synthesize boundaries and snapshot authority/consistency rules are unambiguous and testable |
| `HCM-0.4` | Freeze crate ownership, `handbook-sdk`, CLI JSON, Tauri, and Substrate integration ladder | owner matrix, SDK use-case inventory, transport DTO contract, published proof plan | no use case depends on CLI prose; bridge and permanent boundary are distinct |
| `HCM-0.5` | Freeze contract-membrane and dock protocol boundaries | lifecycle, claim/evidence/verdict/gate contracts; process-dock capability/request/result contract | validators remain witnesses; Resolution limits proof; first proof dock is selected |
| `HCM-0.6` | Research and approve the shipped default artifact set | research dossier; minimal/standard/full candidate comparison; user brainstorming/decision record; approved shipped kind/instance/requiredness list | default set is evidence-backed and explicitly approved; no current enum, template, filename, or illustrative example became a default by inertia |
| `HCM-0.8` | Correct the development orchestration and true-stop handoff control plane discovered after the original HCM-0.1 review | long-lived `07` runner; parent-owned `08` protocol; internal dispatch contract; handoff v1.2 schema/template/validator; split handoff/orchestration proof | one active parent executes a fresh built-in review internally, reconciles results, and writes one v1.2 closeout; internal agents write no global handoffs; prior records/dispatches remain immutable evidence |
| `HCM-0.7` | Approve the implementation program and first slice packet | reviewed phase map plus first `slices/<id>/SPEC.md`, plan, and todo | Phase 0 contracts/default decisions are closed and the first slice is independently implementable with a complete proof wall |

`HCM-0.8` is a corrective insertion and must close before `HCM-0.7`, despite its later-discovered numeric identifier.

`HCM-0.4` consumes the frozen HCM-0.2 semantic identities and HCM-0.3 Resolution/Snapshot/Projection contracts plus the closed HCM-0.8 orchestration control plane. It does not wait for HCM-0.5 contract/dock semantics or HCM-0.6 shipped defaults: those later design slices append their approved use cases/data selections without changing the transport and ownership rules frozen here.

### Phase 0 non-goals

- Rust changes;
- public API publication;
- legacy migration tooling;
- new CLI commands;
- Tauri scaffolding;
- actual dock execution;
- speculative third-party workflow adapters.
- implementation-selected shipped artifact defaults before `HCM-0.6` approval.

## Phase 1 — Profile and artifact semantic kernel

**Purpose:** replace the fixed artifact universe with a versioned profile-selected semantic model.

### `HCM-1.1` — Artifact-kind and schema registry

- implement versioned `ArtifactKindDefinition` and kind-definition meta-validation;
- resolve local canonical schemas with stable IDs, versions, and fingerprints;
- separate structural schema, semantic validation, intake coverage, and external docks;
- refuse remote/ambient/unversioned schema execution;
- prove one repository-defined custom kind without a Rust enum variant.

### `HCM-1.2` — Profile schema, artifact instances, and shipped default

- define typed profile identity/version;
- define `ArtifactInstanceDescriptor` independently from kind definitions;
- encode only the shipped default set approved by `HCM-0.6`;
- validate explicit profile selection and repository profile input;
- do not add a legacy profile merely to preserve old behavior.

### `HCM-1.3` — Descriptor-driven artifact-instance registry

- replace enum-owned universe with profile-resolved kind and instance registries;
- support first-party stable capabilities/roles plus custom kind and instance IDs;
- make requiredness, dependencies, paths, and validators data-driven;
- preserve trusted repo-relative path enforcement.

### `HCM-1.4` — Profile-aware setup and doctor decisions

- make setup/doctor use typed profile decisions;
- keep CLI wording outside engine decisions;
- expose machine-readable profile/capability truth.

### Phase 1 exit gate

- one selected profile determines the complete artifact registry;
- kind definitions remain distinct from repository artifact instances;
- custom kinds/artifacts do not require new enum variants or generated CLI commands;
- shipped defaults exactly match the approved `HCM-0.6` decision;
- no permanent compatibility dispatch remains;
- setup and doctor consume the same resolved profile truth.

## Phase 2 — Canonical YAML artifact authority

**Purpose:** make structured artifact data authoritative and human-readable documents derived.

### `HCM-2.1` — Vertical pilot artifact

Select one lower-risk artifact family after Phase 0 review. Project Context is the current leading implementation candidate because a structured input model and deterministic renderer already exist; pilot selection does not itself add that kind to the approved shipped default set.

- canonical YAML load/validate/write;
- renderer-derived Markdown human-review view produced by the existing fixed deterministic first-party renderer;
- source and rendered-output fingerprints without a Resolution or Projection provenance claim;
- setup, authoring, doctor, and flow integration for the pilot;
- direct cutover of tests and fixtures.

### `HCM-2.2` — Constitutional-root artifact

- cut the constitutional-root artifact to canonical structured truth;
- preserve semantic root authority without requiring a literal filename;
- implement `CharterIntakeDefinition` as the first rich intake coverage contract;
- support guided-adaptive, express, and agent-assisted acquisition through the skill-directed agent, all targeting the same Charter candidate schema;
- preserve immutable intake provenance, explicit known unknowns, validation, approval, and promotion without restoring a nested CLI wizard;
- render Markdown and any other renderer-derived human-review view only through fixed deterministic, non-Resolution, first-party renderers reading approved canonical Charter YAML;
- prove reproducible renderer-derived human-review output and lifecycle behavior.

### `HCM-2.3` — Generic custom-kind registration, intake, and validation proof

- register one repository-defined kind/schema without Handbook code changes;
- use stable generic CLI/SDK operations selected by kind/instance ID;
- validate canonical YAML through the registered custom schema;
- exercise optional intake coverage when supplied;
- prove no dynamic command or filename dispatch is involved.

### `HCM-2.4` — Remaining shipped artifact families

- convert remaining first-party artifacts that need structured manipulation;
- decide explicitly which pre-Phase-3 outputs are renderer-derived human-review views only;
- remove superseded Markdown-authority helpers.

### Phase 2 exit gate

- each targeted artifact has exactly one editable canonical truth;
- all intake modes converge on the same kind-selected canonical schema and expose missing coverage;
- the Charter intake record/candidate/canonical boundaries are auditable and non-competing;
- at least one custom kind registers, validates, and exercises supplied intake coverage without a Rust enum variant or generated command;
- Markdown and other Phase 2 renderer-derived human-review views are reproducibly derived only by fixed deterministic, non-Resolution, first-party renderers and remain outside the Projection contract;
- generic configured custom-kind Projections and all Resolution-aware views remain deferred until `HCM-3.2` and `HCM-3.3` land;
- no user migration tooling or dual-read promise exists;
- every temporary internal cutover bridge named in `06` is deleted.

## Phase 3 — Vocabulary, Context Resolution, Snapshot Memory, and Projections

**Purpose:** make views and agent context profile-aware and resolution-aware.

### `HCM-3.1` — Vocabulary resolution

- axis-based labels;
- lexical and structural conflation;
- deterministic renderer consumption;
- stable machine semantics beneath local terminology.

### `HCM-3.2` — Context Resolution kernel

- configurable ordered stack;
- six explicit dimensions;
- inheritance, mutation, memory, validation, and escalation semantics;
- migration of useful work-level behavior without freezing L0-L3.

### `HCM-3.3` — Deterministic Projection engine

- begins only after the `HCM-3.2` Context Resolution kernel is available;
- generic configured custom-kind Projections and Resolution-aware first-party views;
- reveal and derive;
- collapse/expand request handling;
- omission and lossiness accounting;
- source/profile/projection fingerprints;
- no synthesis in the core implementation.

### `HCM-3.4` — Snapshot Memory and deterministic delta engine

- capture-policy model and strategic hooks;
- immutable normalized `ContextMemorySnapshot`;
- stable/bounded/unstable consistency classification;
- state and record fingerprints;
- deterministic snapshot-to-snapshot deltas;
- expected/justified/unexplained drift signals;
- security redaction, artifact references, retention, and content-addressed deduplication posture;
- paired prior-end/new-start snapshot workflow;
- no model interpretation inside deterministic snapshot/delta semantics.

### `HCM-3.5` — Resolution-aware snapshot, packet, and pipeline adoption

- flow consumes Context Resolution rather than only byte budgets;
- flow produces Resolution-aware snapshot grounding projections rather than loading comprehensive snapshots into every session;
- pipeline scoped inclusion consumes namespaced shared semantics;
- handoffs reference session start/end snapshots and deltas;
- gates distinguish local completion from parent promotion readiness.

### `HCM-3.6` — Project posture resolution and recommendation loop

- resolve a fingerprinted `ProjectPostureKernel` from canonical Charter policy, approved overrides, applicable conditions, contracts, and evidence;
- keep engineering-posture dimensions distinct from Context Resolution dimensions;
- derive typed `PostureRecommendation` records from hard lifecycle triggers and sustained snapshot/evidence signals;
- configure threshold windows, cooldowns, recipients, and acknowledgment/escalation through an approved `PostureEvaluationPolicy`;
- require authorized `PostureTransition` records for canonical policy changes;
- apply hysteresis: immediate raise recommendations may follow hard triggers, while lowering requires sustained evidence and cannot cross floors/red lines;
- reopen only affected intake coverage instead of regenerating the Charter wholesale.

### Phase 3 exit gate

- the same canonical truth produces multiple deterministic Resolution Projections;
- stable world/project snapshots and deterministic deltas can ground session transitions;
- comprehensive snapshots are projected down to the receiving session's Resolution envelope;
- capture instability and redaction are explicit and test-covered;
- custom vocabulary appears consistently in generated surfaces;
- omitted claims cannot be misreported as passed;
- current work-level behavior is either intentionally represented or removed.
- posture recommendations remain evidence-linked and advisory until approved; resolved posture is not a second editable authority.

## Phase 4 — SDK and machine transports

**Purpose:** establish one ordinary-consumer facade and make every product transport thin.

### `HCM-4.1` — SDK owner and use-case inventory

- create or approve `handbook-sdk`;
- move composition out of CLI/compiler compatibility seams where appropriate;
- keep advanced owner-crate APIs public;
- expose capabilities, not internal module topology.

### `HCM-4.2` — Shared DTO and JSON Schema contract

- request/result/error/refusal envelopes;
- schema IDs and versioning;
- generated JSON Schema;
- deterministic serialization and compatibility tests.

### `HCM-4.3` — Complete CLI `--json` parity

- every nontrivial command emits exactly one response document on stdout;
- human rendering derives from typed results;
- progress/logging uses stderr;
- exit-code semantics are stable and documented.

### `HCM-4.4` — Tauri-ready command facade

- prove SDK DTOs can back thin Serde/Tauri commands;
- no normal-operation CLI subprocess dependency;
- no Tauri UI implementation required yet.

### `HCM-4.5` — Capability-driven Handbook skill

- update the installed Handbook skill to discover supported profiles, artifacts, schemas, vocabulary, and Resolution capabilities through machine interfaces;
- let the skill-directed LLM agent select guided-adaptive, express, or agent-assisted intake and conduct the conversation while calling stable generic CLI/SDK operations;
- expose intake coverage, evidence/confidence, unresolved gaps, candidate validation, and approval requirements through typed machine responses;
- have session onboarding request the applicable prior snapshot plus a Resolution-aware grounding projection/delta rather than the complete snapshot;
- keep the agent workflow as structured-input gathering plus supported CLI/SDK invocation;
- prohibit prompt-owned reimplementation and untracked nested synthesis;
- preserve deterministic refusal and doctor/contract closeout.

### Phase 4 exit gate

- CLI, SDK, and Tauri adapter tests exercise the same typed use cases;
- `handbook-compiler` has an explicit retained or retirement posture;
- JSON Schema covers every supported machine response;
- no transport owns domain truth.
- custom artifact kinds and profile vocabulary do not add or rename CLI commands.

## Phase 5 — Contract membrane and docks

**Purpose:** implement executable contract authority and external witness integration.

### `HCM-5.1` — Contract lifecycle and claim model

- identity, version, draft/review/lock/active/close lifecycle;
- claims, invariants, severity, and required evidence;
- canonical structured contract records.

### `HCM-5.2` — Evidence, verdict, and gate engine

- normalized evidence;
- pass/fail/blocked/warning/not-observed/not-applicable/flaky;
- hard-fail semantics independent of weighted score;
- Resolution-qualified proof and promotion.

### `HCM-5.3` — Process dock protocol

- capability manifest;
- JSON request/result;
- isolation, timeout, cancellation, and refusal semantics;
- evidence and artifact references.

### `HCM-5.4` — First real validator dock proof

- choose one existing validator ecosystem;
- execute it through the process protocol;
- normalize evidence;
- prove a real contract gate without making the validator authoritative.

### Phase 5 exit gate

- a locked contract drives a real dock/evidence/verdict/gate path;
- missing or out-of-resolution evidence cannot produce false green;
- process protocol is stable enough for later Rust-native binding;
- `handbook contract ...` use cases exist behind SDK types before CLI polish claims completion.

## Phase 6 — Consumer adoption

### `HCM-6.1` — Bundled CLI Substrate bridge

- exact binary and schema versions;
- JSON-only consumption;
- isolated replaceable adapter;
- real Substrate product seam.

### `HCM-6.2` — Published SDK/owner APIs

- publish exact affected crate versions;
- registry-only external consumer proof;
- no path fallback.

### `HCM-6.3` — Direct Substrate crates.io adoption

- dedicated worktree from current Substrate tip;
- exact published versions;
- real seam using the new API;
- proof wall and bridge-replacement decision.

### `HCM-6.4` — Tauri product implementation

- GUI over the same SDK capabilities, including artifact-kind discovery, intake coverage/candidate review, and approval;
- artifact, snapshot timeline/delta, projection, contract execution, and evidence display;
- no new semantic authority in the frontend.

### `HCM-6.5` — Workflow adapter foundation

- adapter manifests and semantic mappings;
- profile/vocabulary/Resolution translation;
- no adapter marketplace or broad third-party inventory yet.

## Finding-driven decomposition protocol

Implementation and review findings may reveal that a task needs further decomposition. The active session must not silently widen.

Classify each finding as one of:

1. `local_remediation` — inside current packet authority;
2. `child_packet_required` — same slice, but independently reviewable work;
3. `cross_document_repair` — pack/spec/contract inconsistency must be fixed before code continues;
4. `resolution_escalation` — a broader design, authority, or validation decision is required;
5. `external_blocker` — dependency/environment/human action outside the repo;
6. `proof_gap` — implementation may exist, but required evidence is missing;
7. `future_program` — valuable but outside this program's approved target.

The active top-level orchestrator revalidates each finding against pack and live truth, then applies the classification without silently widening:

- `local_remediation` — repair inside the current parent loop, then verify and obtain fresh review;
- `child_packet_required` — create an independently reviewable child packet and execute it internally; the parent slice remains open;
- `cross_document_repair` — pause behavior-changing implementation, repair coupled authority docs, obtain fresh review, then resume when coherent;
- `proof_gap` — dispatch bounded internal proof/review work and reconcile the result;
- `resolution_escalation` — write a top-level handoff only when the broader decision/authority cannot be resolved inside the current authorization;
- `external_blocker` — write a top-level handoff when the named external/human recheck condition prevents further work;
- `future_program` — record the disposition and continue the current authorized work without adding it to this program.

Local remediation, child decomposition, proof gaps, and cross-document repair do not by themselves justify returning an internal dispatch to the user as a new task.

Only the top-level orchestration/design authority may promote a discovered child packet into the active slice plan. Implementation output alone does not change program scope, and creating the child does not mark the parent complete.

## Slice packet layout

When authorized, each implementation slice uses:

```text
slices/<slice-id>/
├── SPEC.md
└── tasks/
    ├── plan.md
    └── todo.md
```

Optional prompt or evidence artifacts may be added only when the slice requires them. Do not copy the whole control pack into each packet.
