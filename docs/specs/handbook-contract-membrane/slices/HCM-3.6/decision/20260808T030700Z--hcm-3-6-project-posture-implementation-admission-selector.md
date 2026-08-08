# HCM-3.6 implementation-admission selector

**Selector ID:** `HCM-3.6-IMPLEMENTATION-ADMISSION-SELECTOR-01`

**Status:** P0 read-only admission subject pending independent review.

## Bound authority and lineage

This selector is bound to increment task `019fdf4f-897a-7573-8f7f-18e387e566cf`,
host `local`, meta workflow `handbook-hcm-3-6-implementation-20260807`, and
dispatch nonce
`2490c1c9da0a426e7e68cde6ac679d5cd4e94ef9266d7261eaa0ab1af8ded889`.

The selected planning closeout is
`20260808T020459Z--HCM-3-6--orchestration--project-posture-planning-closeout-completed`.
It remains immutable planning context and does not itself authorize source
work. The nonce-bound local-only HCM-3.6 implementation contract is the sole
authority for this P0 admission. It neither resumes nor alters the completed
planning parent or the HCM-3.4/HCM-3.5 terminal parents.

| Field | Value |
| --- | --- |
| Phase / slice | `HCM-3` / `HCM-3.6` |
| Active packet | `slices/HCM-3.6/tasks/plan.md` |
| Integrated P0 outcome | `hcm-3-6-project-posture-implementation-admission` |
| Parent orchestration | `handbook-hcm-3-6-implementation-20260807` |
| Expected base/tree | `85c042863adad2d2a58f4dcc3bbbc16177937655` / `8946426c8d390b744b46bec14b1375c2d4272da0` |
| Required ancestor | `1256e724a2b7da6b6250f57d6f63fced1e2cf949` |
| Local integration ref | `refs/heads/orchestration/handbook-hcm-3-6-implementation-20260807` |
| Publication | local-only expected-old CAS; no remote operation |

The task checkout, integration ref, required ancestry, protected checkout,
and `origin/feat/handbook-contract-membrane` local tracking baseline were
verified before source inspection. The protected checkout remains outside this
subject.

## Selected ownership and no-cycle boundary

`handbook-engine::project_posture` is the only prospective semantic owner.
It is private to `handbook-engine`; `ProjectPostureKernel` is derived and
Charter remains the sole editable constitutional authority. The owner may
consume exact/current Charter, profile, condition, evidence, and redacted
HCM-3.4/HCM-3.5 pairs. It may not read raw Snapshot payloads or full delta
signals, introduce ambient time, or create an editable posture artifact.

The prospective read path is:

```text
exact Charter/profile/condition/evidence pairs + FreshnessEvaluationBasis
  -> resolve_project_posture_kernel
  -> ProjectPostureKernel
  -> evaluate_posture_recommendation
  -> NoRecommendation | Recommended | Refused
```

The prospective mutation path is deliberately separate:

```text
reviewed recommendation + exact approval/reassessment/current target
  -> apply_posture_transition
  -> one constitutional-root replace + immutable PostureTransition
  -> re-resolved ProjectPostureKernel
```

Flow, pipeline, SDK, CLI, Tauri, Substrate, HCM-5, and notification delivery
are not semantic owners. They receive no new dependency or call path. HCM-5
remains the exclusive future gate-verdict and parent-promotion owner.

## Frozen P1-P4 ceiling

P0 admits no Rust, product test, fixture, schema, dependency, configuration,
or public API edit. If P0 is CLEAN, its maximum prospective product ceiling is
limited to these private/new paths:

| Packet | Permitted product path | Purpose |
| --- | --- | --- |
| P1 | `crates/engine/src/lib.rs`; new `crates/engine/src/project_posture.rs`; new `crates/engine/tests/hcm_3_6_project_posture.rs` | Private module declaration, exact-ref/kernel/freshness/refusal implementation, owner-local P01-P03/P08 proof. |
| P2 | the same private module and owner-local test | Closed advisory evaluator and P04-P08/P11 proof. |
| P3 | none admitted | See the fail-closed transaction stop below. |
| P4 | the same private module and owner-local test only if P3 becomes separately admitted | Typed notification/acknowledgement boundary proof; no delivery adapter. |

The documentation allowlist is this selector, its paired source-impact proof,
P0 review dispatches, a parent handoff, deterministic `ledger.jsonl`, and an
exact P3/P4 inventory transcription if a reviewer produces one. Existing
planning authority, predecessor records, schemas, definitions, Cargo files,
fixtures, Flow/pipeline/SDK/CLI/Tauri/Substrate/HCM-5 paths, and the protected
checkout are read-only.

No public re-export is admitted: `project_posture` must remain `mod`, not
`pub mod`, and no new public type, DTO, schema, adapter, gate, or transaction
surface may be introduced. The initial P1/P2 implementation must use only new
types/functions in the private module; no existing function, class, method,
or test is selected for modification.

## Exact contract and negative-proof matrix

P1 must reject blank, malformed, mutable-only, ambiguous, stale,
fingerprint-mismatched, missing-profile-currentness, unknown-freshness,
missing-required-evidence, and unresolved-required-condition input. Semantic
hashes must stably sort unordered lists and exclude presentation IDs and
timestamps. Engineering posture dimensions remain global and separate from the
six Context Resolution dimensions.

P2 must keep hard-trigger and accumulated-rule identities mutually exclusive;
emit two recommendations for two affected global dimensions; and refuse or
return no recommendation for threshold/window/count misses, expired evidence,
cooldown, policy mismatch, or insufficient evidence. Recommendation and
notification intent remain immutable and advisory.

P3 requires exact approval, actor, reassessment, and current-target pairs;
one mapped global-dimension `replace`; atomic Charter plus immutable
`PostureTransition` persistence; and re-resolution. Override, scoped,
multi-change, wrong-target, stale, unapproved, and failure-injection cases
must leave Charter untouched. P4 must prove that delivery/acknowledgement
cannot change policy, recipient, deadline, approval, recommendation, Charter,
transition, HCM-5 gate verdict, or parent promotion.

## Admissible dependency evidence and P3 authority boundary

The paired source-impact record establishes the current read-only facilities:

- `resolve_profile_selection` exposes the exact resolved-profile fingerprint
  and condition registry, but has CRITICAL upstream impact and is not editable;
- current `compute_freshness` is a C-03 artifact-freshness report, not the
  required exact `FreshnessEvaluationBasis`; P1 must define its private,
  explicit-basis validation without changing this facility;
- `ProjectConditionRegistry` supplies exact condition-definition identity, not
  mutable or ambient condition truth;
- `CharterAuthorityTransactionServiceV1` reads committed Charter and owns a
  candidate-promotion transaction, while `CharterLifecycleStoreV1` owns
  lifecycle observations; neither exposes the required posture-specific
  atomic Charter-replace plus immutable `PostureTransition` group;
- HCM-3.4 source pairs and HCM-3.5 grounding outputs remain private or bounded
  redacted evidence only, never raw Snapshot/delta access or posture authority.

The last point is an immediate authority boundary. Reusing the existing
candidate-promotion/lifecycle transaction would create the wrong immutable
records and does not admit the required one-replace `PostureTransition`.
Adding a posture transaction primitive, changing its output group, exposing a
private type, or adding a record/schema capability is a broader transaction or
schema decision expressly excluded by this slice. Therefore P3 and P4 cannot
be admitted after P0 without an exact reviewed authority that selects that
primitive and its atomic record semantics.

P1/P2 are not started as a partial success: their required end-to-end owner
and no-write proof depends on the selected P3 transition boundary. This P0
subject must stop truthfully after CLEAN rather than manufacture an
implementation route that cannot meet P3.

## Review and true-stop handling

This complete P0 documentation subject requires one fresh discovery review.
A valid P1/P2 receives one consolidated remediation and a different-fresh
delta closure; P3/P4 follow the inventory protocol. No review runs after
CLEAN. A CLEAN P0 creates a parent-owned v1.4 `authority_boundary` handoff
requesting a separately selected, private, posture-specific atomic transaction
primitive and immutable transition-record capability. It is
`same_slice_adjudicable: false` because the needed transaction/schema authority
is outside the frozen ceiling.

The ordinary validator's known immutable HCM-3.5 lineage contradiction must be
reported as failed/unavailable evidence if encountered. It is not GREEN, is not
rewritten, and is not reclassified by this selector.
