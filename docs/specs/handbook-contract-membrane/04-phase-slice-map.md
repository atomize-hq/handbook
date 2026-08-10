# Phase and Slice Map

## Status

This is the approved implementation program. Phase 0 design/default authority
is closed. Implementation phases execute in numeric phase order and, within a
phase, in the listed slice order unless this map is changed by a separately
reviewed planning slice. A Rust implementation slice is authorized only when
its own `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` packet is present and
review-clean.

HCM-1.1 through HCM-1.4 and HCM-2.1 have landed and closed through separately
reviewed implementation and parent-handoff subjects. They are completed bounded
evidence, not continuing implementation authority. `HCM-2.2-ESC-001` repaired
the candidate/result semantic identity cycle and `HCM-2.2-ESC-002` repaired
partial pending output publication. The later implementation reached fresh
Review 3 and was rejected under `HCM-2.2-AR3-001` because result, witness, and
mutable binding audit bytes remained coherently rewritable under candidate
`1.2`. User-selected `HCM-2.2-ESC-003` first repaired the documentation
authority and the later exact implementation selector authorized its bounded
execution:
candidate `1.3` independently anchors semantic result ref/fingerprint and exact
persisted result JCS+LF SHA-256/length. Result `1.0` and intent `1.2` schemas
remain unchanged. The complete candidate-`1.3` implementation and proof wall
received three Required findings in fresh implementation Review 1. Frozen
result-schema enforcement, exact approval-quorum replay, and exact resolved-
definition closure were repaired and proof-complete. At that review boundary,
adoption still required a different-fresh re-review. Fresh implementation
Review 2 additionally required
version-exact historical candidate validation; distinct closed `1.0`/`1.1`/
`1.2` admission, internal-ID equality, and public replay negatives are now
proof-complete. Fresh implementation Review 3 additionally required exact
candidate provenance reconstruction. The retained-intake/normalized-content
populated-leaf bijection now gates author inventory, approval, promotion,
persistence, and recovery, with eight coherent-resigning negatives at every
runtime seam. Fresh Reviews 4 and 5 returned `CLEAN`; Review 5 binds the exact
53-path subject at
`sha256:62c9fdae649a31d1538ec7770b0f8ce5b2cc0686cb33535fe8747bbe2ddc80ad`.
HCM-2.2 landed at primary commit `6766d3ed4894aad6598faaa7c4a54b493f92c1f6`
and parent closeout `5c31eeefb5adf71d75ec3059b1b6947025d2fd6b`.
HCM-2.3 landed its exact repository-defined `registry-brief` path at
implementation commit `628b672ef33326e87e4fb30be13489e8af04b38c`; final
complete-subject review commit `746fff667f6fbe0270182a467285d60394362530`
binds the exact 111-path subject
`sha256:d510e94e5b47db209022927bc4afa74b8820e27cd645123cc1cdc3aa5f72fdef`.
The exact registry-brief kind/schema and intake subsets are
`RealPathAdopted`, while broader generic/custom-kind intake remains
`TargetOnly`. `handbook-engine` `0.2.0` is the accepted public-enum
compatibility boundary. At the HCM-2.3 closeout, HCM-2.4 and every later slice
remained unauthorized until its own review-clean packet and separate exact
selector.

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
| `HCM-0.8` | Correct the development orchestration and true-stop handoff control plane discovered after the original HCM-0.1 review | long-lived `07` runner; parent-owned `08` protocol; internal dispatch contract; current handoff v1.4 schema/template/validator; bounded authority-continuation grant; split handoff/orchestration proof | one active parent executes fresh built-in review internally and writes one v1.4 closeout; a direct non-completed authority stop may resume only through one independently CLEAN-admitted, same-parent, same-budget grant with finite slots and consumed terminality; internal agents write no global handoffs; prior records/dispatches remain immutable evidence |
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

**Approved output:** exactly six first-party kinds; distinct artifact role `environment_context`; root instances `project_authority`, `project_context`, and `environment_context` at the three `.handbook/project/*.yaml` paths; always/always/optional requiredness with no Environment Context condition; one unique `constitutional_root`; approved role-support, lifecycle/review, first-party intake/renderer posture; and no capitalized Projection selected by the shipped root profile. Environment Context presence permits advisory use and never selects enforcement. The complete authority is [`slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md`](slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md).

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

HCM-2.1 selects and lands Project Context as the one lower-risk vertical pilot. The cutover consumes the HCM-0.6 kind and root instance directly and does not amend the approved catalog, descriptor, or requiredness decisions.

- canonical YAML load/validate/write;
- renderer-derived Markdown human-review view produced by the existing fixed deterministic first-party renderer;
- source and rendered-output fingerprints without a Resolution or Projection provenance claim;
- setup, authoring, doctor, and flow integration for the pilot;
- direct cutover of tests and fixtures.

The completed implementation packet is
[`slices/HCM-2.1/`](slices/HCM-2.1/SPEC.md). It fixes the named Project Context
family as the exact pilot and consumes the already admitted
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
read compatibility path. The immutable planning review, parent planning
closeout, separate explicit implementation selection, bounded implementation
review/remediation loop, native Windows proof, and final clean review gates are
satisfied. The slice starts no HCM-2.2 work.

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

**Implementation status:** completed and review-clean. The earlier candidate-`1.2` subject stopped after
fresh Review 3 and remains a non-authoritative checkpoint. Option 1 added candidate `1.2`
validation-only subject identity and result `1.0`; atomic staging repaired
`W5`-`W7`. Review 3 then proved the result/witness/mutable-binding audit bytes
coherently rewritable without changing candidate `1.2` identity.
`HCM-2.2-ESC-003` selected candidate `1.3` with a
result-independent subject fingerprint and a final-candidate exact result ref,
semantic fingerprint, JCS+LF SHA-256, and LF-inclusive length binding. New
approval binds final candidate, and promotion/intent `1.2` bind it transitively;
result `1.0` and intent `1.2` schemas stay unchanged. The later exact-result
implementation selector authorized the complete repaired identity/replay/
orphan/migration/recovery, atomicity, lifecycle, and product-cutover subject.
Fresh Reviews 4 and 5 returned `CLEAN`; primary commit
`6766d3ed4894aad6598faaa7c4a54b493f92c1f6` and closeout
`5c31eeefb5adf71d75ec3059b1b6947025d2fd6b` adopt it. The checkpoint and proof history at
[`slices/HCM-2.2/`](slices/HCM-2.2/SPEC.md) are evidence only and grant no
authority for HCM-2.3 or later work.

- cut the constitutional-root artifact to canonical structured truth;
- preserve semantic root authority without requiring a literal filename;
- implement `CharterIntakeDefinition` as the first rich intake coverage contract;
- support guided-adaptive, express, and agent-assisted acquisition through the skill-directed agent, all targeting the same Charter candidate schema;
- preserve immutable intake provenance, explicit known unknowns, validation, approval, and promotion without restoring a nested CLI wizard;
- establish one setup-initialized, engine-owned durable repository identity and engine-allocated bounded operation IDs without widening the frozen CLI grammar or granting adapter authority;
- render Markdown and any other renderer-derived human-review view only through fixed deterministic, non-Resolution, first-party renderers reading approved canonical Charter YAML;
- prove reproducible renderer-derived human-review output and lifecycle behavior.

### `HCM-2.3` — Generic custom-kind registration, intake, and validation proof

**Implementation status:** completed for the exact repository-defined
`registry-brief` path. Implementation commit
`628b672ef33326e87e4fb30be13489e8af04b38c`, final complete-subject review
artifact commit `746fff667f6fbe0270182a467285d60394362530`, and the final
proof wall under [`slices/HCM-2.3/`](slices/HCM-2.3/SPEC.md) bind the exact
111-path subject
`sha256:d510e94e5b47db209022927bc4afa74b8820e27cd645123cc1cdc3aa5f72fdef`.
The exact registry-brief kind/schema and intake subsets promote to
`RealPathAdopted`; `PG-KIND-02` closes only for that path, and exact subset
evidence enters `PG-KIND-01` and `PG-ARTIFACT-01`. First-party Charter remains
`ContractCorrectAndProven`; broader generic/custom-kind intake remains
`TargetOnly`. Commit `7b3b5454ef363d08e4c6c78e6a201d8c36c10c5c`
establishes the accepted `handbook-engine` `0.2.0` compatibility boundary for
the landed public-enum additions without claiming publication. This completion
does not authorize HCM-2.4 or a later phase.

- register one repository-defined kind/schema without kind-specific Handbook
  code changes;
- use engine-owned typed operations and stable generic CLI commands selected by
  kind/instance ID; SDK composition remains Phase 4;
- validate canonical YAML through the registered custom schema;
- exercise optional intake coverage when supplied;
- prove no dynamic command or filename dispatch is involved.

### `HCM-2.4` — Remaining shipped artifact families

- complete optional canonical Environment Context YAML as advisory project context;
- keep enforcement separate as an explicit future task-scoped positive-proof gate;
- preserve feature/spec policy and cross-target completion separately from task execution authorization;
- remove the rejected condition-evidence design and superseded Markdown authority;
- keep deterministic pre-Phase-3 outputs as renderer-derived human-review views only.

Completed implementation lineage begins at commits
`5cf41d2f64d68cb7f78abb5eccab9acf307089d3`,
`5a2ccf6939be0e549e7e7355a245c7511c271eef`, and
`00dde0162fcb15576c83b6ed40ab7286488d0890`, plus bounded P2 implementation
commit `9b3edf2745054872939ad48abad50ae5c2d97f9a` and authority commit
`f62141b358239e15f106f04034131202e9db1cac`, and P4 negative-proof commit
`158115daa2b85d41663b79dc677fce69e59fd1e2`:

- P0, P1A, P1B, and P1C are review-clean. Shipped-root `1.2` is selected with
  exactly three root instances, released Charter authority remains on its
  HCM-2.2 `1.1` identity pair, and the five exact successor kind/intake/
  renderer definition closures are package-admitted without changing released
  schemas or public APIs.
- P3 and its P3B CLI/evidence integration are review-clean. Stage 10 now binds
  capture, provenance, feature identity, and handoff authority to canonical
  Work Specification YAML while Feature Spec Markdown is a deterministic
  human-review view.
- The bounded P2 correction is GREEN, independently review-clean, and locally
  committed. The P2S/P2A evaluator/native material is historical only and has
  no active runtime prerequisite. P2 owns optional canonical YAML, advisory
  use, generic safe mutation, deterministic rendering, and non-blocking
  setup/doctor status. Task-gate runtime is deferred to the later
  implementation-contract slice under its explicit GREEN gate.
- The shared P4/P5 coverage-token prerequisite is review-clean and committed at
  `00dde0162fcb15576c83b6ed40ab7286488d0890`; P5 is GREEN and accepted. P4's
  separately authorized no-root/generated-command/inferred-filename/Projection/
  persistent-view proof is review-clean and committed at
  `158115daa2b85d41663b79dc677fce69e59fd1e2`. P4 is GREEN and completed.
- P2 owns removal of legacy Environment Inventory authority and influence.
  The accepted P6 repair removes both bridges and residual normal-path
  fixed-kind authority and installs descriptor-owned aggregate identity at
  primary commit `1209430240eb67c7aa2baac5859d84cb9a84a3b2` and closeout
  `4aecd1b42356592ccd4d093c85486a9c25410dca`. CLEAN in
  `handoffs/dispatches/20260731T004500Z--HCM-2-4--p6-final-review-control-truth-supplemental-2-closure.json`
  accepted P6. The operator explicitly selected P7; its complete proof wall and
  every exit row below are GREEN. Discovery dispatch
  `handoffs/dispatches/20260731T022800Z--HCM-2-4--p7-phase-2-exit-final-review.json`
  returned two P2 control-truth findings and conferred no acceptance. The first
  remediated closure
  `handoffs/dispatches/20260731T024500Z--HCM-2-4--p7-phase-2-exit-remediation-closure.json`
  closed `HCM-P7-FR-0001` but returned FINDINGS because GitNexus's identical
  staged replays reported nondeterministic documentation-symbol counts. The
  supplemental review
  `handoffs/dispatches/20260731T025730Z--HCM-2-4--p7-phase-2-exit-gitnexus-supplemental-closure.json`
  returned FINDINGS: the GitNexus proof was substantively corrected, but
  `HCM-P7-FR-0003` proved that reused finding ownership made the old parent
  unrepresentable as a completed v1.4 handoff. Under
  `slices/HCM-2.4/decision/20260731-p7-final-review-lineage-repair.md`, the old
  parent is historical context only. CLEAN in new-parent dispatch
  `handoffs/dispatches/20260731T034000Z--HCM-2-4--p7-final-review-restart-discovery.json`
  completes P7, HCM-2.4, and the exact Phase 2 exit; FINDINGS leaves all three
  unaccepted. Neither HCM-3.x work nor automatic continuation is authorized.

The formal P7/HCM-2.4/Phase 2 exit above remains immutable completed evidence.
A separately selected post-exit smoke correction is bounded by
`slices/HCM-2.4/decision/20260731-post-phase-2-smoke-repair-selector.md`.
It repairs the setup-created operational-root packet retry, preserves
non-authoring setup and absent/unsafe/legacy-only fail-closed behavior, and
changes no Phase 2 exit row. Proof-stage review
`handoffs/dispatches/20260731T232558Z--HCM-2-4--post-phase-2-smoke-repair-proof-gap-review.json`
is CLEAN. CLEAN in
`handoffs/dispatches/20260731T235300Z--HCM-2-4--post-phase-2-smoke-repair-final-complete-subject-review.json`
completes only the corrective outcome; FINDINGS or BLOCKED leaves it open and
HCM-3.x unauthorized.

### Phase 2 exit gate

- GREEN: each targeted artifact has exactly one editable canonical truth;
- GREEN: all intake modes converge on the same kind-selected canonical schema
  and expose missing coverage;
- GREEN: the Charter intake record/candidate/canonical boundaries are auditable
  and non-competing under direct 1.1 and selected compatible 1.2;
- GREEN: the registry-brief custom kind registers, validates, and exercises
  supplied intake without a Rust product variant or generated command;
- GREEN: Phase 2 human-review views are reproducibly derived only by fixed
  deterministic, non-Resolution, first-party renderers and remain outside the
  Projection contract;
- GREEN: generic configured custom-kind Projections and all Resolution-aware
  views remain deferred until separately selected `HCM-3.2` and `HCM-3.3` work;
- GREEN: no user migration tooling or dual-read promise exists;
- GREEN: every temporary internal cutover bridge named in `06` is deleted.
- future implementation-contract work proves task declarations, positive
  session-capability matching, and a separate feature/spec cross-target
  completion result; orchestration aggregation remains non-authoritative.

The exact proof mapping is
[`slices/HCM-2.4/proof/implementation/P7-phase-2-exit-closeout.md`](slices/HCM-2.4/proof/implementation/P7-phase-2-exit-closeout.md).
The future implementation-contract bullet is a separate deferred product gate,
not a competing canonical-artifact authority or an unproved Phase 2 exit row.

## Phase 3 — Vocabulary, Context Resolution, Snapshot Memory, and Projections

**Purpose:** make views and agent context profile-aware and resolution-aware.

### `HCM-3.1` — Vocabulary resolution

- axis-based labels;
- lexical and structural conflation;
- deterministic renderer consumption;
- stable machine semantics beneath local terminology.

**Completed boundary:** typed label/alias/ambiguity resolution and deterministic
acyclic structural absorption are proof-CLEAN against the exact selected
stable-role registry. The shipped empty vocabulary preserves existing output;
one non-empty repository/test vocabulary changes presentation only through the
fixed Stage-10 Work Specification renderer, with canonical YAML and every
stable-role edge preserved. `PG-VOCAB-01` closes and only the vocabulary subset
of `PG-PROFILE-01` advances. HCM-3.2 is separately selected and completed
locally under the bounded boundary below; HCM-3.3+, other consumers/adapters,
and the Phase 3 exit remain open and unselected.

### `HCM-3.2` — Context Resolution kernel

- configurable ordered stack;
- six explicit dimensions;
- inheritance, mutation, memory, validation, and escalation semantics;
- migration of useful work-level behavior without freezing L0-L3.

**Completed local boundary:** the fresh-parent implementation and full proof
wall are completed at reviewed primary commit
`9fd54c2f746eddd4df5dff37ba3828626395000d` for the bounded engine kernel,
private JCS authority capsule, generic-lineage publication/currentness path,
replacement recovery, and identity-before-mutation quarantine refusal. The
completed v1.4 handoff
[`20260805T153602Z--HCM-3-2--orchestration--context-resolution-quarantine-finalization-completed.json`](handoffs/records/20260805T153602Z--HCM-3-2--orchestration--context-resolution-quarantine-finalization-completed.json)
records the fresh-parent complete-subject `CLEAN` closure, completing the slice
and closing `PG-RES-01` only for this exact kernel. The stopped predecessor
parent remains immutable source evidence. HCM-0.11, HCM-3.4+, consumer
migration, tooling/schema/template work, dependency, public-API, platform, and
transport expansion, release, push, and publication remain separately selected.

### `HCM-3.3` — Deterministic Projection engine

- begins only after the `HCM-3.2` Context Resolution kernel is available;
- generic configured custom-kind Projections and Resolution-aware first-party views;
- reveal and derive;
- collapse/expand request handling;
- omission and lossiness accounting;
- source/profile/projection fingerprints;
- no synthesis in the core implementation.

The selected HCM-3.3 private/internal increment is completed locally under the
bounded 2026-08-05 implementation parent and its final CLEAN review. It admits
one generic
configured custom-kind engine, one currentness-guarded opaque Resolution view,
deterministic `reveal` and allowlisted deterministic `derive`, complete
omission/proof/lossiness/provenance/fingerprint accounting, and the corrected
two-admitted-envelope replay wall. This earns only the exact private/internal
portions of `PG-PROJ-01` and `PG-PROJ-02`. The pre-existing raw protected-index
digest discrepancy is recorded as non-blocking evidence, not a P1/P2 finding;
public API/default catalog/consumer
adoption, Snapshot Memory, HCM-3.4+, release, push, and Phase-3 exit remain
separately authorized.

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

- P0 freezes `handbook_engine::grounding::ground_resolution` and its typed
  request, outcome, refusal, bounded-summary, narrowed-consumer-view, and
  non-promoting-evidence contract; P1 must implement only the listed
  engine-private/public boundary under a fresh selector and impact proof;
- freeze a greenfield typed `handbook-engine` -> `handbook-flow` ->
  `handbook-pipeline` owner/call-path cutover; existing Flow/pipeline callers
  are regression evidence, not a legacy compatibility constraint;
- have Flow consume a bounded Resolution Projection and typed omissions rather
  than byte budgets or comprehensive snapshots; byte budget remains a
  subordinate resource constraint;
- keep HCM-3.4 `snapshot_delta` relation-only and have engine produce a
  bounded/redacted delta-signal summary reconciled with
  `reveal_delta_signals`; Flow and pipeline never directly read full signals;
- have pipeline scoped inclusion consume a namespaced shared Resolution input;
  any temporary `work_level` mapping is one-way, exact, provenance-bearing,
  and fail-closed;
- have parent handoffs reference prior-end/start/grounding/end/delta records
  after capture/currentness validation, without copying snapshot content or
  granting authority;
- preserve a typed non-promoting evidence boundary: local completion and
  parent-promotion eligibility are separately fingerprinted, default false on
  missing, omitted, redacted, stale, or indeterminate evidence, and await the
  selected gate runtime owner;
- plan a standalone Handbook CLI over the future SDK/library and exact
  crates.io Substrate consumption/wrapping. Tier 2 binary/JSON remains
  transitional until the independent published real-seam replacement proof.

### `HCM-3.6` — Project posture resolution and recommendation loop

- resolve a fingerprinted `ProjectPostureKernel` from canonical Charter policy, approved overrides, applicable conditions, contracts, and evidence;
- keep engineering-posture dimensions distinct from Context Resolution dimensions;
- derive typed `PostureRecommendation` records from hard lifecycle triggers and sustained snapshot/evidence signals;
- configure threshold windows, cooldowns, recipients, and acknowledgment/escalation through an approved `PostureEvaluationPolicy`;
- require authorized `PostureTransition` records for canonical policy changes;
- apply hysteresis: immediate raise recommendations may follow hard triggers, while lowering requires sustained evidence and cannot cross floors/red lines;
- map each transition to exactly one fixed-array Charter `level_override` leaf,
  compare the complete expected canonical bytes, and leave the baseline plus all
  other dimensions byte-semantically unchanged;
- derive one current Charter head across candidate-promotion and posture-
  transition journals so reads, lifecycle events, recovery, and later promotion
  agree without fabricating a promotion record; and
- re-evaluate only `engineering_posture.dimensions`, atomically persist the
  resulting Charter, immutable PostureTransition, and private lifecycle rebase,
  and keep the entire owner/schema/test seam crate-private and transport-free.

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

Phase 3 remains open. The narrow HCM-4.1 dependency-order exception and
operator continuation admitted the bounded SDK implementation/cutover now
verified at checkpoint `28fa4ab4f2fced48532d6830a7a69d8bd12a0cd1`.

### `HCM-4.1` — SDK owner and use-case implementation

**Completed bounded implementation/cutover:** checkpoint
`28fa4ab4f2fced48532d6830a7a69d8bd12a0cd1` adds direct typed,
Rust-linkage-only posture ingress and existing-owner SDK composition; cuts over
Setup, Author, Approvers, Artifact, Pipeline, Generate, Inspect, and Doctor
while retaining their observable CLI behavior; and retires normal-path
`handbook-compiler` composition. It creates no canonical operation-definition,
capability descriptor, bootstrap closure, shared Serde/JSON DTO, generated
schema, CLI JSON parity, or Tauri boundary. This checkpoint is bounded
implementation/cutover evidence only: it does not close HCM-3.6,
`HCM3EXIT-P2-CLIPPY-001`, HCM-3.5 P5/P6, or the Phase 3 exit, and it does not
authorize other Phase-4 work.

### `HCM-4.2` — Shared DTO and JSON Schema contract (next separately selected work)

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
- `handbook-compiler` is retired by the verified HCM-4.1 cutover checkpoint;
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

This action classification is independent from the P1-P4 priority in
`09-review-finding-inventory.md`. Priority decides whether the current subject
may close; classification decides what the parent does with the finding.

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

## Reviewability and packet sizing calibration

Decompose before implementation or before the next material review when one
logical packet exceeds any reviewability indicator below:

- roughly 1,000 hand-written changed lines, excluding generated vectors and
  complete deletions that are mechanically checked;
- roughly 20 changed production symbols;
- more than one independent HIGH/CRITICAL subsystem, platform primitive, or
  state machine;
- one source file growing materially beyond roughly 1,500 lines;
- more than 2,000 focused authority/repo/proof context lines for one agent; or
- a reviewer cannot evaluate the subject without loading sibling/future scope.

These are decomposition triggers, not automatic defect classifications. A
packet may remain whole only when its plan explains why splitting would break
one atomic invariant and a fresh reviewer accepts that reviewability argument.

A multi-packet slice may land a stack of independently verified and reviewed
packet commits. Each commit must be internally valid, remain within the parent
slice, and preserve the open parent status. The final parent review covers the
aggregate final subject and proof wall; the final primary tip, rather than a
requirement for one giant commit, anchors completed closeout.

For a high-risk packet, the parent may dispatch a bounded same-fingerprint
review burst with disjoint lenses such as correctness/recovery,
contract/schema, platform/security, and API/scope. Findings are consolidated
before one remediation pass. The default budget then permits one
different-fresh delta-focused closure review plus at most two supplemental
causal remediation/closure cycles for P1/P2s directly caused or unmasked by the
preceding remediation. Unrelated blockers, material scope/risk expansion, or an
exhausted allowance stop non-completed. Review bursts do not replace aggregate
subject-identity and proof verification.

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
