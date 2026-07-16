# Seam Crosswalk

## Purpose

This document separates current artifacts from target semantics. Re-check the live tree before each slice and update affected rows when their classification changes.

## Classification rules

- Classify the seam as a whole, not the best helper inside it.
- Keep artifact existence, semantic correctness, owner-boundary correctness, real-path adoption, and runtime proof separate.
- A published crate proves distribution, not every future API.
- A passing unit test does not prove CLI, Tauri, Substrate, or dock integration.
- Archived planning is provenance, not current implementation truth.

## Current-to-target crosswalk

| Seam | Current live truth | Current classification | Target owner | Required action | Sibling dependencies |
|---|---|---|---|---|---|
| Canonical artifact identities | `CanonicalArtifactKind` is a fixed four-variant enum: Charter, ProjectContext, EnvironmentInventory, FeatureSpec | `UsefulPrecursor` | `handbook-engine` profile/artifact kernel | Replace enum-owned universe with distinct profile-resolved kind definitions and instance descriptors while retaining separate stable-role refs and semantic-capability contracts/selections | kind meta-schema, profile schema, generic validation, setup/doctor/flow |
| Artifact kind/schema registry | HCM-1.1 adds an exact-ref/fingerprint `handbook-engine` owner-library boundary for package-owned stable roles, explicit repository-local Draft 2020-12 schema closures, capability-free `ArtifactKindDefinition` records, deterministic structural validation, and one two-kind/two-schema custom-kind proof corpus; current product paths still bypass it | `BoundaryLanded` for the additive owner-library boundary only | `handbook-engine` | Later authorized slices must add non-vacuous semantic/lifecycle/renderer/Projection/intake definition producers, profile/instance selection, and product-path adoption without weakening the HCM-1.1 fail-closed source/resolver boundary | instance profiles, schema policy, renderers, later Projection engine, generic CLI/SDK operations |
| Shipped default artifact set | HCM-0.6 approves an exact six-kind catalog and the three `project_authority`/`project_context`/conditional `environment_context` root instances; current setup/runtime still implements the older fixed baseline rather than the approved profile data | `TargetOnly` for runtime; Phase 0 target decision is frozen | Phase 0 decision authority, then `handbook-engine` shipped profile data | Publish and implement only the exact approved registry/kinds/condition/instances in later authorized slices; preserve two always-required instances, evidence-gated Environment Context, and no root work/decision/risk instance | kind registry, constitutional-root contract, setup/doctor, skill UX |
| Canonical layout | `CanonicalLayoutContract` parameterizes paths but still has dedicated fields for fixed artifact kinds | `UsefulPrecursor` | `handbook-engine` | Move to descriptor-driven layout resolved from the instance profile | artifact registry, trusted repo-relative paths |
| Structured authoring | Charter, Project Context, and Environment Inventory accept structured YAML inputs and deterministically render Markdown | `UsefulPrecursor` | `handbook-engine` | Invert authority so meaningful YAML is canonical and Markdown is derived through kind-selected fixed renderers producing renderer-derived views; later Resolution-aware views use Projections | artifact-kind schemas, renderer definitions, later Projections, cutover gate |
| Charter intake coverage | `CHARTER_INPUTS.yaml.tmpl` preserves broad questionnaire-shaped fields and deterministic validation exists, while live authoring guidance gathers structured facts through a skill; the retained stage directives still frame interview/synthesis paths inconsistently and no versioned adaptive coverage contract exists | `UsefulPrecursor` | engine intake kernel + skill-directed CLI/SDK adapter | Define `CharterIntakeDefinition`, shared guided/express/agent-assisted coverage, provenance-bearing intake/candidate records, approval/promotion, and targeted reassessment; do not restore a nested CLI wizard | Charter canonical YAML, artifact-kind registry, skill capability reporting, posture kernel |
| Project posture kernel | Charter rendering/validation contains baseline levels, domains, triggers, shortcuts, red lines, and review rules, but there is no resolved fingerprinted posture view or typed recommendation/transition loop | `UsefulPrecursor` | engine pure resolution + SDK/flow orchestration | Resolve effective posture from Charter/overrides/current conditions; derive evidence-backed recommendations from snapshots without automatic policy mutation; require hysteresis and authorized transitions | Charter intake, Snapshot Memory, contracts/evidence, Context Resolution projections |
| Markdown validation | Baseline validation dispatches artifact-specific Markdown validators | `UsefulPrecursor` | `handbook-engine` | Validate canonical structured truth first; treat fixed Markdown as a renderer-derived witness, and validate later Resolution-aware Projections separately where valuable | canonical YAML, renderer proof, later Projection proof |
| Setup scaffolding | Setup emits fixed directories/templates for shipped artifacts | `UsefulPrecursor` | SDK/use case over engine | Generate the selected shipped or repository profile directly; no user legacy migration mode | profile loader, CLI UX |
| Doctor baseline | `handbook doctor --json` reports fixed baseline artifacts and safe next actions | `UsefulPrecursor` | SDK + CLI renderer | Make doctor profile-aware and fully schema-versioned while preserving typed blocked/refused semantics | artifact registry, JSON protocol |
| Flow resolver | `ResolveRequest` selects packet ID plus byte-budget policy; `resolve_with_contract` accepts a layout contract | `BoundaryLanded` for current reduced scope | `handbook-flow` | Add profile and Context Resolution inputs; return projection-aware selection and omission truth | profile kernel, Resolution envelope, SDK DTOs |
| Context budgeting | Flow can keep, summarize, exclude, or refuse by byte size | `UsefulPrecursor` | `handbook-flow` | Keep byte policy as one resource constraint beneath semantic Resolution; do not equate size with granularity | projection engine, omission accounting |
| Pipeline work levels | Stages carry `work_level`; compiler filters `SCOPE` blocks against L0-L3 | `UsefulPrecursor` | `handbook-pipeline` consuming shared Resolution types | Generalize to namespaced Context Resolution and migrate scoped rules without freezing L0-L3 as final semantics | profile resolution stack, pipeline definitions |
| Resolution-aware artifact views | Archived map planned low/normal/high reconstruction; no general live projection API exists | `TargetOnly` | engine/flow split to be frozen | Implement deterministic reveal/derive with provenance and omission truth | canonical YAML, profile, Resolution |
| Snapshot Memory | Engine freshness/manifest fingerprints and pipeline route/capture snapshots provide narrow state/provenance primitives, but no general immutable project/world snapshot, delta, drift, or Resolution projection exists | `TargetOnly` | engine pure model + SDK capture orchestration + flow projection; exact split to freeze | Define capture policy, consistent normalized snapshot, two fingerprints, delta/drift semantics, redaction, retention, and Resolution-aware projection | profile, canonical artifacts, work ledger, contracts, SDK, handoffs |
| Vocabulary | Stage/template/content language embeds sprint, release, feature, slice, packet, task, and fixed artifact labels | `TargetOnly` | profile semantic kernel + renderers | Introduce axis-based vocabulary and explicit lexical/structural conflation | profile schema, fixed renderers, later Projections |
| Public owner crates | `handbook-engine 0.1.1`, `handbook-flow 0.1.1`, and `handbook-pipeline 0.1.2` are published; released-boundary proof exists | `ContractCorrectAndProven` for the exact proved APIs only | existing owner crates | Preserve narrow public capabilities; expand only through reviewed public contracts and new released proof | SDK facade, Substrate consumer tests |
| Compiler seam | `handbook-compiler` is a CLI-facing compatibility/support crate spanning unresolved shell seams | `UsefulPrecursor` | owner crates + `handbook-sdk` + CLI shell | Retire it during HCM-4.1: move ordinary composition to SDK, shell concerns to CLI, and owner behavior back to owners; add no new permanent API | SDK inventory, CLI rewiring, owner-boundary tests |
| Consumer facade | No purpose-named ordinary-consumer SDK crate exists | `TargetOnly` | `handbook-sdk` | Implement the frozen typed use-case/DTO facade without a public untyped dispatcher; keep owner crates public for advanced use | JSON Schema, CLI/Tauri/Substrate adapters |
| CLI transport | CLI owns command parsing and several renderers but still depends on compatibility seams | `UsefulPrecursor` | `handbook-cli` over SDK | Preserve polished UX while removing domain decisions from CLI modules | SDK, common response envelope |
| CLI JSON | `doctor` has explicit `--json`; other JSON behavior is partial or command-specific | `UsefulPrecursor` | SDK DTOs + CLI transport | Map commands to stable operation definitions and emit one versioned response envelope for every recognized nontrivial JSON operation; stdout remains machine-clean and exit/status agree | schema generation, capability discovery, exit policy |
| Installed Handbook skill | Current installed skill directs agents to gather facts, prepare YAML inputs, invoke the CLI, and require `doctor --json`; it is fixed to the current three baseline artifacts and lacks a machine-readable adaptive intake conversation | `UsefulPrecursor` | skill adapter over CLI/SDK capability truth | Preserve skill-directed deterministic CLI use; add capability-driven guided/express/agent-assisted intake against one coverage/schema contract; make artifact/profile/vocabulary/Resolution instructions capability-driven; do not add nested synthesis | SDK capability reporting, full JSON, kind/intake registry, profile kernel |
| Tauri | No Tauri application or adapter exists | `TargetOnly` | future Tauri shell over SDK | Reuse Serde DTOs/use cases; do not shell out for normal operation | SDK and JSON Schema parity |
| Initial Substrate CLI bridge | Approved as a transitional product integration but not implemented for the membrane | `TargetOnly` | isolated Substrate process adapter consuming CLI JSON | Bundle an exact binary; pin operation/schema fingerprints; bound process resources; never parse prose; remove from the normal path after `PG-SUB-RUST-01` | full CLI JSON, capability reporting, `BR-SUB-CLI-01` |
| Direct Substrate imports | Historical dedicated worktree proof shows exact published engine/flow consumption is feasible; the currently inspected Substrate checkout pins Handbook crates but has no live Handbook API call, and its pipeline pin trails Handbook's current workspace version | `UsefulPrecursor` for future membrane adoption; historical proof remains valid only for its exact APIs | Substrate consuming published SDK/owner crates | Preserve exact-version proof discipline, create a current-tip real seam for each downstream-intended API, and prove no path/patch/process fallback before calling the permanent boundary adopted | crates.io publication, registry-only consumer, real worktree seam |
| Contract membrane | The HCM-0.5 canonical design subject now defines exact contract identity/SemVer compatibility, immutable lifecycle transitions, claims/applicability, all-of evidence requirements, provenance/freshness/Resolution/cardinality/consistency rules, closed verdicts, and hard-gate composition; no `handbook-contracts` runtime or real evaluation path exists | `TargetOnly` | `handbook-contracts`; Handbook authority | After Phase-0 proof/review closeout, implement HCM-5.1/HCM-5.2 without changing the frozen semantic model; keep lifecycle separate from evaluation and keep every validator a witness | canonical artifacts, HCM-0.4 SDK operations, docks, Resolution/evidence inputs |
| External docks | The HCM-0.5 canonical design subject now defines exact manifest plus content-addressed implementation/runtime closure and typed launch vector, one-shot process JSON, default-deny isolation, unconditional v1 network denial, total host-outcome precedence, and `handbook.dock.json-schema@1.0.0` as the bounded first future proof target; no dock runner, bundle, manifest, or real dock execution exists | `TargetOnly` | Handbook protocol-neutral DTOs in `handbook-contracts` + separable execution adapters | After Phase-0 proof/review closeout, implement HCM-5.3 and prove the selected Draft 2020-12 dock in HCM-5.4; admit candidates through the membrane rather than treating process output as evidence authority | contract core, JSON schemas, implementation bundle/host allowlist, SDK `dock.run`, gate engine |
| AI synthesis | Handbook has no target requirement for model-generated canonical derived views; Substrate already uses Unified Agent API | `TargetOnly` optional | Substrate or optional Handbook adapter | Keep fixed renderer-derived views and later Projections deterministic; future Handbook synthesis must use UAA programmatically and remain candidate-only | promotion gate, provenance |
| Durable top-level handoff | The pack has immutable version-routed records, a rebuildable ledger, supersession, and optional Snapshot Memory refs; the HCM-0.1 history also demonstrates that writing one record per internal review/remediation round creates an incorrect user-routed session queue | `BoundaryLanded` for record/ledger mechanics only | top-level orchestration closeout protocol consuming future Snapshot Memory | Restrict canonical handoffs and ledger writes to genuine top-level stop/resume boundaries; preserve prior records as immutable evidence; prove scoped resume, supersession, validation, and repository-relative references | delegated orchestration, handoff v1.2 schema, Snapshot Memory |
| Delegated development orchestration | Repository skills require context/specification, implementation or documentation, verification, independent review, remediation, and re-review; the prior onboarding prompt stopped after dispatch instead of executing delegable work through built-in subagents | `BoundaryLanded` for the corrected control-pack contract; full exercise remains open | top-level control-pack slice runner using built-in subagent capabilities | Keep the parent alive for the explicit phase/slice; execute internal dispatches with fresh `default` agents; collect results; enforce review -> fix -> different fresh review; close only after proof/commit or a genuine stop condition | required skills, dispatch envelope, durable top-level handoff, proof ledger |
| Contract-catalog decomposition | `05-contracts-schemas-and-gates.md` remains the one canonical monolithic catalog | `TargetOnly`; HCM-0.9 decomposition was abandoned after terminal Redesign Review 2 was not CLEAN | Handbook Contract Membrane control pack | Retain the monolith. Do not create leaf files, a compatibility index, or an automatic semantic routing engine without a new explicit human decision and new reviewed packet | HCM-0.4 frozen contract baseline, rejected HCM-0.9 evidence checkpoint `f3a33ddb55443d37f3a51ffb58f1c85b74a28b23`, terminal abandonment handoff |

## High-risk coupling zones

### Fixed artifact identity

The fixed artifact enum flows into:

- canonical paths and layout;
- setup scaffolding;
- baseline validation;
- doctor output;
- flow resolver priority and packet assembly;
- rendering and tests.

Do not treat descriptor generalization as a local enum refactor.

Kind definitions and repository instances must not collapse back into one descriptor that mixes reusable schema behavior with path, label, and requiredness state. The HCM-0.6 shipped-default decision is explicit target authority; implementation must consume that exact decision rather than infer or amend it from current enum variants, labels, examples, or historical filenames.

### Markdown-first assumptions

Markdown authority is embedded across:

- authoring write paths;
- starter-template detection;
- baseline validation;
- resolver ingestion;
- tests and fixtures;
- environment-inventory references.

The greenfield cutover may replace these directly, but slice boundaries must keep the tree coherent and delete temporary bridges on schedule.

### CLI and compatibility logic

The current CLI is not yet a pure adapter, and `handbook-compiler` still owns support seams. The SDK slice must first inventory actual use-case composition. It must not create a facade that merely republishes CLI wording.

The target ordinary-consumer facade is typed and capability-oriented. Stable operation IDs, request/result schemas, refusal/blocker/error records, and schema fingerprints exist independently from CLI command paths or Tauri command names. The compiler retirement must not create a dependency cycle by moving owner logic into the SDK.

### Pipeline/Resolution semantics

Current work levels are embedded in stage files, rule filters, catalog rendering, and tests. The new Resolution model must preserve useful scoped behavior while replacing the mixed L0-L3 taxonomy.

### Snapshot consistency and sensitivity

Git, Handbook artifacts, work queues, contracts, and evidence may change while a snapshot is being captured. The snapshot seam must record pre/post revisions and mark or refuse unstable captures rather than pretending to provide atomic world state.

Dirty paths, diffs, command history, environment data, and untracked files may expose sensitive material. Snapshot policies must default to normalized metadata, fingerprints, redacted summaries, and artifact references instead of embedding unrestricted content.

### Intake authority and posture drift

Repository inspection may establish observational facts, but it cannot authorize constitutional policy, exception authority, or red lines. Intake records must distinguish inferred observations, user declarations, defaults, known unknowns, and approvals.

Posture recommendations derived from snapshots are advisory until an authorized transition is approved. Fast automatic raises and lowers would create policy churn; hard triggers may prompt an immediate raise recommendation, while lowering requires sustained evidence and cannot bypass floors or non-negotiables.

### Orchestration versus a session-routing queue

An immutable dispatch is useful for bounded context, audit, and replay, but creating it does not complete orchestration. For `execution_target: internal_subagent`, the parent must immediately use the built-in subagent capability, wait for the result, validate findings, and continue the selected slice.

Internal subagents do not append the canonical handoff ledger. Requiring the user to start a new task for an ordinary review, proof, documentation repair, child packet, or remediation round converts a long-lived slice runner into a manual queue and violates the target seam.

### Contract/dock identity and authority coupling

Contract meaning, evidence admission, verdicts, and gates remain in `handbook-contracts`; process execution remains separable. Do not collapse the two because one host allowlists or launches a validator. A selected manifest is operationally usable only when its exact content-addressed implementation bundle, normalized file manifest, entrypoint digest, typed executable/interpreter/application launch vector plus argument order, and runtime/dependency closure revalidate; that closure still grants no canonical authority.

The first process transport is one bounded JSON document in and one bounded JSON document out, with default-deny grants and unconditional v1 network denial. Every non-completed host outcome and every invalid candidate produces no canonical evidence. A future Rust-native adapter must preserve the same identities, candidate shape, Resolution/provenance limits, evidence-admission boundary, verdict vocabulary, and hard-gate behavior rather than becoming a second semantic path.

`handbook.dock.json-schema@1.0.0` is selected only as the future HCM-5.4 proof target for one exact local Draft 2020-12 schema closure. The design selection, an allowlist entry, or a passing standalone validator does not promote the Contract membrane or External docks rows beyond `TargetOnly`; runtime classification changes only from the exact proof gates in `06-proof-and-regression-ledger.md`.

### Contract catalog remains monolithic

The contract catalog remains canonical at `05-contracts-schemas-and-gates.md`.
HCM-0.9 attempted a documentation-only decomposition plan, but terminal
Redesign Review 2 retained one Required proof defect after the only authorized
remediation. The preauthorized outcome is abandonment, not another repair.

No canonical leaf files or stable-index cutover exist. Historical `05` path,
line, anchor, and manifest references remain valid against the monolith and Git
history. Future slices must cite the monolith path and exact sections they need
unless a new human-authorized decomposition is independently planned and
reviewed.

Both the earlier semantic-routing design and the later eight-span mechanical
candidate are non-authoritative historical evidence. No machine infers
contract-catalog authority or performs transitive loading. A review manifest
still contains only bytes under review; unchanged contextual authority remains
in `authority_refs` and/or `contracts_and_gates`.

## Crosswalk update rule

When a slice changes a seam:

1. update its current live truth;
2. record the exact evidence refs;
3. change at most to the classification supported by that evidence;
4. update `06-proof-and-regression-ledger.md`;
5. let the top-level orchestrator write a closeout handoff only at a genuine stop boundary;
6. leave sibling rows unchanged unless the same proof actually exercised them.
