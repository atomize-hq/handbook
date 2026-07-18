# HCM-2.1 Planning Proof Wall

## Subject and boundary

- Phase: `HCM-2`
- Slice: `HCM-2.1`
- Runtime `ACTIVE_PACKET`: `none`
- Selected resume context:
  `20260718T191425Z--HCM-1-4--orchestration--profile-aware-setup-doctor-landed`
- Entry branch: `feat/handbook-contract-membrane`
- Entry HEAD: `78171f5024a20d23919e41dc2d30f32fa9df3b6e`
- Mode: planning/control-pack documentation only
- Future packet:
  `docs/specs/handbook-contract-membrane/slices/HCM-2.1/`

No Rust, Cargo, definition, template, fixture, test, runtime, generated skill,
or product byte belongs to this planning subject.

## Selective context capsule

| Capsule field | Exact planning truth |
|---|---|
| Slice / objective | create and independently review the HCM-2.1 Project Context canonical-YAML pilot packet; do not implement it |
| Active packet | none at entry; the three-file HCM-2.1 packet is the planning output |
| Dependency / authorization proof | HCM-1.1-HCM-1.4 reviewed implementation ancestry; selected completed HCM-1.4 v1.2 closeout; exact admitted shipped profile/kind/schema/descriptor bytes |
| Selected handoff / validity | exact ledger/record match; completed; clean; no blockers; `resume.execution_target=none`; valid context after the user's explicit HCM-2.1 selection |
| Active Resolution envelope | slice scope, high detail, current planning session, planning approval horizon, documentation validation |
| Grounding snapshot / start delta | Snapshot Memory capability not landed; live branch/HEAD/status, Git history, definitions, source/tests, and selected handoff were revalidated directly |
| Target authority boundary | exact shipped `project_context` instance, `.handbook/project/context.yaml`, exact 1.0 content schema, fixed in-memory Markdown view, author/setup/doctor/flow cutover, and Environment Inventory reference-only selected-path repair |
| Current repo-truth status | HCM-1.4 setup/doctor already inspect selected YAML structurally; author and flow still use the rich 0.1.0 input plus fixed Markdown path; current renderer depends on a timestamp |
| Must-read pack | `00`; `01` authority/canonical-view/invariants; `02` profile/kind/instance semantics; affected `03` rows; `04` HCM-2.1; `05` exact schema/profile contracts; `06` gates/rules; `07`; `08`; HCM-0.6 decision; selected HCM-1.4 closeout |
| Live sources / tests / precedent | engine Project Context schema/kind/profile assets; engine author core; compiler author/setup/doctor; flow resolver/packet result; CLI author/doctor; focused engine/compiler/flow/CLI tests; input template and live-skill smoke |
| Sibling seams in context | Charter, Environment Inventory, Feature Spec, frozen pipeline, condition evaluator, generic custom kinds, Project Authority, SDK/transports |
| Allowed areas | planning packet; `00`, `04`, `06`; planning proof; internal review dispatches; one parent handoff/ledger closeout after clean review/primary commit |
| Explicit non-goals | implementation now; definition edits; compatibility/migration; persisted Markdown; setup materialization; pipeline migration; HCM-2.2+ |
| Applicable contracts / gates | canonical structured truth, kinds/instances distinct, renderer-derived view outside Projection, `PG-YAML-01`, bounded `PG-ARTIFACT-01`/`PG-YAML-02`, regression rules, orchestration gates |
| Required skill chain | using-agent-skills, context-engineering, spec-driven-development, planning-and-task-breakdown, documentation-and-adrs, code-review-and-quality, git-workflow-and-versioning; future implementation chain is frozen in the packet |
| Known correction / conflict | admitted Project Context 1.0 content schema is intentionally smaller than the current rich 0.1.0 author input; the packet resolves this by direct canonical-record input and retirement, not schema expansion or mapping |
| Maximum classification / proof change | planning approval only in this session; no runtime seam promotion |
| Exit proof | docs/link/scope/validator/baseline checks, exact manifest, fresh isolated review CLEAN, primary docs commit, separate v1.2 closeout commit |
| Stop conditions | schema/path contradiction, required scope widening, unavailable mandatory built-in review, or proof that cannot be completed honestly |

## Dependency and handoff proof

The exact ledger query selected one record at:

`docs/specs/handbook-contract-membrane/handoffs/records/20260718T191425Z--HCM-1-4--orchestration--profile-aware-setup-doctor-landed.json`

The record is schema v1.2, `status=completed`, `stop_reason=completed`, reports
available built-in delegation, binds reviewed implementation commit
`353e3bf0e0e8e75e470c19487ee2783c8c1a2aaa`, has no blockers/escalations, and
forbids automatic HCM-2 work. The present user instruction explicitly selects
HCM-2.1, so the handoff supplies valid dependency/resume context without
selecting or widening the slice.

Live definition proof confirmed:

- exact Project Context schema document and registry entry exist;
- exact Project Context kind binds that schema and stable role;
- exact shipped profile selects instance ID `project_context`, kind ref
  `handbook.artifact-kind.project-context@1.0.0`, path
  `.handbook/project/context.yaml`, role `project_context`, `always`
  requiredness, and no intake/renderer/Projection refs; and
- no HCM-2.1 packet existed at entry.

## GitNexus and live-source feasibility

Command:

```text
npx gitnexus analyze --index-only
```

Result: PASS. Incremental index completed with 13,357 nodes, 24,386 edges, 355
clusters, and 300 flows for this Handbook checkout. Free-form query returned no
ranked processes because FTS was unavailable; graph context and live source were
used instead.

Planning impact results:

| Symbol | Risk | Impact |
|---|---:|---|
| `ProjectContextStructuredInput` | LOW | 14 total, 2 direct, author/tests |
| engine `render_project_context_markdown` | LOW | 2 direct |
| compiler `author_project_context_from_input` | MEDIUM | 6 total, 5 direct, CLI author flow |
| compiler `run_setup` | LOW | 2 direct, setup CLI flow |
| compiler `doctor` | LOW | 2 direct, doctor CLI flow |
| flow `resolve_with_contract` | MEDIUM | 5 direct, flow/compiler tests |

No HIGH or CRITICAL planning impact was observed. Future implementation must
refresh impact before every existing-symbol edit.

## Focused current-baseline replay

Commands and results:

```text
cargo test -p handbook-engine --test author_core
PASS: 9 passed

cargo test -p handbook-compiler --test author
PASS: 47 passed

cargo test -p handbook-compiler --test setup
PASS: 11 passed

cargo test -p handbook-compiler --test doctor
PASS: 3 passed

cargo test -p handbook-flow --test resolver_core
PASS: 15 passed

cargo test -p handbook-cli --test author_cli
PASS: 22 passed
```

The replay proves the old input/timestamped renderer, fixed Markdown authoring,
selected setup/doctor structural decisions, and fixed flow behavior are live
baselines. Passing old behavior is precedent and feasibility evidence, not
proof that the HCM-2.1 target has landed.

## Planning-document validation

Commands and results:

```text
git diff --check
PASS

unchecked future-task scan under slices/HCM-2.1
PASS: no checked implementation item

exact changed-path scope assertion
PASS: only 00, 04, 06, and slices/HCM-2.1 planning files

absolute machine-path scan
PASS: no /Users, /home, or drive-letter path

relative Markdown link checker
PASS: 6 changed/new Markdown files checked before this proof file was added

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 42 records, 164 current internal dispatches, 8 admitted legacy dispatches,
42 ledger entries

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
PASS
```

## Planning classification

This subject proposes future implementation authority but does not grant it
before immutable `CLEAN` review and parent closeout. It promotes no runtime
seam, closes no runtime proof gate, changes no definition fingerprint, and does
not activate HCM-2.1 implementation. If cleanly reviewed and separately
selected, the packet permits a future implementation to close `PG-YAML-01` for
one exact family while keeping the program-wide `PG-ARTIFACT-01`, `PG-YAML-02`,
and `PG-KIND-01` boundaries open at their named remaining gaps.

## Review admission

Before review, the parent must:

1. rerun changed/new Markdown link, diff, task, scope, absolute-path, and all
   three handoff-validator checks including this proof file;
2. compute the exact sorted path/SHA-256 subject manifest and aggregate
   fingerprint;
3. write one immutable schema-valid internal-dispatch v1.1 JSON; and
4. spawn a fresh isolated read-only built-in `default` reviewer with no
   implementation reasoning or success-asserting summary.

A review finding is not completion. Valid findings require parent remediation,
full planning proof replay, a new immutable dispatch/fingerprint, and a
different fresh reviewer. Only CLEAN over the complete final subject permits
the primary planning commit.

## Review 1 disposition

Fresh isolated built-in reviewer `/root/hcm_2_1_planning_review_1` replayed the
exact seven-entry manifest and returned `CHANGES_REQUIRED` with six Required
findings. No commit or authorization claim followed that result. The parent
validated and remediated the findings in
[`20260718T211538Z--planning-review-1-remediation.md`](20260718T211538Z--planning-review-1-remediation.md).
A new immutable manifest and different fresh reviewer are required.

Fresh isolated built-in reviewer `/root/hcm_2_1_planning_review_2` then replayed
the exact nine-entry remediated manifest and returned `CHANGES_REQUIRED` with
three new Required contract findings. The parent recorded their validation and
bounded remediation in
[`20260718T213906Z--planning-review-2-remediation.md`](20260718T213906Z--planning-review-2-remediation.md).
Review 2 did not authorize a commit; another new manifest and different fresh
reviewer are required.

Fresh isolated built-in reviewer `/root/hcm_2_1_planning_review_3` replayed the
exact eleven-entry subject, withdrew a preliminary renderer concern after
applying phase-specific authority, and returned `CHANGES_REQUIRED` with two
Required live-carrier/version findings. The parent recorded bounded remediation
in
[`20260718T220005Z--planning-review-3-remediation.md`](20260718T220005Z--planning-review-3-remediation.md).
Review 3 did not authorize a commit; a new complete manifest and fourth
different fresh reviewer are required.

Fresh isolated built-in reviewer `/root/hcm_2_1_planning_review_4` replayed the
exact thirteen-entry subject and returned `CHANGES_REQUIRED` with two Required
exhaustiveness/ownership findings. The parent recorded bounded remediation in
[`20260718T221816Z--planning-review-4-remediation.md`](20260718T221816Z--planning-review-4-remediation.md).
Review 4 did not authorize a commit; a new complete manifest and fifth different
fresh reviewer are required.

Fresh isolated built-in reviewer `/root/hcm_2_1_planning_review_5` replayed the
exact fifteen-entry subject, revalidated all prior repairs, and returned
`CHANGES_REQUIRED` with one remaining Required author-result carrier finding.
The parent recorded bounded remediation in
[`20260718T223318Z--planning-review-5-remediation.md`](20260718T223318Z--planning-review-5-remediation.md).
Review 5 did not authorize a commit; a new complete manifest and sixth different
fresh reviewer are required.
