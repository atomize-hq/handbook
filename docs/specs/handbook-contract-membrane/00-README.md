# Handbook Contract Membrane Control Pack

**Status:** draft active control pack; docs and architecture only  
**Scope:** target architecture, artifact-kind/schema registry, adaptive intake, Charter authority, posture kernel, sequencing, Context Resolution, Snapshot Memory, crate ownership, SDK/use-case and machine-transport boundaries, context assembly, handoff, escalation, and proof gates

**Implementation authorization:** none; implementation begins only from an approved slice-local packet  
**Repo-truth snapshot:** 2026-07-14; re-check live code before every slice

## Purpose

This directory is the durable context-engineering control surface for the Handbook contract-membrane program.

It exists because the work spans canonical artifact representation, configurable artifact kinds and instances, custom schemas, adaptive agent-directed intake, Charter authority, a resolved project-posture kernel, user vocabulary, Context Resolution, deterministic Snapshot Memory, projections, a public SDK facade, CLI/Tauri transports, Substrate consumption, contract lifecycle, and external validator docks. Loading all source discussions, archived plans, and implementation files into every session would create context drift rather than context quality.

The pack separates:

1. **target truth** — what the architecture must become;
2. **repo truth** — what the live implementation does now;
3. **slice truth** — what one bounded packet is allowed to change;
4. **proof truth** — what evidence is required before a seam may be called landed.

An artifact, type, command, test, or published crate is not by itself proof that the target seam has landed.

## Authority stack

Use this order and keep the kinds of authority distinct:

1. **Approved slice-local packet** under `slices/<slice-id>/`
   - exact implementation authority for that slice;
   - `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` define its boundary.
2. **This control pack**
   - target architecture, semantic contracts, program sequencing, escalation rules, and proof gates.
3. **Live code and tests**
   - current implementation truth; they do not override the pack's approved target architecture merely because old behavior exists.
4. **Active idea memos** under `docs/ideas/`
   - concept lineage and design input.
5. **Archived docs** under `archived/`
   - provenance only unless a pack section explicitly revives one bounded rule.
6. **Conversation history, prior summaries, and handoff prose**
   - discovery hints until revalidated against the pack and live tree.

If target docs and live code conflict, record both truths explicitly. Do not silently select whichever makes the current task easiest.

## Greenfield rule

Handbook is greenfield for this architecture:

- do not build user-facing legacy migration tooling;
- do not promise compatibility with previous Markdown-first artifact layouts;
- do not keep permanent dual-readable or dual-writable truth;
- do not preserve a hard-coded behavior merely because it exists today;
- define the desired shipped default profile directly.

A temporary internal bridge is allowed only when it has a concrete architectural purpose in a bounded cutover, is not presented as a public compatibility promise, and has an explicit deletion gate in `04-phase-slice-map.md` and `06-proof-and-regression-ledger.md`.

## Control-pack map

| File | Load when | Canonical content |
|---|---|---|
| [`01-target-architecture.md`](01-target-architecture.md) | deciding ownership, crate boundaries, artifact/intake/posture ownership, memory posture, or transport posture | target layers, owner map, artifact kinds, adaptive intake, posture kernel, Snapshot Memory, CLI/SDK/Tauri/Substrate/dock boundaries, non-negotiable invariants |
| [`02-semantic-model.md`](02-semantic-model.md) | changing artifact kinds/instances, schemas, intake, Charter, posture, profiles, vocabulary, Resolution, memory, snapshots, or projections | separate stable-role and semantic-capability registries, artifact-kind and intake contracts, Charter/posture semantics, configurable terminology, memory record classes, snapshots/deltas, resolution envelope, projection and promotion semantics |
| [`03-seam-crosswalk.md`](03-seam-crosswalk.md) | scoping a slice or checking whether a named capability really exists | current artifacts, current classification, target owner, required action, sibling dependencies |
| [`04-phase-slice-map.md`](04-phase-slice-map.md) | planning, decomposing, executing, or escalating work | phase order, slice boundaries, child-packet rules, non-goals, exit gates |
| [`05-contracts-schemas-and-gates.md`](05-contracts-schemas-and-gates.md) | defining artifact-kind/intake/posture types, YAML/JSON schemas, CLI output, SDK surfaces, or docks | preliminary contract shapes, schema policy, intake/approval semantics, lifecycle, evidence/verdict/gate rules |
| [`06-proof-and-regression-ledger.md`](06-proof-and-regression-ledger.md) | reviewing, validating, closing, or preserving known behavior | current proof tiers, open proof gaps, permanent guard rails, cutover-deletion gates |
| [`07-orchestration-onboarding-prompt.md`](07-orchestration-onboarding-prompt.md) | starting or resuming a top-level phase/slice orchestration run | long-lived slice runner, selective onboarding, built-in delegation, review/remediation/re-review, proof, commit, and true-stop closeout |
| [`08-handoff-ledger-and-escalation-protocol.md`](08-handoff-ledger-and-escalation-protocol.md) | stopping top-level orchestration, reporting a genuine external/authority boundary, or resuming later | parent-owned durable records, internal delegated-run results, status/stop model, ledger validation, and short chat closeout |

## Semantic landing labels

Use these labels for seams, not for isolated files or functions:

| Label | Meaning |
|---|---|
| `TargetOnly` | Approved target semantics exist, but no meaningful implementation seam exists. |
| `UsefulPrecursor` | Reusable implementation exists but does not yet own or enforce the target semantics. |
| `BoundaryLanded` | The correct owner and typed boundary exist, but real-path adoption or runtime proof is incomplete. |
| `RealPathAdopted` | A real product path uses the correct boundary, but the complete proof wall is not closed. |
| `ContractCorrectAndProven` | Correct owner, semantics, real path, enforcement point, and required proof all exist. |
| `Superseded` | A prior model remains only as bounded provenance or temporary cutover scaffolding. |

Do not promote a seam because similarly named code exists. A seam is complete only at `ContractCorrectAndProven`.

## Per-slice context assembly protocol

Context assembly is part of every implementation, review, documentation-repair, and proof slice.

Assemble three bounded packets:

1. **Authority packet — what must be true**
   - exact `04` slice row;
   - affected `03` seam rows;
   - applicable `01` invariants and `02` semantics;
   - exact `05` contract sections;
   - current slice-local `SPEC.md` and task ledgers.
   - approved artifact-kind/default-set decisions when the slice touches setup, authoring, or projections; examples in this pack are not approval of the shipped default set.
2. **Repo-truth packet — what is true now**
   - current production path;
   - most recent applicable end snapshot, new start snapshot, and deterministic delta when Snapshot Memory is available;
   - allowed source/test files;
   - related public types;
   - one relevant precedent;
   - fresh impact/call-path evidence when symbols will change.
3. **Proof packet — how completion is judged**
   - exact `06` gate rows;
   - targeted tests;
   - negative/fail-closed cases;
   - required CLI, downstream, dock, or runtime evidence;
   - the single classification change the evidence may support.

Target fewer than 2,000 focused lines per implementation or review task. Load sections, not entire archives.

Use this capsule at slice start:

```text
SLICE / OBJECTIVE:
PARENT ORCHESTRATION ID:
SELECTED PHASE / SLICE / PACKET:
ACTIVE RESOLUTION ENVELOPE:
GROUNDING SNAPSHOT / START DELTA:
TARGET AUTHORITY BOUNDARY:
CURRENT REPO-TRUTH STATUS:
MUST-READ PACK SECTIONS:
LIVE SOURCE / TESTS / PRECEDENT:
SIBLING SEAMS IN CONTEXT:
ALLOWED AREAS / EXPLICIT NON-GOALS:
APPLICABLE CONTRACTS / PROOF GATES:
BUILT-IN DELEGATION CAPABILITY / ACTIVE RUNS:
REVIEW ROUND / SUBJECT FINGERPRINT:
KNOWN CORRECTIONS OR CONFLICTS:
HANDOFF RECORD TO RESUME, IF ANY:
EXIT PROOF / STOP CONDITIONS:
```

## Top-level orchestration and closeout rule

The user starts one top-level phase/slice orchestration run. That parent remains responsible for the selected slice through context assembly, specification and planning, implementation or documentation, verification, fresh independent review, remediation, fresh re-review, proof-wall closeout, and commit.

Internal implementation, documentation, proof, remediation, and review subagents return structured results to the parent through the built-in subagent channel. They do not write canonical handoff records, append the global ledger, claim slice completion, or require the user to start another task.

A durable handoff record under `handoffs/records/` is written by the top-level orchestrator only when:

- the selected slice or explicitly authorized top-level objective is complete and review-clean;
- human-visible or interactive validation is required;
- an external blocker prevents further progress;
- broader scope, Resolution, or decision authority is required;
- the current top-level context/runtime boundary requires a later resume; or
- mandatory built-in delegation is unavailable.

Writing an internal dispatch is not a stopping condition. A dispatch is an immutable execution/audit envelope that the parent normally executes immediately with a fresh built-in subagent.

Once Snapshot Memory is implemented, the handoff references the session-start snapshot, session-end snapshot, and their delta. A handoff is the normative transition record; the snapshot is the descriptive observation of what was true. Neither replaces the other.

When artifact/intake/posture semantics are involved, the handoff also references the exact kind/instance, intake/candidate/canonical, posture, and shipped-default decision records. References preserve authority boundaries; they do not make an intake record canonical or a posture recommendation self-enacting.

The parent-owned record must distinguish:

- completed work;
- partial work safe to resume;
- a local blocker;
- an escalation requiring broader resolution or authority;
- documentation/control-pack repair;
- further decomposition discovered during execution;
- required review or proof follow-up that could not be completed internally;
- delegated runs, their built-in agent identities/statuses, and review verdicts;
- the exact reason top-level orchestration stopped.

The chat response should then be short: status, handoff ID/path, and one `jq` command. Do not paste the full handoff report into chat when the durable record exists.

## Orchestration loop

```text
user starts top-level orchestrator with explicit phase/slice
  -> select optional resume handoff for that slice
  -> capture/revalidate current state and assemble bounded context
  -> specify/plan/repair the active packet
  -> parent executes work or immediately delegates an internal dispatch
  -> parent verifies and waits for a fresh built-in review subagent
  -> valid findings: parent remediates or delegates a fresh fix subagent
  -> another fresh reviewer checks the remediated state
  -> repeat until clean or genuinely blocked
  -> run proof wall, update control-pack truth, and commit reviewed slice state
  -> write one parent-owned durable handoff only when orchestration stops
  -> validate and commit the mechanical handoff/ledger closeout separately
```

Escalation is a normal resolution transition, not a failure. Silent scope widening and user-mediated hopping between otherwise delegable internal rounds are failures.

## Initial program conclusion

The live repository contains published, reusable owner crates and several valuable precursors, but the artifact-kind/schema registry, researched shipped default set, adaptive intake/Charter promotion system, configurable profile, canonical YAML authority, resolved posture kernel, Context Resolution model, deterministic Snapshot Memory, projection engine, SDK facade, full JSON protocol, contract membrane, and dock protocol are not yet one landed system.

The next authorized work after this pack is reviewed is documentation/design work inside Phase 0. No Rust implementation is authorized by the existence of this pack alone.

In particular, the shipped default artifact set is not yet approved. It requires a dedicated research pass and a user brainstorming/decision session before the default profile or its required artifact instances may be frozen.
