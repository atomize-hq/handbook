# Approved semantics: proposed coverage and landing map

Status: **proposed for human review**, 2026-10-01. This is planning input, not
implementation authority, a new phase numbering scheme, or an amendment of
completed slice outcomes. Product baseline: `b0e73fc54f28d80c3f8f58d37b939db41bbbf0e4`.

The comparison supports retaining the inward owner architecture while extending
and correcting behavior. Existing exact identities, durable evidence, native JSON
Schema integration, grounding, and atomic Charter mutation are useful foundations.
They do not establish the approved evaluation lifecycle or present-use/action chain.
The [approved package](semantic-model/README.md#approved-decision) fixes semantics;
the [phase map](phase-and-slice-map.md) continues to own program order and status.

## How to read coverage

**Already covered** means the bounded behavior exists in inspected source and test
bodies. **Needs changing** means a current behavior cannot serve the approved role
unchanged. **Missing** means absent from the inspected plausible owners, execution
paths and public declarations. **Unverified** means the evidence does not establish
the claim. None is a fresh runtime certification. A whole area can contain all four.

The [research index](../../research/hcm-coverage-landing-2026-10-01/README.md)
provides the source hierarchy, complete subsection/scenario dispositions, stable
claim IDs, evidence paths and limitations. The five areas below are provisional
capability groupings. They are not five implementation packets.

## Coverage and responsibility

| Area and outcome | Already covered foundations | Needs changing / missing | Dependencies and ownership |
| --- | --- | --- | --- |
| **Rules and contract definitions:** independently establish which rules are authoritative, admit candidates, and qualify a contract's permitted uses. | Six authored families; exact immutable definition bindings; inward owner dependencies and owner readmission. | **Change:** non-family Context Resolution authority content currently uses generic artifact publication; reconcile it with its native owner lifecycle. **Missing:** project admission-rule establishment/replacement, deterministic Policy Admission Closure, contract permitted use/qualification, and exact typed authored-input/result relationships. | Handbook supplies establishment rules; each semantic owner admits its policies/definitions; authored documents supply only declared inputs. Dependent requirements and result/action policies need their exact admitted definitions. Concrete criteria and physical placement remain decisions. [Objects](semantic-model/objects-and-governance.md); [R-01–R-14, R-18–R-26](../../research/hcm-coverage-landing-2026-10-01/rules-and-definitions.md#stable-obligations-and-dispositions). |
| **Starting an evaluation:** close complete requirements, preserve diagnostic versus decision-seeking intent, and establish whether this exact plan may proceed. | Nine-dimension posture kernel; exact narrower run preparation; Engine Context Resolution authority; bounded Dock operating equality. Grounding is implemented separately. | **Change:** existing direct-run preparation lacks the complete new requirements closure. **Missing:** scoped posture/assurance selection, both requirements-source identities, permitted-use/intent compatibility, stable Observation Plan/Purpose/occurrence, purpose-bound Engine authorization, independently admitted capability, and Coordination readiness/disposition/basis. Setup/Doctor readiness is a different proposition. | Admission and qualification precede selection. Plan precedes Purpose and Engine issuance; capability is compared afterward. Contracts-domain Coordination compares and coordinates; Engine alone establishes its permission. For governed completion, the action domain must resolve the governing target/assessment relationship before collection commitment; no future completion Intent is required. [Preparation](semantic-model/assurance-and-evaluation-preparation.md); [S-01–S-36](../../research/hcm-coverage-landing-2026-10-01/starting-evaluations.md#stable-behavioral-claims), U-04 below. |
| **Collection and evidence:** record real attempts and close exactly the evidence belonging to one evaluation. | Independent candidate admission; immutable per-record evidence and delivery replay; Dock operational outcomes/recovery; the actual Rust `jsonschema` observation path. | **Change:** run identity/currentness and live repository filtering cannot stand as occurrence-wide closure. **Missing:** owner occurrence allocation, immutable commitment, later-check/recovery lineage, complete attempt/admission terminals, sealed causal frontier, complete evidence-set formation, final context, and full historical reconstruction. Complete pruning needs its own positive proof and atomic commitment/empty-frontier outcome. | Preparation supplies exact attemptable basis and authorization; action-domain governing relationship is required where applicable. Producer/Dock/host establish bounded origin facts, Coordination seals collection, Contracts independently admits evidence and closes set/context. Membership cannot be chosen for favorable results. [Identity and closure](semantic-model/evidence-and-evaluation.md#identity-evidence); [C-01–C-19](../../research/hcm-coverage-landing-2026-10-01/collection-and-evidence.md#behavioral-claims). |
| **Results and explanations:** produce independent findings, named metrics, gate decisions and views of one recorded truth. | Exact integer/rational arithmetic, ordered claim accounting, non-compensatory blockers, pure gate computation and generic consumer transport. | **Change:** invalid context can currently become blocked truth; advisory obligation changes the verdict; score belongs to gate/claim weights and is skipped on blockers; gate has only passed/blocked. **Missing:** independent dimensions, admitted role policies, named metric definitions/results, configurable four-outcome gates, full canonical explanation, authorized concise/detail projections, retained exact result dependencies. | Consume one valid closed context/set and its complete selected definition inventory. Contracts owns applicability/finding/gate/explanation; metric admission and computation stay attributable. Projections preserve source meaning and separately authorized disclosure. Invalid closure refuses before truth; valid insufficiency is interpreted by the selected policies. [Results](semantic-model/evidence-and-evaluation.md#metric-gate); [T-01–T-22](../../research/hcm-coverage-landing-2026-10-01/results-and-explanations.md#behavioral-claims). |
| **Using results and performing actions:** determine what an exact historical result supports now, then distinguish permission, attempt and committed effect. | Separate pure historical gate/local result; Engine Charter authority lock, immediate CAS, durable installation/recovery/replay and truthful write outcomes. | **Change:** Contracts-owned Eligible/Blocked local closeout is historical delivery, not the approved Work Progression authority/facets. **Missing:** independently governed completion, action-specific finite reliance closure, atomic determination/refusal, Work Progression local/parent eligibility, separate Intent/Action Authorization/Attempt/Terminal/Receipt, and complete feedback chain. Existing mutation foundations lack the full new cross-owner through-commit linkage. | Work Progression consumes exact work identities/state/relations from their authoritative owner; it does not mutate state. Each action domain resolves governing targets, reliance and authority separately. State/effect owners establish actual commits. Governing relationship selection feeds preparation; result-dependent reliance follows truth. Posture feedback starts a later context. [Reliance/actions](semantic-model/operational-reliance-and-actions.md); [U-01–U-16](../../research/hcm-coverage-landing-2026-10-01/reliance-and-actions.md#behavioral-coverage-claims). |

These distinctions have observable consequences. A passing direct or diagnostic
run cannot discharge a project-required completion assessment. A duplicate delivery
does not create an execution; a real rerun does. In the approved **named** all-N/A
scenario, metric/gate remain N/A and reliance yields `not_applicable / no_effect`;
the current test instead produces Passed/Eligible. This demonstrates missing
policy/outcome expressiveness, not a universal prohibition on completion after
correctly established non-applicability. A timeout after sending a mutation cannot
establish non-application or permit ordinary retry. See the owning scenario corpora
and [current all-N/A/purity test](../../../crates/contracts/tests/owner_repository.rs#L3366).

## Cross-cutting work belongs inside each affected increment

| Area | Identity | Currentness and refusal | Persistence and replay | Public boundary |
| --- | --- | --- | --- | --- |
| Rules/definitions | Exact candidate, rule/configuration, owner, scope and successor. | Active-at-decision admission closure; stale predecessor and self-authorizing replacement refuse. | Committed activation/cutover and retained old admission meaning; authoring history stays distinct. | Owner-native definition/admission truth through SDK contracts; no seventh authored family or transport authority. |
| Starting | Requirements source, permitted use/intent, stable selections, Purpose, occurrence and Binding. | Owner issuance/checkpoint facts; invalid inputs refuse; valid limitation and held decision-seeking intent remain separate. | Immutable formation/verification lineage; bounded precommit refresh leaves old basis unused. | Selected Plan/requirements/authorization/readiness shapes and refusals; Engine private envelope stays private. |
| Collection | Commitment, real attempts/submissions, frontier, set and final context. | Fresh later checks over fixed basis; overreach, incomplete closure and invalid final verification refuse. | Complete terminal/accounting history, atomic empty route, uncertainty resolution and original checkpoint replay. | Producer/admission/closure outcomes preserve exact occurrence and provenance across Dock/SDK consumers. |
| Results | Expected claims, exact role policies, metric definitions/populations, gate and explanation source. | Context refusal precedes truth; valid indeterminacy/N/A is policy-interpreted; projection never reevaluates. | Immutable truth and owner-resolvable dependencies for declared horizon; later unavailability qualifies views. | Exact numeric types, independent result states, explanation/disclosure/omission and authorized detail. |
| Reliance/actions | Action question/target/effect, reliance occurrence, atomic facets, Intent, Binding, Attempt/Terminal/Receipt. | Historical evaluation Binding verification only for consumed roles binding one; explicit current-use mode; separate action and guarded commit checks. | Exact refusal/outcome replay; uncertainty blocks ordinary retry until authoritative resolution or admitted duplicate-safe recovery. | Owner results, advisory/verified provenance, uncertainty and committed effects; no consumer-inferred completion. |

The approved chapters own the full obligations; reports trace their coverage.
Shared storage and identity
primitives may be reused; semantic owners remain separately attributable. Select
the necessary schemas, storage, time comparison, retention and public operations
with each bounded behavior and its actual selected consumer boundary. This does
not require universal SDK/CLI/Tauri parity or a standalone public API for every
semantic object. Do not defer them to a final integration phase or
assume a new service, universal repository, distributed transaction or IAM platform.

## Landing order and future fit

The semantic dependency order is **admitted rules/qualified inputs → complete
requirements and preparation → committed collection and final context → independent
truth/explanation → present-use reliance → separately authorized effects**. The
governing completion relationship is an earlier action-domain input, not a result
to be discovered after collection. Parent progression and posture feedback consume
their own exact populations and later contexts. This is a partial order: narrowly
selected existing consumer paths need not wait for every proposed capability.

| Remaining membrane outcome | Compatibility conclusion and boundary to preserve |
| --- | --- |
| **HCM-6.1 bridge, 6.2 publication, 6.3 direct Substrate adoption** | **Fits** owner-native SDK/versioned JSON direction. **Change with selected behavior:** exact new/refined identities, refusals, numbers and result meanings must be published as deliberate contracts; generic transport is not semantic closure. **Decision-dependent:** adopter, mechanism, exact versions and replacement choice. Repository evidence does not prove live external use. Preserve current hard predecessors. F-02–F-04/F-16. |
| **HCM-6.4 Tauri product** | **Fits** workflows over SDK owner truth. **Change with selected workflows:** expose missing metric/explanation/reliance/action behavior at owners first, including source availability and disclosure. **Decision-dependent:** UI/density inventory and selected workflow scope. Existing non-UI facade is not the GUI. F-05. |
| **HCM-6.5 workflow adapters and additional Dock tooling** | **Fits** bounded profile/vocabulary/Resolution mappings and separately admitted observations. **Change with selected adapters:** exact capability/operating/attempt/admission contracts must meet their required roles. **Decision-dependent:** integration and adapter owner. Rust offline JSON Schema is the first real path; OpenAPI/tooling examples are options, not a promised roster or sandbox program. F-06/F-15. |
| **Grounding host/trigger and projection workflows** | **Already covered foundation:** completed HCM-4.6 local grounding, four ordinary operations and fixed session composition. **Fits** separate host/consumer adoption with owner-produced start/final outcomes and receipt recovery. **Decision-dependent:** triggers/adopter and any independently justified public projection seam. Four residual reservations are not mandatory slots to fill. F-07–F-08. |
| **Work/dependency progression and broader posture** | **Fits** separate Work Progression, authoritative work/state owners, and forward observation → recommendation → transition-specific reliance/action → Engine mutation. **Change needed before target claims:** implement missing populations, governance, historical/current-use and through-commit linkage. **Decision-dependent:** work model, physical placement, concrete policies/algorithms, notifications/acknowledgement/hysteresis and hosts. F-09–F-11. |

These conclusions are supported by the [future inventory](../../research/hcm-coverage-landing-2026-10-01/future-obligations.md),
the [current Phase-6 outcomes](phase-and-slice-map.md#phase-6--consumer-adoption)
and [rehomed decisions](phase-and-slice-map.md#primary-unresolved-decisions).
Compatibility is architectural and scoped to stated requirements; runtime and
external adoption remain unverified. The model imposes no V1 migration/dual-write
obligation. Stale blanket grounding/Dock summaries and historical empty-registry
checkpoints must not override current source or completed successor delivery.
Preparation/Evidence's broad historical-Binding summaries also need to be read against
the [action owner's exact consumed-role rule](semantic-model/operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule)
and [guide decision 8](semantic-model/README.md#8-a-valid-past-result-does-not-automatically-qualify-for-use-today).
The map preserves that qualified rule; this study does not amend approved chapters.

## Review and next decision

For this work, **a packet finishing returns to this control conversation and stops**.
The user reviews its recap before any successor is planned or dispatched. Human
semantic approval, an admitted slice, and an automated clean verdict cannot supply
successor authorization. The existing auto-continuation route identified in F-18
must be reconciled explicitly before product dispatch; this map does not amend the
general lifecycle/prompts/router or admit a phase/slice.

After human review of this proposed map, select and plan **only the first bounded
increment**, likely at the governance/definition foundation. Its plan must identify
why it is next, existing prerequisites, required concrete choices, one explainable
behavior and meaningful refusal, owner/interface/persistence boundaries and checks.
Do not turn this study into a fully elaborated backlog. No implementation has begun.

Research checks consist of baseline/source/test-body inspection, complete coverage
tables, cross-area reconciliation and local reference checks. Rust tests and live
external checks were not run. GitNexus was stale/degraded and supplied no reliable
graph evidence. Cross-area findings and the final independent verdict are linked
from the research index; they validate this planning comparison, not product
conformance or human roadmap acceptance.
