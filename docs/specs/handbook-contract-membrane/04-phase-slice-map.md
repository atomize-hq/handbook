# Phase and Slice Map

## Status

This is the approved implementation program. Phase 0 design/default authority
is closed. Implementation phases execute in numeric phase order and, within a
phase, in the listed slice order unless this map is changed by a separately
reviewed planning slice. A Rust implementation slice is authorized only when
its own `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` packet is present and
review-clean.

HCM-1.1 through HCM-1.4 have landed and closed through separately reviewed
implementation and parent-handoff subjects. They are completed bounded
dependency evidence, not continuing implementation authority. HCM-2.1 has a
planning-only packet whose authority remains contingent on an immutable clean
planning review and parent closeout; even then, implementation begins only in a
separately selected top-level session. HCM-2.2 and every later slice remain
unauthorized until their own packets are created, reviewed, and explicitly
selected.

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

Phase exit gates are entry gates for the next phase. Within one phase, each
listed slice consumes the reviewed closeout of the preceding listed slice.
Creating a later packet may prepare future work but never waives that dependency
or starts the later slice. Parallel cross-slice execution is not authorized by
this map.

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
| `HCM-0.9` | Abandoned corrective decomposition attempt | immutable rejected planning/review evidence only; no leaf files and no index cutover | terminal Redesign Review 2 was not CLEAN after the only authorized remediation; decomposition is abandoned, `05-contracts-schemas-and-gates.md` remains canonical, and execution requires a new explicit human decision and newly reviewed packet |
| `HCM-0.5` | Freeze one fail-closed contract membrane and dock protocol without runtime work | canonical `00`-`06` subject defining exact contract identity/SemVer compatibility, immutable lifecycle, claims/applicability, evidence cardinality/provenance/freshness/Resolution/consistency, verdict/gate precedence, dock implementation-bundle/typed-launch-vector/runtime closure, process JSON/isolation/total-outcome semantics, bounded JSON Schema proof target, and HCM-0.4-compatible operations | complete proof wall and fresh independent review bind the exact subject; lifecycle stays separate from evaluation, validators stay witnesses, `handbook.dock.json-schema@1.0.0` is selected only as a future target, the `05` monolith remains canonical, and `PG-CONTRACT-01`/`PG-DOCK-01`/`PG-GATE-01` remain open |
| `HCM-0.6` | Research and approve the shipped default artifact set | reviewed research dossier and candidate comparison; explicit user decision record; exact six-kind catalog; exact three-instance root-profile selection; requiredness/condition, role/capability, lifecycle, and support posture | approved decision and `PG-DEFAULT-01` proof bind the complete target set; runtime remains unimplemented; no enum, template, filename, or example became authority by inertia |
| `HCM-0.8` | Correct the development orchestration and true-stop handoff control plane discovered after the original HCM-0.1 review | long-lived `07` runner; parent-owned `08` protocol; internal dispatch contract; handoff v1.2 schema/template/validator; split handoff/orchestration proof | one active parent executes a fresh built-in review internally, reconciles results, and writes one v1.2 closeout; internal agents write no global handoffs; prior records/dispatches remain immutable evidence |
| `HCM-0.7` | Approve the implementation program and first slice packet | reviewed phase map plus first `slices/<id>/SPEC.md`, plan, and todo | Phase 0 contracts/default decisions are closed and the first slice is independently implementable with a complete proof wall |

`HCM-0.8` is a corrective insertion and must close before `HCM-0.7`, despite its later-discovered numeric identifier.

`HCM-0.4` consumes the frozen HCM-0.2 semantic identities and HCM-0.3 Resolution/Snapshot/Projection contracts plus the closed HCM-0.8 orchestration control plane. It does not wait for HCM-0.5 contract/dock semantics or HCM-0.6 shipped defaults: those later design slices append their approved use cases/data selections without changing the transport and ownership rules frozen here.

### `HCM-0.7` implementation-program approval contract

**Dependencies:** completed HCM-0.2, HCM-0.3, HCM-0.4, HCM-0.5, HCM-0.6,
and HCM-0.8 records are dependency evidence. HCM-0.9 is abandoned evidence
only and supplies no catalog topology or resume authority.

**Authorized output:** documentation/planning changes that make this map the
approved sequential implementation program and create the exact HCM-1.1
`SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` packet. HCM-0.7 may add its own
proof/review/closeout evidence. It changes no Rust, Cargo, runtime, schema or
definition asset, current seam classification, or open runtime proof gate.

**First implementation boundary:** HCM-1.1 is an additive `handbook-engine`
kind/schema-registry foundation. It may expose and prove the correct owner
boundary, safe local Draft 2020-12 schema closure, uniform exact-definition
fingerprints, and one capability-free repository-defined custom kind. It does
not replace `CanonicalArtifactKind`, current layout/setup/doctor/flow paths, or
publish the HCM-0.6 profile/instances/first-party kind catalog. Those remain in
HCM-1.2 through HCM-1.4.

**Exit gate:** the phase/list sequencing and per-slice packet rule are exact;
the HCM-1.1 packet freezes owner, public boundary, source/asset topology,
dependency posture, TDD increments, fail-closed cases, full proof wall,
permitted classification/gate change, non-goals, and stop conditions; targeted
documentation checks and a fresh independent review are clean; the reviewed
subject and mechanical parent closeout use separate commits.

**Non-goals:** HCM-1.1 execution; Rust or Cargo changes; first-party content
schema/kind/profile/instance publication; enum/layout/setup/doctor/flow
replacement; canonical YAML cutover; intake, renderer, Projection, SDK, CLI,
Tauri, Substrate, contract, dock, or later-slice work; HCM-0.9 revival; or any
claim that `PG-KIND-01`, `PG-KIND-02`, or a runtime seam has landed.

`HCM-0.9` was an attempted corrective control-maintenance insertion after HCM-0.4. It is now abandoned and does not execute before HCM-0.5. This closeout does not start HCM-0.5; any future HCM-0.5 packet must use the canonical monolithic `05` authority unless a new human decision later authorizes and approves a replacement topology.

### `HCM-0.9` corrective maintenance contract

**Dependencies:** HCM-0.2, HCM-0.3, HCM-0.4, and HCM-0.8 are completed dependency evidence; the exact structural and semantic baseline is `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` at commit `214a5b8eb182fce74478df49d4f55d226d65fdf5` with SHA-256 `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d`.

**Terminal output:** immutable rejected planning subjects, two redesign review dispatches, the non-authoritative evidence checkpoint `f3a33ddb55443d37f3a51ffb58f1c85b74a28b23`, and the terminal abandonment handoff. No index, leaf, verifier, runtime, or HCM-0.5 output exists.

**Terminal disposition:** the decomposition exit gate did not pass. Review 2 retained one Required scope-proof finding, no Review 3 is allowed, and the monolith remains canonical. Any revival is a new human-authorized planning effort, not a resume of this packet.

**Non-goals:** semantic correction or clarification; Rust, Cargo, runtime, CLI, Tauri, Substrate, SDK, public API, schema-version, proof-promotion, or HCM-0.5 work; rewriting historical handoffs/dispatches; changing record schemas solely to carry leaf refs; selecting shipped defaults; or opportunistic control-pack cleanup.

**Review-budget stop:** exhausted. Redesign Review 1, one remediation, and terminal Redesign Review 2 completed. Do not run Review 3, remediate the terminal finding, or authorize execution.

### `HCM-0.5` design-freeze contract

**Dependencies:** HCM-0.2 semantic identities, HCM-0.3 Resolution/Snapshot/Projection contracts, HCM-0.4 owner/SDK/DTO/transport contracts, and HCM-0.8 orchestration mechanics are completed dependency evidence. HCM-0.9 is abandoned evidence and supplies no topology, catalog-leaf, or resume authority.

**Authorized output:** documentation/design changes only in canonical `00-README.md` through `06-proof-and-regression-ledger.md`. The output keeps `05-contracts-schemas-and-gates.md` monolithic; defines one exact protocol-neutral contract/evidence/verdict/gate and dock DTO model; appends only the HCM-0.4-compatible ordinary contract/dock operation definitions; and selects `handbook.dock.json-schema@1.0.0` as the bounded future HCM-5.4 target.

**Exit gate:** exact contract/lifecycle/claim/evidence/verdict/gate and manifest/request/result/isolation/failure matrices are mechanically checked; HCM-0.2/HCM-0.3/HCM-0.4 frozen semantics regress cleanly; final intended `00` status bytes are in the review subject; a fresh independent reviewer returns `CLEAN`; and proof/staging replay the clean subject byte-identically before commit.

**Runtime boundary:** `handbook-contracts`, ordinary SDK operations, process adapters, manifests, implementation bundles, schema files, validators, runners, CLI/Tauri/Substrate surfaces, and the selected first dock remain unimplemented. Documentation cannot promote `PG-CONTRACT-01`, `PG-DOCK-01`, or `PG-GATE-01`.

**Non-goals:** Rust, Cargo, runtime, schema publication, HCM-0.6 research/default selection, HCM-0.7 approval, HCM-0.9 repair, catalog leaves/index/routing, a universal validator, waiver semantics, remote registry, or marketplace.

### `HCM-0.6` shipped-default decision contract

**Dependencies:** frozen HCM-0.2 kind/instance/registry/intake/constitutional-root semantics, HCM-0.3 renderer/Projection separation, HCM-0.4 owner/transport boundaries, HCM-0.5 contract/evidence/gate separation, and HCM-0.8 orchestration controls are completed dependency evidence. Current enums, templates, paths, examples, and historical artifacts are precedent only.

**Approved output:** exactly six first-party kind refs at `1.0.0`; additive `handbook.roles.core@1.1.0` with exact fingerprint and distinct artifact role `environment_context`; root instances `project_authority`, `project_context`, and `environment_context` at the three `.handbook/project/*.yaml` paths; always/always/conditional requiredness; exact managed-operational-surface condition identity and six-outcome evidence policy; one unique `constitutional_root`; approved role-support, lifecycle/review, first-party intake/renderer posture; and no capitalized Projection selected by the shipped root profile. The complete authority is [`slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md`](slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md).

**Exit gate:** research and candidates remain review-clean provenance; the explicit user decision is transcribed without inventing subordinate schemas or implementation; affected `00`-`06` rows agree; `PG-DEFAULT-01` closes only for the documentation decision; a different-fresh independent reviewer reports `CLEAN`; final proof and staged change detection replay the exact subject; and the two-commit completed handoff closes the slice.

**Runtime boundary:** registry/kind/schema/profile/condition/intake/renderer publication, setup/doctor behavior, materialization, canonical YAML content, migration, adapters, and Projections remain unimplemented. `PG-PROFILE-01`, `PG-KIND-01`, `PG-ARTIFACT-01`, `PG-INTAKE-*`, `PG-CHARTER-01`, `PG-YAML-*`, and runtime/transport gates remain open.

**Non-goals:** Rust, Cargo, runtime, schema/definition publication, setup or empty-artifact scaffolding, content-field decisions, adapter/source-of-truth mappings, HCM-0.7 work, or widening any deferred kind into the six-kind catalog.

### Phase 0 non-goals

- Rust changes;
- public API publication;
- legacy migration tooling;
- new CLI commands;
- Tauri scaffolding;
- actual dock execution;
- speculative third-party workflow adapters.
- implementation-selected defaults that differ from or infer beyond the approved HCM-0.6 decision.

## Phase 1 — Profile and artifact semantic kernel

**Purpose:** replace the fixed artifact universe with a versioned profile-selected semantic model.

### `HCM-1.1` — Artifact-kind and schema registry

- implement versioned `ArtifactKindDefinition` and kind-definition meta-validation;
- resolve local canonical schemas with stable IDs, versions, and fingerprints;
- separate structural schema, semantic validation, intake coverage, and external docks;
- refuse remote/ambient/unversioned schema execution;
- prove one repository-defined custom kind without a Rust enum variant.

The approved execution packet is [`slices/HCM-1.1/`](slices/HCM-1.1/SPEC.md).
This slice is additive: `CanonicalArtifactKind`, fixed layout, setup, doctor,
baseline validation, and flow remain on their current path until HCM-1.3 and
HCM-1.4. HCM-1.1 may promote only the Artifact kind/schema registry seam to
`BoundaryLanded` and record the kind/schema structural subset of `PG-KIND-01`
plus the registration and structural-validation subset of `PG-KIND-02` for the
exact owner-library and repository-fixture proof its packet requires. Both
gates remain open: later slices must prove non-vacuous lifecycle/Projection and
supplied-intake coverage. HCM-1.1 cannot claim product-path adoption,
shipped-profile publication, or a released downstream API.

### `HCM-1.2` — Profile schema, artifact instances, and shipped default

- before enabling non-empty semantic-capability or other later-owned kind
  dependencies, define their exact typed source/fingerprint producers and a
  machine-readable binding-shape compatibility contract with non-vacuous proof;
- define typed profile identity/version;
- define `ArtifactInstanceDescriptor` independently from kind definitions;
- encode only the shipped default set approved by `HCM-0.6`;
- validate explicit profile selection and repository profile input;
- do not add a legacy profile merely to preserve old behavior.

HCM-1.2 landed at reviewed implementation commit
`832716a66241bdcf86e2a82ffb3ae72680a7c2cd`; its selected v1.2 closeout is
`20260717T125103Z--HCM-1-2--orchestration--profile-boundary-landed`. Those
records are completed dependency evidence for HCM-1.3 and do not authorize
additional HCM-1.2 work.

### `HCM-1.3` — Descriptor-driven artifact-instance registry

- replace enum-owned universe with profile-resolved kind and instance registries;
- support first-party stable capabilities/roles plus custom kind and instance IDs;
- make requiredness, dependencies, paths, and validators data-driven;
- preserve trusted repo-relative path enforcement.

The planning packet is [`slices/HCM-1.3/`](slices/HCM-1.3/SPEC.md). The
implementation adds only the additive `handbook-engine` owner API
`ResolvedArtifactRegistry`, its focused integration tests, and the bounded
control-pack classification/proof updates. It proves shipped and custom
descriptor-driven membership, role/capability/schema/validator metadata,
dependency providers/order, structural validation routing, deterministic source
permutations, unchanged package-owned definitions, and no setup/doctor/flow or
fixed-product adoption. HCM-1.4 remains unauthorized until a separate selected
planning/implementation handoff.

HCM-1.3 landed at reviewed implementation commit
`8194f9f4534b2d27e1077ffab2c89d12da5ff456`; its selected v1.2 closeout is
`20260717T183202Z--HCM-1-3--orchestration--artifact-registry-landed`. Those
records are completed dependency evidence for HCM-1.4 and authorize no
additional HCM-1.3 work.

### `HCM-1.4` — Profile-aware setup and doctor decisions

- make setup/doctor use typed profile decisions;
- keep CLI wording outside engine decisions;
- expose machine-readable profile/capability truth.

The reviewed implementation packet is
[`slices/HCM-1.4/`](slices/HCM-1.4/SPEC.md). The landed boundary is one
engine-owned typed profile-decision and repository-inspection closure consumed
identically by setup and doctor. It replaces their fixed artifact-selection
path. Conditional descriptors bind exact definitions but remain
explicitly `unresolved`/`evidence_contract_unavailable` until a separate verified
evidence/evaluator contract exists. The slice writes no canonical YAML,
invents no compatibility profile, repairs no unrelated reset transaction,
generates no command, and starts no Phase 2 content-authority work.
Its machine-readable capability truth is exactly the instance/capability/
contract-ref/contract-fingerprint identity projection; binding, cardinality,
and semantic-validator metadata remain owned by the selected registry.
The only production authoring exception is the exact behavior-preserving
`author/mod.rs` cfg-selected local acquisition-operation portability hunk
required to make the mandatory compiler Windows target build; existing helper
signatures/bodies and the Unix `LOCK_UN` drop path remain byte-unchanged, lock
semantics do not change, and fresh HIGH-risk impact plus complete author
regressions passed. Actual Windows MSVC runtime refusal tests, Unix workspace
tests/clippy, the literal 29-member engine package replay, and compiler
source-tree checks passed. The classification ceiling is `BoundaryLanded` for
setup/doctor decision/readiness adoption only; content authority,
materialization, semantic validation, condition evaluation, SDK transport,
renderers/Projections, and HCM-2 remain open.

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

Select one lower-risk artifact family after Phase 0 review. Project Context is the current leading implementation candidate because a structured input model and deterministic renderer already exist and HCM-0.6 includes its kind and one root instance; pilot selection does not amend the approved catalog, descriptor, or requiredness decisions.

- canonical YAML load/validate/write;
- renderer-derived Markdown human-review view produced by the existing fixed deterministic first-party renderer;
- source and rendered-output fingerprints without a Resolution or Projection provenance claim;
- setup, authoring, doctor, and flow integration for the pilot;
- direct cutover of tests and fixtures.

The candidate planning-only implementation packet is
[`slices/HCM-2.1/`](slices/HCM-2.1/SPEC.md). It fixes the named Project Context
candidate as the exact pilot and consumes the already admitted
`handbook.schemas.artifacts.project-context@1.0.0` schema plus the shipped
`project_context` descriptor at `.handbook/project/context.yaml` without
changing any definition byte. Authoring accepts that canonical record directly;
setup remains non-authoring; doctor reports exact source/rendered fingerprints;
and flow uses a named temporary mixed-family bridge that reads Project Context
only from the selected descriptor while fixed sibling content authority remains
unchanged. The packet also cuts Environment Inventory's reference-only
dependency to that selected YAML path so the installed all-three flow has no
dangling legacy Project Context Markdown reference.

The fixed first-party Markdown renderer is an in-memory, clock-free human-
review transformation. It writes no Markdown file and claims no Context
Resolution or capitalized Projection provenance. The old rich Project Context
input/timestamped Markdown model receives no mapper, importer, alias, or dual-
read compatibility path. The packet becomes future implementation authority
only after an immutable fresh review returns `CLEAN` over its exact bytes, the
parent planning closeout records that result, and a separate explicit session
selects it; this planning subject changes no Rust, runtime, template, test, or
fixture byte and starts no HCM-2.2 work.

The packet freezes one retained inspection observation for doctor, a closed
YAML emitter and exhaustive Markdown transform with literal boundary goldens,
exact owned-path/rendered-fingerprint/budget/log flow DTO semantics, and a typed
non-Unix fail-before-mutation boundary with required native Windows proof. The
owned selected path is closed through flow and compiler refusal/blocker carriers,
and the changed C04 public result envelope advances exactly to
`reduced-v1-m8.2` while C03 stays `reduced-v1-m8` generation `1`.
Engine identity/ingest/baseline/manifest paths become owned with borrowing-only
C03 encoder adaptation and byte-identical preimage proof. CLI setup adds only
the three new inspection-reason names required by the public enum closure.
The Project Context author result also owns the descriptor-selected path; the
legacy fixed Markdown path constant and re-exports are removed without an alias.

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

- exact `contract_id@full-SemVer` refs/fingerprints and closed compatibility rules;
- immutable `draft -> review_ready -> locked -> active/deprecated -> closed` authority transitions, including the complete closed adjacency table and distinct lock authority;
- typed claims, selectors/applicability, gate effects, all-of evidence requirements, and canonical structured contract records;
- no evaluation, verdict, score, or gate outcome encoded as lifecycle state.

### `HCM-5.2` — Evidence, verdict, and gate engine

- validate one untrusted candidate into one immutable canonical evidence record with exact provenance/freshness/source/subject/case/run identity;
- enforce per-kind/case cardinality, repeated-observation consistency, complete claim partitions, and dimension-by-dimension effective Resolution;
- compute the closed `pass`/`fail`/`blocked`/`warning`/`not_observed`/`not_applicable`/`flaky` vocabulary;
- compose hard/required/advisory gates so hard failure, required missing evidence, stale bindings, or incomplete accounting outrank weighted score;
- compute local closeout and parent promotion separately.

### `HCM-5.3` — Process dock protocol

- exact manifest identity plus content-addressed implementation bundle, normalized bundle manifest, entrypoint digest, typed native-or-bundled-interpreter launch vector/fingerprint, and normalized bundle-only runtime/dependency-closure descriptor/fingerprint;
- one bounded UTF-8 JSON request and one bounded JSON result with exact fingerprints and no stdout side channel;
- default-deny filesystem/environment/process/resource grants, unconditional v1 network denial, safe artifact refs, and bounded output admission;
- host-monotonic timeout, idempotent cancellation, typed refusal, ordered mutually exclusive crash/protocol/cleanup outcome semantics, and no partial evidence;
- process executor emits only operational records and untrusted candidates.

### `HCM-5.4` — First real validator dock proof

- implement the already-selected `handbook.dock.json-schema@1.0.0` bounded adapter over the existing local Draft 2020-12 ecosystem;
- validate one JSON-compatible instance against one exact offline schema/ref closure and refuse remote refs, executable hooks, unsupported dialects, and fingerprint mismatch;
- execute through the process protocol, validate one candidate into canonical evidence, and prove positive/negative/refusal/timeout/fingerprint/Resolution/gate paths;
- prove a real contract gate without giving the validator, runner, host allowlist, SDK, or transport canonical authority.

### Phase 5 exit gate

- an exact active contract with a valid prior independent-lock transition drives a real selected-dock -> execution record -> admitted evidence -> verdict -> gate path; a bare `locked` definition cannot start evaluation;
- missing, stale, refused, failed, malformed, inconsistent, wrong-subject/case, or insufficient-Resolution evidence cannot produce false green or partial proof;
- manifest/implementation/runtime-closure substitution, shell/PATH/ambient-runtime discovery, network use, unsafe output, timeout/cancellation/crash, and cleanup uncertainty fail closed;
- process protocol semantics are stable enough for a later Rust-native binding with no semantic privilege or evidence-shape divergence;
- the frozen ordinary `contract.*`/`dock.*` use cases exist behind SDK types before CLI polish claims completion;
- `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` close only for the exact runtime evidence exercised.

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
