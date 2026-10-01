# Operational reliance and actions: coverage and landing

Baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`, checked against HEAD on
2026-10-01. Proposed research input only; no implementation authority or runtime
certification. This report owns the full approved Operational reliance/actions
chapter, including all scenarios and deferrals. HR-004 and HR-006 are resolved
semantic decisions; their concrete realization remains open. Only this file was
written. Coverage labels refer to the precise behavior stated, not name similarity.

## Section coverage and authority

All links below resolve from this report; source line locations refer to the baseline.

| Owning chapter subsection read in full | Claims / disposition |
| --- | --- |
| [Purpose and scope](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#purpose-and-scope), [canonical vocabulary](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#canonical-vocabulary) | U-01–U-12: exact historical truth, reliance, effect question, authorization and mutation remain separate. |
| [Ontology placement](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#ontology-placement) | U-15: all introduced definitions, owner records, requests/attempt provenance and views retain their six-category placement. |
| [Historical truth and present use](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#historical-truth-and-present-use) | U-01, U-02, U-12, U-13: immutable history, exact operational resolution, pure advisory terminal, bounded lifecycle exceptions. |
| [Later collection checks and action boundaries](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#later-collection-checks-and-action-boundaries) | U-01, U-03: retain frontier/check provenance; collection permission supplies no effect authority. |
| [Operational Reliance Policy and basis formation](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#operational-reliance-policy-and-basis-formation) | U-02, U-05, U-12: admitted finite boundaries, action question, role/use-intent accounting, occurrence/replay/refusal. |
| [Independently governed completion](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#governed-completion) | U-04: precollection governing relationship, independent current assignment, assessment floor, through-commit coverage. |
| [Exact current-use and Engine-verification rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule) | U-03, U-09, U-11: conditional evaluation-Binding mode and universal fresh action/target checks. |
| [Eligibility, disposition, and refusal vocabulary](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#eligibility-disposition-and-refusal-vocabulary) | U-05: one atomic result/fixed facets; formation refusal is outside that result. |
| [Work Progression ownership](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership), [local closeout](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#local-closeout) | U-06, U-07: semantic owner versus state/structure owners; pure local result, qualifying complete-pruning role. |
| [Parent eligibility and parent action](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action) | U-08: separately bounded relation/result populations, proposition roles, parent assessment and later action. |
| [Action authorization and mutation](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#action-authorization-and-mutation), [authority without IAM](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#authority-provenance-without-a-handbook-iam-platform) | U-09–U-12: Intent/Binding/refusal/Attempt/Terminal/Receipt, advisory effects, guarded commit and uncertain recovery. |
| [Work Specification relationship](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-specification-relationship) | U-13: exact declared inputs and later outcome references preserve causal order. |
| [Posture-feedback boundary](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#posture-feedback-boundary) | U-14: observation, recommendation, transition-specific reliance and Engine effect; later evaluation only. |
| [Currentness, invalidation, and replay](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#currentness-invalidation-and-replay), [responsibility map](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#semantic-responsibility-map) | U-01–U-16: all change conditions and owner/adapter exclusions assigned below. |
| [Discriminating scenarios](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#discriminating-scenario-corpus), [settled/deferred](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#settled-versus-deferred) | Scenario table and deferral table below; no settled decision reopened. |

The [guide, explanations 8–12](../../specs/handbook-contract-membrane/semantic-model/README.md#8-a-valid-past-result-does-not-automatically-qualify-for-use-today)
maps to U-01/U-03, U-05, U-04/U-06/U-08, U-10, and U-11/U-12 respectively.
[HR-004](../../specs/handbook-contract-membrane/semantic-model/review-decision-register.md#hr-004--requirements-sources-contract-permitted-use-and-governed-completion)
supplies U-04's governing assignment and assessment floor, and U-10's advisory
permission; [HR-006](../../specs/handbook-contract-membrane/semantic-model/review-decision-register.md#hr-006--fresh-verification-of-an-unchanged-committed-basis)
preserves later checks under an unchanged basis and originally permitted same-Binding
recovery. Neither creates a universal new-Binding-per-retry requirement. The
[guide's current-product boundary](../../specs/handbook-contract-membrane/semantic-model/README.md#current-product-versus-proposed-model)
excludes a compulsory V1 bridge, alias, dual write or migration design.

## Inspected implementation evidence

These paths were followed through owner calls, exports, declarations and test bodies;
negative findings are scoped to these plausible implementations and their current
public call paths. They do not assert that no external work/host service exists.

| Evidence ID | Source, public declaration and test evidence inspected |
| --- | --- |
| E-1 Contracts pure gate/local path | [composition 175–286](../../../crates/contracts/src/gate_composition.rs#L175) repeatedly derives verdicts, rechecks source/lifecycle state, constructs separate gate/local values and persists nothing; [identity 1112–1240](../../../crates/contracts/src/evaluation_identity.rs#L1112) closes content-addressed bodies; [run 75–90, 169–187](../../../crates/contracts/src/evaluation_run.rs#L75) binds contract/lifecycle/subject/cases/policies/nonce/currentness; [owner readmission 583–667](../../../crates/contracts/src/owner_repository.rs#L583) resolves exact lifecycle/policy closure. [Contracts exports 63–66](../../../crates/contracts/src/lib.rs#L63), [manifest 18](../../../crates/contracts/Cargo.toml#L18). [Owner tests 3366–3497, 3684 onward](../../../crates/contracts/tests/owner_repository.rs#L3366) inspect pure distinct identities, unchanged persisted bytes and final currentness races. |
| E-2 Public Contracts route | [SDK execute_gate 741–807](../../../crates/sdk/src/contract_membrane/contract_evidence_operation.rs#L741) reads the submitted exact run, calls E-1 and projects verdict/gate/local documents; [exports 59–79](../../../crates/sdk/src/contract_membrane/mod.rs#L59), [operation inventory 755–760](../../../crates/sdk/src/contract_membrane/operation.rs#L755), [gate result schema](../../../crates/sdk/schemas/2020-12/handbook.operation.contract-gate-evaluate-result/1.0.0.schema.json), [local result schema](../../../crates/sdk/schemas/2020-12/handbook.contract-local-closeout-result/1.0.0.schema.json). [SDK tests 296–458](../../../crates/sdk/tests/hcm_5_2_packet6_sdk.rs#L296) inspect append replay/conflict and verdict/gate response readmission; not target operational-action replay. |
| E-3 Engine posture authority/transaction | [facade 227–632](../../../crates/engine/src/posture_transition_facade.rs#L227) resolves exact repository/canonical authority, Charter candidate/validation/approvals, private posture/lifecycle/intent records and transaction owner. [transaction 1989–2033](../../../crates/engine/src/charter_authority_transaction.rs#L1989) locks authority, recovers journals, compares retained exact intent for replay/conflict; [2101–2210](../../../crates/engine/src/charter_authority_transaction.rs#L2101) stages durably and CAS-checks immediately before canonical replacement; [5947–6217](../../../crates/engine/src/charter_authority_transaction.rs#L5947) revalidates semantic/approval authority closure. [Engine exports 264–269](../../../crates/engine/src/lib.rs#L264), [SDK methods 86–100](../../../crates/sdk/src/lib.rs#L86). [Direct SDK tests 556–591, 749–836](../../../crates/sdk/tests/direct_typed_posture_api.rs#L556) inspect actual commit, exact replay, fault recovery and changed/stale-request refusal; [private posture tests 455–593](../../../crates/engine/src/charter_posture_transaction_tests.rs#L455) inspect rollback and synthetic/unknown authority refusals. |
| E-4 Existing receipts/uncertainty | [public posture receipt 152–170](../../../crates/engine/src/posture_transition_facade.rs#L152) is explicitly a non-durable summary of exact prior/successor canonical and transition records. [SDK owner bridge 812–869](../../../crates/sdk/src/contract_membrane/posture.rs#L812), [success projection 1070–1105](../../../crates/sdk/src/contract_membrane/posture.rs#L1070) produces two WriteReceipts for canonical truth and semantic transition under one atomic group; [request schema](../../../crates/sdk/schemas/2020-12/handbook.operation.posture-transition-apply-request/1.0.0.schema.json), [result schema](../../../crates/sdk/schemas/2020-12/handbook.operation.posture-transition-apply-result/1.0.0.schema.json). [owner effect vocabulary](../../../crates/engine/src/owner_facts.rs#L1) distinguishes incomplete, installed/durability-unconfirmed and durable outputs. These are real effect foundations; they do not by themselves prove the approved attempt-bound Mutation Receipt. |
| E-5 Artifact/credential foundations | [artifact descriptors 27–107](../../../crates/engine/src/artifact_instance.rs#L27) declare artifact-instance/capability dependencies; [artifact mutation 531–750](../../../crates/engine/src/artifact_mutation.rs#L531), [generic lineage result 178–216](../../../crates/engine/src/artifact_lineage_store.rs#L178) and [generic transaction output 2195–2318](../../../crates/engine/src/artifact_lineage_store.rs#L2195) retain mutation outputs/evidence. [credential authority admission 120–207, 303 onward](../../../crates/engine/src/authority_admission.rs#L120) binds existing exact credential/registry/action/subject/scope/time facts. Neither dependency registry nor credential admission is an operational work hierarchy or the new reliance-bound action authorization. |
| E-6 Pipeline/grounding | [route state 50–61, 196–207](../../../crates/pipeline/src/route_state.rs#L50) is routing/ref/runner/profile state; [set with effects 570–742](../../../crates/pipeline/src/route_state.rs#L570) guards revision and exposes actual write effects; [mutation journal 56–160](../../../crates/pipeline/src/pipeline_mutation_journal.rs#L56) locks operation/key, replays exact request fingerprint or refuses conflict. [Flow session adoption](../../../crates/flow/src/session_grounding.rs#L1), [Pipeline inclusion](../../../crates/pipeline/src/session_grounding.rs#L1), [SDK grounding composition 338–430](../../../crates/sdk/src/contract_membrane/session_grounding.rs#L338) preserve Engine capture outcomes; [recomposition test 607–681](../../../crates/sdk/src/contract_membrane/session_grounding.rs#L607) retains committed receipt and avoids recapture. [Flow exports](../../../crates/flow/src/lib.rs#L8), [Pipeline exports](../../../crates/pipeline/src/lib.rs#L20), [Pipeline manifest](../../../crates/pipeline/Cargo.toml#L15), [SDK manifest](../../../crates/sdk/Cargo.toml#L37), [HCM-4.6 SDK tests](../../../crates/sdk/tests/hcm_4_6_packet_5.rs#L58). |

## Behavioral coverage claims

**U-01 — Exact historical truth and distinct present use.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#historical-truth-and-present-use): resolve immutable owner result identity/content/input closure/time/validity; views, authored status and ambient latest-pass lookups cannot substitute.
**Already covered:** E-1 exact content bindings, E-2 owner DTO projection and pure gate/local computation.
**Missing in inspected path:** historical gate/explanation/result repository resolution for a separately identified action reliance; E-1 returns recomputed values over current checks, not retained historical action inputs.
Later launch checks/frontier/recovery remain part of historical resolution; a later success repairs neither unauthorized collection nor an earlier failure. Pure historical display/replay requires no reliance; pure advisory Determination needs no effect chain.
Native complete-pruning commitment is Coordination closure; bounded admission-governance establishment/activation follows its own lifecycle, without fabricated prior evaluation or recursive action requirements. These are not general configuration exemptions.

**U-02 — Exact action question, admitted policy and finite population.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#operational-reliance-policy-and-basis-formation): close kind/version, target/expected state, effect/scope and every eligibility-relevant parameter before reliance; admit category-2 definitions through deterministic policy admission.
**Already covered foundation:** E-1 exact policy registry/readmission rejects substitution; current local policy is [explicit gate-passed rules](../../../crates/contracts/policies/contract-local-closeout-policy-gate-passed-1.0.0.json).
**Missing:** action-specific policy/basis occurrence, independent selection provenance and complete finite source-result boundary in E-1/E-2/E-3. Policy-selected owners/resolvers/nominated references, roles/intent/time/congruence/cardinality/order, inclusion/exclusion and missing/duplicate/conflict/refusal accounting must precede favorable direction.
Diagnostic permission is per role, direction and action question; accepted upstream refusals must themselves resolve exactly. Bare governance links, authored safety assertions and caller subsets cannot satisfy admission or result accounting.

**U-03 — Two explicit evaluation-authorization modes.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule): every consumed result role resolves historical owner truth; when its exact closure binds an Evaluation Authorization Binding, policy explicitly selects `historical_validity_only` or `continuing_current`.
**Missing in inspected paths:** E-1 run has lifecycle/source currentness, not this exact historical Binding/mode; E-5 credential/source authority is not evaluation authorization. No universal Binding is fabricated for a result role whose closure does not bind one.
Both modes require historically valid exact Binding issuance/checkpoint lineage; invalid/unverifiable/unavailable/ambiguous/substituted history refuses. Only `continuing_current` adds fresh Engine reliance-time verification. In historical-only mode later non-currentness is not an input.
Separate action-authorization/commit policy may require fresh Engine facts even in historical-only mode. Target, current reliance-policy selection, congruence and all declared dependencies remain mandatory. Owner propositions cannot be locally reconstructed.

**U-04 — Independently governed completion, selected before collection.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#governed-completion): action domain independently establishes actual project/effect/target/scope, governing policy/selection, Assurance Selector/Posture Scope, assessment coverage and authoritative work boundary.
**Missing:** this relationship and its precollection/no-collection-commitment resolution in E-1 preparation/readmission and E-2 local-closeout invocation. It needs no future completion Intent or Determination to exist; admission to a population is not governing assignment.
Positive completion requires exact Decision-seeking eligible contract + `posture_selected` requirements + `decision_seeking` assessment with actual coverage. Direct/diagnostic passes, narrower child results, wrappers, typed exceptions and weak fresh candidates cannot waive this floor.
At reliance resolve current assignment independently; preserve coverage/governing conditions through every supported completion commit, including a generic setter with the same actual effect. Changed required content/acceptance relation may need fresh assessment; changed completion policy alone need not rerun a still-applicable assessment, and unused revisions invalidate nothing universally.

**U-05 — Atomic facets and formation refusal.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#eligibility-disposition-and-refusal-vocabulary): one Determination owns immutable `eligible/allow`, `ineligible/block`, `indeterminate/hold`, or `not_applicable/no_effect` facets and shared identity/accounting/time.
**Needs changing for target behavior:** E-1 local enum has only Eligible/Blocked; policy maps indeterminacy to blocked. E-2 transports that historical result, not an atomic target Determination.
**Missing:** separate reliance-formation refusal with no Determination/facets, plus all four fixed states. Missing identity/owner/currentness/selection is refusal; only a valid explicitly uncertain semantic input may yield hold. Facets cannot be separately persisted/superseded/replayed.
`allow` permits forming an Intent/requesting authorization; block/hold/no_effect do not mutate work, queues, priorities or success state.

**U-06 — Work Progression and authoritative work facts.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership): Work Progression owns local/parent policies, governing relationship, target/dependency closure, reliance identity/basis/Determination, accounting, replay/currentness.
**Needs changing:** E-1/E-2 local authority is Contracts, consistent with completed [HCM-5.2 historical scope](../../specs/handbook-contract-membrane/slices/HCM-5.2/README.md#local-closeout-and-parent-promotion-boundary), not target conformance.
**Missing in inspected owners/exports:** operational work identity/state/structure supply and Work Progression reliance closure. E-5 artifact lifecycle/dependencies and E-6 Pipeline routing state are different propositions, despite real mutation mechanisms.
Exact work state/relations originate from their authoritative owner; eligibility cannot invent them or issue action authorization/mutation. Physical colocation is open, with separately attributable owners. Other action domains remain bounded, not a universal enforcement owner.

**U-07 — Pure local closeout and qualifying all-pruned assessments.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#local-closeout): local Basis independently closes exact target, policy, finite results/intent, scope, mandatory governing completion and declared dependencies.
**Already covered:** E-1 pure sibling result and no work/parent/posture writes; inspected purity test asserts unchanged repository bytes.
**Needs changing:** current named rules derive Eligible from Passed plus current sources/lifecycle. The E-1 test at 3366–3497 explicitly derives Passed/NotComputed and local Eligible from an N/A verdict; this is V1 history, not the approved named all-N/A `not_applicable/no_effect` example ([owning rule 566–583](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership)). No universal N/A conversion is selected.
**Missing:** target-local governing provenance/coverage and later local action chain. Correct complete pruning may fill a qualifying assessment role under its exact outcome policy; it is not automatically pass, missing assessment, child closure or parent completion.

**U-08 — Parent population, child proposition roles and separate parent action.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action): separately form parent policy/Basis with finite authoritative relation boundary, owner/resolver/snapshot, parent, kinds/direction, reach/time cut/currentness, cycle treatment and complete membership accounting.
**Missing in E-1/E-2/E-5/E-6:** parent-target eligibility, declared relation population and role-specific exact local/state/Terminal/Receipt inputs. [Legacy parent_promotion accessor](../../../crates/engine/src/grounding.rs#L275) always returns Unavailable; it is not a parent model or evidence that HCM-4.6 remains incomplete.
Eligibility, actual child closed-state and mutation receipt are non-substitutable. Local eligible cannot satisfy missing children or parent's own governed assessment. Changed membership/child target/required local result requires new parent closure; unsupported cycles or incomplete boundary refuses.
Parent allow remains pure; promotion/advancement needs distinct exact authority/attempt/through-commit relation checks/state-owner receipt. Parent hierarchy, state machine, policy and physical owner remain unselected.

**U-09 — Intent, authorization, attempt and terminal identities.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#action-authorization-and-mutation): requester forms congruent Intent only after allow; authorization owner binds rule/mode/lineage/actor/exact Determination/effect/conditions/permitted attempts or refuses; executor allocates exact Attempt only after Binding and owner checks.
**Already covered foundation:** E-3 private intent and bounded key identify a real transaction; E-5 credential admission binds exact actor/action/scope authority.
**Missing:** full separately attributable reliance-bound Intent/Action Binding/refusal/Attempt/one durable Terminal chain in E-1–E-5 public paths. Existing private posture transaction intent and transport terminal are not proof of those semantic occurrences.
Pre-attempt authorization refusal creates no Attempt/Terminal/Receipt. Terminal distinguishes post-attempt refusal, no-mutation completion, failure and uncertainty; committed mutation is reported only by binding authoritative Receipt. Changing effect requires new reliance/Intent; changing actor/source requires new Intent/authorization; authorization-rule changes may reuse unchanged Intent.

**U-10 — Advisory authority can authorize real effects.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#authority-provenance-without-a-handbook-iam-platform): admitted action rule may accept prompt/configured-role provenance and explicitly label claimed principal/delegation/authority advisory.
**Missing for the approved new chain:** policy-selected advisory action Binding in inspected E-3/E-5 owner paths; existing credential path is a stronger bounded foundation, not a universal requirement.
**Unverified/external:** live host enforcement and future adopter authority. Ambient OS capability proves no claimed principal/delegation or action eligibility. Advisory may permit actual effect with all target/attempt/commit checks; stronger-policy facts cannot be fabricated. No new general IAM, login or additional confirmation is selected.

**U-11 — Guarded commit and authoritative receipt.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#action-authorization-and-mutation): state owner guards same current Binding, target, authorized effect, mandatory completion conditions and declared through-commit dependencies at commit/linearization, then records exact predecessor/successor/commit time/Attempt in durable Receipt.
**Already covered narrow foundation:** E-3 authority lock, preflight, immediate byte CAS and durable atomic Charter installation/recovery; E-4 real write projections; E-6 revision-guarded route writes. These are substantive mutation foundations.
**Missing for target conformance:** linkage to U-04/U-09 and causal/compare guard for every required cross-owner condition through commit. An earlier check with an unguarded gap is insufficient; inability to establish required cross-owner condition prevents commit/Receipt.
E-4 public summary/WriteReceipts and generic artifact outputs preserve actual committed evidence, but do not establish the approved exact Action Attempt-bound authoritative Mutation Receipt and executor Terminal contract. Other action-kind state-owner correctness is unverified, not inferred from Charter CAS.

**U-12 — Replay, invalidation and uncertain non-idempotent retry.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#currentness-invalidation-and-replay): same reliance/authorization/Attempt identity+content returns same owner outcome/refusal; conflict refuses; new present use has new reliance occurrence even over equal results.
**Already covered narrow foundation:** E-3 exact committed replay/conflict and interrupted-install recovery, E-6 request fingerprint replay; E-4 distinguishes installed/unconfirmed outputs from durable effects.
**Missing:** full U-02/U-09 occurrences/refusal retention and separately admitted uncertain-effect recovery. Historical replay substitutes no current policies/checks, reacquires no evidence and supplies no present authority; historical receipt never proves current target state.
Ordinary new non-idempotent attempt needs authoritative non-application, all fresh checks and exact existing Binding or successor that permits another attempt. Fresh verification is not necessarily fresh issuance. Uncertain prior application blocks ordinary retry; only admitted duplicate-safe recovery with fresh authorization accounting for possible commit permits it. Timeout, unchanged appearance, transport success/failure or recomputed eligibility supplies no non-commit proof.

**U-13 — Authored Work Specification references and causal direction.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-specification-relationship): permitted exact declared inputs require independent resolution; later revision may reference already closed Determination/facet, authorization refusal, Terminal or Receipt, without causing/reclassifying it.
**Already covered foundation:** E-5 artifact identity/lineage treats authored artifacts separately; current [canonical template](../../../crates/engine/src/canonical_artifacts.rs#L56) remains authored plan text.
**Missing:** new exact relationship roles/resolvers wired to operational records. A completed checkbox/status/prose/approval never establishes work completion; operational effect must not silently overwrite its predecessor authored revision. Concrete reference representation/storage remains deferred.

**U-14 — Forward-only posture feedback.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#posture-feedback-boundary): historical result → observation-specific reliance → independent observation admission → independently governed recommendation → transition-specific reliance → Intent → Engine action authority → guarded transition/Terminal/Receipt → later posture/evaluation.
**Already covered narrow foundation:** E-3/E-4 exact atomic Charter successor with authority/approval/replay. **Missing in inspected path:** preceding observation/recommendation/reliance responsibilities; facade recommendation reference resolves a Charter candidate/validation at 393–446, not that semantic chain. [Profile selection 461–475](../../../crates/engine/src/profile_selection.rs#L461) rejects non-null posture evaluation policy; reserved [inventory 724–728](../../../crates/sdk/src/contract_membrane/operation.rs#L724) recommendation operations are not implemented proof.
No source evaluation posture/requirements/evidence/gate may be changed. Recommendation cannot self-admit observations or establish transition eligibility. Algorithms, approvals, acknowledgement/hysteresis and host realization remain deferred.

**U-15 — Ontology, identity, owner durability and semantic-time obligations.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#ontology-placement): no introduced action object is an Artifact Instance. Kind/policies are definitions; target/Basis/Determination/refusals/Bindings/Receipt are owner results; Intent/Attempt/Terminal are durable provenance; open requests are transient; views are projections.
**Already covered foundation:** E-1 content-addressed semantic values and E-3 private durable transactions distinct from authored Charter, E-4 public representations. **Missing:** exact new reliance occurrence, owner-attributed currentness propositions and required durable request/attempt/terminal/receipt outcomes.
Retention mechanism/duration, identity allocator, repositories/resolvers and physical record count remain decisions. Action Question is a bounded identity-participating value, not mandatory standalone repository. Durably retaining semantic records does not recursively require effect authorization unless an independently admitted kind targets that lifecycle.

**U-16 — Public surface closes with the selected behavior.**
[Rule](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#semantic-responsibility-map): SDK/CLI/Tauri/Flow/Pipeline carry bounded owner truth, advisory provenance, exact refusal/currentness/replay/uncertainty/receipt; no transport selects policies, authenticates claims or creates semantic action identity.
**Already covered foundations:** E-2 current gate DTO/schema/operation export, E-3/E-4 direct and operation posture routes, E-6 completed HCM-4.6 capture/adoption/recomposition. **Needs changing with selected target behavior:** replace current Contracts local ownership/facets; close new outcome distinctions/owner verification and exact public contracts in the same increment as their behavior.
**Missing:** selected Work Progression/action public paths in inspected SDK exports/inventory/schemas. Reserved operations establish no owner implementation. **Unverified:** live CLI/Tauri/external consumer execution, published versions/adopter proof. Each selected behavior needs its appropriate owner export, SDK route/schema/catalog/manifest and consumer tests; no catch-all integration phase is justified.

## Explicit scenario dispositions

All names below refer to the owning [scenario corpus](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#discriminating-scenario-corpus), including comparisons within each row. “Foundation” preserves bounded current evidence; it does not mark target scenario coverage complete.

| Scenario | Claim / coverage disposition |
| --- | --- |
| Historical pass, current local closeout | U-01/U-04/U-07; missing target current-use chain; E-1 pure local foundation. |
| Pass insufficient under another policy: missing versus uncertain dependency | U-02/U-05; missing refusal-versus-hold semantics. |
| Failed gate enables remediation but blocks closeout | U-02/U-05; missing separate action questions/policies; historical gate remains exact. |
| Pure advisory prioritization | U-01/U-05; missing atomic pure Determination; no effect objects required. |
| Diagnostic result, remediation versus mandatory positive completion | U-02/U-04; missing role/direction/intent acceptance and completion floor. |
| Ambient favorable-result selection | U-02; missing finite operational source population/refusal. |
| Historical-only mode, later non-current versus invalid/unverifiable history | U-03; missing explicit mode and historical Binding resolution. |
| Continuing Engine dependency declared/superseded | U-03; missing required fresh owner verification/refusal. |
| Eligible but authorization refused | U-09; missing target pre-attempt refusal/no-attempt closure. |
| Authorized but ineligible | U-05/U-09; missing exact Determination-binding authority constraint. |
| Same kind/target, changed effect | U-02/U-09/U-12; missing new reliance congruence chain. |
| Target changes between allow and commit | U-11; E-3/E-6 guards already covered narrowly; full target dependencies missing. |
| Uncertain commit followed by retry | U-12; E-3 recovery foundation; general admitted recovery/authorization missing. |
| One child locally eligible | U-08; parent model missing; no inference from E-1. |
| Complete parent closure | U-08; separate pure parent Determination missing. |
| Parent omission; actual child closeout substituted by eligibility | U-08; membership and proposition-role refusal missing. |
| Dependency added after parent eligibility | U-08/U-11; parent membership recheck/new closure missing. |
| Parent eligible, promotion refused | U-08/U-09/U-11; separate authority/commit chain missing. |
| Work Specification says completed | U-13; artifact separation foundation; target operational refusal missing. |
| Later Work Specification records outcome | U-13; exact new outcome-reference path missing. |
| Compact gate projection offered for action, result unresolved | U-01/U-16; DTO carriage foundation; operational exact-result refusal missing. |
| Gate feeds posture through later stages | U-14; atomic Charter foundation; forward chain missing. |
| Historical replay after policy supersession | U-12; new reliance replay missing; source identity foundation. |
| Exact attempt replay; changed-content comparison | U-12; E-3 exact transaction replay covered narrowly; target Attempt chain missing. |
| Transport success without receipt | U-11/U-12/U-16; E-4 truthful effects foundation; target uncertain Terminal missing. |
| Explicitly permitted standalone advisory authority | U-10; full effectful advisory path missing; stronger host enforcement unverified. |
| Prompt offered where enforced authority required | U-10; E-5 credential foundation; exact new policy refusal missing. |
| Agent-authored policy candidate with bare Decision Record link | U-02; shared admission dependency; target policy closure missing. |
| Direct eligible-contract pass as required final assessment | U-04; refusal floor missing in current local route. |
| Ready posture-selected diagnostic pass | U-04; diagnostic provenance cannot substitute; target floor missing. |
| Weak fresh contract/promising name/weaker admitted policy | U-02/U-04; governing assignment independent of admission; missing. |
| Unavailable governing mapping/convenient narrower child | U-04/U-08; target refusal/no exemption missing. |
| Complete pruning considered for completion | U-04/U-07; qualifying role/policy required; current all-N/A example needs changing. |
| Governing requirements change after pass; unused revision comparison | U-04/U-11; exact applicability/coverage invalidation missing; no blanket rerun rule. |
| Qualifying assessment/reliance and ordinary advisory instruction | U-04/U-10/U-11; full permissible effect chain missing; no extra login required. |

## Future fit and bounded landing dependencies

The [phase map's rehomed work](../../specs/handbook-contract-membrane/phase-and-slice-map.md#reduced-boundary-and-rehomed-work) and [crosswalk owner seam](../../specs/handbook-contract-membrane/seam-crosswalk.md#crosswalk) preserve completed HCM-5.2 and Charter-only transition; target semantics change future ownership without reopening those exits. F-IDs below refer to [future-obligations](future-obligations.md); original authorities support each relationship.

| Claims / future IDs | Relationship and bounded landing constraint |
| --- | --- |
| U-01–U-03/U-15; F-11/F-12/F-13/F-14 | Fits admitted upstream truth direction; change needed beyond current run/lifecycle representation. Exact historical closure/resolution, policy admission, required provenance/modes and finite source population must exist before a dependent reliance can succeed. No all-API redesign or universal persistence program. |
| U-04/U-06/U-07; F-10/F-11 | Change needed: target Work Progression, authoritative work facts, independent governing relationship and precollection assessment selection precede local reliance; target/coverage must remain enforceable through selected effect commit. Choose concrete owners/mappings/policies first, not a new mandatory governing record/service. |
| U-08; F-10/F-11 | Decision-dependent work hierarchy/state/relations and parent policy, fitting separate parent direction. Parent basis depends on complete authoritative relation cut plus exact role results and own qualifying assessment. Do not infer parent from historical local, artifact dependency registry or grounding accessor. |
| U-09–U-12; F-11/F-16 | Change needed for selected action chain, fitting current owner-first mutation foundations. Congruent Intent/authority/Attempt checks and guarded state-owner commit precede truthful Terminal/Receipt/replay/recovery; exact existing Binding may support permitted retries with fresh checks. Close public owner/SDK/schema/manifests/consumer behavior alongside each selected increment. |
| U-13/U-15; F-12/F-16/F-17 | Fits authored/artifact ontology; concrete references/resolvers/storage decision-dependent. Outcome recording follows closed owner outcome and cannot become an earlier causal input or operational authority. |
| U-14; F-09/F-11 | Fits rehomed broader posture; change needed for independent observation/recommendation/transition-specific reliance around the existing guarded Charter path. Source closure precedes observation/recommendation; successor affects only later evaluation. Concrete algorithm/host remains decision-dependent. |
| U-16; F-02–F-08/F-16 | Fits completed local grounding and future consumer/publication direction. Current HCM-4.6 is completed, not an inert whole seam; external triggers/adopter/publication are separate decisions/evidence. A real consumer must preserve selected owner outcomes; external adoption need not be invented as predecessor to local semantic realization. |

## Complete deferral boundary and checks

| Owning deferred choice (chapter 1050–1065) | Consequence retained |
| --- | --- |
| Concrete local/parent/release/deployment/remediation/posture policies, thresholds and selected populations | F-09/F-10/F-11/F-17 decision-dependent; choose only what selected behavior consumes. U-02/U-04/U-08 fixed floors cannot be relaxed. |
| Work hierarchy/dependency model, work/parent state machines, actor/approver rules, authorization mechanism, mutation effects, owner placement | F-10/F-11/F-17 decision-dependent; choose exact authoritative facts/guards; do not select parent promotion, deployment product or IAM platform here. |
| Persistence/retention, identity allocation, schemas/fields/types/APIs/operations/repositories/registries/resolvers/cache/status encoding | F-16/F-17 decision-dependent; satisfy U-12/U-15 semantic identity/durability/replay without requiring a particular count or V1 compatibility. |
| SDK/CLI/Tauri/Flow/Pipeline/skills integration, UI/explanation projections | F-05/F-06/F-16/F-17 decision-dependent; U-16 closes the chosen public behavior with its owner increment, preserves uncertainty/facets/refusal and claims no live consumer proof here. |
| Task slices, acceptance, sequence and rollout; posture algorithms/notifications/acknowledgement/hysteresis/host | F-09/F-17/F-18 decision-dependent; semantic dependency ordering above is planning input, not packets or implementation authorization. Human review precedes later increment progression. |

- Read AGENTS.md, research README/future obligations, full owning chapter, guide explanations/current-product/deferrals and HR-004/HR-006; inspected cited owners/call paths/exports, schemas/policy/manifests and test bodies. All source/test claims are inspection, not fresh passing test outcomes.
- Ran `git rev-parse HEAD` (exact baseline) and `node .gitnexus/run.cjs query closeout --repo handbook --limit 3`. Query returned empty data with missing Protocol/Category FTS tables, 20-commit staleness and skipped foreign-worktree entry. No graph evidence was used and no reindex was run.
- No Rust tests, builds, Clippy, live host/UI, publication or external-adopter checks were run. Historic slice closeouts establish delivery history only. No indexed symbol was edited or commit made; impact is inapplicable to this report-only addition. Coordinator retains pre-commit detect_changes gate.
- Remaining choices are concrete policy/model/owner/representation/host selections and runtime proof of their implementation. Settled semantics, complete-pruning boundaries, advisory effects and permitted same-Binding recovery are not reopened.
