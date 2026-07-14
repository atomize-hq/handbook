# Handbook Contract Membrane Architecture Memo

## Status

Architecture memo capturing the parent-session decision and explanation for how handbook should work once the contract shim / contract membrane is added.

This memo remains the concise architecture lineage. The active program control surface is now [`docs/specs/handbook-contract-membrane/00-README.md`](../specs/handbook-contract-membrane/00-README.md), which expands this direction into target architecture, semantic contracts, sequencing, proof gates, orchestration, and durable handoff rules.

Treat this memo as design input and the control pack as the current program authority. Slice-local implementation authority exists only when an approved packet supplies `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md`.

## Executive summary

The intended architecture is:

- **handbook is the canonical contract membrane**
- **Substrate code-intelligence remains part of Substrate, not handbook**
- **Substrate code-intelligence consumes handbook’s contract system for orchestration and gating, but does not become a second peer contract authority**

In practice, that means handbook owns the locked truth and verification semantics, while Substrate owns how those results are used inside Substrate workflows.

A second important principle is scope control:

- handbook should **not** try to become a new universal validator
- handbook should instead own the contract system, evidence model, and runner semantics that unify existing validation ecosystems

A third principle is structured, configurable, resolution-aware truth:

- semantically meaningful artifacts move from Markdown-first authority to canonical structured YAML
- reusable `ArtifactKindDefinition` records own versioned canonical schemas, optional intake coverage, lifecycle, validation, and projections; profile-selected `ArtifactInstanceDescriptor` records own repository path, label, requiredness, and dependency state
- a shipped opinionated instance profile defines the default artifact instances, vocabulary, and Resolution stack only after a dedicated research pass and user brainstorming/decision session approves the default set
- repositories may define their own schema-backed artifact kinds and instances, requiredness, vocabulary, and intentional workflow-role conflations without Rust enum variants or dynamic CLI commands
- Markdown, packets, CLI text, GUI views, OpenAPI, and external workflow formats are projections or adapter outputs
- projections are selected by an explicit namespaced Context Resolution envelope rather than only a token/byte budget
- initial projection behavior is deterministic reveal/derive; model synthesis is candidate-only and not part of canonical projection

Handbook is greenfield for this architecture. It should not build user-facing migration tooling or permanent dual-read compatibility for prior Markdown-first formats. Any temporary internal bridge requires a bounded architectural purpose and an explicit deletion gate.

## Product and consumer surfaces

The intended capability boundary is transport-neutral:

- a purpose-named `handbook-sdk` facade should expose ordinary consumer use cases while narrower owner crates remain public for advanced use
- the CLI remains a thin, polished product adapter with complete versioned `--json` support
- a future Tauri application calls the same typed non-CLI use cases
- Substrate may initially bundle and invoke the Handbook CLI through its JSON protocol
- the permanent Substrate posture remains direct consumption of exact published Handbook crate versions from crates.io

The initial CLI bridge counts as a supported integration milestone. It does not satisfy the downstream Rust public-API completion gate.

## Resolution and durable orchestration

Context Resolution is a first-class semantic model with separate scope, detail, temporal, authority, memory, and validation horizons.

It controls:

- what canonical truth is projected into a run
- which broader constraints are inherited
- what the run may mutate
- where observations may be persisted
- what completion may be claimed
- when the work must escalate to a broader orchestration/design context

Every program session writes a durable handoff record. Blockers, newly discovered decomposition, cross-document repairs, and broader-resolution decisions return through an orchestration session that updates authority and emits the next bounded dispatch prompt. Long chat closeout reports are not the durable program ledger.

Snapshot Memory complements that transition record with immutable, deterministic observations of repository, artifact, work, contract, evidence, and session state at strategic hooks such as session start/end and artifact/contract/gate transitions. Handoffs reference snapshots and deltas rather than duplicating them. New sessions compare the prior end snapshot with a fresh start snapshot, then receive only the grounding projection allowed by their Context Resolution envelope.

The historical Charter questionnaire becomes a versioned `CharterIntakeDefinition` used by the Handbook skill's external LLM agent. Guided-adaptive, express, and agent-assisted acquisition all target one Charter candidate schema; Handbook owns coverage/evaluation/promotion while the agent asks questions and invokes stable CLI/SDK operations. Approved canonical Charter YAML produces Markdown and other projections. A deterministic resolved project-posture kernel may combine that Charter with approved overrides and current conditions, while Snapshot-driven posture recommendations remain advisory until an authorized transition changes canonical policy.

## Architecture decision statement

Once the contract shim / contract membrane lands, **handbook should own the canonical contract system**:

- contract truth
- claims / invariants
- evidence model
- dock protocol
- validator / verdict semantics
- canonical artifact representation

Substrate code-intelligence should then **consume** that system to drive orchestration, evidence sequencing, and gating inside Substrate, without defining a second competing contract authority.

## Owner split

### Handbook owns

- the canonical contract truth
- contract lifecycle and lock state
- claims and invariants
- the evidence model
- the dock protocol
- validator behavior and verdict semantics
- canonical artifact representation
- normalized verification output

### Handbook may also own a separable first-party dock / adapter layer

Concrete docks and adapters do **not** all need to live inside handbook core itself.

But they should still belong to the **handbook contract system**, not a parallel authority.

That layer can reasonably hold:

- concrete surface docks
- normalization adapters
- engine-specific evidence collection glue

### Substrate code-intelligence owns

- Substrate-side orchestration
- when and where evidence is collected in Substrate flows
- sequencing dock runs
- turning handbook verdicts into Substrate gates
- Substrate-specific reporting and operator behavior

## Contract lifecycle

The membrane is not only about “run verification against a schema.” It also carries governance and closeout state.

The original lifecycle shape is:

- `draft`
- `review_ready`
- `locked`
- `active`
- `passed`
- `blocked`
- `closed`
- `deprecated`

### Why lifecycle matters

Without lifecycle, the membrane can sound like only:

- contract exists
- evidence is checked
- a gate result appears

But the intended system is broader:

- a contract is authored
- reviewed until authoritative
- locked before execution truth is judged against it
- evaluated while active
- used to decide whether work is blocked, accepted, closed, or superseded

This is part of why handbook should own contract authority rather than only validation helpers.

## Verification mental model

The intended mental model is:

1. **contract** = source of truth
2. **dock** = collect witness / evidence
3. **adapter or engine** = check witness against claims
4. **runner** = aggregate verdict and apply gate

### End-to-end shape

```text
locked contract
  -> dock collects witness evidence
  -> adapter/engine normalizes and checks evidence
  -> handbook evaluates claims/invariants
  -> verdict is produced
  -> runner aggregates verdicts
  -> gate decision is applied
```

### Important rule

- code is not the source of truth
- tests are not the source of truth
- docs are not the source of truth
- traces are not the source of truth

They are all **witnesses** evaluated against the locked contract.

## Severity, scoring, and hard-gate semantics

The verification model should stay richer than a simple binary pass/fail.

### Verdict vocabulary

Useful verdict states include:

- `pass`
- `fail`
- `blocked`
- `warning`
- `not_observed`
- `not_applicable`
- `flaky`

### Gate rule

Scores are useful for progress tracking, prioritization, and agent steering, but they must not override hard failures.

The important rule is:

> A weighted score is helpful, but a hard-fail claim still blocks. A `7/10` score is not green.

That matters for Substrate consumption because code-intelligence needs to distinguish:

- hard contract failure
- missing witness
- warning-level degradation
- flaky or inconsistent evidence

not just “verification succeeded” versus “verification failed.”

## Use existing validation ecosystems; do not build a universal validator

A core guardrail from the original draft is:

- do **not** build a new universal validator from scratch
- do build the orchestration and contract layer that unifies existing contract ecosystems

That means handbook should own:

- contract truth
- claim semantics
- dock protocol
- evidence normalization
- verdict and gate logic

But specialized ecosystems should still do specialized validation work where appropriate, such as:

- OpenAPI / AsyncAPI validation
- UI interaction or accessibility validation
- test frameworks
- schema validators
- drift checkers
- policy engines

This keeps the membrane ambitious in semantics without making handbook responsible for re-implementing every validator category itself.

## Dock taxonomy

The dock model should be understood as a general witness framework, not just a docs/test wrapper.

Useful dock families include:

- implementation dock
- test dock
- trace dock
- API docs dock
- CLI dock
- UI dock
- DB dock
- policy dock
- agent-output dock

This helps keep the architecture clean:

- handbook core defines protocol and truth semantics
- docks gather witness evidence from concrete surfaces
- adapters normalize surface output into the evidence model
- runners aggregate the resulting verdicts into gate outcomes

## How docs, API, UI, test, and trace docks fit

All of these surfaces fit the same model: they emit evidence, and handbook verifies that evidence against contract claims.

### Docs dock

Docs are **witnesses, not authorities**.

A docs dock should:

- collect documentation evidence
- normalize it into the contract evidence model
- let handbook verify whether the docs match contract claims

That same framing applies to adapter targets more broadly:

- OpenAPI is not the source of truth
- AsyncAPI is not the source of truth
- Storybook is not the source of truth
- generated docs are not the source of truth

Docs must not become a second truth system.

### API dock

An API dock can emit evidence such as:

- schemas
- request/response shapes
- generated client or spec output
- observed API transcripts

Handbook then checks that evidence against the contract’s claims and invariants.

OpenAPI or AsyncAPI outputs are best treated as:

- adapter outputs
- drift-check inputs
- interoperability artifacts

not as the canonical contract authority.

### UI dock

A UI dock can emit evidence such as:

- component states
- accessibility evidence
- visual or interaction evidence

That evidence is still witness material evaluated against the contract.

### Test dock

A test dock can emit evidence such as:

- test results
- assertion outputs
- coverage or execution traces
- captured artifacts

The point is not “tests passed,” but “tests witnessed the required contract claims.”

### Trace dock

A trace dock can emit evidence such as:

- runtime traces
- side-effect logs
- tool execution records
- event or decision logs

This gives handbook a way to compare observed behavior to required invariants.

## Planning-pack / seam-closeout integration

The membrane is not only an abstract verification model. It should feed real closeout decisions.

In the original direction, verification results are meant to support decisions like:

- can a seam close?
- is a slice accepted?
- is a packet blocked?
- what remediation should happen next?

That means handbook should be able to produce outputs that Substrate code-intelligence can consume for:

- closeout decisions
- blocked vs accepted execution state
- gate reporting
- agent-readable remediation guidance

This is the main place where handbook’s contract authority meets Substrate’s orchestration behavior.

## Structured evidence and verdict outputs

Evidence and verdicts should be treated as first-class artifacts, not just transient console summaries.

The membrane should support durable machine-readable outputs for at least:

- contract records
- normalized evidence artifacts
- verdict artifacts
- gate results
- remediation-oriented reports

That matters especially if the CLI grows a true `contract` surface, because commands like:

- `handbook contract verify`
- `handbook contract status`
- `handbook contract report`
- `handbook contract gate`

need machine-readable outputs, not only human-readable summaries.

## Downstream public API proof gate

If handbook continues to develop as a standalone CLI while preserving real Substrate integration, then downstream-facing API work needs a stricter completion gate than local handbook success alone.

### Core rule

Any feature-level handbook item that introduces or changes a **downstream-intended public API** is **not done** until:

1. the relevant handbook crates are published to **crates.io**
2. a dedicated temporary Substrate worktree is created from current Substrate tip
3. that worktree imports the exact new handbook crate version(s) from **crates.io**
4. Substrate uses the new API in a **real seam**
5. the proof seam passes its verification wall

This is stronger than:

- proving the API works inside the handbook workspace
- proving the symbol is public
- proving path dependencies compile locally

### Why this gate exists

The point is to prove all of the following at once:

- the new public API is actually publishable
- the published crate surface is actually sufficient
- Substrate can really consume the API under the intended distribution model

That keeps the standalone handbook CLI honest while preserving the long-term integration contract through published crates.

### What does not count

The downstream proof should **not** rely on:

- sibling-path dependency shortcuts
- workspace-only coupling
- unpublished internal-only imports
- toy binaries that do not exercise a real Substrate seam

The intended proof model is:

- publish first
- consume from crates.io second
- verify in a real temporary Substrate worktree third

### Scope of the rule

This gate should apply to **downstream-intended public API changes**.

It does **not** need to apply to handbook-internal-only changes that do not claim reusable downstream surface area.

## What the future handbook CLI likely looks like

If handbook owns the membrane, the CLI should expose a **first-class `contract` surface** rather than hiding everything under `pipeline`, `inspect`, or `doctor`.

Likely command family:

```text
handbook contract list
handbook contract show
handbook contract lint
handbook contract lock
handbook contract status
handbook contract verify
handbook contract report
handbook contract dock list
handbook contract dock run
handbook contract evidence list
handbook contract gate
```

### Conceptual split

- `doctor` = baseline health / recovery / unblock
- `inspect` = proof and packet evidence review
- `contract` = executable conformance, witness evaluation, and gate result

That makes the contract membrane visible as a real product surface.

## What handbook is not responsible for

To keep boundaries honest, handbook is **not** responsible for:

- owning Substrate code-intelligence orchestration
- becoming the home of all Substrate workflow logic
- creating a second authority inside Substrate
- treating docs, tests, code, or traces as peer truth sources
- burying contract semantics inside generic pipeline plumbing
- forcing every concrete dock implementation into handbook core

Handbook owns the **meaning** of contract verification.

Substrate owns **how Substrate uses that meaning**.

## Biggest architectural mistakes to avoid

1. **Creating two authorities**
   - handbook and Substrate must not both define contract truth

2. **Letting docs become truth**
   - docs are witnesses and should verify against the contract, not replace it

3. **Collapsing orchestration into authority**
   - running gates is not the same as owning contract semantics

4. **Hiding the membrane under unrelated CLI surfaces**
   - if the membrane is real, `handbook contract ...` should be explicit

5. **Forcing all docks into handbook core**
   - handbook core should own protocol and semantics; concrete docks can be separable

6. **Allowing surface-specific drift**
   - API, UI, docs, tests, and traces must normalize back to one contract model

7. **Letting code-intelligence become a peer contract system**
   - it should consume handbook’s membrane, not compete with it

8. **Trying to make handbook a universal validator**
   - handbook should unify validator ecosystems, not replace all of them

9. **Losing lifecycle and closeout semantics**
   - the membrane is about reviewed governance and execution closeout, not only raw validation

10. **Calling a downstream API complete before crates.io consumer proof exists**
   - downstream-facing handbook APIs should not be considered done until Substrate consumes them from crates.io in a real temp worktree seam

## Source context used for this memo

- parent-session architecture discussion summarized in local memory
- original concept source:
  - `/Users/spensermcconnell/Downloads/substrate_executable_contracts_architecture.md`
- highest-value original draft sections:
  - `0. Executive recommendation`
  - `2. Vocabulary`
  - `3. Contract lifecycle`
  - `5. Recommended contract model`
  - `6. Dock architecture`
  - `7. Possible architecture shapes`
- Substrate code-intelligence lineage context:
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-contracts-and-gates.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-program.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-workstream-orchestration.md`
- reinforcing live repo context:
  - `/Users/spensermcconnell/__Active_Code/system/README.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/SUPPORTED_COMMANDS.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/CLI_PRODUCT_VOCABULARY.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
  - `/Users/spensermcconnell/__Active_Code/system/crates/cli/src/main.rs`
