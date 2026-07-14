# Proof and Regression Ledger

## Purpose

This ledger records what is actually proven, what remains only architectural intent, and which current behaviors must survive target-architecture work.

It is not a task checklist. Slice-local `tasks/todo.md` files own execution status.

## Proof levels

| Level | Question answered |
|---|---|
| `Exists` | Does an artifact, type, command, crate, or test exist? |
| `SemanticallyCorrect` | Does it encode the approved target meaning? |
| `BoundaryLanded` | Does the correct owner expose/enforce it? |
| `RealPathAdopted` | Does a real product path use that boundary? |
| `RuntimeProven` | Does required runtime/e2e/negative evidence exercise the path? |
| `ReviewClean` | Has an independent review found no remaining actionable issue? |

No lower proof level implies a higher one.

## Current proven baselines

### `PR-001` — Published owner crates

**Current evidence:**

- `handbook-engine = 0.1.1` is published;
- `handbook-flow = 0.1.1` is published;
- `handbook-pipeline = 0.1.2` is published;
- the released pipeline proof rejects path dependencies and checks registry provenance;
- prior dedicated Substrate proofs showed real published engine/flow and pipeline consumption in bounded seams.

**Classification:** `ContractCorrectAndProven` only for the exact published APIs and proof seams exercised.

**Must preserve:**

- registry-only proof;
- exact version assertion;
- real downstream seam;
- distinction between engine/flow proof and pipeline proof;
- no claim that every future membrane API is already importable.

### `PR-002` — Structured baseline input parsing

**Current evidence:** engine exposes typed YAML parse/validate models for Charter, Project Context, and Environment Inventory.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic typed parsing/validation value, not Markdown authority.

**Must not overclaim:** the existing per-artifact Rust models are not a generic artifact-kind/schema registry, and the current templates/directives are not a versioned adaptive intake coverage contract.

### `PR-003` — Deterministic Markdown rendering

**Current evidence:** engine exposes deterministic Markdown renderers for the three baseline authoring families.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic human review projection where still valuable.

**Must not preserve:** independently editable Markdown as canonical truth.

### `PR-004` — Trusted repo-relative artifact access

**Current evidence:** canonical loading/path contracts enforce bounded repo-relative access and reject unsafe states such as disallowed symlinks.

**Classification:** `BoundaryLanded` for current fixed artifacts.

**Must preserve:** trusted path normalization and no-follow behavior when descriptors become dynamic.

### `PR-005` — Work-level scoped rule filtering

**Current evidence:** pipeline stages carry work levels and compiler inclusion filters honor scoped blocks.

**Classification:** `UsefulPrecursor`.

**Must preserve:** the ability to select relevant rule/context sections for a declared working scope.

**Must not preserve:** L0-L3 as the final mixed taxonomy if Context Resolution replaces it.

### `PR-006` — Doctor JSON baseline

**Current evidence:** `handbook doctor --json` emits a typed serialized report.

**Classification:** `UsefulPrecursor`.

**Must preserve:** machine-readable baseline/refusal/next-action semantics.

**Gap:** JSON parity and common envelope do not yet cover all commands.

### `PR-007` — Flow resolver typed decisions

**Current evidence:** flow exposes `resolve_with_contract`, typed selection, refusal, blockers, budget outcome, and next actions.

**Classification:** `BoundaryLanded` for the current reduced request model.

**Must preserve:** typed semantic decisions and consumer-owned rendering.

**Gap:** no profile or Context Resolution input; byte budgets are not semantic projections.

### `PR-008` — Narrow snapshot and fingerprint primitives

**Current evidence:** engine freshness/manifest records compute deterministic fingerprints; pipeline route basis records revision/fingerprint state; capture logic uses rollback snapshots for write safety.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic normalization/fingerprint value, revision-bound route provenance, and safe capture rollback behavior.

**Must not overclaim:** these primitives do not implement general Snapshot Memory, strategic capture hooks, project/world state records, snapshot deltas, drift analysis, or Resolution-aware snapshot projection.

### `PR-009` — Charter questionnaire-shaped coverage and posture validation

**Current evidence:** the Charter structured-input template and engine types retain project shape, constraints, operational reality, posture, domains, nine engineering dimensions, exceptions, debt, and decision-record fields; validation refuses incomplete/placeholder required content and rendering emits deterministic posture sections.

**Classification:** `UsefulPrecursor`.

**Must preserve:** the semantic coverage and deterministic validation value unless a Phase 0 decision explicitly removes or replaces an item.

**Must not preserve:** a rigid terminal questionnaire, prompt-owned authority, Markdown as canonical truth, or one artifact-specific implementation as the generic intake architecture.

## Open program proof gates

| Gate | Required proof | Current state |
|---|---|---|
| `PG-PROFILE-01` | selected profile resolves complete artifact/vocabulary/Resolution truth with deterministic fingerprint | open |
| `PG-DEFAULT-01` | focused research plus a user brainstorming/decision session explicitly approve the shipped kind set, default instances, and requiredness; examples/current enums do not count | open |
| `PG-KIND-01` | a versioned `ArtifactKindDefinition` resolves a safe canonical schema, validation, optional intake, lifecycle, and projections independently from repository instance state | open |
| `PG-KIND-02` | repository-defined custom kind passes meta-schema/structural validation without a new Rust enum variant, executable hook, remote schema fetch, or generated CLI command | open |
| `PG-ARTIFACT-01` | a profile-selected `ArtifactInstanceDescriptor` binds a kind to path/label/requiredness/dependencies and participates in validation/doctor/flow | open |
| `PG-INTAKE-01` | guided-adaptive, express, and agent-assisted acquisition use one intake definition and produce the same candidate schema while exposing missing coverage | open |
| `PG-INTAKE-02` | intake provenance distinguishes user declarations, evidenced inference, defaults, unknowns, contradictions, waivers, and approvals; normative fields cannot be silently inferred into authority | open |
| `PG-CHARTER-01` | `CharterIntakeDefinition` covers approved questionnaire domains, promotes only an approved schema-valid candidate to canonical Charter YAML, and deterministically projects Markdown | open |
| `PG-YAML-01` | one artifact family is canonically YAML, structurally validated, and deterministically rendered | open |
| `PG-YAML-02` | no dual editable Markdown/YAML truth remains for converted families | open |
| `PG-VOCAB-01` | lexical and structural conflation render correctly without losing stable role resolution | open |
| `PG-RES-01` | six-dimension envelope validates inheritance, authority, memory, and validation horizons | open |
| `PG-PROJ-01` | same source truth yields multiple deterministic Resolution projections with provenance | open |
| `PG-PROJ-02` | omitted required claims remain visible and cannot false-pass | open |
| `PG-SNAP-01` | same selected stable state and capture policy produce the same normalized state fingerprint with deterministic ordering | open |
| `PG-SNAP-02` | capture detects concurrent revision changes and returns stable, bounded, or non-promotable unstable truth honestly | open |
| `PG-SNAP-03` | prior-session end and next-session start snapshots produce a deterministic delta that detects stale handoff and unexplained drift | open |
| `PG-SNAP-04` | Resolution-aware snapshot projection includes only authorized/relevant fields and enumerates omissions | open |
| `PG-SNAP-05` | redaction tests prove secrets, unsafe environment values, and unrestricted diff/command contents are excluded | open |
| `PG-SNAP-06` | planned-versus-actual signals distinguish durable justified divergence from unexplained scope/proof/semantic drift | open |
| `PG-POSTURE-01` | identical Charter/override/condition/evidence state resolves the same `ProjectPostureKernel` fingerprint without creating a second editable authority | open |
| `PG-POSTURE-02` | policy-defined hard and accumulated triggers produce evidence-linked advisory recommendations with typed notification/acknowledgment; only authorized transitions change policy, and lowering honors sustained-evidence/floor/red-line rules | open |
| `PG-SDK-01` | CLI and direct Rust consumer call the same use case and receive equivalent typed results | open |
| `PG-JSON-01` | every supported nontrivial CLI operation emits one schema-valid JSON envelope | open |
| `PG-TAURI-01` | thin Tauri command adapter serializes the same SDK DTO without CLI subprocess | open |
| `PG-CONTRACT-01` | locked contract drives claim evaluation and lifecycle-aware gate | open |
| `PG-DOCK-01` | real external process validator emits normalized evidence under declared protocol/Resolution | open |
| `PG-GATE-01` | hard failure blocks regardless of weighted score; required not-observed cannot green | open |
| `PG-SUB-CLI-01` | Substrate uses exact bundled CLI/schema in a real replaceable seam | open |
| `PG-PUBLISH-01` | new downstream-intended API passes exact crates.io external consumer proof | open |
| `PG-SUB-RUST-01` | current-tip Substrate worktree uses exact new crates.io API in a real seam | open |
| `PG-HANDOFF-01` | a blocked slice writes a valid durable handoff; orchestration produces a bounded dispatch without manual report copy | open |
| `PG-HANDOFF-02` | once snapshots land, handoffs reference start/end snapshots and delta; orchestration rechecks current state before dispatch | open |

## Greenfield deletion gates

Temporary scaffolding may be introduced only when a row is added here first.

| Bridge ID | Architectural purpose | Allowed lifetime | Deletion proof |
|---|---|---|---|
| none | no temporary bridge approved yet | n/a | n/a |

There is no approved user migration tool, legacy importer, dual-read mode, or compatibility profile.

## Regression rules

Every implementation slice must preserve applicable baselines:

1. trusted repo-relative/no-follow filesystem behavior;
2. deterministic structured parsing and rendering where retained;
3. typed refusal/blocker/next-action semantics;
4. published owner-crate boundaries not explicitly replaced;
5. registry-only released proof for public APIs;
6. consumer-owned product wording;
7. strict separation of docs/artifacts/evidence from contract authority;
8. no human-output parsing by machine consumers;
9. no promotion beyond evidence Resolution;
10. no implicit legacy compatibility commitment;
11. snapshot records remain immutable, descriptive, redacted, and separate from canonical/transition authority;
12. comprehensive snapshots are never injected wholesale into a narrower agent context;
13. artifact kinds remain distinct from profile-selected repository instances;
14. custom schemas/kinds do not create executable hooks, Rust enum requirements, or dynamic CLI commands;
15. all intake modes converge on one canonical schema and expose missing coverage/provenance;
16. agent inference cannot promote constitutional or normative decisions without required approval;
17. posture recommendations remain advisory and cannot auto-mutate Charter policy.

## Slice closeout evidence record

When a slice closes, update only the affected rows and cite:

- commit/tree state;
- exact source boundary;
- exact tests and commands;
- real-path proof when required;
- negative/fail-closed proof;
- published/downstream evidence when required;
- independent review result;
- handoff record ID.

Do not replace evidence refs with “all tests passed.”

## Control-pack proof gate

Before `HCM-0.1` may close:

- all control-pack files exist and link correctly;
- handoff schema and template are valid JSON;
- README selective-loading and authority rules are complete;
- orchestration prompt can select latest or specified handoff;
- Snapshot Memory semantics are threaded through architecture, contracts, phase sequencing, proof, orchestration, and optional handoff refs;
- artifact-kind/instance separation, repository-defined schemas, adaptive intake, Charter authority, and posture recommendation semantics are threaded through architecture, contracts, sequencing, proof, and skill/orchestration guidance;
- the shipped default artifact set is explicitly unresolved pending `HCM-0.6` research and user brainstorming/decision;
- escalation protocol distinguishes local remediation, decomposition, docs repair, broader design, external blocker, and proof gap;
- active docs point to this pack without treating archived docs as authority;
- no Rust files changed;
- `git diff --check` passes;
- independent review is requested before the pack is treated as frozen implementation authority.
