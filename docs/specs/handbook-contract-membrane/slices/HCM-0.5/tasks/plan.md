# Implementation Plan: HCM-0.5 Contract Membrane and Dock Protocol Freeze

## Overview

First complete a bounded documentation-only planning continuation. Repair only HCM-0.5-R4-001 and HCM-0.5-R4-002, replace the exhausted shared review counter with explicit stage-scoped review authority, and obtain a clean Continuation Planning Review before any canonical design work. A clean continuation produces an approved planning commit and a separate partial/context-boundary handoff for a fresh design-freeze session. This session never edits canonical `00`-`06`; runtime contract/dock implementation remains deferred to Phase 5.

## Authority and sequencing decisions

- Use the explicit HCM-0.5 selection and exact resume record `20260715T202049Z--HCM-0-5--orchestration--planning-review-budget-exhausted`.
- Treat the HCM-0.4 completion record as dependency evidence only.
- Ignore HCM-0.9 as resume/design authority and create no catalog leaves.
- Keep `05-contracts-schemas-and-gates.md` canonical.
- Preserve historical Planning Reviews 1-4 and their dispatches unchanged; they are consumed evidence and never renumbered.
- Use Continuation Planning Review 1 and, only after one permitted bounded remediation, Continuation Planning Review 2. Never name a continuation review `Review 5`.
- Reserve Design Reviews 1-3 and at most two design remediations for the fresh canonical design-freeze session. Planning reviews do not consume or satisfy that budget.
- Stop immediately on `CLEAN`. A non-clean terminal review transitions byte-identically to its non-completion handoff; no reviewed subject byte may be corrected after that review.
- Separate contract lifecycle from evaluation/verdict/gate outcomes.
- Use exact contract ref/fingerprint selection, a closed SemVer-impact table, a total lifecycle transition/authority table, and closed evidence-cardinality/precedence rules.
- Bind each all-of evidence-kind requirement to its own case/cardinality/stability tuple; omitted score weight means no score participation.
- Select the Draft 2020-12 JSON Schema process dock as the first proof target because the live repository and design lineage establish a deterministic, offline, low-authority precedent.
- Deny all process-dock networking in protocol v1.
- Bind every dock manifest/run/evidence record to one exact content-addressed implementation bundle, entrypoint digest, and bundled runtime/dependency closure verified before spawn.
- Extend the frozen HCM-0.4 ordinary-operation catalog; do not modify its owner/DTO/transport/bridge/publication rules.
- Keep runtime proof gates open and make no seam classification promotion beyond frozen target design.

## Dependency graph

```text
live preflight + completed HCM-0.2/0.3/0.4/0.8 evidence
  -> historical Planning Reviews 1-4 + Remediations 1-3
  -> selected exhausted-budget handoff
  -> explicit human continuation authority
  -> repair only R4-001/R4-002 + define stage-scoped review authority
  -> Continuation Planning Review 1
       CLEAN -> complete packet validation + GitNexus detection
             -> approved planning-continuation commit
             -> partial/context-boundary v1.2 handoff + ledger commit
             -> fresh design-freeze execution session
       BOUNDED LOCAL PLANNING FINDINGS ONLY -> one planning remediation
                                            -> Continuation Planning Review 2
                                                 CLEAN -> same approved-planning closeout
                                                 NON-CLEAN -> Task 7 byte-identical non-completion handoff; stop
       ANY OTHER NON-CLEAN -> Task 7 byte-identical non-completion handoff; stop

fresh design-freeze execution session + approved planning commit
  -> Tasks 3-5 canonical 01-06 design freeze
  -> complete proof wall (SPEC items 1-12 with captured results)
  -> Design Review 1
       CLEAN -> byte-identical proof replay + staging checks -> Tasks 9-10 completion
       FINDINGS -> design Remediation 1 -> proof wall -> Design Review 2
                       CLEAN -> byte-identical proof replay + staging checks -> Tasks 9-10 completion
                       FINDINGS -> design Remediation 2 -> proof wall -> Design Review 3
                                      CLEAN -> byte-identical proof replay + staging checks -> Tasks 9-10 completion
                                      NON-CLEAN -> Task 7 byte-identical non-completion handoff; no Design Review 4
```

## Phase 1: Planning authority

### Task 1: Validate scope, dependencies, and candidate authority

**Description:** Reconfirm the live branch/status, phase row, completed HCM-0.4 evidence, abandoned HCM-0.9 boundary, canonical `05` fingerprint, active control-pack sections, and existing JSON Schema/validator precedent.

**Acceptance criteria:**
- [x] HCM-0.5 exists and is authorized as docs/design-only.
- [x] Worktree is attributable and has no overlapping uncommitted work.
- [x] HCM-0.4 completion and current handoff validator modes validate.
- [x] HCM-0.9 is excluded from resume and implementation authority.
- [x] First-dock candidates are compared using live evidence.

**Verification:** `git status --short`; exact `jq` record/ledger queries; three `validate_handoffs.py` modes; scoped `rg` evidence inspection.

**Dependencies:** None.

**Files likely touched:** None.

**Estimated scope:** S.

### Task 2: Create the independently executable packet

**Description:** Create `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` with exact semantic questions, owner boundaries, lifecycle, evidence/verdict/gate and process-dock contracts, security posture, negative cases, first-proof selection, affected 01-06 sections, proof wall, non-goals, review budget, and stop rules.

**Acceptance criteria:**
- [x] All three required files exist and use only repository-relative durable refs.
- [x] Every requested planning topic is explicit and testable.
- [x] No Rust/runtime/HCM-0.6/HCM-0.9 authority is introduced.

**Verification:** required-heading/phrase assertions; Markdown fence/link checks; `git diff --check`; scoped diff inspection.

**Dependencies:** Task 1.

**Files likely touched:** `slices/HCM-0.5/SPEC.md`, `slices/HCM-0.5/tasks/plan.md`, `slices/HCM-0.5/tasks/todo.md`.

**Estimated scope:** M.

### Task 2A: Repair the authorized planning continuation

**Description:** Correct HCM-0.5-R4-001 and HCM-0.5-R4-002 in the three-file planning subject and replace the shared review counter with the historical-planning, planning-continuation, and future-design review identities.

**Acceptance criteria:**
- [x] No terminal review permits a local subject correction without another independent review.
- [x] A terminal non-clean review's handoff records the identical subject manifest/fingerprint with no intervening subject-byte change.
- [x] The live planning prohibition names Tasks 3-6 and 8-10, leaving Task 7 executable for a required terminal non-completion handoff.
- [x] Historical Planning Reviews 1-4 remain immutable; Continuation Planning Reviews 1-2 and Design Reviews 1-3 are disjoint.
- [x] The continuation and future design budgets match the human authority exactly.

**Verification:** exact phrase/state-transition assertions; subject-manifest replay; historical dispatch hash/status inspection; `git diff --check`; scoped diff inspection.

**Dependencies:** Task 2 and the selected human-authorized continuation handoff.

**Files likely touched:** `slices/HCM-0.5/SPEC.md`, `slices/HCM-0.5/tasks/plan.md`, `slices/HCM-0.5/tasks/todo.md`.

**Estimated scope:** S.

### Task 2B: Obtain continuation planning approval and close the context boundary

**Description:** Submit the complete three-file subject to Continuation Planning Review 1. Stop on `CLEAN`; otherwise allow one remediation only for bounded in-authority planning defects and submit the new complete subject to a different-fresh Continuation Planning Review 2. After a clean result, commit the exact reviewed planning subject and continuation dispatches, then create and separately commit a partial/context-boundary v1.2 handoff authorizing a fresh design-freeze execution session.

**Acceptance criteria:**
- [ ] Every continuation dispatch has a replayable sorted manifest and aggregate fingerprint over the complete three-file subject.
- [ ] Every reviewer is a fresh isolated built-in `default` agent using `using-agent-skills` and `code-review-and-quality`.
- [ ] A clean result ends the planning loop; Continuation Planning Review 2 is invoked only after the one allowed remediation and is terminal.
- [ ] The approved planning commit contains no canonical `00`-`06` edit.
- [ ] The separate handoff/ledger commit records the approved planning commit, full historical/continuation lineage, reserved Design Review budget, exact packet path, and the prohibition on treating planning review as final-design review.

**Verification:** complete planning validation; manifest/result fingerprint replay; `git diff --check`; staged GitNexus detection; primary commit inspection; three handoff validator modes; deterministic ledger rebuild; mechanical staged detection.

**Dependencies:** Task 2A.

**Files likely touched:** the three planning files, additive continuation-review dispatches, one final planning-continuation handoff, and `handoffs/ledger.jsonl`.

**Estimated scope:** M.

### Checkpoint A: Stage-scoped planning authority

- [x] Preserve immutable historical Planning Reviews 1-4 and Remediations 1-3.
- [x] Define Continuation Planning Reviews 1-2 with at most one remediation and no `Review 5` alias.
- [x] Reserve Design Reviews 1-3 with at most two remediations and no Design Review 4.
- [x] Require byte-identical non-completion handoff transition after every non-clean terminal review.
- [ ] Obtain `CLEAN` from Continuation Planning Review 1 or permitted Continuation Planning Review 2 before Tasks 3-6 or 8-10 execute.
- [ ] On terminal non-clean, execute Task 7 only; do not edit the reviewed subject or canonical files.

## Phase 2: Canonical design freeze

### Task 3: Freeze architecture and semantic boundaries

**Description:** Update affected 00-04 sections with the frozen lifecycle/evaluation separation, claim/applicability rules, evidence/Resolution limits, witness authority, dock posture, first proof target, and accurate runtime classification.

**Acceptance criteria:**
- [ ] HCM-0.2/0.3/0.4 frozen semantics are unchanged.
- [ ] Validator witness and Resolution proof limits are consistent across 01-04.
- [ ] HCM-0.5 exit rules and JSON Schema first-proof selection are explicit.
- [ ] HCM-0.6 remains unstarted/unselected.

**Verification:** targeted cross-file assertions, including proof that the fresh session selected the validated planning-continuation handoff and that the handoff binds the approved planning commit; scoped diff inspection.

**Dependencies:** completed Task 2B: continuation planning `CLEAN`, the approved planning-continuation commit, the validated partial/context-boundary handoff that binds that commit and exact packet path, and a fresh design-freeze session selecting that exact handoff. Until every dependency is satisfied, Tasks 3-6 and 8-10 must not execute. Task 7 remains executable only for the terminal non-completion handoff required by a non-clean continuation review.

**Files likely touched:** `00-README.md`, `01-target-architecture.md`, `02-semantic-model.md`, `03-seam-crosswalk.md`, `04-phase-slice-map.md`.

**Estimated scope:** M (documentation-only).

### Task 4: Freeze canonical contract and dock schemas/rules

**Description:** Replace only the preliminary HCM-0.5 sections in canonical `05` with exact field tables, lifecycle/claim/applicability/evidence/verdict/gate rules, manifest and process request/result contracts, execution/failure matrices, validator authority boundary, first proof-dock contract, and ordinary operation definitions.

**Acceptance criteria:**
- [ ] Every field has owner/default or omission/validation/non-goal semantics.
- [ ] Contract identity/version compatibility, exact lifecycle adjacency/transitions/authorities, all-of per-kind evidence cardinality, repeated-evidence consistency/precedence, score omission, and gate-effect/verdict compatibility are closed matrices.
- [ ] State machines and failure/refusal behavior are total and fail closed.
- [ ] Process JSON framing, content-addressed implementation/runtime selection, default-deny isolation/no-network/timeout/cancellation rules are executable without guessing.
- [ ] Rust-native future transport preserves identical semantic DTO/evidence meaning.
- [ ] HCM-0.4 sections are not modified except the preauthorized ordinary catalog extension point.

**Verification:** fenced-example parse harness; lifecycle/partition/failure/operation matrix assertions; prior-section hash comparisons where feasible.

**Dependencies:** Task 3.

**Files likely touched:** `05-contracts-schemas-and-gates.md`.

**Estimated scope:** M (one canonical file, large semantic change).

### Task 5: Freeze proof and regression authority

**Description:** Add the HCM-0.5 documentation-freeze proof gate and permanent regression rules while keeping PG-CONTRACT-01, PG-DOCK-01, and PG-GATE-01 open.

**Acceptance criteria:**
- [ ] Proof gate covers every objective and negative case.
- [ ] No runtime/classification proof is promoted.
- [ ] Canonical monolith and no-leaf boundary are asserted.

**Verification:** gate/row/regression assertions plus no-runtime promotion scan.

**Dependencies:** Task 4.

**Files likely touched:** `06-proof-and-regression-ledger.md`.

**Estimated scope:** S.

### Checkpoint B: Design verification

- [ ] Complete every SPEC proof-wall item 1-12 and capture its result before constructing the Design Review 1 dispatch.
- [ ] Parse all new JSON/YAML examples.
- [ ] Run identity/SemVer, exact lifecycle adjacency/transition/authority, applicability, all-of per-kind evidence cardinality/precedence, score omission, exhaustive gate-effect x verdict, implementation-substitution, protocol, isolation/no-network, and ordinary-operation assertions.
- [ ] Run HCM-0.2/0.3/0.4 regression assertions.
- [ ] Run Markdown links/anchors/fences, archive boundary, three handoff validator modes, no-Rust/no-HCM-0.6/no-leaf checks, `git diff --check`, and scoped diff inspection.

## Phase 3: Independent final review and remediation

### Task 6: Submit the complete design subject to Design Review 1

**Description:** Assemble a fresh immutable Design Review 1 dispatch over the complete HCM-0.5 packet, affected `00`-`06` files, and proof results. Use a fresh isolated built-in `default` reviewer with no implementation narrative, planning-review conclusions, or previous design findings.

**Acceptance criteria:**
- [ ] Subject manifest and aggregate fingerprint recompute.
- [ ] The manifest includes the final intended `00-README.md` status bytes plus the complete packet and every other affected canonical `01`-`06` file.
- [ ] Reviewer checks correctness, API stability, security/isolation, owner boundaries, Resolution semantics, HCM-0.4 regression, and proof coverage.
- [ ] Result is `CLEAN` or provides typed, bounded findings.

**Verification:** parent recomputes manifest and validates result fingerprint/status.

**Dependencies:** approved planning-continuation commit and Checkpoint B items 1-12 green with captured results in the fresh design-freeze execution session.

**Files likely touched:** one immutable review dispatch; no reviewer edits.

**Estimated scope:** S.

### Task 7: Enforce stage-scoped terminal non-completion branches

**Description:** This task is the sole exception to the pre-planning-clean prohibition. In the current continuation it executes when Continuation Planning Review 1 returns a terminal non-bounded result or Continuation Planning Review 2 is non-clean. In the future design session it executes after a terminal non-clean Design Review 3. It creates the honest v1.2 non-completion handoff and deterministic ledger without changing any reviewed subject byte.

**Acceptance criteria:**
- [ ] The handoff's subject manifest/fingerprint equals the terminal review subject byte-for-byte.
- [ ] No local correction occurs after the terminal review.
- [ ] The handoff names the exact remaining defect, consumed stage budget, and human resume condition without calling the slice abandoned.
- [ ] No continuation `Review 5` or Design Review 4 is scheduled or invoked.
- [ ] No canonical design edit occurs on a non-clean continuation branch.

**Verification:** replay the terminal manifest against the handoff subject; assert no subject diff after review; validate the exact stage lineage, no forbidden next review, record/index parity, and all handoff validator modes.

**Dependencies:** a terminal non-clean Continuation Planning Review 1 or 2, or a terminal non-clean Design Review 3. This task makes no reviewed-subject edit and does not depend on planning `CLEAN`.

**Files likely touched:** one exact-status v1.2 non-completion handoff and the deterministic ledger on every terminal non-completion branch; no reviewed subject edit.

**Estimated scope:** bounded by finding.

## Phase 4: Proof, commit, and closeout

### Task 8: Replay proof byte-identically and run staged scope detection

**Description:** After a `CLEAN` Design Review 1, 2, or 3, rerun every Checkpoint B proof against byte-identical reviewed bytes, then stage those exact bytes and run repository-required GitNexus change detection. This is a replay/staging gate, not the first complete proof wall, and it may not repair or otherwise change the subject after the clean review.

**Acceptance criteria:**
- [ ] The complete Checkpoint B proof wall and captured results existed before the clean Design Review dispatch.
- [ ] All proof commands pass again on byte-identical reviewed bytes.
- [ ] GitNexus reports documentation-only expected scope.
- [ ] Staged paths contain only reviewed HCM-0.5 subject/dispatch evidence.
- [ ] The staged subject fingerprint equals the clean Design Review subject fingerprint exactly.
- [ ] The staged and committed `00-README.md` hash equals its clean Design Review manifest entry; no post-`CLEAN` status-byte edit occurs.

**Verification:** captured command results, staged diff, `git diff --cached --check`, staged change detection.

**Dependencies:** approved planning-continuation commit, Checkpoint B green before dispatch, and a clean Design Review 1, 2, or 3.

**Files likely touched:** None beyond proof artifacts already in approved scope.

**Estimated scope:** S.

### Task 9: Commit reviewed HCM-0.5 design authority

**Description:** Stage only the reviewed subject and immutable proof-relevant HCM-0.5 dispatches, then create one scoped documentation commit.

**Acceptance criteria:**
- [ ] Primary commit contains the exact final reviewed subject.
- [ ] Commit message explains the HCM-0.5 semantic freeze.
- [ ] Worktree contains no accidental unrelated staged changes.

**Verification:** `git show --stat --oneline HEAD`; replay final manifest against the primary commit.

**Dependencies:** Task 8.

**Files likely touched:** Git index only.

**Estimated scope:** XS.

### Task 10: Create the completed v1.2 handoff and mechanical closeout commit

**Description:** Create one parent-owned completed handoff bound to the primary commit/final review, rebuild the ledger deterministically, run every validator mode and mechanical diff check, run GitNexus change detection, and commit only the handoff/ledger closeout.

**Acceptance criteria:**
- [ ] Handoff uses `status: completed`, `stop_reason: completed`, and `resume.execution_target: none`.
- [ ] It records the approved planning commit, planning-review lineage, Design Review lineage, and design remediations exactly without treating a planning review as final-design review.
- [ ] Ledger rebuild is byte-identical and all validator modes pass.
- [ ] Second commit contains only the new record and ledger mutation.
- [ ] HCM-0.6 is not started.

**Verification:** three validator modes; exact record/index parity; final `git status`; two commit hashes.

**Dependencies:** Task 9.

**Files likely touched:** one `handoffs/records/*.json`, `handoffs/ledger.jsonl`.

**Estimated scope:** S.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Lifecycle/evaluation states remain conflated | High | explicit separate state machines and mechanical vocabulary assertions |
| Dock output is mistaken for canonical evidence | High | candidate -> membrane validation -> immutable evidence boundary; reviewer gate |
| Process dock receives ambient host authority | High | default-deny manifest/grant intersection, unconditional v1 network denial, and negative isolation matrix |
| Dock executable/runtime substitution preserves a trusted identity | High | content-addressed bundle manifest, exact entrypoint/runtime closure, host allowlist mapping, and pre-spawn digest verification |
| HCM-0.4 transport contracts drift | High | append only named operation definitions; compare frozen sections and review explicitly |
| First dock is selected by implementation inertia | High | evidence-backed three-candidate comparison and narrow selection criteria |
| Stage review budget exhausted | Medium | keep planning and design identities separate; stop immediately on clean; terminal non-clean transitions byte-identically to an honest handoff |
| Monolith size makes review unfocused | Medium | exact section ranges and a bounded manifest; no decomposition or leaf creation |

## Open questions

None at planning time. If live review establishes that JSON Schema first-dock selection needs product authority beyond the HCM-0.5 objective and existing pack/repository evidence, stop with `human_input` and present the candidate table rather than choosing implicitly.
