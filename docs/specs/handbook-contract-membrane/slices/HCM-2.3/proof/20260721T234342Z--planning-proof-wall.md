# HCM-2.3 planning proof wall

> **Status (2026-07-21): pre-review documentation-subject evidence.** This
> record does not claim `CLEAN`, authorize implementation, modify HCM-2.2, or
> close the orchestration. Only a later exact-subject review and parent-owned
> planning handoff may record those results.

## Entry identity and selection

The session began in `C:\hcm22ar-doc-repair` on branch
`codex/hcm-2-2-exact-result-authority-repair` at exact clean HEAD
`5c31eeefb5adf71d75ec3059b1b6947025d2fd6b`. Commit
`6766d3ed4894aad6598faaa7c4a54b493f92c1f6` is an ancestor of that HEAD and is
the required HCM-2.2 primary implementation commit. After branch, HEAD,
ancestry, staged, unstaged, and untracked state were verified, the parent
created `codex/hcm-2-3-planning` at the same exact HEAD.

The selected dependency record is
[`../../../handoffs/records/20260721T215654Z--HCM-2-2--orchestration--exact-result-authority-repair-landed.json`](../../../handoffs/records/20260721T215654Z--HCM-2-2--orchestration--exact-result-authority-repair-landed.json).
It establishes completed review-clean HCM-2.2, primary commit `6766d3ed...`,
closeout commit `5c31eeef...`, and final Review 5 subject
`sha256:62c9fdae649a31d1538ec7770b0f8ce5b2cc0686cb33535fe8747bbe2ddc80ad`.
It is dependency and transition context only and grants no HCM-2.3 authority.
No HCM-2.3 packet, handoff selector, or implementation selector existed at
entry. The user's explicit selector authorizes this planning subject only.

## Instruction and skill admission

The parent read the repository `AGENTS.md`, found no nested `AGENTS.md` in the
selected documentation path, and read the required start, control-pack,
orchestration, handoff-protocol, and selected-handoff documents completely.
The repository-local `.agents/skills/using-agent-skills/SKILL.md` named by the
request is absent in this checkout. The parent used the installed equivalent
`agent-using-agent-skills` resolver and recorded that substitution before work.

The planning phases admitted and applied the requested workflows in order:

1. `agent-using-agent-skills` for resolution and definition-of-done control;
2. `agent-context-engineering` for the bounded authority capsule;
3. `agent-source-driven-development` for canonical-pack and live-code evidence;
4. `agent-spec-driven-development` for the implementation-grade contract;
5. `agent-planning-and-task-breakdown` for ordered RED/GREEN increments;
6. `agent-documentation-and-adrs` for the narrow boundary decision;
7. `agent-code-review-and-quality` for the parent five-axis audit.

`agent-git-workflow-and-versioning` remains a later mandatory phase gate before
staging or committing. No implementation workflow has been invoked.

## Bounded slice capsule

| Capsule field | Frozen planning value |
|---|---|
| objective | one repository-defined custom-kind registration, exact schema validation, supplied optional intake, lineage, and stable generic CLI real path |
| dependency | completed HCM-2.2 is immutable evidence; HCM-1.1/HCM-1.3 `registry-brief` is the reusable custom lineage |
| allowed paths | HCM-2.3 packet plus the four specifically stale canonical status documents and review/closeout audit artifacts |
| prohibited paths | Rust, Cargo, runtime tests, production assets, CLI/SDK code, HCM-2.2 authority, preservation locations |
| identity | `example.artifact-kind.registry-brief@1.0.0`, `example.schemas.registry-brief@1.0.0`, instance `registry_brief`, repository intake `example.intake.registry-brief@1.0.0` |
| authority split | kind owns reusable schema; optional intake targets kind; descriptor selects instance/path/intake; IDs remain request data |
| product boundary | engine owner APIs plus stable generic CLI only; no `handbook-sdk`, transport catalog, Tauri, Substrate, publication, or Phase 3+ |
| maximum classification | one atomic two-cell set after later proof: exact registry-brief subset of `Artifact kind/schema registry` and exact registry-brief subset of `Charter intake coverage`, each `TargetOnly` to `RealPathAdopted`; no other cell moves |
| stop | product decision, insecure/cyclic lineage, Phase 4 conflict, unexpected critical expansion, generated/filename/enum/executable/remote/Projection dependency, or inability to reach `CLEAN` |

This capsule was recorded before any documentation edit.

## Source reconstruction

The parent assembled only relevant sections of `01` through `06` and direct
HCM-1.1, HCM-1.3, HCM-2.1, and HCM-2.2 precedents. The resulting contract is
grounded in these live facts:

- the existing HCM-1.3 custom lineage is capability-free, uses stable role
  `project_context`, selects the exact closed `registry-brief` schema, and is
  not a shipped default;
- profile child replacement fields are complete lists rather than append
  merges; source bindings are explicit exact-ref/effect pairs;
- the current selection request has ten definition-source classes and no
  intake-definition source class;
- the current descriptor resolver preserves one exact Charter closure and
  otherwise rejects non-null lifecycle/intake/renderer/Projection/overlay refs;
- the canonical operation table separates pure evaluation, intake append,
  candidate validation, candidate append, approval append, and promotion, and
  gives each mutation its own exact write set; and
- `PG-KIND-01`, `PG-KIND-02`, and `PG-ARTIFACT-01` remain open outside their
  exact proven subsets.

The packet therefore reuses the custom kind/schema/instance, adds a separate
repository intake definition and a new repository profile version, keeps the
kind-to-intake direction acyclic, preserves every shipped byte, and freezes
separate operation receipts and candidate/canonical authority.

## Live GitNexus graph evidence

The GitNexus index initially reported stale and was refreshed with
`npx gitnexus analyze`. Generated instruction-file rewrites from analysis were
mechanically reversed before packet work; the graph remained current and the
worktree retained no such changes. Semantic full-text query was unavailable
because the local DuckDB full-text extension could not load, so the parent used
the current graph's context, Cypher, and upstream impact surfaces rather than
filename inference.

| Symbol | Upstream blast radius | Planning disposition |
|---|---:|---|
| `load_artifact_kind_registry` | CRITICAL; 385 impacted, 1 direct caller, 35 processes, 20 modules | consume unchanged |
| `load_artifact_kind_registry_admitted` | CRITICAL; 357 impacted, 1 direct caller, 35 processes, 20 modules | consume unchanged |
| `SchemaRegistry::load_with_request_budget` | CRITICAL; 7 impacted, 1 direct caller, 30 processes, 20 modules | consume unchanged |
| `ArtifactInstanceRegistry::resolve` | CRITICAL; 427 impacted, 8 direct callers, 37 processes, 20 modules | anticipated one-field intake admission only; fresh impact and warning required before later edit |
| `resolve_profile_selection` | CRITICAL; 458 impacted, 21 direct callers, 31 processes, 20 modules | consume unchanged |
| `ResolvedArtifactRegistry::validate_json` | CRITICAL; 351 impacted, 6 direct callers, 38 processes, 20 modules | consume unchanged |

These CRITICAL results were reported before documentation edits. This planning
session edits no indexed function, class, method, or other Rust symbol. Any
later implementation expansion to another HIGH/CRITICAL edit is a stop for
explicit review.

## Preservation baselines

Both prohibited preservation locations were inventoried read-only before
packet edits.

| Location | Baseline |
|---|---|
| `C:\hcm22ar` | branch `feat/hcm-2-2-atomic-stage-authority-repair`, HEAD `486458acfe8373a977e595a4854ac786166e3e76`, 26,713 files, 30,740,323,544 bytes, metadata fingerprint `bed9acf2982b33635ad77da91b6d273b371c7d7cd472832e86b0c490a234d140` |
| `C:\Users\spmcc\Documents\__Project_Code\_hcm22ar_preservation\486458acfe8373a977e595a4854ac786166e3e76-20260721T121830Z` | 28,210 files, 61,597,363,998 bytes, metadata fingerprint `cc80738ac1eb7a7daa18eecea6c64c527ff4e30fd594dbc4266bbcfb0d9a0b30` |

The source worktree is intentionally dirty preserved evidence; it was not
cleaned, reset, imported, or treated as authority. Both baselines and archive
sentinels must compare at pre-commit and final closeout.

## Pre-review subject and parent audit

The complete remediated planning subject is limited to:

- `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md`;
- one narrow source-selection/Phase-4-boundary decision;
- one prose runtime contract, three planning-only JSON schemas, three normative
  planning-only vector files, this proof wall, review dispatches, and
  remediation evidence; and
- factually stale HCM-2.2 status corrections in `00-README.md`,
  `03-seam-crosswalk.md`, `04-phase-slice-map.md`, and
  `06-proof-and-regression-ledger.md`.

The JSON/schema/vector files are packet-local implementation oracles, not
production assets or shipped definitions. No JSONL, Rust, Cargo, runtime test,
production asset, CLI code, SDK code, or HCM-2.2 implementation-authority file
is added or changed.
The parent audit already corrected two material planning ambiguities: normalized
candidate content is a separate content-addressed object rather than inline
record authority, and intake append/candidate append/promotion remain three
separate operation transactions with their canonical write sets.

The exact review subject will be sorted, raw-byte SHA-256 fingerprinted under
`repo-path-null-sha256-newline-v1`, schema-bound in an immutable dispatch, and
given to a fresh isolated read-only built-in default reviewer. The reviewer
must derive findings independently in Critical, Required, Optional, Nit order.
Every valid finding is repaired without waiver and a different fresh reviewer
must admit the new exact subject before `CLEAN` can be claimed.

## Pre-review verification state

Before dispatch, the parent must still replay link/anchor/fence/reference,
fingerprint, scope, archive, handoff, whitespace, preservation, and GitNexus
change-detection checks. `git diff --check` is clean at this proof-wall entry.
The three handoff validator modes already passed at preflight over 3 schemas,
2 dispatch schemas, 2 templates, 51 records, 256 current dispatches, 8 legacy
dispatches, and 51 ledger entries, and will be rerun against the final subject.

No commit is permitted until the complete documentation subject is `CLEAN` and
unchanged. The reviewed planning commit must precede a parent-only handoff and
ledger closeout commit. The resulting handoff must still state that HCM-2.3
implementation requires a separate explicit user selector.
