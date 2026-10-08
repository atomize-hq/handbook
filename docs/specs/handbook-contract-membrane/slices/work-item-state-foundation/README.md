# Native work-item state foundation

Status: **bounded slice authority prepared; product execution awaits human
authorization of Packet 1**. Prepared 2026-10-08 against
`2ce0871f7516f9850577922e9efb19b577f6d1bc` on
`feat/handbook-contract-membrane-clean`.

Descriptive slice identity: `work-item-state-foundation`; no phase number or
independent-local-check Packet 5 is assigned. After the Work Progression semantic
integration recap, the human requested dispatch of the first bounded native/SDK
preparation and the normal causal review process, permitting local subagents
while fresh top-level sessions remain restricted. This authorizes preparation,
source verification, review and a local documentation commit. It supplies no
product execution permission or actual runtime instruction, actor or grant.

The [program shape](../../coverage-and-landing-map.md#successor-readiness-after-independent-local-check),
[entry point](../../README.md), [lifecycle](../../work-unit-lifecycle.md),
[review policy](../../review-and-closeout-policy.md) and
[authority-preparation prompt](../../authority-preparation-prompt.md) control this
one slice. The human-selected [Work Progression amendment](../../semantic-model/amendments/work-progression-kinds-and-relations.md)
and [operational reliance chapter](../../semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
control its semantic boundary.

## Outcome and reason for this cut

An ordinary typed Rust SDK caller can register one bounded leaf work item in an
already positively registered local project, obtain the native owner's committed
`open` snapshot and registration receipt, and resolve that same item or original
request after reopening the repository. Several independent leaf items may be
registered; each operation concerns exactly one item. An explicit supported
composition establishes the item's meaning, while its display label supplies no
taxonomy or authority.

The first creation route is limited to a project whose **original initial
local-advisory authority remains current over its original positive deferred
basis**, before local-check successor terms are adopted or D→G occurs. Existing
completed-local-check or established-G projects cannot use this initial route.
That is a source constraint and an explicit capability limit, not a new universal
work lifecycle order. Registration can precede local-check adoption on a supported
runtime project; later adoption or governance changes do not retroactively close
the item or restore creation permission. Current item/history reads resolve the
work owner's retained facts separately from current creation authority.

The caller can inspect the item's completion support separately. The result says
that governed local-closeout eligibility is **unsupported / not assessed**, and
refuses a request to derive eligibility from a Contracts pass, a Work Specification
checkbox, or an asserted `completed` value. It issues no Operational Reliance
Determination, eligibility/disposition facet, action authorization or completion
receipt. `open` means registered work that has not been operationally closed; it
does not assert candidate availability, execution, review or assessed requirements.

This is an authoritative-state prerequisite for separate Work Progression
eligibility, not delivery of positive eligibility or a complete work workflow.
The selected first increment deliberately ends here. The landed independent
check resolves evaluation truth and history; it supplies neither the operational
target nor governing completion assignment, supported target coverage, a finite
reliance population, current qualification or completion effects. Building all
those boundaries with registration would make the first landing substantially
larger and conceal an independently meaningful state-owner checkpoint.

**Example:** before local-check adoption on a positively deferred project, a caller
with prior human-approved local operating terms expressly
permitting registration writes under the selected owner root registers
`inspect-session-renewal`, displayed as “Investigate session renewal,” with a
bounded work question. The receipt confirms creation of the exact `open` item.
Reopening yields the same identity and snapshot. A passing independent contract
result may remain useful history, but a closeout-support inspection reports
`not assessed`; it cannot turn the item into eligible or closed work.

**Meaningful refusal:** a project has adopted local-check successor terms or G,
initial operating terms allow only the local-check stores,
or the caller supplies a missing/revoked authority locator, another physical root,
changed content under an existing item/request key, a parent/prerequisite, a kind
binding, or a `closed` state. Native registration refuses. It does not expand the
old terms, treat a local-check action class as work authority, allocate a new
project, or erase an uncertain prior registration.

## Scope, owners and selected decisions

| Decision | Selection and source-grounded rationale |
| --- | --- |
| Physical state/structure owner | A purpose-named `handbook-engine::work_state` module owns item identity, admitted composition, immutable initial state, registration execution and receipts, current resolution and history. Engine already supplies repository identity, retained-root durability and guarded local operating authority. This bounded placement adds no Engine dependency on Contracts or SDK. |
| Work Progression placement | Positive Work Progression remains downstream and unimplemented. Future reliance must consume work-state facts and Contracts truth; putting that consumer inside Engine would create an Engine-to-Contracts dependency reversal. This slice selects only an explicit unsupported-completion boundary, not a policy or eligibility owner implementation. |
| Definition/composition | Use one package-owned, immutable versioned support descriptor and explicit per-item composition: a bounded leaf, initially `open`, no operational relations, no supported completion transition. Descriptor conformance is owner-established; it is not governing assignment. A separate named kind registry has no purpose for this one composition and remains deferred. |
| Authority and creation effect | Reuse current original initial local-advisory operating terms over their original deferred basis, before local-check successor adoption or D→G, only when exact prior-approved write/read reach covers this store and the supported registration effect. The new native owner validates the exact creation question and retains permission provenance separately from its committed effect. No successor authority, new permission administration or automatic path expansion is selected. |
| First lifecycle | Register, resolve current item, resolve original request, and recover that same registration. The only work state is `open`; no start/close/reopen/cancel/archive/delete/edit/rebind operation or direct state setter is supported. A registration receipt proves creation, never successful completion. |
| Identity and storage | Exact project and retained physical root; caller-supplied bounded item key and separate request key; owner-derived project-scoped item identity; immutable canonical owner records and decided-prefix recovery. Store under `.handbook/work-state/v1/`, outside reset-managed `.handbook/state`. No database, migration or editable file master. |
| Governance/applicability | Registration discharges no governed completion obligation. It establishes no exemption or governing standard. The existing positive Contracts scope basis retains its original limited meaning. Completion applicability, policy qualification/assignment and assessment protocol are unsupported here, so completion consideration cannot form a Determination. |
| SDK | Add typed methods to the existing `HandbookSdkV1` facade; keep owner truth inward of SDK. Public request, locator, snapshot, receipt, refusal and uncertainty are distinct types. No generic catalog, ordinary operation-inventory or CLI/Tauri/installed-skill expansion. |

These are bounded preparation selections for human review of the first plan,
not claims that the source already implements them. The [Packet-1 authority](packet-1.md)
details the next increment only. Existing vocabulary roles/absorptions, Flow
packet selection and Pipeline routing are not equivalent operational composition
or work-state registries. Reuse their terminology or serializers only where their
actual contracts apply; do not extend them by implication.

The [structured-data standard](../../structured-data-format-standard.md) has
**PROPOSED general defaults**, **Contracts JSON fixed**, and controlling existing
owner/versioned contracts. This slice explicitly selects additive owner-managed
JSON because its supported interface is a typed native operation. It adopts no
other proposed default, YAML importer, human-view editing route or migration.
Engine's existing parser/canonical primitives and retained-root mechanisms must
be investigated before adding a helper; SDK-private codecs cannot become inward
dependencies. Historical byte domains stay unchanged.

## Predecessors and admission

| Predecessor | Current evidence and implementation consequence |
| --- | --- |
| Selected Work Progression semantics | Integrated at `2ce0871`; the [amendment](../../semantic-model/amendments/work-progression-kinds-and-relations.md) is current semantic input. It does not authorize this implementation. |
| Native initial setup and independent local check | The [completed four-packet slice](../independent-local-check/README.md) through `be7d040` has supplied meta aggregate APPROVE, closeout and human recap. Its setup/identity, operating-authority and owner-history foundations are reusable. Its product suites are not rerun or reclosed by this preparation. |
| Current source mechanisms | [Engine local operating authority](../../../../../crates/engine/src/local_authority.rs), [retained durability](../../../../../crates/engine/src/repository_durability.rs), [initialization](../../../../../crates/engine/src/repository_initialization.rs), [definition identity/codecs](../../../../../crates/engine/src/definition_identity.rs), and [typed setup](../../../../../crates/sdk/src/local_setup.rs) exist at preparation HEAD. Packet 1 must prove the new consuming path and held authorization, rather than count those sources as new behavior tests. |
| Implementation authorization | **Unsatisfied.** Meta must present the bounded Packet-1 plan, its choices and packet review boundary for human authorization before dispatching product work. Current source and every actual runtime project's required prior terms must separately resolve at execution. |

No broad native permission platform, Context Resolution authority migration,
Substrate integration, publication or adapter is a hard predecessor for this cut.
The selected local route uses native initial authority, not generic authored-artifact
promotion of a Context Resolution capsule. If source investigation reveals a
necessary new dependency on that route or a consequential selection outside this
authority, stop before product edits and return the exact dependency to meta.

## Packet and human boundary

| Packet | Bounded outcome | Dependency | Human review |
| --- | --- | --- | --- |
| 1 — native registration and usable SDK | Committed `open` item, exact current/history/recovery, supported-composition and authority refusals, and explicit unsupported completion inspection through an ordinary SDK consumer. | Predecessors above and human approval of this concrete plan. | **Packet boundary.** The state and registration receipt are independently meaningful, so return the implementation result and stop. This one-packet outcome is also the slice's complete acceptance boundary. |

There are no hidden preparatory packets or future packets inside this slice.
Packet 1 may settle local type/function names, module split, canonical descriptor
bytes and transaction-file layout just in time from this authority and the listed
sources, without changing the chosen composition, effect, authority route, identity
semantics, recovery obligations or response behavior. Material decision refinements
receive focused causal review; unresolved consequential choices return to meta.
After packet completion, meta checks the eight conditions below and integration
once, performs the single aggregate closeout, recaps the result and stops. No
second mechanical commit or durable dispatch/review record is required.

## Complete slice acceptance

1. Native registration admits only the exact supported leaf composition, same
   positively registered project/root, bounded identity/content and initially
   `open` state; names alone create no operational meaning or completion claim.
2. Prior-approved current operating authority and its declared reach are held
   through the creation commit. Local-check successor adoption, D→G, missing,
   stale, revoked, mismatched or insufficient
   terms refuse; portable locators, filesystem access and local-check action classes
   cannot substitute. Advisory assertions remain labeled advisory.
3. One owner-confirmed receipt binds the exact creation request, instruction/
   authority basis, project, item, support descriptor and initial snapshot. It
   reports registration only. Failed validation, noncommit and uncertain commit
   cannot be rendered as created/closed success.
4. Exact same original request resolves/replays the original owner outcome after
   restart without a second item. Conflicting request/item reuse refuses. Current
   item resolution and historical request resolution retain separate meanings;
   history does not restore present permission. Later local-check adoption or D→G
   preserves existing `open` item truth/history while refusing new creation via
   the initial route.
5. Competing registrations/readers, interrupted persistence, substituted root or
   owner lock, partial/unknown inventory, missing/corrupt committed records and
   lost replies preserve truthful refusal/noncommit/uncertainty and exact recovery.
   No scan for a newest record, absent-file fallback or blind new-key retry repairs
   uncertain state.
6. Registration history and item truth survive ordinary SDK refresh/runtime-state
   reset. Loss of operating state blocks new writes without deleting or reclassifying
   existing work; recovery never allocates a replacement project or item identity.
7. Completion inspection reports supported-state facts separately from
   `unsupported / not assessed` completion. Assertions, legacy closeout results,
   independent passes and authored `completed` values create no Determination,
   disposition, authorization or closed-state receipt. Unsupported relations/kinds/
   lifecycle effects refuse rather than silently dropping their requested meaning.
8. A normal-dependency SDK consumer exercises registration, reopen, history,
   recovery/conflict and completion refusal using real native paths. Usage and
   response/recovery guidance make each next step clear without test-only authority
   construction or knowledge of the private file layout.

## Deferred capabilities and evidence limits

Positive local/parent eligibility, governed completion standard establishment and
assignment, exact assessment/result populations and supported current qualification,
separate close Intent/Authorization/Attempt/Terminal/Receipt and through-commit
completion checks remain subsequent separately authorized outcomes. So do optional
kinds, additional supported compositions/roles, candidate/result bindings, hierarchy,
typed prerequisites/timing/cycle closure, grouping, vocabulary selection, edits,
rebinding, reopen/cancel/removal and authority replacement/delegation. First support
of relations must retain the selected same-project one-parent and finite/cycle rules;
this leaf-only slice does not promise arbitrary configurable depth or graphs.

The four linked [research sources](../../semantic-model/amendments/work-progression-kinds-and-relations.md#sources-and-evidence)
remain proposed/datable evidence. This plan adopts only their findings that
explicit composition can precede a kind registry, Pipeline/Flow/artifact labels
are not work truth, and completion requires separate owner boundaries. Checkout
policies, taxonomies, retrospective/prospective protocols, parent cuts and external
examples are not promoted into this slice.

Preparation used source inspection, not runtime verification. Inherited local-check
limits remain with its completed evidence: Windows broader-suite limits, Unix
unexecuted, and the ordinary debug consumer's documented stack requirement. New
work must report its own actual platform and verification limits. GitNexus query/
context were attempted: the foreign/stale index, failed FTS tables, unsupported
Node 22.15 and status identity-hash failure prevented reliable navigation. Direct
source inspection supplements that degradation; no index repair is selected.
