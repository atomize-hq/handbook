# Configurable work compositions and optional immutable kinds

Status: **Packet 1 recommendation A implemented and verified at `7129b5c`;
meta validation/human recap and subsequent discussion complete; recommendation B
human-selected; complete Packet-2 plan approved at `d7ff969`; implementation and
selected acceptance verified; focused causal APPROVE; no unresolved P1/P2** — 2026-10-09.

The human selected per-version approval B and authorized bounded
[Packet-2 preparation](packet-2.md), with review of the concrete plan before
implementation. The human subsequently approved the complete reviewed plan at
`d7ff969`, including implementation, verification, focused causal review and one
coherent local commit. This authorizes optional immutable whole-composition kinds
and their usable native/SDK workflow, supplying no runtime project approval or
slice completion. Final slice integration and the human recap remain with meta.

After reviewing the capacity limitations, the human selected a separate near-term
[capacity/performance research follow-up](../../coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up)
while keeping the current compositions/kinds direction moving within its bounds.
This adds no storage redesign to this slice. B's selection is the separate
subsequent kind-definition decision recorded below.

After reviewing the revised plan at `277674e2`, practical workflows and the exact
separately defined Contracts requirements-input reference, the human approved
recommendation A and all of Packet 1: “ok. approved. Lets proceed with
implementation”. This selects finite token/locator scope, human-only same-basis
successors, pending suspension, unchanged-scope D→G and no reinstatement.
The authorization includes implementation, verification, focused causal review
and one coherent local commit. Packet 1 ended at its completed human recap; after
review/discussion the human selected B, authorized Packet-2 planning, and then
approved its complete reviewed implementation plan at `d7ff969`.
Runtime project terms and each exact kind version/content still require their
own explicit supplied human approval.

Descriptive slice identity: `work-compositions-and-kinds`. This owns exactly
[near-term outcome 2](../../coverage-and-landing-map.md#proposed-near-term-sequence-and-landing-checkpoints).
After the completed foundation recap, the human authorized this slice's
preparation, then rejected the parallel permission streams proposed at
`b7e0a48c788ca4d42cef326c8206cf1170356073` and selected **one composition-aware
registration authority model for a greenfield project**. This revision supersedes
that proposal. The subsequent approval authorizes Packet-1 product implementation,
verification, focused causal review and one coherent local commit, while supplying
no actual runtime project's terms. The [lifecycle](../../work-unit-lifecycle.md),
[review policy](../../review-and-closeout-policy.md) and
[task-session prompt](../../task-session-prompt.md) control implementation.

## Outcome and selected boundary

A typed Rust SDK caller chooses an Engine-supported composition for a new
same-project `open` leaf. **Ordinary, planning-output and requirements-input use
one registration path and one permission model.** Human-approved terms identify
the exact permitted compositions and parameter scope. Release recognition of a
new composition never expands an existing grant; active permission expansion
requires explicit authorization. Existing items and receipts keep their exact
meaning. Kind-definition authority stays separate from work-registration authority.

These are carried-forward **human selections**, not reopened recommendations:

| Selected input | Consequence here |
| --- | --- |
| [Foundation menu](../work-item-state-foundation/README.md#composition-and-kinds-successor-compatibility-gate) and [compatibility cases](../work-item-state-foundation/packet-1.md#selected-successor-compatibility-acceptance) | Ordinary open leaf; planning-output leaf with a bounded required-output question; requirements-input leaf with an exact versioned Contracts definition locator. Same project, initial `open`, no live relations, completion `unsupported / not assessed`. Display token at most 128 bytes; label at most 4,096 bytes. |
| Planning required-output role | Declare the question before issuance, with no guessed future contract locator. Actual owner-established complete output and the planning item's own completion requirements are later independent eligibility facts. |
| Exact requirements-input role | Declare one exact versioned definition, with no claim of source availability, qualification, assessment, governing assignment or target coverage. |
| Optional whole-composition kind and explicit equivalent | An immutable version binds the exact supported descriptor, role schema and parameter bounds, lifecycle and compatibility version. It adds no primitive or permission. Per-item values vary only within declared bounds; unknown roles, assessment-result inputs, relations and arbitrary hooks refuse. |
| Owner separation | Engine admits composition conformance and work identity; Contracts resolves its definitions/truth; Work Progression later admits coverage and governing completion. No Engine dependency on Contracts/SDK and no generic Artifact Kind reuse. |
| Authority and history | No silent scope widening, item reinterpretation, rebinding or substituted original-request authority. Definition/current-default changes cannot rewrite retained truth. |

The [selected amendment](../../semantic-model/amendments/work-progression-kinds-and-relations.md),
[decision register](../../semantic-model/review-decision-register.md#work-progression--kinds-relations-and-governed-completion)
and [Work Progression ownership](../../semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
remain semantic input. Their wider relation/assessment semantics are not support
in this slice. Independent Contracts use needs neither work registration nor kinds.

## Predecessor evidence and source constraints

Foundation is implemented at `cbfbf7629a0642b8cb08aefea36fa556009d9274`, after
selection recording `cb00b9da351095c5cb17c8bc4e754f908b0565d0`, on
`feat/handbook-contract-membrane-clean`. Supplied meta evidence confirms all eight
conditions, integration, aggregate closeout and the reviewed human recap. This
preparation checked lineage and focused source/tests; it does not rerun or reclose
foundation or independent-local-check. Preparation started at `b7e0a48`;
the approved implementation baseline is `277674e2`.

Inherited foundation evidence: Engine 3/3, ordinary SDK consumer 6/6, durability
7/7 and integrity 9/9; compilation/diff checks passed; final fresh causal APPROVE
with no unresolved P1/P2. Debug scan cost remains P3 advisory. Windows was
exercised; Unix/full-workspace verification remains unclaimed.

| Foundation source evidence before Packet 1 | Constraint, not authority to freeze the design |
| --- | --- |
| [Engine types](../../../../../crates/engine/src/work_state/types.rs), `WorkRegistrationTermsV1`, `WorkRequestLocatorV1` | Foundation permission names one exact ordinary composition. Decoder recognition cannot expand it. The current API shape can be replaced in greenfield development. |
| [Transactions](../../../../../crates/engine/src/work_state/transaction.rs), `activation_compatible`, `current_grant` | Only initial activation and one exact compatible D→G predecessor exist. Same-basis scope change is new work; reserved `terms_change` does not implement it. Pending successor/revocation already blocks dependent new effects. |
| [Held authority seam](../../../../../crates/engine/src/local_authority/work_registration.rs) | Holds original native operating/tree guards, captures exact historical proof, and supplies no work permission by itself. SDK observation alone is not through-commit authority. |
| [Store](../../../../../crates/engine/src/work_state/store.rs), `inventory`, `CAPACITY` | One retained owner/lock and positive inventory; 32 transactions including authority/candidate/noncommit history, 128 MiB, 1 MiB records. Parent currently permits only `work.lock` and `v1`; new schemas/layout need closed admitted inventory. |
| [SDK facade](../../../../../crates/sdk/src/work_state.rs), [usage](../../../../../crates/sdk/docs/work-state.md), [owner tests](../../../../../crates/engine/tests/work_state.rs), [SDK tests](../../../../../crates/sdk/tests/work_state.rs) | Typed prepare/save-original-locator/invoke/current/history/recover exists. No positive menu, kind or scope-change operation is delivered. |

Foundation is complete. The human selected recommendation A and approved the
bounded Packet-1 plan at `277674e2`; the implementation task confirmed the live
dispatch prerequisites. No CR-first dependency, generic IAM, migration platform
or storage redesign is introduced.

## Development compatibility and retained truth

**Development compatibility:** old SDK method/type shapes and development
fixtures do not automatically require permanent support. The greenfield public
surface may be revised to one typed composition-aware interface, and disposable
test setup may be updated. There is no promise of old-source compilation,
backwards mixed-binary writes, permanent legacy API or production migration tool.
Updating fixtures must not erase tests of retained historical bytes.

**Authority and historical truth:** greenfield does not authorize deleting
retained owner state. The minimum actual obligation is to read and verify retained
items/receipts, exact grants/terms and every accepted unresolved request with its
original bytes, fingerprints, basis, reservations and publication proof. Preserve
the item identity scheme across release/codec/composition versions. Keep bounded
schema-specific decoding and original-request recovery where needed for that
retained population; do not expose a second operating permission stream.

Within unified admission, a retained foundation grant has only its original
singleton ordinary scope and existing text bounds, **no display-level field or new
roles**. Its original basis/currentness still governs use. It can authorize only
ordinary parameters already permitted; recognition of new types grants nothing
further. A normal explicit scope-change operation may name it as exact predecessor,
like any other retained head. This is the same lifecycle, not a V1-to-menu upgrade.

Dropping old record decoders would make retained items/receipts unverifiable and
unresolved requests unrecoverable; resetting the store would destroy history and
reservations; rewriting old grants would corrupt approved scope and original
request identities. None is authorized. If implementation proposes abandoning
any retained population, it must identify that exact loss and obtain separate
human authorization first. No abandonment is recommended here.

## Unified owner and permission boundaries

Engine `work_state` remains the work-definition/conformance, registration,
authority-history and persistence owner. Its fixed operation-specific human
establishment/change rule sits outside candidate terms. The narrow local-authority
seam supplies held current initial/check/Charter/D/G proof; work admission checks
the selected active grant, composition and parameters under those guards through
terminal publication. SDK forwards typed inputs and attributed native outcomes;
it neither mints authority nor verifies Contracts truth.

There is one registration authority lineage per existing supported project/check
identity, at most one current usable head, one expected-predecessor contract and
one registration request path. Record versions distinguish historical evidence;
they do not select parallel permission populations. Fixed menu admission establishes
conformance, not operating permission. Local-check, evaluation, correction, CR,
filesystem or pack permission cannot substitute. Advisory approval/principal/
delegation remain labeled claims; no authentication or host enforcement is added.

<a id="proposed-authorization-lifecycle"></a>

## Selected authorization lifecycle

The unified direction and the following **recommendation A** are **human-selected**
for Packet 1. This bounded lifecycle/scope contract includes
explicit same-basis scope changes next so an existing ordinary-only project can
authorize P/I without another stream.

| Lifecycle step | Selected bounded contract |
| --- | --- |
| Initial establishment | Original claimed human approves exact terms on positively current local-check `DeferredV2` D or `EstablishedFromDeferredV3` G. Positive complete owner/history proves no prior committed head or unresolved establishment. Bind project/root, original initial/check/Charter lineage, current basis, original principal/check agent, source/instruction/delegation, approved-content fingerprint, register-open-leaf effect, fixed full-owner reach, reserved decisions, revocation principal and retained-history rule. Candidate terms cannot authorize themselves. |
| Exact scope | Nonempty finite subset of the three exact descriptor/schema/compatibility bindings. Human-supplied text maxima within owner ceilings; explicit omission permission and/or finite display-token set; bounded planning question; finite set of exact requirements definition locators. No latest/family wildcard, arbitrary predicates, implicit defaults or future-menu reach. Complete closed terms/request at most 1 MiB. Packet 1 details the clauses. |
| Explicit scope change | Original claimed human uses the fixed external change rule, fresh approval/instruction and exact expected current committed predecessor. Create immutable successor on the same current D/G basis. Permit only composition additions/removals and parameter widening/narrowing; principal/agent, lineage, effect, reach, assurance, reservations, revocation and retention stay fixed. Even narrowing is explicit. No in-place edits, agent administration, branching or grant unions. Commit makes successor sole usable head; prior terms remain exact history. |
| Pending change or revocation | Accepted intent reserves the head and conservatively suspends predecessor use for new effects. Candidates unusable; competing administration refuses. Exact positive noncommit releases administrative reservation and permits unchanged predecessor again only if its original basis is still current and unrevoked. Uncertainty keeps suspension. The human selected this availability tradeoff. |
| Revocation | Declared original principal submits exact head and source/instruction. New terminal publication requires original live basis and guards, as in foundation; committed revocation is retained historical proof. Accepted revocation suspends new use; committed revocation permits no new registrations. Reinstatement is unsupported next; never fall back to an earlier grant. Lost/stale live authority already blocks effects. |
| D→G | Old D permission/preparations become stale. Explicit original-human re-establishment on current G names exact retained latest unrevoked predecessor and preserves entire scope/fixed fields. It may follow multiple scope revisions; no special version upgrade. Combining scope change with D→G refuses: re-establish unchanged scope, then separately authorize a scope change on G. No automatic carry-forward. |
| Pending registration after change | Never retarget to successor, G or newly matching scope, even if permission overlaps. A missing terminal can continue only while exact original grant remains usable head on original live basis. Otherwise original recovery proves committed history, positive noncommit or uncertainty, retaining conflicting item/request reservations. No bulk cancellation or rewrite. |
| Authority-operation recovery | Original establishment/change/revocation rule, exact request/predecessor and original basis govern continuation; no candidate/substituted grant completes activation. Exact committed proof replays history without reactivation. Terminal/prefix and noncommit rules apply to authority and registration alike. |

The [foundation terminal/prefix contract](../work-item-state-foundation/packet-1.md#durable-prefix-and-authority-transition-contract)
controls commit, original-proof replay and noncommit. Snapshot/grant candidate
without terminal is pending. Historical resolution must not require present
operating permission or reset-managed state for past truth. Missing publication-
attempt evidence cannot become absence/noncommit.

**Human selection of A:** the human selected this exact scoped lifecycle, including finite
locator/token clauses, same-basis human-only successors, pending suspension,
unchanged-scope D→G and no reinstatement. These consequential details are
explicit selections, not implications of selecting one model. If revised, update and
review the affected Packet-1 plan before execution. Actual project terms remain
separately approved/supplied at runtime.

<a id="kind-definition-authority-recommendation"></a>

## Selected kind-definition authority

The human selected separation from registration and subsequently selected
**recommendation B, per-version approval**: original claimed human approves each exact immutable kind
version/content through a separate fixed Engine definition-establishment operation
on current supported D/G. A kind remains retained definition truth after its basis
changes; new registration still needs current composition/parameter permission.
Kind conformance cannot widen permission or supply primitives.

Per-version approval adds a separate human decision for every version. No reusable
kind-establishment grant is selected. The [concrete Packet-2 plan](packet-2.md)
details exact definition/binding/parameter identity, kind-first owner bootstrap,
retained definition truth, equivalent explicit meaning and unchanged registration
permission, with typed SDK read/resolve/recovery and acceptance/fault coverage.
The complete plan is human-approved at `d7ff969`. Implementation approval and
technical review supply no actual runtime approval of a kind or work grant.

## Packet outline and human checkpoints

Reassess the cut around a complete usable registration lifecycle rather than an
additive menu stream. Authority, conformance and SDK/persistence steps inside
Packet 1 depend on each other; they are not hidden preparatory packets. Two
independently meaningful outcomes remain:

| Packet | Outcome and dependency | Human review boundary |
| --- | --- | --- |
| 1 — unified explicit-composition registration | One authority lineage and typed path for ordinary/P/I; exact scope and bounded scope change/D→G/revocation/recovery; retained foundation truth. Requires complete foundation, selection of A and approval of [next plan](packet-1.md). Kinds unsupported. | **Packet:** a continuing project explicitly authorizes and registers P/I while old grants cannot widen and old items stay exact. Stop for recap. A separately delivered earlier meaningful authority outcome triggers the lifecycle's earlier checkpoint. |
| 2 — optional immutable whole-composition kinds | Exact reusable versions and equivalent explicit meaning; separate establishment and unchanged registration authorization. Packet-1 review is complete, B is human-selected and the complete [detailed plan](packet-2.md) is approved at `d7ff969`. Implementation and selected acceptance are verified; focused causal review returned APPROVE with no unresolved P1/P2. | **Packet:** reusable definitions/kind-backed work are meaningful; return for review. Meta checks final slice acceptance/integration here and recaps once. |

Only the next increment, Packet 2, is now detailed and approved for execution.
Adjacent preparation remains outside this increment. Local names/module
split/tagged record layout may be resolved within the reviewed contract; authority
scope, lifecycle, history loss and kind establishment cannot be silently changed.

## Complete slice acceptance

1. Precisely the ordinary/P/I menu and bounded display vocabulary through one
   route; same-project `open` leaves, no relations or positive completion. Unsupported
   roles/hooks/transitions refuse without discarded fields.
2. Exact approved scope controls parameters through commit; menu recognition,
   kind conformance and unrelated grants never expand it. Selected lifecycle yields
   at most one usable head with exact predecessor history.
3. Kinds bind complete supported descriptor/schema/bounds/lifecycle/compatibility
   under separate selected definition authority. Conflicting same-version content,
   latest aliases, uncommitted kinds and overrides refuse. Equal explicit/kind
   values have equal meaning while retaining admitted form/provenance.
4. Retained ordinary items, grant scope, descriptor/request bytes, identities,
   receipts and unresolved requests survive support, reopen, scope changes, D→G,
   revocation/reset without reinterpretation or substituted recovery.
5. Exact native terminal plus matching snapshot/receipt/original authority is
   commit. Pending candidates supply no truth/permission; uncertainty retains
   reservations; positive noncommit requires complete proof.
6. One closed retained owner/inventory/lock and combined 32-transaction/128 MiB
   budget including authority, kinds, candidates/noncommit history. Overflow preserves
   old reads/recovery; no truncation, per-version budget or storage redesign.
7. Normal-dependency SDK consumers prove actual paths, saved original locators,
   clear response guidance and meaningful refusals. Focused tests prove retention,
   integrity and through-commit authority; report actual evidence/platform limits.

Packet-1 acceptance is [here](packet-1.md#acceptance-and-verification). Meta checks
complete slice/integration once after final authorized packet; preparation supplies
no product completion claim or new predecessor closeout.
Packet-2 acceptance is [here](packet-2.md#ordered-implementation-and-acceptance).

## Non-goals and format boundary

Hierarchy, eligibility, closing, CR binding, item edits/rebinding/reopen/removal,
assessment-result roles, general IAM/agent grant administration, CLI/Tauri/host
integration, production migration tooling, permanent legacy APIs, parallel
registration streams, V1-to-menu upgrade systems and storage/performance redesign
are excluded. Phase-6 order and adjacent planning remain unchanged.

The [structured-data standard](../../structured-data-format-standard.md) retains
**PROPOSED general defaults**, **Contracts JSON fixed**, and existing versioned
owner contracts controlling. Recommend closed owner-managed JSON with explicit
new schema/fingerprint domains, preserving retained old byte domains. Typed
operations are the authoring surface; no YAML importer, direct file editor, second
editable human-view master or broad format migration is selected.
