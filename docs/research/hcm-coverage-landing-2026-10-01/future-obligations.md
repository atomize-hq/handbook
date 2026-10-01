# Future HCM obligations and decision boundaries

Research baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`, checked against
workspace HEAD on 2026-10-01. Scope: remaining outcomes, dependencies, explicit
deferrals, compatibility constraints, and delivery routing. This is proposed
planning input, not implementation authority or runtime certification. Only this
report was written. The [research brief](README.md#assignment-and-authority)
supplies the supervised delivery requirement.

**Disposition vocabulary:** **settled** means approved semantic meaning or a
current architectural constraint; **decision required** means concrete realization
is still unselected; **completed** preserves a recorded delivery boundary;
**superseded/rejected** excludes old assumptions from the future queue. Coverage
labels describe evidence here: **already covered**, **needs changing**, **missing**,
or **unverified**. In particular, an unverified implementation is not proven
missing. References below resolve from this report; line locations are at the
baseline.

## Sources read and disposition inventory

| Current source / sections read | Disposition and claims |
| --- | --- |
| [Program README](../../specs/handbook-contract-membrane/README.md): current status, authority/navigation, historical recovery | Authority classification; completed history preserved; bounded historical recovery only. F-01, F-19. |
| [Phase map](../../specs/handbook-contract-membrane/phase-and-slice-map.md): status/order, prior orientation, Phases 3–6, all unresolved-decision rows | Complete remaining roadmap inventory and rehomed work. F-02–F-11, F-15–F-16. |
| [Architecture](../../specs/handbook-contract-membrane/architecture-context.md): topology, owner direction/placement, projection/adapters, trust, gate/dependency/posture, external proof, drift | Stable constraints; stale summaries qualified, not adopted as current truth. F-01, F-07–F-19. |
| [Crosswalk](../../specs/handbook-contract-membrane/seam-crosswalk.md): labels, every seam row, drift | Routing constraints rather than a work queue. F-07–F-16, F-19. |
| [Baseline](../../specs/handbook-contract-membrane/implemented-baseline.md): scope/status, domain table, Phase-3 residuals, HCM-4.1/4.2 and current anchors | Historical/product checkpoint; narrow source checks below, no blanket recertification. F-07–F-11, F-16, F-19. |
| [Semantic approval guide](../../specs/handbook-contract-membrane/semantic-model/README.md): approval, complete-path explanations, current/greenfield distinction, complete explicit deferrals, source history; [register](../../specs/handbook-contract-membrane/semantic-model/review-decision-register.md#non-blocking-current-context-follow-up), approval/resolution references | Approved meaning, implementation separate; HR-001–HR-006 are resolved, remaining concrete choices are intentional deferrals. F-01, F-09–F-19. |
| [Objects](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md): policy admission/establishment, permitted use, ontology settled/deferred (665–736), authored relationships settled/deferred (1264–1327) | Governance/ontology/artifact boundaries fixed; concrete realization open. F-12, F-17. Detailed conformance belongs to the rules report. |
| [Preparation](../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md): causal order (343–410), complete requirements (528–615), both settled/deferred sections (485–525, 1138–1175) | Assurance, authorization and readiness remain distinct; posture feedback forward only. F-09, F-13, F-17. Detailed conformance belongs to starting evaluations. |
| [Evidence](../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md): both settled/deferred sections (1010–1085, 1498–1544) | Collection identity/closure and result/explanation commitments fixed; explicit expansions remain deferred. F-14, F-17. Detailed conformance belongs to collection and results reports. |
| [Actions](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md): Work Progression (529–583), parent model (623–705), posture feedback (843–877), settled/deferred (989–1065) | Work Progression responsibility and pure local/parent determinations fixed; concrete model, effects and placement unselected. F-09–F-11, F-17. Detailed conformance belongs to reliance/actions. |
| [HCM-4.6](../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md): current/outcome, source matrix, outward closure, adopter handoff, non-goals | Completed local seam; four residual reservations deliberately unclosed; host/adopter remains future. F-07–F-08. |
| [HCM-5.2](../../specs/handbook-contract-membrane/slices/HCM-5.2/README.md): local/parent boundary (483–503), non-goals (792–812); [HCM-5.3](../../specs/handbook-contract-membrane/slices/HCM-5.3/README.md): threat boundary (103–153), exact 5.4 handoff (553–580); [HCM-5.4](../../specs/handbook-contract-membrane/slices/HCM-5.4/README.md): non-goals (667–690) | Preserve slice-scoped history. Completed successor delivery disposes of 5.3's 5.4 handoff; excluded confinement is not a successor obligation. F-10–F-11, F-15, F-19. |
| [Lifecycle](../../specs/handbook-contract-membrane/work-unit-lifecycle.md), [review](../../specs/handbook-contract-membrane/review-and-closeout-policy.md), [authority prompt](../../specs/handbook-contract-membrane/authority-preparation-prompt.md), [task prompt](../../specs/handbook-contract-membrane/task-session-prompt.md), [meta prompt](../../specs/handbook-contract-membrane/meta-task-prompt.md), [active router](../../../.codex/skills/orchestrate-top-level-tasks/SKILL.md), read in full | Evidence and completion boundaries remain useful; automatic continuation conflicts with this user's supervision requirement. F-18. These were inspected as research sources, not invoked to dispatch work. |
| `git show b0e73fc` (phase-map row; HCM-5.3 threat/invariant passages) | Exact current correction: established-tool integrations, not arbitrary executables/general confinement. F-15, F-19. No wholesale pre-reset archive scan or adoption. |

## Remaining roadmap outcomes

**F-01 — Approval and authority. Settled; already covered as documentary authority.**
The [semantic guide, lines 11–49](../../specs/handbook-contract-membrane/semantic-model/README.md#approved-decision)
records human approval on 2026-09-30, including ownership, currentness, refusal and
scenario outcomes. Approval supplies planning input only. The [phase map,
lines 40–57](../../specs/handbook-contract-membrane/phase-and-slice-map.md#ordering-and-authority-rules)
supplies hard predecessors, with the preceding listed slice normally required and
phase exit normally required before next-phase implementation. A roadmap entry
never authorizes execution. Detailed choices and implementation authorization
remain necessary; documentary completeness does not establish runtime conformance.

The complete Phase-6 inventory is the [five-row table, lines 182–196](../../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption).
No Phase-6 slice authority was found in the current `slices/` file inventory.
Their local implementations are **unverified** here, not declared absent from a
keyword search. External state is **unverified/external-unknown** throughout.

| ID / roadmap state | Active outcome and remaining choice | Dependency and compatibility boundary |
| --- | --- | --- |
| **F-02 — HCM-6.1; decision required** | A replaceable, isolated, version-bound JSON CLI bridge at a real Substrate product seam. Choose the actual external adopter/seam; local bridge code alone cannot satisfy adoption. | Phase-5 exit is recorded satisfied; adopter decision is still required. Bridge preserves SDK/owner truth and exact transport contracts. It does not become a semantic owner. |
| **F-03 — HCM-6.2; decision required** | Publish exact affected SDK/owner API versions and obtain current external-consumer proof. Publication mechanism, artifacts and versions remain unresolved. | Default preceding-slice closeout plus publication decision; local manifests/source do not prove publication. Version selection must follow the eventual model, without imposing V1/legacy compatibility. |
| **F-04 — HCM-6.3; decision required** | Direct Substrate registry adoption against exact published versions, then an explicit bridge-replacement decision. | Published APIs and current external-owner participation; actual consumer proof precedes replacement claims. Adoption is not inferable from local tests or historic publication. |
| **F-05 — HCM-6.4; planned** | Tauri product GUI for artifact, snapshot, projection, contract, execution and evidence workflows over the same SDK capabilities. Concrete product/UI realization remains unselected. | Default preceding-slice closeout and inherited Phase-4/5 exits. Preserve owner decisions, disclosure, typed refusal/currentness, receipts and replay; no frontend semantic authority. A completed non-UI facade is only its local predecessor. |
| **F-06 — HCM-6.5; decision required** | Bounded workflow-adapter manifests and semantic mappings for profile, vocabulary and Resolution translation. Select the adapter owner and exact mapping boundary. | Default preceding-slice closeout plus adapter-owner decision. Translation must preserve meaning and exact admitted bindings; no marketplace or broad adapter inventory is implied. |

Phase-6 exit requires current proof for **selected** local/external consumers;
[unselected external consumers are not an exit failure, lines 198–200](../../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption).
These are current navigation dependencies, not a newly chosen implementation
sequence. Concrete authority must resolve selection and any permitted dependency
exception before implementation.

**F-07 — Grounding host/adopter. Local handoff completed; host realization decision required.**
Coverage: **already covered** for the narrowly inspected operation/SDK declarations;
**unverified** for end-to-end production behavior and actual external adoption.
[HCM-4.6 handoff, lines 488–511](../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md#phase-6-adopter-handoff-and-proof-boundary)
requires later authority to name trigger host, trusted repository/workspace/session
binding, exact start/end events, cancellation/delivery behavior, UX/operations
owner, deployment boundary and adoption evidence. It receives four ordinary
operations and a fixed non-discoverable session-boundary SDK composition. The
host must preserve start and final-delta/handoff results, recover a committed
receipt without recapturing, and never supply predecessor/currentness/grounding
truth. No particular IDE, watcher, agent host or background service is selected.

Source checks: [Engine exports, lines 313–341](../../../crates/engine/src/lib.rs#L313),
[four-operation definitions, lines 13–26 and dispatcher 294–322](../../../crates/sdk/src/contract_membrane/snapshot_operation.rs#L13),
[SDK fixed use case/recomposition, lines 330–379](../../../crates/sdk/src/lib.rs#L330),
and [public SDK projection exports, lines 205–218](../../../crates/sdk/src/contract_membrane/mod.rs#L205)
establish a real declared local seam, superseding blanket “grounding unavailable”
claims. [Packet-5 test source, lines 58–129](../../../crates/sdk/tests/hcm_4_6_packet_5.rs#L58)
asserts `62 / 35 / 180 / 1 / 7` inventory/capability/schema/manifest/use-case truth;
it was inspected, not run. [The source matrix, lines 194–221](../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md#production-source-and-currentness-matrix)
already closes the five HCM-5 observation slots inside the existing `evidence`
family. Do not create a remaining “connect HCM-5 evidence to grounding” obligation
from an older inert placeholder. Matrix runtime behavior was not independently
checked here.

**F-08 — Residual projection/snapshot reservations. Decision-dependent, not mandatory closure.**
Coverage: **already covered** for truthful omission in inspected inventory/admission
selection and tests; any future independent consumer/closure is **unverified**.
[HCM-4.6, lines 278–283](../../specs/handbook-contract-membrane/slices/HCM-4.6/README.md#exact-outward-seam-and-inventory-delta)
explicitly leaves `projection.definition.read`, `projection.create`,
`snapshot.verify_current` and `snapshot.resolve_applicable` absent: necessary
semantics are private policy/currentness/predecessor work and no independent
ordinary consumer justifies closure. [Inventory rows/selection, lines 683–835](../../../crates/sdk/src/contract_membrane/operation.rs#L683)
and [omission assertions, lines 27–32, 78–86](../../../crates/sdk/tests/hcm_4_6_packet_5.rs#L78)
support that boundary. Reservations are not authority, a full-62 closure target,
or a reason to expose raw snapshots. Any later ordinary projection workflow must
first establish owner, consumer need and complete admission/schema/transport
closure; F-05's projection workflow does not itself select those four operations.

## Rehomed Phase-3 work and approved semantic dependencies

**F-09 — Broader posture and feedback. Settled boundaries; concrete policies/host decision required.**
Coverage: **unverified** for complete semantic conformance; **already covered** for
the narrow current null-policy refusal. The [reduced Phase-3 boundary,
lines 91–109](../../specs/handbook-contract-membrane/phase-and-slice-map.md#reduced-boundary-and-rehomed-work)
rehomes multi-source posture evaluation, recommendations, notifications,
acknowledgements and hysteresis to later posture authority. It preserves the
completed Charter-only atomic transition rather than reopening Phase 3.
[Profile resolution, lines 461–475](../../../crates/engine/src/profile_selection.rs#L461)
explicitly rejects non-null `posture_evaluation_policy`; this alone does not prove
the absence of every future posture component.

[Approved causal order, preparation lines 343–410](../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#causal-order-and-run-binding)
and [action chapter lines 843–877](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#posture-feedback-boundary)
fix the future path: exact historical result → observation-specific reliance →
separately admitted observation → separately owned recommendation →
transition-specific atomic reliance → exact Intent → Engine authorization →
final approvals/current-state/commit checks → terminal and committed receipt →
posture for a **later** evaluation. Gate truth, recommendation, eligibility and
mutation remain separate. Notification/acknowledgement/hysteresis model,
algorithms, policies, approvals, schemas/APIs, physical placement, host integration
and rollout remain unselected. None may alter the source evaluation's posture,
requirements or historical result. This depends on F-12–F-14 and the action floor
F-11; it need not wait on an arbitrarily chosen external adopter.

**F-10 — Work Progression and parent/dependency model. Settled semantic owner; realization decision required.**
Coverage: current Contracts local-closeout composition **already covered** by narrow
source inspection; target Work Progression implementation **unverified**. The
[owning rule, lines 529–583](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
assigns exact work/dependency closure, local and parent policy admission, governing
completion selection, atomic reliance determinations, accounting/currentness and
replay to Work Progression. Exact work identities, state and relations come from
the authoritative work-structure/state owner. Physical co-location is allowed
only with separately attributable responsibilities. The greenfield owner does
**not** retain Contracts' `LocalCloseoutResultV1` as local-closeout authority.

[Parent rules, lines 623–705](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action)
require a finite authoritative relationship boundary: owner/resolver, exact
parent, supported relation kinds/directions, declared reach, semantic-time cut,
currentness, cycle treatment and deterministic complete accounting. Policies
must distinguish child eligibility, actual closed-state facts, terminals and
receipts. Missing/ambiguous/cyclic/changed membership cannot be inferred satisfied.
Local eligibility cannot establish parent eligibility, parent eligibility cannot
commit promotion, and each governed parent action needs its own governing
assessment. Work hierarchy/dependency model, state machines, concrete result
populations, policies, target bindings, work owner, action/state owner placement,
consumers and slice remain open; no parent-promotion implementation is selected.

[HCM-5.2 lines 483–503](../../specs/handbook-contract-membrane/slices/HCM-5.2/README.md#local-closeout-and-parent-promotion-boundary)
fix historical Contracts-owned pure local closeout and exclude parent machinery.
[Contracts export, lines 63–66](../../../crates/contracts/src/lib.rs#L63) and
[composition, lines 175–286](../../../crates/contracts/src/gate_composition.rs#L175)
show the current gate/local result without a work mutation. This is limited
inspection, not proof of full owner currentness/recovery. [Legacy grounding
accessors, lines 275–280](../../../crates/engine/src/grounding.rs#L275) always return
`Unavailable`; they do not define future Work Progression requirements.

**F-11 — Governing completion, reliance and effects. Settled floor; concrete action choices open.**
Coverage: **unverified** for implementation of the approved chain. [Settled rules,
action chapter lines 993–1048](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#settled-versus-deferred)
require exact action kind/target/effect/scope closure, one atomic eligibility and
fixed disposition determination, distinct formation refusal, exact historical
Evaluation Authorization Binding validity for consumed result roles whose closure
binds one, explicit `historical_validity_only` or
`continuing_current` reliance mode, separate fresh action authorization, exact
attempt, through-commit target/currentness checks, terminal and state-owner
Mutation Receipt. Uncertain effect/commit cannot authorize ordinary non-idempotent
retry without authoritative non-application or admitted duplicate-safe recovery
with fresh authorization.

Governed completion independently establishes actual target/coverage and requires
Decision-seeking eligible contract, posture-selected requirements and
decision-seeking assessment before collection/no-collection commitment, preserved
through effect commit. Direct/diagnostic/all-pruned history cannot silently replace
that assessment. Permitted advisory instruction may authorize a real effect while
preserving exact checks and truthful provenance; no general IAM platform is
selected. [Deferred realization, lines 1050–1065](../../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#settled-versus-deferred)
leaves actor/approver rules, action policies, effects and mechanisms open. Release,
deployment, environment change, remediation and posture each require their own
bounded owner/policy; the shared vocabulary does not commission universal action
products. Dependencies are F-10 for work actions, F-09 for posture and F-12–F-14
for admitted upstream facts.

**F-12 — Governance, ontology and authored inputs. Settled dependency; concrete realization open.**
Coverage: **unverified**, detailed source conformance assigned to rules/definitions.
[Shared admission, lines 18–167](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#policy-admission)
requires deterministic owner-side closure, exact active project configuration and
owner rule, finite candidate accounting, relevant governance inputs and mechanical
non-relaxable checks. Fixed Handbook establishment authority, prior-authority
replacement, immutable semantic identity, activation cutover/history, committed
activation and uncertain-effect treatment precede dependent admissions. Candidate
self-authorization, bare links, artifact promotion and open-ended AI judgment are
not substitutes. Permitted standalone setup instruction needs no obligatory
Decision Record, login or second prompt.

[Permitted use, lines 182–235](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#contract-permitted-use)
requires exact Contracts qualification of each Diagnostic-only or Decision-seeking
eligible revision; changed permitted use requires newly admitted semantic content.
[Ontology settled section, lines 667–711](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#settled-versus-deferred)
and [authored settled section, lines 1266–1302](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#authored-relationships)
fix six categories, six authored families, Work Specification, owner-specific
durability where required, optional shared primitives and non-authoritative
catalogs. Exact artifact references do not transfer authority; profile conformance
and semantic-input requiredness differ. Concrete fields, qualification criteria,
governing mappings, resolvers, cardinalities, placement and storage must be selected
before affected dependent operations can succeed. This is an upstream requirement
for F-09–F-14, not a seventh artifact family or a universal persistence program.

**F-13 — Assurance and evaluation preparation. Settled dependency; concrete policies/producer choices open.**
Coverage: **unverified** here; starting-evaluations report owns detailed code evidence.
[Complete requirements, lines 528–615](../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#evaluation-requirements)
fixes both `posture_selected` and `direct_contract` sources, exact one-contract
rooted complete requirements, immutable source/permitted-use/intent provenance
and refusal before incompatible Purpose formation. Assurance precedes capability;
capability cannot weaken requirements. [Settled preparation, lines 1140–1159](../../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#evaluation-preparation)
fixes Contracts-domain Evaluation Coordination, Engine authorization, plan and
selection closure, readiness/basis formation, current verification checkpoints,
diagnostic/decision behavior and capability/ceiling/actual-reach/enforcement
separation. Concrete assurance mappings, requirements, producer/capability policies,
host constraints, enforcement, retry/timeouts/caches and physical placement remain
open. Dependent collection and consumer invocation must retain exact provenance
and refusal; a declaration of operating limits cannot certify host enforcement.

**F-14 — Collection closure, evaluation truth and explanation. Settled dependency; realization open.**
Coverage: **unverified** here; collection/results reports own detailed code evidence.
[Identity settled section, lines 1014–1054](../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#identity-evidence)
fixes occurrence versus content/delivery/attempt identity, immutable basis
commitment, separately recorded later launch checks, complete-pruning entry and
no-collection commitment, frontier, deterministic complete evidence membership,
final context, semantic times, historical resolution and replay. Complete pruning
is a bounded supported route, not generalized observation-free evaluation.
[Result settled section, lines 1500–1519](../../specs/handbook-contract-membrane/semantic-model/evidence-and-evaluation.md#metric-gate)
fixes independent claims/metrics/gates, policy compatibility, gate purity and
precedence, complete accounting and canonical explanation. One owner truth supports
concise explanation and authorized drill-down; density/disclosure is independent
of evaluation and never causes reevaluation. Concrete metric algorithms/catalogs,
thresholds, selected populations, evidence-formation policy, explanation inventory,
names/omissions/density/UI and storage remain open. These closures are dependencies
for F-09–F-11 and future consumers; favorable displayed output cannot replace them.

## Integration, compatibility and deferred choices

**F-15 — Additional established-tool Dock integrations. Decision required; confinement rejected.**
Coverage: first JSON Schema integration is **completed** in current slice records;
its complete runtime path and further integrations are **unverified** here.
[Current unresolved row](../../specs/handbook-contract-membrane/phase-and-slice-map.md#primary-unresolved-decisions)
requires later authority to select each established-tool integration's scope and
observation-admission requirements. No particular next tool, second manifest,
marketplace or integration count is selected. [HCM-5.3 threat boundary,
lines 137–146](../../specs/handbook-contract-membrane/slices/HCM-5.3/README.md#threat-model-and-enforcement-boundary)
and correction commit `b0e73fc` expressly exclude arbitrary third-party executable
admission and general hostile-code confinement as inferred successor requirements.
The declared portable host controls remain evidence-bounded. Future producer
observations must cross independent semantic-owner admission and the approved
capability/authorization/readiness chain; neither tool status nor Dock execution
may construct semantic evidence, gate, local/parent eligibility or mutation truth.
New integrations depend on F-12–F-14 and any selected host/adapter rules, not on a
general filesystem/network/syscall/process-tree/CPU/memory sandbox program.

**F-16 — Public seams and future compatibility. Settled constraints; concrete surface decisions open.**
Coverage: inward crate dependency edges **already covered** by inspected
[workspace](../../../Cargo.toml#L1), [Contracts](../../../crates/contracts/Cargo.toml#L21),
[Dock](../../../crates/dock/Cargo.toml#L31) and [SDK](../../../crates/sdk/Cargo.toml#L37)
manifests; full API/current consumer behavior **unverified**.
[Architecture owner direction](../../specs/handbook-contract-membrane/architecture-context.md#semantic-ownership-and-dependency-direction)
and [adapter/trust boundaries](../../specs/handbook-contract-membrane/architecture-context.md#projection-and-adapter-boundary)
keep owners inward of SDK and transports, independent owner admission distinct
from structural validity, and instance identity distinct from schema-byte identity
and transport framing. SDK composes owner truth; generic CLI/Tauri/skill consumers
preserve exact terminal/currentness/replay/receipt outcomes and do not invent
operation identity. Current public snapshot declarations checked in F-07/F-08 are
inputs for future consumers, not proof of every promised future workflow.

[Phase-map inventory-expansion row](../../specs/handbook-contract-membrane/phase-and-slice-map.md#primary-unresolved-decisions)
requires separate authority and complete schemas/admission for new semantic
operations/use cases. Other omitted inventory rows (including posture, profile,
vocabulary, Resolution, artifact/render/record and setup reservations in
[inventory lines 702–763](../../../crates/sdk/src/contract_membrane/operation.rs#L702))
are **decision-dependent candidate seams**, not mandatory closure because reserved.
Fixed use cases remain outside ordinary discovery under their existing boundary;
future closure needs a real consumer, owner and explicit decision. Do not invent
CLI posture commands, special snapshot commands or skill-specific semantic paths.

[Greenfield approval guide, lines 359–381](../../specs/handbook-contract-membrane/semantic-model/README.md#current-product-versus-proposed-model)
selects no V1 shape/alias, migration bridge, dual-write or dual behavior. Historical
CLI legacy support and completed exact counts are evidence of prior selected
scope; they do not impose greenfield compatibility. Semantic ownership, exact
bindings, historical resolvability and truthful outcomes remain obligations even
when representation changes. Future publication/version/cutover choices remain
explicit decisions rather than assumed migration chores.

**F-17 — Complete explicit deferral inventory. Decision required, not extra approved features.**
Coverage: **already covered** as explicit documentary deferrals; implementation
choices **unverified/unselected**. The [guide's complete list, lines 383–411](../../specs/handbook-contract-membrane/semantic-model/README.md#explicitly-deferred)
is elaborated by all seven owning settled/deferred sections mapped above:

| Deferred choice family | Owning passages and bounded implication |
| --- | --- |
| Policy contents and selection | Objects 729–736, 1321–1327; Preparation 504–511, 1167–1170; Evidence 1079–1080, 1521–1528; Actions 1050–1057. Policies/selectors/mappings, requirement sets, contract qualification, governing project/effect/target mappings, typed exceptions/actors/approvers, metrics/thresholds/algorithms, selected producers and result populations are open. Required inputs must be selected before dependent admission/completion; this report supplies none. |
| Physical realization and public contracts | Objects 729–736, 1321–1325; Preparation 1161–1167; Evidence 1074–1079, 1521–1524; Actions 1059–1063. Fields/schema/types/APIs/operation names, owners' physical placement, repositories/storage/catalogs/registries/resolvers/indexes, transports/encodings/status codes, schema generation and expression languages remain open. Prescribed semantic identity is not prescribed runtime type count. |
| Identity/time/durability/operations | Evidence 1074–1079; Preparation 1164–1166; Actions 1053–1061. Allocation/clock technology, retention duration/policy, caches, retries/timeouts, activation concurrency/recovery and action effect/authorization mechanisms remain open. Exact historical resolution, uncertainty and through-commit rules still constrain their design. |
| Work and posture | Actions 1050–1065; Preparation 509–524. Work hierarchy/dependency model, work/parent state machines, physical Work Progression/action/state owners, parent bindings, policies/effects, posture observations/recommendations/transition algorithms, notification/acknowledgement/hysteresis and host integration remain open. F-09–F-11 fix the responsibilities. |
| Explanation and UI | Objects 1315–1325; Evidence 1521–1528; Actions 1062–1063; guide 401–403. Inventory/names/omissions/density-level count, disclosure implementation, UI layouts/interactions and concrete representations remain open. Canonical truth, concise default, authorized drill-down and independent disclosure are settled. |
| Semantic expansions expressly unselected | Evidence 1029–1033 defers relation-level obligation applicability; 1081–1085 and Preparation 1161–1163 defer initially empty/generalized observation-free evaluation, retained observation-free claims, cross-occurrence evidence import/reuse and later closure-integrity repair/quarantine/incident response. History/accounting does not authorize these expansions. |
| Delivery and external realization | All owning deferral sections and guide 404–411: task decomposition, acceptance, sequence, integration, cutover/migration choices, feature flags and rollout; publication and Substrate/other adopter selection remain open. Phase-6 supplies direction without choosing them. No metric marketplace, universal confidence score or general IAM program is selected. |

These choices may be necessary for a selected outcome; their listing does not make
every option an implementation obligation. Landing should preserve the approved
dependencies: governance/qualified inputs → complete requirements → Engine
authorization and readiness → immutable collection/closure → canonical truth →
current-use reliance → separately authorized effects/receipts. Any posture feedback
starts a later context; any parent result uses its own exact relation closure.
This partial semantic order leaves packet decomposition and scheduling open.

## Delivery mismatch and retired premises

**F-18 — Supervised increment review versus automatic continuation. Needs changing for later delivery.**
The [research brief](README.md#assignment-and-authority) explicitly records the
user's required human review of each later implementation increment **before
progression**. Existing [lifecycle line 33 and transition table](../../specs/handbook-contract-membrane/work-unit-lifecycle.md#execution-modes)
delegates continuation without packet confirmation; [meta prompt lines 23, 51
and 78](../../specs/handbook-contract-membrane/meta-task-prompt.md#consume-compact-results)
directs immediate next-packet dispatch; [router meta steps](../../../.codex/skills/orchestrate-top-level-tasks/SKILL.md#meta-task)
operationalize automatic sequencing; [task prompt line 51](../../specs/handbook-contract-membrane/task-session-prompt.md#prepare-delegated-packet-authority-before-product-edits)
continues directly from delegated preparation into implementation. That route must
be reconciled with the user's supervision before later execution. Research does
not amend these files, select increment/packet boundaries or authorize a dispatch.
A later concrete routing decision must preserve human review at the requested
increment boundary; semantic approval or a clean automated review cannot bypass it.

[Review policy](../../specs/handbook-contract-membrane/review-and-closeout-policy.md#causal-review)
still requires proportionate checks, one controlling focused review when applicable,
P1/P2 remediation and review of changed subjects, with P3/P4 advisory. Packet
completion is distinct from one aggregate slice closeout. Human supervision does
not require restoring ledgers, proof walls, fixed review bursts, mechanical
commits or archived orchestration state. No product/control-plane edit is made
by this finding.

**F-19 — Superseded/rejected premises and source reconciliation. Needs changing in summaries; settled exclusions.**

| Premise | Current disposition and evidence |
| --- | --- |
| Full historical Phase-3 aspiration is unfinished Phase 3 | Superseded by the selected reduced exit; local-closeout moved to HCM-5.2, ordinary grounding to completed HCM-4.6, external adoption to Phase 6 and broader posture to later authority. [Phase map](../../specs/handbook-contract-membrane/phase-and-slice-map.md#reduced-boundary-and-rehomed-work). No reopening of exited phases follows. |
| Projection/snapshot/grounding is wholly precursor-only, current witness is still 31/170/6, or Dock is currently absent | Stale general summaries. [Architecture topology](../../specs/handbook-contract-membrane/architecture-context.md#reading-the-topology) conflicts with its later HCM-4.6 section; [crosswalk evidence row](../../specs/handbook-contract-membrane/seam-crosswalk.md#crosswalk) contains a slice-scoped Dock-absence phrase despite completed Dock rows. [Register lines 430–443](../../specs/handbook-contract-membrane/semantic-model/review-decision-register.md#non-blocking-current-context-follow-up) expressly requires reconciliation/qualification. F-07/F-08 source declarations and tests support a bounded correction; no blanket runtime recertification is asserted. |
| Semantic package is still awaiting approval because chapters/architecture say PROPOSED | Superseded as approval state by [guide's explicit 2026-09-30 decision](../../specs/handbook-contract-membrane/semantic-model/README.md#approved-decision). Representation and concrete realization remain deferred. HR-001–HR-006 are resolved semantic items, not six new pending human decisions. |
| HCM-5.3's empty manifest and HCM-5.4 handoff are still future work | Completed successor authority/phase exit disposes of that handoff. Preserve the empty-registry checkpoint as 5.3 history and the 5.4 one-manifest proof as successor history. [Current status](../../specs/handbook-contract-membrane/README.md#current-status). Full validator runtime independently unverified here. |
| A gate/local closeout or inert `parent_promotion` accessor already provides parent eligibility | Rejected inference. Contracts historical local-closeout truth is pure; approved Work Progression uses a separate local and parent model, F-10. Neither selects actual parent mutation. |
| Existing V1 records, aliases, old Python validator or generic “derived artifact” mode must survive | Not selected for the greenfield model. [Guide current-product boundary](../../specs/handbook-contract-membrane/semantic-model/README.md#current-product-versus-proposed-model), [Objects settled exclusions](../../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#settled-versus-deferred), [5.4 non-goals](../../specs/handbook-contract-membrane/slices/HCM-5.4/README.md#explicit-non-goals). Preserve semantics/historical accountability without inventing compatibility bridges. |
| Stronger Dock confinement / arbitrary executable admission is an assumed remaining program | Explicitly rejected by `b0e73fc` and current 5.3 lines 137–146/invariant 12; F-15. Established-tool integrations still need bounded authority. |
| Policy admission or real action requires a new IAM platform or repeated human login/confirmation | Rejected universal requirement. Approved advisory setup/action authority remains possible under admitted rules without replacing owner checks; F-11/F-12. |
| Pre-reset plans/disabled router, idea documents or old packet closeouts authorize new execution | Historical provenance only under [README recovery rules](../../specs/handbook-contract-membrane/README.md#historical-material). No older sections were adopted here as active scope. Old 4.x/5.x non-goals are bounded slice exclusions, not permanent bans or automatic future feature promises. |

## Checks and limits

- Confirmed HEAD is the exact research baseline with `git rev-parse HEAD`; inspected
  `git status --short` and `git show b0e73fc` including the correction diff.
- Enumerated current HCM files with `rg --files`, read the sources/sections listed
  above, and inspected the cited exports, inventory/admission-selection path,
  snapshot dispatcher/fixed SDK seam, current gate/local composition and crate
  manifests. These support only the explicitly bounded source claims here.
- Ran `node .gitnexus/run.cjs query grounding --repo handbook --limit 3`.
  It returned no results with degraded FTS errors (`Protocol`/`Category` tables
  missing), reported an index 20 commits behind HEAD, and skipped a foreign-worktree
  registry entry. Exit 0 did not establish successful graph evidence. An earlier
  multi-word query was rejected by the runner's argument splitting. No graph
  result was used, and no reindex or product edit was performed.
- Inspected HCM-4.6 Packet-5 test source; no Rust test, build, Clippy, external
  publication or live adopter check was run. Completed closeouts remain historical
  evidence, not fresh tests. Other reports must establish detailed approved-model
  conformance from their own owner paths, exports, assets and tests.
- Checked all 84 local Markdown links: every target exists and every semantic
  heading/explicit anchor resolves. Line-fragment references were used for code
  locations. No external link or system state is certified.
- No indexed symbol was edited and no commit was made; symbol impact analysis is
  therefore inapplicable to this report-only addition. The coordinator retains
  the pre-commit change-detection gate. `git diff --check` exited 0, but the report
  is untracked, so that command does not itself validate its new-file contents.

Open planning gaps are concrete owner/policy/model/host choices and evidence of
runtime conformance, not uncertainty about the approved semantics. This inventory
selects no API, policy, packet plan, new product or external action.
