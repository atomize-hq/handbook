# Target Architecture

## Executive decision

Handbook will become a profile-driven, resolution-aware contract membrane with one transport-neutral capability layer:

> Canonical structured truth is interpreted through separate stable-role and semantic-capability registries, schema-defined artifact kinds, profile-selected artifact instances, agent-directed intake coverage, explicit Context Resolution envelopes, a Charter-led project-posture kernel, deterministic Snapshot Memory and projections, executable contracts, normalized evidence, and hard gate semantics.

The same non-CLI use cases must support:

- the polished `handbook` CLI;
- the future Tauri application;
- an initial bundled-CLI integration inside Substrate;
- the permanent direct Rust integration through published crates;
- future process and Rust-native docks;
- future workflow adapters and overlays.

## Target topology

```mermaid
flowchart TD
    PROFILE["Instance profile\nartifact instances + vocabulary + resolution stack"]
    KINDS["Artifact kind registry\nschema + intake + projections"]
    INTAKE["Agent-directed intake\ncoverage + candidate + approval"]
    YAML["Canonical YAML truth"]
    POSTURE["Resolved project-posture kernel\nCharter-led + evidence-aware"]
    CONTRACT["Contract membrane\nclaims + lifecycle + evidence + verdicts + gates"]
    SNAP["Snapshot Memory\nimmutable capture + delta + projection"]
    PROJ["Resolution projection\nreveal + derive"]
    SDK["handbook-sdk\ntransport-neutral use cases + DTOs"]

    PROFILE --> KINDS
    KINDS --> YAML
    INTAKE -. "validated promotion" .-> YAML
    PROFILE --> PROJ
    YAML --> CONTRACT
    YAML --> PROJ
    YAML --> POSTURE
    PROFILE --> SNAP
    YAML --> SNAP
    CONTRACT --> SNAP
    SNAP --> POSTURE
    POSTURE --> PROJ
    POSTURE --> SDK
    SNAP --> PROJ
    SNAP --> SDK
    CONTRACT --> SDK
    PROJ --> SDK

    SDK --> CLI["handbook-cli\nhuman UX + --json"]
    SDK --> TAURI["Tauri adapter"]
    SDK --> RUST["Published Rust consumers"]
    SDK --> HTTP["Optional future HTTP/OpenAPI adapter"]
    SDK -. "strategic capture hooks" .-> SNAP

    CLI --> SUBCLI["Initial bundled-CLI Substrate bridge"]
    RUST --> SUBRUST["Permanent direct Substrate integration"]

    DOCKS["External validator docks"] --> CONTRACT
    CONTRACT --> DOCKS
    UAA["Optional Unified Agent API synthesis adapter"] -. candidate only .-> PROJ
```

## Authority split

### Handbook owns

- canonical structured artifact truth;
- a versioned replayable stable-role registry for typed places/workflow vocabulary and separate semantic-capability contracts for behavioral conformance;
- instance-profile validation and resolution;
- artifact-kind definitions, schema registry, artifact-instance descriptors, and requiredness semantics;
- intake coverage definitions, candidate validation, approval/promotion semantics, and deterministic Charter authoring boundaries;
- resolved project-posture semantics and typed recommendation/transition records without automatic canonical mutation;
- vocabulary and intentional conflation semantics;
- Context Resolution schemas and projection semantics;
- Snapshot Memory capture policy, immutable record, consistency, fingerprint, delta, drift, and projection semantics;
- exact contract-definition identity, compatibility, immutable lifecycle records, claims, applicability, and invariants;
- canonical evidence admission and normalization from untrusted witness candidates;
- claim verdict, score, hard-gate, local-closeout, and parent-promotion semantics;
- protocol-neutral dock identities, capability ceilings, request/result DTOs, and implementation-bundle/runtime-closure bindings;
- transport-neutral request/result DTOs;
- schema identifiers and compatibility rules.

### The CLI owns

- command hierarchy and discoverability;
- help, examples, and polished operator language;
- argument parsing;
- human-readable rendering;
- `--json` selection and stdout/stderr discipline;
- process exit-code mapping;
- executable-shell concerns such as cwd/repo discovery.

The CLI must not own contract evaluation, artifact semantics, resolution projection, or dock normalization.

### `handbook-sdk` owns

`handbook-sdk` is the preferred name for the ordinary-consumer facade. The name communicates an embeddable library rather than a network daemon. Internal modules may use service/use-case terminology.

It owns:

- stable consumer-oriented use cases;
- request/result DTOs shared by CLI, Tauri, and ordinary Rust consumers;
- transport-neutral error/refusal types;
- immutable bootstrap capability reporting plus exact profile/schema catalog listing and schema retrieval;
- composition over the narrower owner crates.

The Rust facade exposes typed methods, not a public `serde_json::Value` dispatcher. Stable operation IDs and generated JSON Schemas describe those same typed methods to machine transports. An immutable distinct-ref-per-major bootstrap descriptor breaks capability-discovery circularity; snapshot-bound paged operation/profile/schema/governed-record catalogs, exact record/vocabulary/Resolution/Projection reads, and applicable-snapshot selection expose complete restart-safe machine inputs rather than inferred defaults or process-local state. Advanced consumers may still import owner crates directly. The SDK exposes capabilities, not internal module topology, and no owner crate depends back on the SDK.

### Crate ownership

The target should preserve the useful decoupling already present:

- `handbook-engine`: canonical data, artifact-kind/intake schemas, profile/semantic validation, Charter/posture resolution, snapshot normalization/fingerprints/deltas, and other pure transformations;
- `handbook-flow`: request-scoped selection, context assembly, Resolution envelope application, posture/snapshot projection, and packet/projection results;
- `handbook-pipeline`: declarative workflow compilation/capture/handoff and execution sequencing;
- `handbook-contracts`: purpose-named owner for contract identity/lifecycle, claims, evidence admission, verdicts, hard gates, and protocol-neutral dock types; the HCM-0.5 design subject defines those type semantics without implementing the crate;
- `handbook-compiler`: current compatibility/support seam with an approved retirement posture, not a target facade or SDK dependency;
- `handbook-sdk`: ordinary-consumer composition and transport DTO facade;
- `handbook-cli`: executable transport only.

The target dependency direction is acyclic:

```text
handbook-flow      depends on handbook-engine
handbook-contracts depends on handbook-engine
handbook-pipeline  depends on handbook-engine
handbook-pipeline  may additionally depend on handbook-contracts in Phase 5
handbook-sdk       depends on the implemented owner crates
transports         depend on handbook-sdk
```

The diagram expresses allowed inward dependencies, not a requirement that every crate import every predecessor. `handbook-contracts` may depend on exact semantic types from `handbook-engine`; `handbook-pipeline` may consume `handbook-contracts` when contract execution lands; `handbook-sdk` composes the four owner crates. `handbook-engine`, `handbook-flow`, `handbook-pipeline`, and `handbook-contracts` never depend on `handbook-sdk`, a transport, or Substrate. Process-dock implementations remain separable adapters over `handbook-contracts`; HCM-0.5 freezes their protocol before an implementation crate name is selected.

### HCM-3.5 future Resolution-adoption cutover

HCM-3.5 freezes a future greenfield owner boundary without selecting a Rust
signature, public DTO, schema, or implementation:

- `handbook-engine` remains the sole owner of snapshot/delta compatibility,
  currentness, redaction-before-read, deterministic Resolution Projection,
  and an engine-produced bounded/redacted delta-signal summary. The immutable
  HCM-3.4 `snapshot_delta` remains a relation-only observation; neither Flow
  nor pipeline may read its complete signals or recreate its model.
- `handbook-flow` consumes one exact typed engine grounding result and owns
  packet-ready/refused handling, typed omissions, and provenance forwarding.
  It does not own snapshot capture, delta derivation, currentness, redaction,
  or a promotion decision.
- `handbook-pipeline` consumes one exact namespaced engine Resolution input
  for shared scoped inclusion. It owns compilation/capture/handoff sequencing,
  but cannot retain raw `work_level` or L0–L3 as final semantic authority and
  cannot become a second resolver.
- `handbook-sdk` will compose the selected owner operations for ordinary
  consumers; the standalone Handbook CLI will be an SDK adapter. Substrate
  will consume or wrap the exact published crates.io SDK/library boundary;
  Tier 2's exact-binary/versioned-JSON bridge remains isolated and
  transitional until the separate published real-seam proof replaces it.

The cutover is intentionally greenfield. Future purpose-named adoption paths
must not extend the existing resolver or compiler request/result contracts,
and their current callers are evidence for regression proof rather than a
legacy compatibility promise. A temporary one-way, definition-pinned mapping
may translate legacy scoped metadata to the namespaced shared Resolution input
only during cutover; unknown, stale, malformed, ambiguous, or unapproved
mapping refuses. The normal adoption path switches only after the complete
HCM-3.5 proof wall; it never falls back from a typed Resolution refusal to raw
`work_level` filtering.

`handbook-compiler` is retired during HCM-4.1: composition that belongs to ordinary consumers moves to `handbook-sdk`, executable-shell behavior moves to `handbook-cli`, and already-owned behavior remains in its owner crate. It may exist as bounded cutover scaffolding only until the CLI no longer depends on it; no new downstream API or permanent domain owner is added there.

### Agent-facing Handbook skill

The primary agent workflow is skill-directed CLI use:

- the accompanying installed Handbook skill teaches an AI agent which repository facts to gather and which Handbook operations to invoke;
- for authoring, the skill invokes a guided-adaptive, express, or agent-assisted intake workflow against a Handbook-provided coverage contract; all modes target the same artifact schema;
- the LLM agent running the skill conducts the conversation and submits structured observations/declarations through the CLI or, later, the SDK; Handbook does not hide a nested model call inside deterministic authoring;
- the agent supplies structured inputs and requests through the supported CLI/SDK contract;
- Handbook performs deterministic parsing, validation, projection, contract evaluation, and writing;
- the skill must not recreate Handbook semantics in prompt prose or introduce an untracked nested model call;
- future skill instructions consume profile, capability, schema, and Resolution truth from Handbook rather than hard-coding one artifact/vocabulary set;
- session onboarding requests a Resolution-appropriate projection of the latest applicable snapshot instead of loading the complete world view by default.

The skill is an onboarding/orchestration adapter. The CLI/SDK remains the executable product authority.

### Development orchestration control surface

The repository-owned control-pack runner is a development workflow, not a second Handbook runtime or a replacement for Substrate orchestration.

- one top-level orchestrator owns an explicitly selected phase, slice, and optional packet through proof and closeout;
- a selected handoff supplies resume context but does not choose the work or replace the slice packet;
- durable dispatches are bounded execution envelopes and audit/replay artifacts;
- delegable dispatches default to fresh built-in `default` subagents with isolated context and an explicit required-skill chain;
- the parent waits for built-in results and remains responsible for validation, integration, proof, and commit;
- implementation/documentation agents do not self-review; actionable findings flow through remediation and a different fresh reviewer;
- multi-packet slices may use bounded packet reviewers, but final closeout uses a different fresh reviewer over the complete final subject and proof wall;
- internal subagents return structured results and do not write the global handoff ledger;
- a new top-level task is reserved for completion, human interaction, external blockers, authority/context boundaries, or unavailable mandatory delegation.

This orchestration discipline governs development of Handbook. It does not add an agent runtime dependency to Handbook's product crates.

### Substrate owns

- Substrate-side orchestration and runtime behavior;
- when agents are dispatched;
- enforcement of tool, filesystem, process, and runtime authority;
- context collapse/expansion during execution;
- sequencing Handbook docks and gates in Substrate workflows;
- Substrate-facing product wording;
- optional model synthesis that belongs to Substrate's agent harness;
- replacement of the initial CLI bridge with published Rust consumption.

Substrate consumes Handbook contract meaning. It does not become a second contract authority.

## Integration ladder

### Tier 1 — Handbook product path

The CLI calls transport-neutral use cases and offers complete JSON output for every nontrivial operation.

### Tier 2 — Initial Substrate CLI bridge

Substrate may bundle and invoke an exact Handbook binary version and consume only its versioned JSON protocol.

This is a supported transitional integration milestone. It pins the binary, immutable bootstrap descriptor, operation-definition fingerprints, response schemas, time/resource bounds, and replacement gate. Sensitive bodies travel only through bounded stdin or trusted exact refs, never argv, environment, stderr, process titles, or ambient temporary files. It is not proof that a downstream-intended Rust API is complete and it is removed from the normal Substrate path once Tier 4 satisfies `PG-SUB-RUST-01`.

### Tier 3 — Tauri parity

The Tauri application invokes the same SDK use cases and Serde DTOs directly. Tauri command names and scheduling are adapter concerns; the command payload and result remain the SDK DTOs. It does not shell out to the CLI for normal operation, throw prose in place of typed expected outcomes, or evaluate domain policy in the frontend.

### Tier 4 — Permanent Substrate Rust boundary

Substrate imports exact published `handbook-sdk` and/or advanced owner-crate versions from crates.io and uses them in a real seam. The proof starts from current Substrate tip and must not rely on sibling paths, `[patch]` overrides, unpublished workspace internals, or the Tier-2 process bridge. Substrate still owns runtime orchestration and product wording; direct linking changes transport, not semantic authority.

## Canonical artifacts, renderer-derived views, and Projections

- An `ArtifactKindDefinition` defines reusable schema, intake, semantic-validation, lifecycle, and projection capabilities; an `ArtifactInstanceDescriptor` binds a kind to a repository-specific identity, path, label, requiredness rule, and dependencies.
- The shipped default profile selects the exact three-instance set approved by HCM-0.6 from an exact six-kind first-party catalog. The decision is target data, not implementation; examples, current enums, templates, and filenames remain non-authoritative.
- Repository-defined custom kinds use the same registry and generic operations as shipped kinds; adding one must not require a new Rust enum variant or CLI subcommand.
- YAML is the durable canonical representation where structure is semantically meaningful.
- A **renderer-derived view** is a fixed deterministic pre-Phase-3 human-review output produced by a first-party renderer. It accepts no Context Resolution input and is outside the capitalized Phase-3 `Projection` request/result/provenance contract.
- A capitalized **Projection** is a Phase-3 generic Resolution-aware derived view. Generic configured custom-kind views and Resolution-aware Markdown, CLI text, GUI views, packets, OpenAPI, or external workflow formats enter this contract only after the Context Resolution kernel and deterministic Projection engine exist.
- Renderer-derived views, Projections, and adapter outputs cannot silently become a second editable authority.
- Every Projection identifies its source fingerprint, projection definition, target Resolution, vocabulary profile, and lossiness. A renderer-derived view does not prematurely claim this Resolution-aware provenance contract.
- Expansion may reveal or deterministically derive existing truth. It may not invent canonical detail.

HCM-3.1 lands the bounded vocabulary owner in `handbook-engine` and applies its
resolved presentation only in the existing fixed Stage-10 Work Specification
renderer in `handbook-pipeline`. The renderer preserves canonical YAML and
stable role IDs, emits every declared absorption edge, and remains a
renderer-derived view without Context Resolution or capitalized Projection
authority. Other renderers, adapters, and product surfaces remain later work.

## Charter intake and project-posture kernel

The historical Charter questionnaire becomes a versioned `CharterIntakeDefinition`, not a restored question-by-question CLI wizard.

- it defines coverage, conditional branches, inferable versus user-declared fields, evidence expectations, specificity/completeness rules, approval requirements, and mapping into Charter semantics;
- guided-adaptive, express, and agent-assisted acquisition modes all produce the same Charter candidate schema;
- the external LLM agent using the Handbook skill asks the questions and calls stable generic CLI/SDK operations;
- Handbook validates candidate completeness and provenance, then promotes only through an explicit approval boundary;
- canonical Charter YAML is authoritative; before Phase 3, Markdown and other fixed human-review outputs are renderer-derived views, while Phase-3 Resolution-aware GUI, packet, and agent-context outputs are Projections;
- an immutable intake record explains how a candidate was produced but never becomes a competing Charter authority.

The `ProjectPostureKernel` is a fingerprinted resolved view, not another independently editable document. It is led by canonical Charter policy and may incorporate approved domain overrides, applicable project conditions, contracts, and current evidence. Snapshot deltas may trigger a typed `PostureRecommendation` under an approved threshold/window/cooldown/notification policy, but only an authorized `PostureTransition` may change canonical policy. Raising posture may react to one hard trigger; lowering posture requires sustained evidence and may never cross approved floors or red lines. Adapters may deliver notifications but do not own recommendation or approval semantics.

## Snapshot Memory posture

Snapshot Memory is an immutable, deterministic, provenance-bearing observation of selected repository, artifact, workflow, contract, evidence, and session state at a declared point in time and Context Resolution.

It is a memory record class, not an additional Resolution horizon. Strategic, coordination, execution, and operation horizons may each have snapshots.

The target supports:

- policy-driven capture at session, work-item, artifact, contract, gate, commit, merge, publish, blocker, and escalation boundaries;
- stable normalized state and record fingerprints;
- consistency classification when state changes during capture;
- deterministic snapshot-to-snapshot deltas;
- planned-versus-actual and expected-versus-observed drift signals;
- Resolution-aware projection so an agent receives only the snapshot fields appropriate to its envelope;
- references from handoffs, dispatches, evidence, and gates without duplicating snapshot contents.

Snapshots are descriptive evidence, not contract or artifact authority. A previous end snapshot accelerates onboarding, but a new session must capture or verify current state before acting on it.

## Synthesis boundary

The initial projection engine is deterministic.

If Handbook later supports AI synthesis directly:

- it must use the `unified-agent-api` crates programmatically;
- it must be isolated from the canonical engine through an optional adapter/use case;
- it must emit provenance-bearing candidate output;
- it must never auto-promote synthesized content into canonical truth;
- review/lock gates must control promotion.

Substrate may instead remain the only synthesis owner. The core Handbook crates must not depend on an agent runtime for deterministic projection.

## Tauri and API posture

- Rust DTOs and JSON Schema are the primary transport-neutral contracts.
- The reviewed Rust DTO and its checked-in generated Draft 2020-12 JSON Schema form one fingerprint-bound public contract; neither may drift alone.
- Tauri commands are thin adapters around SDK use cases.
- OpenAPI is generated only when an HTTP boundary exists; it is not the canonical authority for CLI/Tauri payloads.
- Human CLI wording is not an API schema.
- Expected blocked/refused outcomes are structured results, not unparseable stderr prose.
- Stable operation IDs are independent of CLI command paths, Tauri command names, profile vocabulary, and custom artifact IDs.

## Dock posture

Handbook defines one semantic dock protocol and keeps semantic authority separate from process execution.

- `handbook-contracts` owns contract, claim, canonical evidence, verdict, gate, and protocol-neutral dock DTO semantics; a separable future process adapter owns spawn, isolation, resource enforcement, and cleanup only.
- One-shot process JSON is the first universal transport for isolation and language neutrality. A future Rust-native dock must preserve the same request identity, evidence-candidate shape, canonical admission rules, verdict vocabulary, gate behavior, and authority boundary rather than creating a privileged semantic path.
- A dock manifest is an immutable exact identity that binds the manifest fingerprint, a content-addressed implementation bundle, normalized bundle manifest, entrypoint digest, exact typed launch vector/fingerprint, and a normalized recomputable runtime/dependency-closure descriptor. A bundled-interpreter launch binds interpreter, application, standard-library/package bytes, exact bundle-only import resolution, and argument order. Host selection maps that identity to verified local bytes; it does not confer authority.
- Capability declarations are ceilings. The host intersects the request with manifest and policy limits, re-verifies the complete bundle closure before body access or spawn, and denies network unconditionally for every v1 process dock.
- Validators emit untrusted evidence candidates and diagnostics only. The Handbook membrane validates one candidate into one immutable canonical evidence record; validators, runners, transports, and adapters cannot emit canonical verdicts, gates, lifecycle transitions, `blocked`, or `not_applicable` truth.
- Evidence can satisfy only the exact contract/claim/subject/case tuple it observed, within the dimension-by-dimension effective Resolution envelope and freshness/source bindings. Omitted, stale, insufficient-Resolution, refused, failed, or malformed observations cannot produce green truth.
- Timeout uses host monotonic time; cancellation is host-owned and idempotent; refusal, crash, protocol error, output violation, or cleanup uncertainty fails closed and yields no partial evidence.
- `handbook.dock.json-schema@1.0.0` is the bounded first future proof target for one exact local Draft 2020-12 schema closure. The selection is design input for HCM-5.4, not a running dock or proof that `PG-DOCK-01` passed.

## Non-negotiable invariants

1. **One contract authority** — Handbook owns contract meaning; consumers orchestrate it.
2. **Canonical structured truth** — no permanent Markdown/YAML dual authority.
3. **Stable semantics beneath custom language** — user vocabulary may rename or conflate roles without deleting machine meaning.
4. **Resolution is explicit** — context size alone is not a Resolution contract.
5. **Omission limits proof** — a projection cannot pass claims it did not expose or observe.
6. **Thin transports** — CLI, Tauri, HTTP, and Substrate adapters do not reimplement domain decisions.
7. **Greenfield directness** — no user migration framework or legacy compatibility tax.
8. **Typed JSON everywhere** — every nontrivial operation has a versioned machine contract.
9. **External validators remain witnesses** — docks do not become peer truth systems.
10. **Published means consumable** — downstream-intended Rust APIs are complete only after exact crates.io real-seam proof.
11. **CLI bridge is transitional by design** — it has an explicit replacement gate.
12. **Escalation is durable** — genuinely blocked or broadened top-level work is recorded for resume; it is not carried only in chat history.
13. **Snapshots are immutable observations** — Snapshot Memory records what was observed, not what must be true or what should happen next.
14. **Snapshot access is resolution-aware** — comprehensive capture does not imply comprehensive disclosure to every agent.
15. **Snapshot drift is explained, not merely scored** — authorized plan changes and escalations remain distinguishable from unexplained divergence.
16. **Kinds and instances are distinct** — reusable schema/behavior definitions do not carry repository-specific path or requiredness state.
17. **Intake modes converge on one schema** — express or inferred acquisition cannot create a structurally weaker class of canonical artifact.
18. **Inference is not authority** — agent-derived Charter candidates expose evidence, confidence, gaps, and approval needs before promotion.
19. **Posture recommendations do not self-enact** — observed drift can recommend a transition but cannot rewrite the Charter or approved policy.
20. **CLI operations remain stable** — profile vocabulary and custom artifact kinds never generate or rename commands.
21. **Development orchestration remains parent-owned** — delegable implementation, review, proof, and remediation rounds run as built-in subagents inside one active top-level slice loop; internal agents do not create global continuation records.
22. **Contract lifecycle is not evaluation state** — `draft`, `review_ready`, `locked`, `active`, `deprecated`, and `closed` describe immutable definition authority; the definition fingerprint binds its author/admission authority, transitions check that authority and chain exact non-circular fingerprints, and each evaluation/evidence/verdict/gate identity separately binds and revalidates the current active transition plus its immediately prior independent-lock basis.
23. **Dock identity closes over executable bytes, invocation, and dependency resolution** — manifest selection binds the implementation bundle, normalized file manifest, entrypoint digest, typed executable/interpreter/application launch vector and argument order, and a canonical bundle-only runtime-closure member/edge/policy descriptor before any process may run.
24. **Host selection confers no semantic authority** — an allowlisted executable remains a witness, and only the membrane may admit canonical evidence or compute verdict/gate truth.
25. **Process failures cannot leak partial proof** — every admitted run yields one total outcome-discriminated operational record; valid completion atomically retains bounded fingerprinted normalized request/result admission bases plus its untrusted candidate bundle for crash-resumable membrane replay without ephemeral state, while every non-completed host outcome, result/candidate identity or claim validation failure, or cleanup uncertainty yields no exposed candidate or canonical evidence.

## Explicit non-goals

- dynamic CLI command renaming from vocabulary profiles;
- generated CLI command families for repository-defined artifact kinds;
- treating illustrative artifact names or examples as the authority for the approved HCM-0.6 shipped default set;
- a hidden or prompt-wrapped model call inside Handbook's deterministic intake/authoring kernel;
- automatic mutation of the Charter or posture policy from Snapshot Memory or recommendations;
- a universal validator reimplementation;
- waiver semantics hidden in extensions, scores, manifests, adapters, or dock assertions;
- ambient interpreters, package-manager installation, shell/PATH discovery, mutable runtime references, or v1 dock network access;
- treating the HCM-0.5 documentation subject, first-dock selection, or an allowlisted bundle as runtime proof;
- arbitrary graph-shaped Resolution topology in the first version;
- model-generated canonical projections in the first version;
- raw command logs, secrets, credential material, or unrestricted full diffs embedded in Snapshot Memory;
- a remote organization-profile registry in the first version;
- a third-party adapter marketplace in the first program;
- user migration tooling for legacy Handbook formats;
- preserving obsolete hard-coded behavior without a target-architecture reason.
