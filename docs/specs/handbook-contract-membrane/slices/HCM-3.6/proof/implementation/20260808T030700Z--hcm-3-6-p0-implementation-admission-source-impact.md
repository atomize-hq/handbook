# HCM-3.6 P0 implementation-admission source and impact record

**Status:** pending independent P0 discovery review.

## Local-only preflight

| Check | Observation | Result |
| --- | --- | --- |
| Task HEAD/tree | `85c042863adad2d2a58f4dcc3bbbc16177937655` / `8946426c8d390b744b46bec14b1375c2d4272da0` | passed |
| Local integration ref/tree | `refs/heads/orchestration/handbook-hcm-3-6-implementation-20260807` at the same head/tree | passed |
| Required ancestry | `1256e724a2b7da6b6250f57d6f63fced1e2cf949` is an ancestor of the expected base | passed |
| Task worktree | assigned detached checkout, clean before P0 documentation writes | passed |
| Protected checkout | `C:/Users/spmcc/Documents/__Project_Code/handbook` clean at `1256e724a2b7da6b6250f57d6f63fced1e2cf949` / `ad5bac4904912e9a52b23ac862414deb79613154` | passed |
| Remote observation | local `origin/feat/handbook-contract-membrane` remains `1256e724a2b7da6b6250f57d6f63fced1e2cf949`; no network command ran | passed |

`npx --no-install gitnexus analyze` rebuilt the absent local index with 22,121
nodes, 47,163 edges, 475 clusters, and 300 flows. It reported unavailable FTS/
BM25. It changed only generated count hunks in `AGENTS.md` and `CLAUDE.md`;
those hunks were restored byte-for-byte and are outside the P0 subject.
The required GitNexus query returned no process matches because FTS is
unavailable; that unavailable search evidence is not treated as GREEN.

## Existing source symbols: read-only availability and impact

No existing function, class, method, test, fixture, or schema is proposed for
modification by P1-P4. The only existing product file in the conditional
P1/P2 ceiling is `crates/engine/src/lib.rs`, where a private module declaration
would be added without changing an existing symbol. Each facility below was
inspected only to establish an admissible dependency boundary.

| Existing facility | Real `context` / upstream `impact` result | HCM-3.6 disposition |
| --- | --- | --- |
| `resolve_profile_selection` in `profile_selection.rs` | exact context; **CRITICAL**: 353 impacted, 51 direct callers, 20 processes/modules | Read-only only. It provides `ResolvedInstanceProfile::resolved_profile_fingerprint`; no edit or compatibility change is allowed. |
| `compute_freshness` in `freshness.rs` | exact context/impact run; C-03 artifact freshness, not a posture evaluation basis | Read-only only. Its `FreshnessTruth` cannot substitute for explicit `FreshnessEvaluationBasis`. |
| `ProjectConditionRegistry` in `project_condition_registry.rs` | exact-file context/impact run; ambiguous struct/impl graph reports max LOW and zero direct dependents | Read-only exact condition-definition identity only; no ambient condition state. |
| `CharterAuthorityTransactionServiceV1` in `charter_authority_transaction.rs` | exact-file context/impact run; ambiguous struct/impl graph reports max LOW and zero direct dependents | Read-only Charter fingerprint/read and existing candidate-promotion capability; not an admissible posture transition primitive. |
| `CharterPromotionWorkflowServiceV1` in `charter_promotion_workflow.rs` | exact-file context/impact run; ambiguous struct/impl graph reports max LOW and zero direct dependents | Excluded: candidate promotion is not a one-replace posture transition. |
| `CharterLifecycleStoreV1` in `charter_lifecycle_store.rs` | exact-file context/impact run; ambiguous struct/impl graph reports max LOW and zero direct dependents | Excluded as a posture-write substitute: it persists lifecycle observation state, not `PostureTransition`. |
| `ground_resolution` / `GroundingOutcome` in `grounding.rs` | exact context/impact; LOW, zero direct callers/processes | Read-only HCM-3.5 evidence boundary only. No raw source/delta access or ownership transfer. |

The CRITICAL profile-selection result was surfaced before any possible product
edit. It does not authorize a risky edit: the function remains untouched.

## Current-facility admission result

`CharterAuthorityTransactionServiceV1::begin_retained_authority` can lock and
read committed Charter authority, but its only write route accepts a
`CharterPromotionRequestV1` consisting of candidate promotion and lifecycle
transition bytes. `CharterPromotionWorkflowServiceV1::promote` creates that
candidate-promotion closure. `CharterLifecycleStoreV1::record_event` creates
lifecycle observations/transitions. None can atomically write exactly one
mapped Charter root `replace` plus an immutable HCM-3.6 `PostureTransition` and
then re-resolve the posture kernel.

Providing that missing group would require either a new transaction primitive
or a change to the existing transaction's record/output surface. Both are
outside the selector's no-schema/no-broader-transaction ceiling. P3 must
therefore remain blocked; P1/P2 are not begun independently because their
required end-to-end no-write and transition proof cannot close under this
authority.

## HCM-3.4/HCM-3.5 evidence pairs

| Predecessor | Admissible HCM-3.6 use | Excluded use |
| --- | --- | --- |
| HCM-3.4 private Projection source pair at `cc6d84926e6435c3960af86e5927a18991dd04fd` | Exact current source-pair relation, five-family currentness, provenance/fingerprint evidence | Public export, raw Snapshot payload, full delta payload, adoption claim, or posture ownership. |
| HCM-3.5 corrective grounding at `50b16e30b6b5ffd1d3b12d54f395b673c38359f2` | Typed redacted evidence pairs and bounded currentness/provenance only | Flow/pipeline adoption, hidden source reads, raw signals, gate/promotion claim, or an HCM-3.6 mutation route. |

Both completed handoffs state that their scope does not grant HCM-3.6 work.
They are read-only evidence pairs subject to the exact/current/redacted P1/P2
rules if a later selector admits a complete implementation route.

## Conditional source/test/documentation ceiling

| Surface | P0 disposition |
| --- | --- |
| `crates/engine/src/project_posture.rs` | New private owner only; no source file exists yet. |
| `crates/engine/src/lib.rs` | At most a private `mod project_posture;` declaration; no public re-export or existing-symbol modification. |
| `crates/engine/tests/hcm_3_6_project_posture.rs` | New owner-local test only; no fixture, integration, or consumer test adoption. |
| HCM-3.6 decision/proof/dispatch/handoff/ledger docs | P0 review and true-stop control artifacts only. |
| Any existing Charter transaction/lifecycle source, definition/schema, Cargo/configuration, Flow/pipeline/SDK/CLI/Tauri/Substrate/HCM-5 path | prohibited. |

The P0 implementation selector and this record are the complete admission
subject. They authorise no product source or test change until a fresh
independent review is CLEAN, and they prohibit P3/P4 without new authority.
