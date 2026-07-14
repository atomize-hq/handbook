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
| Canonical artifact identities | `CanonicalArtifactKind` is a fixed four-variant enum: Charter, ProjectContext, EnvironmentInventory, FeatureSpec | `UsefulPrecursor` | `handbook-engine` profile/artifact kernel | Replace enum-owned universe with distinct profile-resolved kind definitions and instance descriptors while retaining stable semantic role/capability IDs | kind meta-schema, profile schema, generic validation, setup/doctor/flow |
| Artifact kind/schema registry | Current artifact-specific Rust models and templates imply reusable kinds, but there is no first-class `ArtifactKindDefinition`, repository-local custom-kind schema registry, or kind-definition meta-validation | `TargetOnly` | `handbook-engine` | Define versioned kind/schema/intake/projection capabilities; validate custom kinds without Rust enum variants or executable schema hooks | instance profiles, schema policy, projection engine, generic CLI/SDK operations |
| Shipped default artifact set | Current setup and skill assume a fixed baseline set; candidate names appear across active and historical docs, but no approved research-backed future default set exists | `TargetOnly` | Phase 0 design authority, then shipped profile data | Research candidate sets and complete a user brainstorming/decision session before freezing shipped kinds, default instances, and requiredness | kind registry, constitutional-root contract, setup/doctor, skill UX |
| Canonical layout | `CanonicalLayoutContract` parameterizes paths but still has dedicated fields for fixed artifact kinds | `UsefulPrecursor` | `handbook-engine` | Move to descriptor-driven layout resolved from the instance profile | artifact registry, trusted repo-relative paths |
| Structured authoring | Charter, Project Context, and Environment Inventory accept structured YAML inputs and deterministically render Markdown | `UsefulPrecursor` | `handbook-engine` | Invert authority so meaningful YAML is canonical and Markdown is derived through kind-selected projections | artifact-kind schemas, projections, cutover gate |
| Charter intake coverage | `CHARTER_INPUTS.yaml.tmpl` preserves broad questionnaire-shaped fields and deterministic validation exists, while live authoring guidance gathers structured facts through a skill; the retained stage directives still frame interview/synthesis paths inconsistently and no versioned adaptive coverage contract exists | `UsefulPrecursor` | engine intake kernel + skill-directed CLI/SDK adapter | Define `CharterIntakeDefinition`, shared guided/express/agent-assisted coverage, provenance-bearing intake/candidate records, approval/promotion, and targeted reassessment; do not restore a nested CLI wizard | Charter canonical YAML, artifact-kind registry, skill capability reporting, posture kernel |
| Project posture kernel | Charter rendering/validation contains baseline levels, domains, triggers, shortcuts, red lines, and review rules, but there is no resolved fingerprinted posture view or typed recommendation/transition loop | `UsefulPrecursor` | engine pure resolution + SDK/flow orchestration | Resolve effective posture from Charter/overrides/current conditions; derive evidence-backed recommendations from snapshots without automatic policy mutation; require hysteresis and authorized transitions | Charter intake, Snapshot Memory, contracts/evidence, Context Resolution projections |
| Markdown validation | Baseline validation dispatches artifact-specific Markdown validators | `UsefulPrecursor` | `handbook-engine` | Validate canonical structured truth first; validate rendered views only as projections/witnesses where valuable | canonical YAML, renderer proof |
| Setup scaffolding | Setup emits fixed directories/templates for shipped artifacts | `UsefulPrecursor` | SDK/use case over engine | Generate the selected shipped or repository profile directly; no user legacy migration mode | profile loader, CLI UX |
| Doctor baseline | `handbook doctor --json` reports fixed baseline artifacts and safe next actions | `UsefulPrecursor` | SDK + CLI renderer | Make doctor profile-aware and fully schema-versioned while preserving typed blocked/refused semantics | artifact registry, JSON protocol |
| Flow resolver | `ResolveRequest` selects packet ID plus byte-budget policy; `resolve_with_contract` accepts a layout contract | `BoundaryLanded` for current reduced scope | `handbook-flow` | Add profile and Context Resolution inputs; return projection-aware selection and omission truth | profile kernel, Resolution envelope, SDK DTOs |
| Context budgeting | Flow can keep, summarize, exclude, or refuse by byte size | `UsefulPrecursor` | `handbook-flow` | Keep byte policy as one resource constraint beneath semantic Resolution; do not equate size with granularity | projection engine, omission accounting |
| Pipeline work levels | Stages carry `work_level`; compiler filters `SCOPE` blocks against L0-L3 | `UsefulPrecursor` | `handbook-pipeline` consuming shared Resolution types | Generalize to namespaced Context Resolution and migrate scoped rules without freezing L0-L3 as final semantics | profile resolution stack, pipeline definitions |
| Resolution-aware artifact views | Archived map planned low/normal/high reconstruction; no general live projection API exists | `TargetOnly` | engine/flow split to be frozen | Implement deterministic reveal/derive with provenance and omission truth | canonical YAML, profile, Resolution |
| Snapshot Memory | Engine freshness/manifest fingerprints and pipeline route/capture snapshots provide narrow state/provenance primitives, but no general immutable project/world snapshot, delta, drift, or Resolution projection exists | `TargetOnly` | engine pure model + SDK capture orchestration + flow projection; exact split to freeze | Define capture policy, consistent normalized snapshot, two fingerprints, delta/drift semantics, redaction, retention, and Resolution-aware projection | profile, canonical artifacts, work ledger, contracts, SDK, handoffs |
| Vocabulary | Stage/template/content language embeds sprint, release, feature, slice, packet, task, and fixed artifact labels | `TargetOnly` | profile semantic kernel + renderers | Introduce axis-based vocabulary and explicit lexical/structural conflation | profile schema, all projections |
| Public owner crates | `handbook-engine 0.1.1`, `handbook-flow 0.1.1`, and `handbook-pipeline 0.1.2` are published; released-boundary proof exists | `ContractCorrectAndProven` for the exact proved APIs only | existing owner crates | Preserve narrow public capabilities; expand only through reviewed public contracts and new released proof | SDK facade, Substrate consumer tests |
| Compiler seam | `handbook-compiler` is a CLI-facing compatibility/support crate spanning unresolved shell seams | `UsefulPrecursor` | owner crates + proposed SDK | Do not promote it by default; decide whether remaining logic moves to SDK, CLI shell, or owner crates | owner-boundary design slice |
| Consumer facade | No purpose-named ordinary-consumer SDK crate exists | `TargetOnly` | proposed `handbook-sdk` | Define minimal use cases and DTOs; keep owner crates public for advanced use | JSON Schema, CLI/Tauri/Substrate adapters |
| CLI transport | CLI owns command parsing and several renderers but still depends on compatibility seams | `UsefulPrecursor` | `handbook-cli` over SDK | Preserve polished UX while removing domain decisions from CLI modules | SDK, common response envelope |
| CLI JSON | `doctor` has explicit `--json`; other JSON behavior is partial or command-specific | `UsefulPrecursor` | SDK DTOs + CLI transport | Provide one versioned JSON envelope for every nontrivial operation; stdout must remain machine-clean | schema generation, exit policy |
| Installed Handbook skill | Current installed skill directs agents to gather facts, prepare YAML inputs, invoke the CLI, and require `doctor --json`; it is fixed to the current three baseline artifacts and lacks a machine-readable adaptive intake conversation | `UsefulPrecursor` | skill adapter over CLI/SDK capability truth | Preserve skill-directed deterministic CLI use; add capability-driven guided/express/agent-assisted intake against one coverage/schema contract; make artifact/profile/vocabulary/Resolution instructions capability-driven; do not add nested synthesis | SDK capability reporting, full JSON, kind/intake registry, profile kernel |
| Tauri | No Tauri application or adapter exists | `TargetOnly` | future Tauri shell over SDK | Reuse Serde DTOs/use cases; do not shell out for normal operation | SDK and JSON Schema parity |
| Initial Substrate CLI bridge | Approved as a possible first product integration; not yet implemented for the membrane | `TargetOnly` | Substrate adapter consuming CLI JSON | Bundle exact binary/protocol version and isolate replaceable bridge | full CLI JSON, capability reporting |
| Direct Substrate imports | Engine/flow proof and released pipeline proof show real published consumption is viable | `RealPathAdopted` for narrow existing seams | Substrate consuming published owner/SDK crates | Preserve exact-version proof and extend to each downstream-intended membrane API | crates.io publication, real worktree seam |
| Contract membrane | Idea docs define lifecycle, claims, evidence, verdicts, gates, and dock semantics; no general live system exists | `TargetOnly` | owner decision pending; Handbook authority | Freeze typed core and lifecycle without creating a universal validator | canonical artifacts, SDK, docks |
| External docks | Dock taxonomy and mental model exist only in architecture docs | `TargetOnly` | Handbook dock protocol + separable implementations | Define semantic protocol; implement process JSON first; normalize evidence | contract core, JSON schemas, capability manifest |
| AI synthesis | Handbook has no target requirement for model-generated canonical projections; Substrate already uses Unified Agent API | `TargetOnly` optional | Substrate or optional Handbook adapter | Keep initial projections deterministic; future Handbook synthesis must use UAA programmatically and remain candidate-only | promotion gate, provenance |
| Orchestration handoff | Prior sessions relied heavily on chat closeout and manual copy/paste; the pack now defines immutable handoff/dispatch records but runtime Snapshot Memory is not implemented | `BoundaryLanded` for the docs protocol | control-pack protocol consuming future Snapshot Memory | Require immutable handoff records, snapshot refs/deltas when available, escalation classification, and durable next-session dispatches | orchestration prompt, handoff schema, Snapshot Memory |

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

Kind definitions and repository instances must not collapse back into one descriptor that mixes reusable schema behavior with path, label, and requiredness state. The shipped-default decision is a design/research authority question, not something implementation should infer from current enum variants or historical filenames.

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

### Pipeline/Resolution semantics

Current work levels are embedded in stage files, rule filters, catalog rendering, and tests. The new Resolution model must preserve useful scoped behavior while replacing the mixed L0-L3 taxonomy.

### Snapshot consistency and sensitivity

Git, Handbook artifacts, work queues, contracts, and evidence may change while a snapshot is being captured. The snapshot seam must record pre/post revisions and mark or refuse unstable captures rather than pretending to provide atomic world state.

Dirty paths, diffs, command history, environment data, and untracked files may expose sensitive material. Snapshot policies must default to normalized metadata, fingerprints, redacted summaries, and artifact references instead of embedding unrestricted content.

### Intake authority and posture drift

Repository inspection may establish observational facts, but it cannot authorize constitutional policy, exception authority, or red lines. Intake records must distinguish inferred observations, user declarations, defaults, known unknowns, and approvals.

Posture recommendations derived from snapshots are advisory until an authorized transition is approved. Fast automatic raises and lowers would create policy churn; hard triggers may prompt an immediate raise recommendation, while lowering requires sustained evidence and cannot bypass floors or non-negotiables.

## Crosswalk update rule

When a slice changes a seam:

1. update its current live truth;
2. record the exact evidence refs;
3. change at most to the classification supported by that evidence;
4. update `06-proof-and-regression-ledger.md`;
5. write the closeout handoff record;
6. leave sibling rows unchanged unless the same proof actually exercised them.
