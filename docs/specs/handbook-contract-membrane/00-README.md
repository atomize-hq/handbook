# Handbook Contract Membrane Control Pack

**Status:** active control pack; Phase 0 design/default authority is closed; HCM-1.1 through HCM-1.4, the bounded HCM-2.1 Project Context pilot, HCM-2.2 candidate-`1.3` exact-result authority, and the exact HCM-2.3 repository-defined `registry-brief` implementation have landed; HCM-2.3 landed at implementation commit `628b672ef33326e87e4fb30be13489e8af04b38c`, received final complete-subject review at commit `746fff667f6fbe0270182a467285d60394362530`, and binds the exact 111-path subject `sha256:d510e94e5b47db209022927bc4afa74b8820e27cd645123cc1cdc3aa5f72fdef`; only the exact registry-brief subsets of the Artifact kind/schema registry and Charter intake coverage promote to `RealPathAdopted`

**Scope:** target architecture, artifact-kind/schema registry, adaptive intake, Charter authority, posture kernel, sequencing, Context Resolution, Snapshot Memory, crate ownership, SDK/use-case and machine-transport boundaries, contract definitions, claims, evidence, verdicts, gates, dock protocol, context assembly, handoff, escalation, and proof gates

**Implementation authorization:** HCM-1.1 through HCM-2.3 are completed bounded evidence, not continuing authority. HCM-2.3 closes `PG-KIND-02` only for the proven repository-defined registry-brief path, adds the exact earned registry-brief subsets to `PG-KIND-01` and `PG-ARTIFACT-01`, preserves first-party Charter as `ContractCorrectAndProven`, and leaves broader generic/custom-kind intake `TargetOnly`; it adds no shipped kind, SDK or public transport support, Projection engine, remote schema fetching, generated command, broader artifact-family conversion, or later-slice authority. `handbook-engine` `0.2.0` is the accepted compatibility boundary for the landed public-enum additions; it is not a publication or downstream-adoption claim. HCM-2.4 and every later slice remain unauthorized until separately selected.
**Repo-truth snapshot:** 2026-07-25 at `7b3b5454ef363d08e4c6c78e6a201d8c36c10c5c`; re-check live code before every slice

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
| [`05-contracts-schemas-and-gates.md`](05-contracts-schemas-and-gates.md) | defining artifact-kind/intake/posture types, YAML/JSON schemas, CLI output, SDK surfaces, contracts, gates, or docks | frozen HCM-0.2 through HCM-0.4 contracts plus the HCM-0.5 proof/review subject for contract lifecycle, evidence/verdict/gate rules, and process-dock protocol |
| [`06-proof-and-regression-ledger.md`](06-proof-and-regression-ledger.md) | reviewing, validating, closing, or preserving known behavior | current proof tiers, open proof gaps, permanent guard rails, cutover-deletion gates |
| [`07-orchestration-onboarding-prompt.md`](07-orchestration-onboarding-prompt.md) | starting or resuming a top-level phase/slice orchestration run | long-lived slice runner, selective onboarding, built-in delegation, review/remediation/re-review, proof, commit, and true-stop closeout |
| [`08-handoff-ledger-and-escalation-protocol.md`](08-handoff-ledger-and-escalation-protocol.md) | stopping top-level orchestration, reporting a genuine external/authority boundary, or resuming later | parent-owned durable records, internal delegated-run results, status/stop model, ledger validation, and short chat closeout |
| [`09-review-finding-inventory.md`](09-review-finding-inventory.md) | classifying, deferring, comparing, or resolving review findings | P1-P4 blocking/advisory rubric, duplicate comparison, and the single durable review-debt inventory |

Corrective slice [`HCM-0.9`](slices/HCM-0.9/SPEC.md) is abandoned. Terminal Redesign Review 2 was not CLEAN, so no decomposition is authorized and the monolithic [`05-contracts-schemas-and-gates.md`](05-contracts-schemas-and-gates.md) remains canonical. The rejected planning subjects and review dispatches are immutable historical evidence only. Reviving decomposition requires a new explicit human decision, a new plan, and a new review budget; no automatic semantic routing engine is authorized.

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
   - the single classification change the evidence may support;
   - only the `09` finding entries whose affected scope intersects the packet.

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
KNOWN P3/P4 ADVISORIES:
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
  -> parent verifies and executes one fresh review or a bounded same-subject review burst
  -> valid P1/P2 findings: parent consolidates and remediates or delegates a fresh fix subagent
  -> valid unfixed P3/P4 findings: parent schedules exact `09` inventory registration
  -> one different-fresh closure reviewer checks the remediated material delta
  -> clean: continue; remaining P1/P2 at the declared review-budget boundary:
     bounded non-completed stop for explicit scope/priority adjudication
  -> run proof wall, update control-pack truth, and commit reviewed slice state
  -> write one parent-owned durable handoff only when orchestration stops
  -> validate and commit the mechanical handoff/ledger/advisory closeout separately
```

Escalation is a normal resolution transition, not a failure. Silent scope widening and user-mediated hopping between otherwise delegable internal rounds are failures.
The default automatic review budget is one discovery review/burst, one
consolidated remediation, and one delta-focused closure review; extending it
requires explicit authority rather than another automatic loop.

## Initial program conclusion

The live repository contains published, reusable owner crates and several valuable precursors. HCM-0.2 through HCM-0.4 freeze the target semantics for the artifact/profile/intake kernel, Context Resolution/Snapshot/Projection model, and SDK/transport boundary; HCM-0.5 freezes the contract-membrane/dock design; and HCM-0.6 approves the exact six-kind catalog plus three-instance shipped root-profile selection. Authored documentation does not mean the corresponding runtime seams exist: the approved shipped defaults, canonical YAML cutover, runtime semantic kernels, SDK facade, complete JSON transport, contract evaluator, gate engine, dock runner, and first proof dock are not yet one landed system.

The contract membrane and external docks therefore remain `TargetOnly`. `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` remain open, and the HCM-0.5 design freeze authorizes no Rust, schema publication, process execution, CLI, Tauri, Substrate, or SDK implementation.

HCM-1.1 through HCM-1.4 have landed and closed as reviewed dependency boundaries: kind/
schema ownership, exact profile/descriptor selection, the selected-profile
artifact registry, and bounded profile-aware setup/doctor decision/readiness
adoption. HCM-1.4 setup writes no canonical artifact and conditional truth
remains explicitly indeterminate until a separately reviewed evidence/evaluator
contract exists. Its parent v1.2 handoff and deterministic ledger update are
complete.

HCM-2.1 has landed the exact Project Context canonical-YAML pilot. The engine
owns the closed canonical record, selected no-follow read, deterministic
in-memory Markdown renderer, and distinct source/render fingerprints; author,
setup, doctor `1.1.0`, Environment Inventory's reference-only dependency, and
the bounded mixed-family flow path consume that owner without reading or
persisting legacy Project Context Markdown. The implementation changed no
HCM-1 definition/schema/profile/descriptor byte, leaves shipped readiness
conditional and `INDETERMINATE`, closes `PG-YAML-01` only for this one family,
and leaves the temporary flow bridge for deletion no later than HCM-2.4.
HCM-2.2 produced the broad implementation subject for the first-party
constitutional-root cutover. Option 1
repaired the candidate/result identity cycle; `HCM-2.2-ESC-002` then selected
atomic whole-file output staging. Fresh implementation Review 3 rejected the
result under `HCM-2.2-AR3-001`: semantic-equal result, witness, and mutable
sidecar-binding audit bytes could be coherently rewritten because candidate
`1.2` selected only semantic result identity. `HCM-2.2-ESC-003` now selects
candidate `1.3` as the independent downstream anchor containing semantic result
ref/fingerprint and exact persisted result JCS+LF SHA-256/length in final
candidate identity. Candidate-subject identity stays result-independent;
result `1.0` binds only that subject; new approval binds final candidate; and
promotion/intent `1.2` bind it transitively without schema changes. Candidate
`1.0`/`1.1`/`1.2` and approvals remain immutable history with no fallback or
carry-forward. The rejected worktree is preserved untouched at `486458ac` in a
verified external archive. The completed implementation supplies
the candidate `1.3` binding, stable locked replay, orphan refusal, independent
approval/promotion currentness, atomic publication/recovery proof, and selected
product cutover as one proof-complete subject. Fresh exact-result implementation
Review 3 required every current candidate gate to recompute the exact ordered
populated-leaf provenance bijection from retained intake and normalized content;
the author, approval, promotion, recovery, and normative-vector repair is
proof-complete. Fresh Reviews 4 and 5 returned `CLEAN`; the exact Review 5
53-path subject is `sha256:62c9fdae649a31d1538ec7770b0f8ce5b2cc0686cb33535fe8747bbe2ddc80ad`,
the primary implementation is `6766d3ed4894aad6598faaa7c4a54b493f92c1f6`,
and parent closeout is `5c31eeefb5adf71d75ec3059b1b6947025d2fd6b`.
HCM-2.3 then completed the exact repository-defined `registry-brief` path.
Implementation commit `628b672ef33326e87e4fb30be13489e8af04b38c`,
final complete-subject review commit
`746fff667f6fbe0270182a467285d60394362530`, and the exact 111-path subject
`sha256:d510e94e5b47db209022927bc4afa74b8820e27cd645123cc1cdc3aa5f72fdef`
prove the bounded kind/schema, optional supplied-intake, descriptor-selected
canonical validation, immutable lineage, atomic publication, replay, refusal,
restart, concurrency, and actual-binary path. Only the exact registry-brief
kind/schema and intake subsets move to `RealPathAdopted`; broader
generic/custom-kind intake remains `TargetOnly`. `PG-KIND-02` closes only for
that proven path, while `PG-KIND-01` and `PG-ARTIFACT-01` receive only their
exact earned subset evidence. HCM-2.4 and all later slices remain unauthorized
pending their own packet and selector.

The shipped default artifact set is approved in [`slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md`](slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md)
and its exact kind, profile, descriptor, and selected-registry data now exist.
It must not be inferred from current enums, templates, filenames, or
illustrative examples. Setup/doctor now consume the typed selected-profile
decision and structural-inspection closure, but condition evidence/evaluation,
canonical content authority, materialization, intake, semantic approval, and
renderer behavior remain separately gated rather than implied by that bounded
adoption.
