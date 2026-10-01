# Starting evaluations: coverage and landing research

Research baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`, 2026-10-01. Proposed planning input only; no product changes, implementation authority, or runtime conformance certification. The owning [Assurance and evaluation preparation chapter][A] was read in full, together with the [approval guide][G], [HR-001/004/005/006 decisions][HR], applicable collection identity/refresh rules, and current source paths below. Although individual chapter headings retain `PROPOSED`, the guide/register record human semantic approval on 2026-09-30; implementation remains separately authorized.

**Finding:** the nine-dimension posture kernel, exact Contracts run preparation, Engine Resolution authority, bounded Dock protocol, and completed grounding are foundations. They do not yet implement the complete approved pre-evaluation chain. Requirements selection, purpose-bound Engine authorization, and Contracts-domain coordination need distinct realization. Existing V1 representations impose no migration, bridge, or dual-behavior obligation.

Coverage labels describe the precise obligation, not an entire crate: **already covered**, **needs changing**, **missing**, **unverified**. “Missing” below is bounded to the inspected owner modules, exports, schemas/operation surfaces, and declarative assets; it is not derived solely from a keyword search. Test citations mean test source inspected, not executed. Future tags are **fits** (compatible foundation/direction), **change** (current path needs different semantics), and **decision-dependent** (concrete policy/placement not selected). Every future tag inherits [phase-map ownership/dependency rules][P] and the human-review boundary in the [register][HR].

## Owning-section coverage

| Chapter section/subsection (heading is the exact semantic citation within [A]) | Claims/disposition |
| --- | --- |
| Reading map; Posture and assurance selection; Canonical model and vocabulary | S-01–S-09, S-27–S-29; vocabulary separates selection, authority, truth, and effects. |
| Current product facts and semantic gap | S-01/S-07/S-16/S-32–S-35 already covered foundations; S-02–S-06 missing; S-10 needs changing. |
| Posture resolution and assurance selection invariants / Classification does not control posture | S-01 covered kernel; S-31 unverified broader workflow. |
| Posture is a scoped dimension vector | S-01 vector covered; S-02 scope missing. |
| Assurance selection precedes evaluation | S-03/S-04/S-05/S-09 missing. |
| Admission before assurance selection | S-06 missing at this seam; admission owning rules remain in Objects. |
| Assurance and Context Resolution remain orthogonal | S-08/S-14/S-19/S-20 missing; S-07 foundation covered. |
| Trust invariants do not relax | S-04/S-06/S-09/S-19; no low-posture waiver. |
| Exceptions are typed authority | S-06 missing; nomination, authorization, admission, application distinct. |
| Bounded monotonicity and vector order | S-05 missing. |
| Causal order and run binding | S-09/S-24/S-28 missing; S-34 existing atomic transition is a foundation only. |
| Operational reliance and enforcement separation | S-27 missing here; reliance report owns determinations/actions. |
| Responsibility map | S-35 foundation covered; S-14/S-17–S-20/S-27–S-29 missing roles. |
| Discriminating scenario corpus (posture) | All 17 scenarios accounted below. |
| Settled versus deferred (posture) | S-30 decision-dependent; do not select mappings, thresholds, IAM, hierarchy, placement, or feedback algorithms. |
| Complete evaluation requirements from two sources (including eight-row compatibility table and development example) | S-10 needs changing; S-03/S-11/S-12 missing; all eight combinations below. |
| Evaluation authorization, capability, and readiness / Canonical model and vocabulary | S-13–S-23 missing full objects/chain; S-16/S-25/S-26 preserve existing bounded facts. |
| Semantic owner placement and identity | S-13/S-14/S-17/S-29; no SDK ownership or Engine decision duplication. |
| Observation-plan, evaluation-purpose, and authorization lifecycle | S-13/S-14/S-15/S-22; Plan precedes Purpose, then Engine issuance. |
| Readiness, use intent, and basis formation | S-11/S-18/S-19/S-20/S-21/S-22/S-25; closed disposition rules and immutable provenance. |
| Currentness checkpoints and historical truth | S-15/S-23/S-24/S-27; separate launch, final truth, and present-reliance checkpoints. |
| Capability, operating, and observation invariants | S-16/S-17/S-26; operating equality is one adapter contract. |
| Retry, cache, supersession, and reuse rules | S-21/S-23/S-24/S-25/S-27; event cases mapped below. |
| Advisory and enforceable behavior | S-26/S-29/S-30; actual host facts separate, no general confinement program inferred. |
| Responsibility map (preparation) | S-29; missing semantic roles cannot be replaced by transport or producer behavior. |
| Discriminating scenario corpus (preparation) | All 23 scenarios accounted below. |
| Settled versus deferred (preparation) | S-30; approved rules constrain later choices, no all-future API/policy design. |

## Current evidence register and negative-evidence scope

| Key | Direct source, public surface/manifest, and inspected tests; meaning/limit |
| --- | --- |
| E1 posture | [kernel][E1] lines 16–62 fixes nine dimensions; 181–247 derives baseline/overrides and fingerprints canonical Charter/vector, and replays exactly. [tests][T1] lines 35–70 cover dimension ordering and baseline/replay; 134/179 cover one-leaf/CAS restrictions. [Charter source][C] lines 45/130–143 keeps classification and free-form stance/domain text separate; [Charter schema][CS] declares five classifications, `1-5`, baseline and dimension bounds. This kernel has no exact domain/component/operation scope resolver. |
| E2 selection/admission | [profile selection][E2] lines 475/1063 refuses non-null posture evaluation policy; [shipped profile][PF] explicitly has `posture_evaluation_policy: null`. [Contracts policy registry][EP] lines 568–682 supports Admission/Freshness/Stability/Gate/LocalCloseout, not assurance selectors, typed requirement exceptions, capability/readiness policies, or a complete metric-definition inventory. [Engine capability registry][EC] begins with artifact binding/cardinality/validator rules, not producer observation capability. Owner/module exports in E7 and declarative `crates/engine/definitions` were searched for alternate selectors/exception/readiness owners. |
| E3 Engine authority | [Context Resolution][E3] lines 699–710/824–1022 admits exact repository-bound authority and checks currentness; 1719–1809 forms envelope input; 1814–1968 closes profile/stack/parent/constraints and rejects dimension expansion; 2486 onward owns escalation/promotion registry. [exports][EX] lines 214–230 exposes this kernel. [tests][T3] lines 993/1058 cover current and historical authority chains; 2368 covers six-dimension narrowing; 2536 covers supersession. These are envelope/authority-use results, not an Evaluation Purpose/Plan-bound issuance or the proposed checkpoint observation vocabulary. |
| E4 Contracts run | [definition][E4] lines 26–61 declares exact claims, author/authority/lifecycle, matcher/gate/extensions; no permitted-use field. [run][ER] lines 75–188/466–482 binds contract/lifecycle, subject/cases/claims, requested coordinates, matcher/freshness/stability/gate/local-closeout, caller nonce, and owner lifecycle currentness. [owner preparation/readmission][EO] lines 503–678 atomically retrieves owner definition/lifecycle, rejects policy/claim substitutions, and produces/checks this run. [owner test][T4] line 1250 tests exact preparation, substitution, and stale lifecycle refusal. The whole preparation/readmission path lacks new requirements-source, intent, Observation Plan, Engine Evaluation Authorization, and readiness closures. |
| E5 Dock facts | [protocol][E5] lines 26–67 carries current run identity; 239–260 requires requested resolution equal manifest ceiling; 702–733 requires completed actual equal request/ceiling. [manifest][DM] declares supported input schema/kinds and limits. [production manifest][PM] fixes the native offline/read-only JSON Schema integration, one supported input/evidence kind and exact Resolution ceiling. [Dock admission][DA] lines 30–79 independently re-admits Contracts run, not purpose-bound Engine authority. [kernel tests][T5] lines 700–740 test exact reach and evidence-kind binding. Wider capability is not admitted here as a separately owned versioned semantic statement. |
| E6 executor/replay | [executor][E6] lines 262–408 admits bundle/request, resolves store replay, stages exact executable bytes, clears environment, uses private cwd/direct spawn, and records operational terminals; 465–595 enforces bounded timeout/cancellation/output handling. [executor tests][T6] lines 319/486/609/634 inspect direct launch, timeout, conflicting key, exact replay without execution; [store][DS] owns operational durable request/execution/candidate observations. No per-Selection original retry allowance, readiness check outcome, or semantic occurrence commitment is implemented by this operational store. Network/read-only declarations and coordinates do not establish arbitrary host restrictions. |
| E7 public boundary | [Contracts exports][CE], [Engine exports][EX], [Dock exports][DE], [SDK exports][SE], [SDK membrane exports][SM], [schema generator][SG] and parsed [schema manifest][M] were inspected. The manifest includes contract evaluation run/lifecycle, Dock, four snapshot operations, and session-grounding use-case schemas, but no new Purpose/Plan/requirements/readiness/authorization/basis roots. [Contracts Cargo][CC] depends inward on Engine; [Dock Cargo][DC] depends on Contracts/Engine. Source-wide exact-name and descriptive searches covered `crates/{contracts,engine,dock,sdk}/src`, `core`, and Engine definitions; finite exports and actual preparation/executor paths support the scoped missing claims. |
| E8 unrelated readiness | [SDK profile readiness][E8] lines 10–17/133–218/333–360 projects Setup/Doctor artifact/profile inspection into Ready/ActionRequired/Indeterminate/Invalid. [tests in same file][E8] lines 524–565 cover required versus advisory optional artifacts. This is repository readiness, not proof-plan comparison into `ready`/`known_limited`/`unknown`, Readiness Refusal, or Attempt Disposition. |
| E9 completed grounding | [Engine grounding exports][EX] lines 313–337 and [SDK grounding use case][SE] lines 330–378, plus [membrane exports][SM] lines 201–221 and snapshot/session schema entries in E7 confirm an implemented seam. [HCM-4.6 authority][H46] records completion; [phase map][P] lines 198–202 separates future automatic triggers/adopter placement. Grounding is not missing, nor does it supply evaluation authorization/readiness. Historical “unavailable/inert” summaries must not be copied forward. |

## Stable behavioral claims

Semantic citations below name exact owning headings in [A], or the collection/reliance owning anchors where explicitly cross-referenced. Evidence keys resolve to primary paths above; shared rules are not redefined here.

| ID | Required behavior and semantic citation | Coverage and evidence | Future relationship / dependency |
| --- | --- | --- | --- |
| S-01 | Preserve all nine effective levels from explicit baseline/override; classification cannot override persisted posture in resolution. [A: Classification does not control posture; Posture is a scoped dimension vector][A1]. | **already covered** for deterministic persisted vector derivation: E1 consumes no classification heuristic. Scope/selection are separate S-02/S-03; broader workflow is S-31. | **fits** reduced Phase-3 posture boundary [P]; preserve no scalar collapse. |
| S-02 | Resolve immutable, fingerprinted exact Posture Scope with Charter revision, subject boundary, all dimensions and currentness; ambiguous/stale/unsupported scope refuses. [A: scoped dimension vector][A1]. | **missing** exact scoped snapshot; E1 fingerprints Charter/vector but no scope/currentness closure; E2/E7 contain no alternate exposed resolver. | **decision-dependent** concrete scope representation and posture authority; precedes posture-selected requirements. |
| S-03 | Select exactly one admitted selector/policy before capability/reach/readiness; bind exact content/currentness and all candidates, including exclusions/refusals. [A: Assurance selection precedes evaluation][A1]. | **missing** E2/E7; null profile and finite policy registry do not implement mapping. | **decision-dependent** admitted selectors/mappings and governance; follows S-02/S-06, never capability-driven fallback. |
| S-04 | Selection closes exact contract-rooted complete inventory, compatible role-specific Evidence Evaluation Policies, configured metrics, one gate, exceptions, accounting/explanation; does not alter meaning/trust or metric scale. [A: Assurance selection; Trust invariants][A1]. | **missing** full resolved assurance closure E2/E4/E7; existing fixed exact policies are useful but narrower. | **change** requirements/final-context path; depends on Objects admission and Results exact definitions. |
| S-05 | Within fixed family/version/scope/non-posture inputs, increasing a consumed dimension cannot weaken requirements; crossed vectors remain incomparable, with explicit strength relation. [A: Bounded monotonicity and vector order][A1]. | **missing** policy relation/selection E2, not supplied by numeric posture bounds E1. | **decision-dependent** concrete admitted policy families/strength relations; follows S-03. |
| S-06 | Policy candidates follow established active-at-decision admission rules; typed exception policy, exact record authorization, admission and bounded selection application remain attributable; Charter prose cannot execute as waiver. [A: Admission before selection; Exceptions are typed authority][A1], [Objects admission][O]. | **missing** assurance/exception chain E2/E7. Existing lifecycle/constitutional authority is not this role-specific governance closure. | **decision-dependent** rule/exception contents; Objects owns establishment/cutover under HR-001, this seam consumes its outcomes. |
| S-07 | Engine owns exact full-envelope admission, currentness and non-expansion, independently of Contracts requirements. [A: orthogonality][A1]. | **already covered** envelope/authority foundation E3, including parent and six-dimensional rank checks; this does not satisfy S-14/S-15. | **fits** Engine ownership; reuse owner results without copying rank/authority decisions into Contracts/SDK. |
| S-08 | Lack of authorized or capable reach preserves selected requirements and blocks preparation; never smaller requirements, waiver, or ordinary gate indeterminacy. [A: orthogonality][A1]. | **missing** cross-owner comparison E2/E4/E8; Dock equality E5 does not prove it. | **change** launch path after exact requirements, S-14/S-17–S-20. |
| S-09 | Prospective and final contexts exactly realize one requirements identity and scoped subject; omit/substitute/add/incompatible multiplicity/unresolved dependency refuses before truth, not N/A/not-computable. [A: Assurance selection; Causal order][A1]. | **missing** new two-stage congruence; E4 already exact-checks its narrower claim/policy closure. | **change** collection/final closure dependency; collection report owns evidence/frontier and Results owns truth. |
| S-10 | Direct resolver closes nominated exact usable admitted contract plus complete compatible definitions/currentness/accounting, without fabricated posture/selector or self-admission. [A: Complete evaluation requirements][A2]. | **needs changing** E4 owner preparation provides genuine exact direct-contract groundwork but lacks complete new source closure, role/composition/metric inventory and provenance. | **change** Contracts owner before common coordination; no separate weaker evaluator. |
| S-11 | Bind owner-qualified permitted use and requested diagnostic/decision-seeking intent; reject Diagnostic-only + decision-seeking before valid Purpose/Engine request, never downgrade intent. [A: requirements compatibility][A2]. | **missing** E4 fields/path and E7 public schemas lack permitted use/intent; lifecycle Active is not Decision-seeking eligible. | **decision-dependent** contract-use qualification in Objects; then common Purpose compatibility checks. |
| S-12 | Both sources enter identical semantics; changing source/intent creates new occurrence; direct/diagnostic pass never fills mandatory completion assessment, and no automatic fallback. [A: complete requirements][A2], [governed completion][GC]. | **missing** source/intent provenance E4/E7. Current Contracts local-closeout is distinct historical delivery, not governing completion implementation. | **change** source-to-result chain; actual governing relationship must be established by action owner before collection/empty commitment. |
| S-13 | Plan completely accounts roles/multiplicity over exact ordered unique stable Producer Selections; Purpose forms afterward and binds source/use/plan/subject/objective. Occurrence identity is Coordination-owned, separate from content fingerprint and transport nonce. [A: owner placement; lifecycle][A3], [occurrence identity][I]. | **missing** E4/E5 run/nonce and protocol partition have neither stable Selection nor Purpose/occurrence contract. | **decision-dependent** allocation/representation; Contracts-domain Coordination precedes Engine issuance. |
| S-14 | Engine issues exact purpose-bound authorization after admitting full envelope and complete selection-request containment; changed lineage/plan/stack/subject/objective/use cannot transfer Binding. Public projection contains stable facts/opaque lineage, no private envelope/capsule/paths/credentials. [A: authorization lifecycle][A3]. | **missing** E3/E7 despite real envelope authority. E4/E5 coordinate DTOs cannot mint it. | **change** Engine owner issuance and Contracts consumption; choose public shape later, preserve private boundary. |
| S-15 | Immutable issuance and supersession relation differ from Engine-owned current/historical verification observations with named checkpoint/time and closed outcomes. [A: vocabulary; currentness checkpoints][A3]. | **missing** evaluation-specific observations/relations E3/E7; existing authority witnesses are a foundation. | **change** Engine verification/retention after S-14; no currentness boolean inside immutable Binding. |
| S-16 | Keep request, admitted adapter operating ceiling, and completed actual reach distinct; current first-party equality remains one operating contract. [A: capability/operating invariants][A3]. | **already covered** bounded Dock equality E5 with source/test support; generalized partial/Engine-bound admission is separately missing S-26. | **fits** existing JSON Schema integration [P]; additional integration choice may need broader ceiling or partial collection. |
| S-17 | Independently admit exact versioned producer semantic capability and stable prerequisite/retry/partial definitions; stable selections bind definitions, readiness binds checkpoint facts. [A: vocabulary; owner placement][A3], [stable/checkpoint roles][I]. | **missing** E2/E5/E7; manifest supported schemas/kinds and Engine artifact semantic capability are different scopes. | **decision-dependent** capability authority/policies and chosen producer; precedes valid readiness. |
| S-18 | Positive readiness proves complete per-role/Selection reach, capability, ceiling, host prerequisites, and composition; valid limitation => known_limited, valid inconclusive facts => unknown. It predicts neither evidence nor pass. [A: vocabulary/readiness][A3]. | **missing** E4/E5/E8 cross-owner proof-plan comparison; no namesake readiness substitution. | **change** Contracts Coordination after complete inputs; explicit policy choices S-17 required. |
| S-19 | Invalid/missing/unadmitted/stale/mismatched/unsupported inputs or failed authorization produce Refusal, not a readiness state or semantic truth. [A: readiness][A3]. | **missing** new boundary E4/E8; narrower owner/Dock refusals exist E3–E6 but do not supply this result. | **change** exact attributable refusal before launch; source failures must survive projection. |
| S-20 | Closed disposition: decision-seeking+ready attemptable; valid limited/unknown held; diagnostic+any valid assessment attemptable; failed formation/authorization refused. Basis forms only when attemptable and retains exact limitations/use. [A: readiness table][A3]. | **missing** E4/E7/E8; diagnostic semantics must not branch evaluators or relabel result. | **change** Coordination pure result/basis before launch; actual enforcing host remains separate. |
| S-21 | Same exact formation request replays identity, conflicting content refuses; same-occurrence precommit refresh only with all stable inputs/rules/Purpose/Binding exact and positively current, no commitment; old basis unused/ineligible. [A: basis formation][A3], [bounded precommit reformation][I]. | **missing** E4 nonce/currentness and E6 operational idempotency are not this lifecycle. | **change** Coordination identity/state; depends on collection owning commitment definition, not a new scheduler requirement. |
| S-22 | Complete pruning alone allows zero Selections/explicit empty requested population with real envelope authorization, positive ready, closing checks in final basis; no fabricated capability/coordinates/attempt. Exact Coordination outcome commits/seals empty frontier. [A: lifecycle/readiness][A3], [complete pruning][Z]. | **missing** E4/E5 mandatory nonempty cases/claims/evidence-kind paths contain no empty-plan authorization/readiness route; no-collection commitment is collection owner's gap. | **change** after positive applicability/accounting and action-owner governing relation; initially empty/retained observation-free claims remain deferred HR-005. |
| S-23 | Each launch/staggered Selection/retry requires fresh applicable checks including plan-wide dependencies; final truth needs separate current Engine verification; negative/unavailable checks block exact boundary and remain history. [A: checkpoints/retry][A3], [postcommit verification][V]. | **missing** E4/E5 lifecycle re-admission and E6 operational facts omit Binding/Coordination checkpoints. | **change** collection owner accounts later checks/outcomes without replacing committed formation records; Results requires final verified context. |
| S-24 | Changed fixed input/approved escalation requires earliest affected new chain; committed occurrence cannot adopt replacement basis. Same-Binding recovery requires original continuity/recovery rules and cannot erase failures or reopen sealed frontier. [A: causal order; retry/cache][A3], [V]. | **missing** new chain E4–E7; real Engine escalation registry E3 remains covered foundation. | **change** Engine successor + Coordination/collection continuity; stronger authorization never upgrades old observations. |
| S-25 | Real execution retry has new Selection-bound attempt identity/original bounded allowance; cached readiness is original checkpoint only, not fresh permission; partial/timeouts retain exact terminals. [A: basis/retry/cache][A3]. | **missing** semantic Selection allowance/continuity and exact basis-check provenance; operational replay is separately covered S-32/E6. | **change** extend collection contract without merging attempts/observations or turning completed A into authorization for B. |
| S-26 | Actual above request/authority/ceiling refuses admission, never clamps; lower actual allowed only under exact producer partial policy; host applied/refused restrictions have actual attributable facts, request/acknowledgment is insufficient. [A: capability/operating; advisory/enforceable][A3]. | **missing** generalized partial-policy and Engine-bound actual admission/host prerequisite provenance. Narrower equality rejection is covered S-16/E5; actual process controls are S-33/E6. | **decision-dependent** chosen adapter/actual host restrictions; no general hostile-code confinement/IAM obligation inferred from current Dock. |
| S-27 | Preserve historical results after authorization change. For every consumed Historical Semantic Result whose exact closure binds an Evaluation Authorization Binding, present reliance resolves that exact Binding's historical validity and declares one mode historical_validity_only/continuing_current. Accepted upstream refusal roles do not acquire invented Bindings. Cached truth/determination is not effect authority. [A: checkpoints/reuse; enforcement separation][A3], [reliance][R]. | **missing** evaluation Binding-dependent reliance E3/E4/E7; pure existing truth does not implement new reliance roles. | **change** downstream reliance owner after exact closed result; separate action authorization/target checks stay in reliance report. |
| S-28 | Gate feedback only through observation-specific reliance, separately admitted observation and recommendation; exact transition reliance eligible/allow precedes separate intent/authorization/attempt/commit/terminal/Engine receipt. P_n run never silently rebinds P_n+1. [A: causal order][A1], [posture feedback][FB]. | **missing** feedback/transition-specific reliance chain E2/E7; existing atomic transition is separately covered S-34. | **decision-dependent** broader posture authority [P: primary decisions]; preserve Engine mutation owner, no gate mutation shortcut. |
| S-29 | Exact owner projection: Contracts Coordination compares, Engine authorizes, independent owner supplies capability, Dock operates, host enforces, producer reports; SDK/CLI/Tauri carry outcomes. [A: both responsibility maps][A]. | **missing** new responsibilities' executable seams S-13–S-27/E7; existing dependency direction is covered S-35 and future adoption unverified S-36. | **change** owner seams before consumer claims [P]; no adapters become semantic owners or publication proof from repository source. |
| S-30 | Keep all listed concrete policies, scope qualification, APIs/types, storage/time/allocation/cache/retry mechanisms, host/adopter, mappings/thresholds/hierarchy and rollout deferred; generalized zero-observation/import beyond approved route remains unselected. [A: both settled/deferred sections][A], [guide deferrals][G]. | **unverified** concrete implementation choices deliberately unselected, not missing semantic decisions. | **decision-dependent** next human-reviewed bounded increment; no implementation or new task implied. |
| S-31 | Reclassification alone cannot mutate/disappear explicit posture; provisional defaults remain provisional until separately authorized persistence. [A: Classification does not control posture][A1]. | **unverified** broader authoring/reclassification workflow outside E1 kernel; no end-to-end authoring proof claimed. | **fits** explicit Charter authority; later authoring-path validation must preserve this invariant. |
| S-32 | Exact same operational request/key replay returns durable outcome without another execution; conflicting key/content refuses and a separate execution is distinct. [A: retry/replay rules][A3]. | **already covered** E6 store/executor and T6 lines 609/634; full Selection-based retry semantics remain missing S-25. | **fits** idempotent delivery foundation; do not equate operational key to semantic occurrence. |
| S-33 | Truthfully attribute actual bounded process controls/outcomes and never infer host restriction enforcement from protocol declaration. [A: advisory/enforceable behavior][A3]. | **already covered** E6 direct staged launch, environment/cwd and timeout/cancellation/output controls; no proof of general network/filesystem/process confinement. Proposed exact prerequisite/Host Enforcement Fact closure is S-26. | **fits** evidence-bounded Dock threat boundary; further exact host mechanism remains decision-dependent. |
| S-34 | Preserve separately owned Engine atomic posture mutation with exact expected authority/leaf, approval/CAS/persistence/recovery/replay. [A: Current product facts; Causal order][A1]. | **already covered** E1 preparation/CAS, [facade][PT] lines 227–617, [atomic transaction][AT] lines 2101–2189, [transaction tests][TT] lines 455/531; approved upstream feedback/action vocabulary not supplied by this foundation. | **fits** completed Phase-3/4 boundary; do not reopen history or transfer mutation to gate. |
| S-35 | Semantic owners remain inward of SDK/transports; operational Dock depends on Contracts/Engine, not vice versa. [A: responsibility maps][A]. | **already covered** E7 Cargo/module exports; full new cross-owner comparison remains S-29. | **fits** Phase-6 adapter direction; dependencies alone do not establish semantic conformance. |
| S-36 | Future consumers/host adoption must preserve exact owner results and prove actual current external use. [A: advisory/enforceable; responsibility maps][A3], [Phase-6][P]. | **unverified** actual publication/adoption/host enforcement; local exports/manifests cannot certify external state. | **decision-dependent** selected host/adopter/published versions, separate from completed E9 grounding. |

## Scenario accounting

The posture corpus is [A: Discriminating scenario corpus under Posture][A1]. Every scenario maps to the precise claim status/evidence above; existing foundation tests do not establish the missing composed scenario.

| Scenario | Required outcome / claim mapping |
| --- | --- |
| Mixed dimension levels without collapse | Retain all nine; consume only declared axes: S-01 covered vector, S-03 missing selector. |
| Classification versus explicit posture | Explicit persisted value controls: S-01 kernel covered; S-31 broader reclassification workflow unverified. |
| PII adapter versus docs-only tool | Exact separate scopes; ambiguous “area” refuses: S-02 missing. |
| Lighter assurance selection and trust | Missing provenance/unaccounted claims refuse at every level: S-04/S-06 missing full closure. |
| Selected proof plan exceeds current reach | Preserve plan and report/refuse preparation, never lighter selection: S-08/S-18–S-20 missing. |
| Posture changes mid-run | Retain P_n; new P_n+1 chain distinct: S-09/S-28 missing integration. |
| Approved Context Resolution escalation | New Binding/basis/context; unchanged requirements/history: S-24 missing composed chain, E3 escalation foundation covered. |
| Failed gate feeds posture reasoning | Observation-specific reliance then separately admitted observation/recommendation non-mutating; transition reliance/action/receipt required: S-28 missing feedback chain. |
| Relevant-dimension monotonicity | No weakening within exact policy boundary: S-05 missing. |
| Incomparable posture vectors | No global scalar/strictness guess: S-01 vector covered; S-05 selection missing. |
| Missing policy mapping | Refuse absent/ambiguous mapping; no nearest/default: S-03 missing. |
| Different enforcement, same truth | Exact distinct atomic reliance determinations, same immutable gate: S-27 missing; reliance report owns facets. |
| Free-form shortcut and red line | Prose, unauthorized record, selector-created waiver cannot select: S-06 missing valid typed chain. |
| User- or agent-authored policy candidate | Authorship/link/prompt is not owner admission: S-06 missing consumption; Objects owns admission. |
| Advisory CLI posture-transition request | Permitted advisory authority truthful; stronger policy refuses unsupported claims: S-28/S-29 missing action-chain integration; no universal IAM choice. |
| Deterministic requirement replay | Exact identity/accounting/explanation replay; equal-looking different authority distinct/refused: S-03/S-04 missing. |
| Exact resolved-requirements realization | Prospective + evidence-bound final closure both exact, mismatches refuse before truth: S-09/S-14/S-20 missing. |

Requirements compatibility ([A: eight-row table][A2]) is entirely **missing** in current source/intent/permitted-use machinery (S-11/S-12, E4/E7). Diagnostic-only with either source allows diagnostic formation only; both decision-seeking combinations refuse before valid Purpose. Decision-seeking eligible with either source allows diagnostic formation and decision-seeking formation; valid limited/unknown decision-seeking is held. Only eligible + posture-selected + decision-seeking can potentially fill mandatory completion, with actual governing selection/coverage and subsequent reliance/action checks. C1 diagnostic response-shape pass may inform C2 authoring; C3 independently admitted/selected evaluation is never a relabeled prior pass or mandatory promotion pipeline.

The preparation corpus is [A: Discriminating scenario corpus under Preparation][A3]:

| Scenario | Required outcome / claim mapping |
| --- | --- |
| Complete pruning | Explicit empty Plan, real authorization, positive ready, closing basis, empty commitment/frontier: S-22 missing. |
| New required check before no-collection commitment | Explicitly commit refreshed B1; B0 unused; no redundant recheck solely after formation: S-21/S-22 missing. |
| Empty plan without proof | Unresolved claim/shared retained role/tools absence cannot justify pruning: S-22 missing. |
| Ready multi-producer decision-seeking run | Positive complete A/B role/composition proof => attemptable; no success prediction: S-13/S-18/S-20 missing. |
| Known-limited decision-seeking run | Valid B limitation => held, no basis or A-only launch: S-18/S-20 missing. |
| Known-limited diagnostic run | Deliberate new intent/Purpose/Binding, exact limited provenance; no completion substitution: S-11/S-12/S-20 missing. |
| Valid but inconclusive capability knowledge | Valid unknown knowledge => unknown, decision-seeking held: S-18/S-20 missing. |
| Invalid readiness formation | Missing/stale/unsupported input => Refusal, no assessment: S-19 missing. |
| Coordinates without Engine binding | Refuse before basis/launch despite valid DTO/Dock acceptance: S-14/S-19 missing. |
| Same coordinates, different lineage | Nontransferable subject/objective/parent/constraints/use/stack => refuse/new chain: S-14/S-15 missing. |
| Plan changed after purpose formation | Role/Selection/composition/reach mismatch refuses; complete new chain: S-13/S-14/S-24 missing. |
| Current Dock equality | E5 equality covered; it proves neither Engine authority nor enforced restrictions: S-14 missing. |
| Ceiling above request | Positive readiness may be ready; request remains bound, no widening: S-16 adapter-specific equality covered; general S-18 missing. |
| Honest partial observation | Exact permitted B partial lineage, independent claim adequacy, no implicit merge or readiness rewrite: S-25/S-26 missing general path. |
| Actual exceeds a bound | Refuse admission without clamping: S-26 equality-covered narrower path; Engine/general bound closure missing. |
| Authorization superseded before launch | Refuse launch, retain immutable issuance/successor: S-15/S-23 missing evaluation checkpoint. |
| Authorization fails final verification after collection | Preserve attempts/observations; refuse new truth: S-23 missing, collection/Results consume boundary. |
| Authorization changes after a pass | Preserve pass; universal historical resolution, explicit continuing-status mode, separate fresh action authority: S-27 missing. |
| Approved escalation | Fresh authorization/basis/attempts/evidence/final context; prior history not upgraded: S-24 missing composed chain. |
| Per-selection retry after timeout | B distinct allowed retry, A unchanged, no merge; delivery replay only idempotent: S-32 operational replay covered, S-25 semantic allowance/Selection missing. |
| Stale readiness cache | Fresh applicable checks; precommit reformation versus immutable postcommit basis; fixed replacement new occurrence: S-21/S-23/S-24/S-25 missing composed rules. |
| Protocol-only restriction | Request/acknowledgment advisory, not enforcement: S-33 covered truthful limit of E5/E6; S-26 missing proposed Host Enforcement Fact. |
| Host cannot enforce | Required host restriction causes refusal/hold, never silently weaken/claim: S-17–S-20/S-26 missing coordination; actual host selection deferred. |

Retry/cache table event coverage is explicit: exact prelaunch request replay and new evaluation/changed plan => S-13/S-14/S-21; unchanged-basis retry, unavailable/negative/contradictory checks, partial/timeout, cache refresh => S-23–S-25; prelaunch/non-collection authorization loss and final loss => S-15/S-19/S-22/S-23; after-truth changes, supersession and result caches => S-24/S-27; approved/refused escalation => S-07/S-24; refreshed empty-route closing facts and exact/uncertain outcome replay => S-21/S-22 with collection-owned resolution. Refused escalation grants nothing and changes no otherwise-current authority. No cache can relabel diagnostics, bypass owner checks, merge identities, or authorize effects.

## Ownership and landing implications

| Cross-cutting concern | Assigned owner/boundary |
| --- | --- |
| Identity | Posture resolver owns exact scoped snapshot; Assurance/direct resolver own source closures; Coordination owns Plan/Selection/Purpose/occurrence/readiness/basis; Engine owns Binding/verification/successor. Collection owns committed basis/frontier/evidence closure and Results owns final contexts/truth. Caller nonce/content equality cannot collapse occurrences (S-02–S-04/S-10/S-13–S-15/S-21). |
| Currentness | Owners establish exact facts at named semantic time; precommit refresh S-21, postcommit continuation S-23/S-24, final closure S-23, reliance modes S-27. Formation time and later verification are separate immutable provenance. HR-006 is resolved; a fresh check alone is not a new basis. |
| Refusal | Source/compatibility/Plan formation, Engine issuance/verification, readiness formation, launch/commit/final closure each retains exact owner reason; refuse is not unknown readiness, held disposition, insufficient evidence, or gate indeterminacy (S-03/S-10/S-11/S-14/S-19/S-22/S-23). |
| Persistence/replay | Preserve exact definitions, selection accounting, issuance and historical resolvability, unused versus committed basis state, later check/outcome history and recovery. Physical stores/retention remain deferred. Existing Contracts/Dock durability is foundation, not automatic conformance for these new roles (S-21–S-25/S-27). |
| Public interfaces | Engine projection opaque/private-safe; SDK carries exact owner results and complete new identity/refusal distinctions inward-first. E7 existing generated schema/catalog discipline fits; new types/roots/operations require explicit scope authority and matching exports/manifests. Profile readiness stays a separate use case (S-14/S-18/S-29). |

Bounded dependency order for later planning: establish needed Objects admission/permitted-use definitions and exact subject/scope; close either complete requirements source; close prospective applicability and Plan/Purpose/occurrence; obtain Engine Binding; compare independently admitted capability/operating facts into readiness/disposition/basis; only then allow collection or complete-pruning commitment and final closure. A governed-completion assessment additionally needs the action-owner governing target/selection relationship before either commitment route; it does not depend on a future completion Determination, Action Intent, or action result. This is an outcome dependency map, not selected packets or permission to implement. Formation rules that affect final truth must be fixed before favorable observations.

The [current phase map][P] fits inward ownership and completed Phase-3/4/5 history, but its remaining Phase-6 consumer/publication/GUI/adapter slices are not sufficient by themselves to supply S-02–S-27. New semantic owner work needs explicit future authority before or alongside any consumer that claims it. Broader posture remains a named unresolved owner choice; external grounding trigger/adopter remains separate from completed E9. Existing JSON Schema integration stays established tooling integration, with additional integrations individually selected. Reconciliation with the concurrently produced [future-obligations report][F] is:

| Future IDs | Preparation fit/change/decision relationship |
| --- | --- |
| F-01/F-18 | Approved semantics and supervised increment review constrain all S claims; this research neither routes implementation nor creates automatic continuation. |
| F-02–F-06/F-16 | Consumer bridge/publication/adoption/GUI/workflow adapters project owner results S-14/S-19/S-29; APIs/versions/adopter remain decision-dependent. No legacy/V1 compatibility obligation inferred. |
| F-07/F-08/F-19 | E9 local grounding completed; triggers/adopter and residual reservations remain separately selected. Grounding/currentness operations are not substitute evaluation authority or mandatory reservation closure. |
| F-09 | S-01/S-02/S-03/S-05/S-28 broader scope/selection/feedback change without reopening atomic transition history. |
| F-10/F-11 | S-12/S-22 require action-owner governing relationship before collection/empty commitment; S-27/S-28 consume later reliance/action chain, with no reverse dependency on future action truth. |
| F-12 | S-06/S-10/S-11 require exact established/admitted owner definitions and permitted use; concrete qualification/governance choices precede dependent formation. |
| F-13 | S-02–S-26 are this report's detailed source-grounded preparation coverage; owners and refusal outcomes settled, concrete mappings/producers/mechanisms unselected. |
| F-14 | S-09/S-13/S-21–S-27 preserve collection-owned occurrence/commitment/frontier and Results-owned final truth; no readiness result becomes evidence. |
| F-15/F-17 | S-16/S-17/S-26/S-30 preserve adapter-specific equality and evidence-bounded host controls; further established-tool integrations and exact host/policy/storage choices need bounded authority. General hostile-code confinement, generalized observation-free evaluation and evidence import are not implicit scope. |

Concrete open choices for the next increment are the exact posture scope and assurance mappings, applicable contract-use qualification/governance rules, complete direct/selected definitions, capability authority and initial producer/host facts, public privacy-safe authorization representation, and placement/persistence of Coordination and historical resolution. These are implementation choices under resolved HR semantics, not reopened HR questions. Do not choose every future policy, timer, retry count, external adopter, or generalized observation-free path in this research.

## Verification and limits

Read-only source/export/schema/manifest searches and inspection were performed; no Rust tests/build/schema regeneration was run. Test assertions listed above are inspected evidence only. No product symbol was edited, no impact-analysis editing gate was triggered, and no commit was made. Coordinator retains precommit change-detection responsibility.

Local reference targets, code line fragments, and semantic anchors were checked for resolution; no broken reference was found. This checks navigability, not implementation behavior.

GitNexus MCP was unavailable. `node .gitnexus/run.cjs query readiness` returned a foreign-worktree registry warning, degraded FTS failures (`Table Protocol` / `Table Category` missing), empty results, and an index 20 commits behind; local metadata identifies `a35297d` (2026-08-25), before HCM-4.6. The runner also warned its current Node version was unsupported. These graph results establish neither absence nor current implementation; no reindex was attempted. An earlier multiword CLI invocation failed argument parsing; the single-concept retry above is the operative attempt. Negative coverage relies on actual finite owner paths/exports/manifests E1–E8.

[A]: ../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md
[A1]: ../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#posture-assurance
[A2]: ../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#evaluation-requirements
[A3]: ../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#evaluation-preparation
[G]: ../../specs/handbook-contract-membrane/semantic-model/README.md
[HR]: ../../specs/handbook-contract-membrane/semantic-model/review-decision-register.md
[O]: ../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#policy-admission
[I]: ../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#evaluation-occurrence-and-purpose-identity
[V]: ../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#post-commit-verification
[Z]: ../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#complete-pruning
[R]: ../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#reliance-actions
[GC]: ../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#governed-completion
[FB]: ../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#posture-feedback-boundary
[F]: future-obligations.md
[P]: ../../specs/handbook-contract-membrane/phase-and-slice-map.md
[H46]: ../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md
[E1]: ../../../crates/engine/src/project_posture.rs#L181
[T1]: ../../../crates/engine/src/project_posture_tests.rs#L35
[C]: ../../../crates/engine/src/charter_artifact.rs#L130
[CS]: ../../../crates/engine/definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json
[E2]: ../../../crates/engine/src/profile_selection.rs#L475
[PF]: ../../../crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml
[EP]: ../../../crates/contracts/src/evaluation_policy.rs#L568
[EC]: ../../../crates/engine/src/semantic_capability_registry.rs#L41
[E3]: ../../../crates/engine/src/context_resolution.rs#L1719
[T3]: ../../../crates/engine/tests/context_resolution_kernel.rs#L2368
[E4]: ../../../crates/contracts/src/definition.rs#L26
[ER]: ../../../crates/contracts/src/evaluation_run.rs#L75
[EO]: ../../../crates/contracts/src/owner_repository.rs#L503
[T4]: ../../../crates/contracts/tests/owner_repository.rs#L1250
[E5]: ../../../crates/dock/src/protocol.rs#L239
[DM]: ../../../crates/dock/src/manifest.rs
[PM]: ../../../crates/dock/production-manifests/x86_64-pc-windows-msvc/manifest.json
[DA]: ../../../crates/dock/src/run_admission.rs#L30
[T5]: ../../../crates/dock/tests/packet1_kernel.rs#L700
[E6]: ../../../crates/dock/src/executor.rs#L262
[T6]: ../../../crates/dock/tests/packet3_executor.rs#L634
[DS]: ../../../crates/dock/src/store.rs
[CE]: ../../../crates/contracts/src/lib.rs
[EX]: ../../../crates/engine/src/lib.rs#L214
[DE]: ../../../crates/dock/src/lib.rs
[SE]: ../../../crates/sdk/src/lib.rs#L330
[SM]: ../../../crates/sdk/src/contract_membrane/mod.rs#L201
[SG]: ../../../crates/sdk/src/contract_membrane/schema_generation.rs#L7389
[M]: ../../../crates/sdk/schemas/manifest.json
[CC]: ../../../crates/contracts/Cargo.toml
[DC]: ../../../crates/dock/Cargo.toml
[E8]: ../../../crates/sdk/src/profile_readiness.rs#L133
[PT]: ../../../crates/engine/src/posture_transition_facade.rs#L227
[AT]: ../../../crates/engine/src/charter_authority_transaction.rs#L2101
[TT]: ../../../crates/engine/src/charter_posture_transaction_tests.rs#L455
