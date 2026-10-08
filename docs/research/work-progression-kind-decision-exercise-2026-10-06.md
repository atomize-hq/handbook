# Work-kind decision exercise: composition versus reusable kinds

**Status: PROPOSED decision exercise for human review — 2026-10-06.**
This companion to the [initial research](work-progression-configurable-kinds-research-2026-10-06.md)
recommends a product direction; it adopts no model and authorizes no runtime,
registry, storage, API, adapter or implementation work. Baseline:
`feat/handbook-contract-membrane-clean`, HEAD
`70019b7356bd2451b2d72d389741bec0f7f1f525`. Only this companion is written.

## Decision summary

**Recommend approving optional immutable, versioned operational work kinds as a
bounded product-model direction, while keeping explicit composition valid and
deferring any separate registry implementation.** A kind names one reusable
semantic composition and its supported compatibility contract. It neither adds
new behavior nor independently assigns governing policies or authority.

The exercise finds no behavior that B can express and A cannot. Its positive case
is a shared semantic identity: “this item and these adapter mappings use this
exact composition contract”, with an attributable unit of definition review and
revision. Given the intended custom work vocabulary and future family adapters,
that is useful beyond reducing typing. It justifies an optional abstraction, not
a mandatory taxonomy or a new execution platform.

This is a stronger recommendation than the first report's conditional deferral,
because the same supported roles have different completion meanings in the maps
below, and a whole-composition identity gives those meanings a reusable reference.
The recommendation is a design judgment from paper cases, not measured runtime
benefit or proof of broad adapter coverage. A remains simpler for isolated work.

## Common facts, assumptions and authority

All IDs, states and component selections below are **illustration-only**, not
implemented primitives or proposed serialized fields. Assume a future supported
menu of bounded-work lifecycle and relation capabilities, and owners able to
verify the necessary facts. If that menu or a verification route is unavailable,
both alternatives refuse the affected operation; a kind cannot supply it.

Use these responsibility names without choosing a crate, repository or physical
host:

- **Work owner W:** authoritative identities, state snapshots, role/composition
  bindings, containment, prerequisite edges, grouping and ordering for these local
  items. W establishes exact relation cuts and authorized state changes.
- **Dataset owner X:** authoritative external Dataset D revision and verification
  of its readiness/provenance. W owns each local prerequisite edge to D, not D's
  state. Supported cross-owner verification is assumed for this exercise.
- **Definition responsibilities:** W's supported composition/lifecycle conformance;
  Work Progression's admitted local/parent policies; the existing vocabulary owner;
  and, for B, establishment/resolution of kind definitions under an as-yet-unselected
  responsibility. These remain attributable even if co-located.
- **Work Progression, assessment and action owners:** Work Progression establishes
  current eligibility from exact owner facts and independently resolved governing
  requirements; Contracts preserves assessment truth; Engine supplies its required
  authority/posture facts; action authority and W establish authorization and actual
  committed work-state effects separately.

This follows the approved [ownership and local closeout boundary](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership),
[parent boundary](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action)
and [independently governed completion](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#independently-governed-completion).
Neither A nor B obtains an exemption from them.

**Policies and authority are independent of packaging.** An item may propose or
bind a policy reference through a supported input role. Its effective operational
selection and any governing assignment must independently resolve under prior
authorized terms for the actual project, target, effect and scope. The kind's
references constrain/propose compatible inputs; kind admission or binding does
not activate governance, assign obligations or authorize completion. Applicable
current governance may require additional inputs or reject the proposed choice.

Independent contract authoring, completeness validation and supported direct
evaluation remain usable **without work-item/kind registration or any skill pack**.
Authoring and completeness validation retain their applicable owner rules.
Supported direct evaluation additionally requires positive governance-applicability
resolution, independent operating authority, compatible complete definitions and
intent, readiness and truth closure under [G5](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#g5--independent-evaluation-and-governed-reliance).

## Same two cases in both alternatives

**Delivery case.** Milestone M, “Checkout v1”, contains Phase P, “Guest checkout”.
P contains small Plan A, “Create endpoint”, and Plan B, “Add UI”. Both plans are
one visible item combining implementation unit, execution envelope and atomic
action. Wave 1 groups Plan A; Wave 2 groups Plan B; Wave 1 precedes Wave 2.
These group/order facts describe the selected execution arrangement. They do not
create more completable items, execution authorization or an automatic scheduler.

This uses the milestone/phase/plan and wave distinctions in the previously
inspected [GSD Core v1.13.0 command reference](https://raw.githubusercontent.com/open-gsd/gsd-core/v1.13.0/docs/COMMANDS.md),
especially `spec-phase`, `execute-phase`, `audit-milestone` and `complete-milestone`.
The checkout example and Handbook mapping are invented; GSD summaries, verification
files, archive and tag effects are not Handbook completion receipts.

**Investigation case.** Experiment E asks whether caching improves latency by at
least 20%. It contains Trials T1 and T2; both require external Dataset D, exact
revision 3, under X's verified provenance/readiness. Reproducible observations
show gains of 2% and −1%. The hypothesis is disproven, while the investigation can
be properly completed. The separate delivery requirement for a 20% improvement
remains unsatisfied. T1/T2 each combine the same three roles as Plan A/B.

Only four relation meanings are used:

| Relation | Exact initial population / meaning | Authoritative responsibility |
| --- | --- | --- |
| Containment | M → P; P → Plan A/B; E → T1/T2. Each parent has its own finite direct-member cut; no universal transitive rollup. | W owns membership and snapshots; Work Progression closes the declared boundary. |
| Prerequisite | T1 → D and T2 → D. D is shared and external, not a child or duplicate item. | W owns the edges; X owns the target facts and verifies their declared currentness. |
| Grouping | Wave 1 groups Plan A; Wave 2 groups Plan B. Waves have no completion state here. | W owns membership; a view may render it. |
| Ordering | Wave 1 precedes Wave 2. This is an order fact, not a prerequisite or permission to start work. | W owns the fact; any later execution enforcement would need separately selected semantics. |

This small example establishes neither a universal tree nor arbitrary graph
support. Unsupported cycles, incomplete boundaries or mixed relation cuts refuse.

## Shared components: A receives full reference reuse

The following conceptual definitions are shared by both alternatives. `@1` means
an exact established version/content identity for this illustration, not a schema
proposal. Policy names describe their questions, not executable rule syntax.

| Component | Supported meaning assumed for the exercise |
| --- | --- |
| `Bounded work@1` | A native lifecycle supporting attributable work-state observation and a separately authorized explicit close action. No caller-authored transitions or general state-machine language. |
| `Delivery adequacy@1` | Local assessment of the actual item's delivery requirements, with complete declared inputs/currentness. |
| `Milestone readiness@1` | The milestone's own scoped assessment; child closure alone is insufficient. |
| `Investigation adequacy@1` | Adequate conduct, accounting and justified conclusion about E's question, independent of whether the hypothesis is favorable. |
| `Trial integrity@1` | Correct conduct and closed observations for a trial, independent of the measured gain's direction. |
| `Closed members@1` / `Closed trials@1` | Parent policies requiring exact current committed child closure, complete direct-membership accounting and the parent's own applicable requirements. They consume actual state/receipts, not copied eligibility labels. |
| `Composition conformance@1` | Shared owner-supported validation of role/lifecycle/relation/policy compatibility, with finite supported combinations. A can reuse this validation and exact policy references; it does not reauthor policies per item. |
| `Vocabulary@1` | Local display/lookup choices, preserving typed roles and semantic identities. Work labels below are illustrative; they do not extend the implemented vocabulary contract automatically. |

The governing selection relationship and actual assessment role inventory remain
independent per target/use. Reusing a policy does not share an evaluation occurrence,
an authorization, a target snapshot or a result between work items.

## Side-by-side conceptual instance map

Role abbreviations stand for exact existing registry identities:
H = `coordination_horizon`; D = `delivery_unit`; I = `implementation_unit`;
E = `execution_envelope`; A = `atomic_action`. The multiple-role combinations
are assumed supported by future work-owner conformance, not proven operational
behavior of today's [registry](../../crates/engine/definitions/stable-roles/handbook.roles.core/1.1.0.yaml)
or [vocabulary absorption fixture](../../crates/engine/tests/fixtures/hcm_3_1_vocabulary_resolution/definitions/vocabularies/example.vocabulary.hcm-3-1/1.0.0.yaml).

All local rows retain `Bounded work@1`, W-owned identity/state and their exact
per-item scope and snapshots in both alternatives. The same actual relation
population above applies to A and B.

| Exact item(s) and roles | A: explicit selections on each item | B: exact optional kind binding and its same composition |
| --- | --- | --- |
| M: H + D | `Milestone readiness@1`, `Closed members@1`; containment participation; label Milestone. | `Milestone@1` fixes H + D, lifecycle and those policy references/compatible containment participation. M binds it. |
| P: D | `Delivery adequacy@1`, `Closed members@1`; containment participation; label Phase. | `Delivery phase@1` fixes D, lifecycle and the same references/participation. P binds it. |
| Plan A, Plan B: I + E + A | Each references `Delivery adequacy@1`; no parent policy; grouping/order participation; label Plan. Shared definitions are referenced twice, not duplicated. | Each binds `Delivery plan@1`, fixing the same roles/lifecycle/reference/participation. Group membership/order facts remain per-item. |
| Experiment E: I | `Investigation adequacy@1`, `Closed trials@1`; containment participation; label Experiment. | E binds `Experiment@1`, fixing I, lifecycle and the same references/participation. The hypothesis and trial membership remain per-item. |
| Trials T1, T2: I + E + A | Each references `Trial integrity@1`; no parent policy; prerequisite participation; label Trial. | Each binds `Trial@1`, fixing the same roles/lifecycle/reference/participation. Each still binds its own prerequisite edge and trial target. |
| Dataset D revision 3 | Exact external target reference and X-owned observations; no locally owned work state or implied close action. | Identical. No Handbook kind binding is needed to fabricate ownership of D. |

In A, creation or an authorized revision supplies the exact selection tuple; W
validates it using shared conformance and authoritative dependencies. It can be
copied as a candidate from another item, inspected, compared and given a
deterministic content identity. Copying does not admit or activate it. A reusable
generic helper or validation policy is permitted; A lacks only a separately
established **named whole-composition contract**.

In B, the definition responsibility first establishes that whole composition;
W validates the item's exact binding and item-specific facts. Work Progression
still resolves its effective policies and governing applicability independently.
The kind identifies the reusable contract; it does not own state, membership,
evidence or eligibility. It is a semantic definition, not another authored family
under the approved [ontology](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#generic-artifact-inclusion-and-exclusion).

Both retain per-item identity, question/scope, current state, actual relation cuts,
assessment targets/results, effective governing selection, authority provenance,
reliance bases, attempts and receipts. B additionally retains kind binding and
definition history. Neither chooses actual children or an external Dataset revision
from a kind default.

## Completion and change walkthrough

**Completion question.** Plan A/B ask whether their scoped delivery requirements
are met; P and M each have their own requirements and parent decisions. Trials
ask whether they were conducted correctly. E asks whether the experiment was
adequately conducted, accounted for and concluded. Favorable cache performance is
not the latter proposition. In both maps, the negative observations can support
a positive, properly qualified decision-seeking assessment of investigation
adequacy. Diagnostic provenance cannot substitute for the mandatory positive
assessment role; calling failed delivery “Experiment” cannot change its actual
governing obligation.

**Eligibility → authorization → committed state.** After T1/T2 actually close,
their W-owned state/receipts and X's required verification enter E's complete
current parent basis alongside E's own qualifying assessment. Work Progression
may produce `eligible / allow`. Separate congruent action authorization and an
admitted attempt are still needed. W enforces the expected target, governing
conditions and declared dependencies through commit, then issues a receipt. A
later Work Specification revision may reference that outcome; an authored
`completed` field establishes none of these propositions. This chain is identical
for A and B and follows the approved [action boundary](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#action-authorization-and-mutation).

The changes below are considered separately from that successful path:

| Event | A: explicit composition | B: optional kind |
| --- | --- | --- |
| E remains active; supported v2 investigation policies add a calibration-verification requirement and corresponding parent accounting | Establish exact `Investigation adequacy@2` and `Closed trials@2` under their prior authorized lifecycles. Validate their supported combination. Propose a new E composition binding replacing those references; independently resolve whether it is permitted and satisfies current governance. | Establish the same policy versions; establish `Experiment@2` containing their coherent combination under prior definition authority. E stays on `Experiment@1` until a supported, authorized explicit rebind to v2 commits. Kind establishment does not itself change E or policy assignment. |
| Shared defaults now recommend v2 | New proposals may use v2. Existing item tuples keep their exact refs; affected items are identified by those refs/selection tuples. | New proposals may bind v2. Existing kind bindings keep v1; affected items are identified by exact kind bindings as well as policy dependencies. |
| Active E adopts the change | W preserves the prior binding and establishes a new snapshot/binding. Reconcile actual target, policy selection and governing obligations; invalidate affected reliance and obtain required fresh/current-qualified owner results. | Same checks and history, with an explicit v1 → v2 kind-binding change. No automatic rebind of trials, no hidden bulk migration, and no semantic equivalence inferred from a version number. |
| Closed items used v1 | Preserve original bindings, results and receipts. Current future-use governance is resolved independently. | Preserve v1 definitions/bindings/results/receipts. A closed v1 item is not silently relabeled v2 or used to freeze current governance. |
| T3 is added after E was eligible over T1/T2, before close commit | Authorized membership change establishes a new relation cut. The prior determination remains history; E needs a new parent basis/result and affected assessment coverage. Through-commit checks reject the stale closure. | Identical. A kind binding does not identify current membership, make an old determination current or excuse T3. |

New calibration requirements may need additional verification, assessment or
observations where the supported protocol cannot qualify existing results. Neither
alternative promises automatic reruns or automatic reuse. The actual effects of
policy replacement remain governed by prior authorized terms under
[G1/G2/G5](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules).

## Where kind identity helps, and where it does not

| Benefit test | Strong composition-only case | Additional value and limit of kinds |
| --- | --- | --- |
| Coherent combinations | A's shared conformance rule validates each tuple; exact policies already carry reusable behavioral meaning. It need not permit incoherent choices. | B supplies one established identity for a selected combination and its compatibility contract. It reduces rediscovery of the same combination; it does not replace item conformance, current governance or substantive policy adequacy. |
| Cross-family adapters | A can map exact supported tuples/policy refs, independent of local labels. Role matching alone is insufficient: plans and trials share roles but ask different completion questions. | B lets selected family mappings name one exact reusable contract. An external “Task” may map to `Delivery plan@1` only through an expressly supported mapping preserving all required meaning. A “Trial” mapping is different even with the same roles. No arbitrary cross-kind semantic equivalence is implied. |
| Shared identity/revision | A can fingerprint the actual tuple and compare versions of shared component refs. If an existing named composition-selection mechanism already serves as the established whole contract, an additional kind would duplicate it. | B makes the whole contract an explicit reusable product concept, independently referenceable from any one item. Review and mapping compatibility can follow that contract across items/families rather than reconstructing it from each tuple. A named immutable composition contract is functionally B even if called a profile. |
| Item variation | A allows supported exact variations without introducing a definition for each one. | Recommend no silent override of a kind's meaning. Vary declared per-item inputs within its supported bounds; otherwise choose an explicitly established variant kind or explicit composition and stop claiming the unchanged kind binding. Both routes need authorized validation; neither creates a policy exception. |
| Version compatibility | A can validate v2 tuples and maintain exact adapter input conditions. Policy-only changes often gain nothing from another version wrapper. | B provides a compatibility subject, not automatic compatibility. Unchanged display names or additive-looking edits prove nothing. Declare supported mappings for exact versions; refuse affected mappings until their required meaning is supported. |

**Boolean-only adapter.** Assume a tracker exposes only `done`, without exact
completion reason, hypothesis verdict, membership, prerequisites or authority.
Either map may display its observed Boolean with explicit loss. Neither can turn
it into Trial integrity, Experiment completion, cancelled/archived work or a
Handbook receipt. Kind B identifies *which* meaning cannot be preserved but adds
none of the missing facts. Refuse an affected effect mapping unless an independently
supported authoritative route supplies the full required facts. Conversely, an
outward coarse Boolean may be a disclosed projection; it is not a lossless sync
or proof about the external system's state. The previously inspected
[OpenSpec workflow](https://raw.githubusercontent.com/Fission-AI/OpenSpec/9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461/docs/workflows.md)
reinforces why archive and governed completion cannot be conflated.

**Strongest B case:** users and multiple adapters deliberately reuse and evolve
the same coherent semantic combination. A whole-contract identity makes that
shared meaning attributable and its compatibility review explicit. This is more
than a template or convenient shorthand.

**Strongest A counterargument:** all required behavior, safety, exact history and
component reuse are already achievable through supported composition. B adds
another admission/resolution/version/binding obligation; proliferation of nearly
identical kinds or exceptions can make it harder to understand the actual rules.
This exercise cannot measure whether that cost pays back in ordinary use.

**Concrete discriminator:** if adapters and users reference/evolve
`Experiment@1` as one contract across independently owned items, keep the optional
kind direction. If references are only to its investigation policy and every
item chooses a different combination, defer kinds and use explicit composition.
If another owner-established named composition contract already provides the same
identity and compatibility lifecycle, reuse it rather than create a parallel
registry. The recommendation changes to deferral if kind adoption requires novel
workflow primitives or a mandatory wrapper around independent Contracts use.

## Product approval versus realization

Product-model approval would select optional reusable work kinds, their semantic
limits, exact binding/variation rules and independent ownership/authority
boundaries. It would not establish a shipped kind population, registry service,
`.handbook` layout, schema, native operation, persistence mechanism or CLI route.
Later bounded planning selects the work-state/structure and definition
responsibilities and implements usable native owner/SDK operations; adapters
consume those results later. Preserve [SDK-first sequencing](../specs/handbook-contract-membrane/phase-and-slice-map.md#current-delivery-sequencing)
and the existing [HCM-6.5 decision boundary](../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption).
Pack installation grants no authority under [G6](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#g6--future-packs-and-adapters).
No hooks, arbitrary rule language, programmable transitions or universal workflow
engine are selected.

The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
still has proposed general defaults and controlling existing versioned contracts,
including Contracts JSON. No format choice or migration follows from kind semantics.
Current Work Progression policy content, concrete work primitives and placement
[remain unimplemented/unselected as described by their authority](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#settled-versus-deferred).
This exercise is independent of P3/P4 and selects no successor packet or phase.

## Verification and limits

The original report and its verified sources were reused; current relevant
ownership, completion, ontology, G1/G2/G5/G6, format status and sequencing sections
were rechecked. GSD's pinned wave/verification distinctions and OpenSpec's pinned
archive distinction were refreshed through primary-source browsing. Concrete
items, numeric observations, supported menu and v2 policies are declared
illustrations; no documentation is claimed to implement them. The prior
standalone-use review advisory is stated explicitly above.

Verification covers local links, source fidelity, equal scenario inputs, explicit
assumptions and companion-only scope. No runtime tests, code prototype, symbol
edits, index repair or commit are needed or performed. One focused local reviewer
evaluates the recommendation and authority fidelity under the P1/P2 blocking,
P3/P4 advisory rubric; the session return supplies its disposition. Existing staged,
unstaged and untracked work, including the original report, is preserved.

## Human product decisions ready now

| Decision | Recommendation | Consequence |
| --- | --- | --- |
| Approve optional operational kind semantics now, or stay composition-only? | **Approve optional immutable versioned semantic compositions as kinds; retain explicit composition as a supported alternative.** Reuse an equivalent existing whole-contract mechanism if one is later selected. | Establishes the product direction and scope of the concept. A separate registry/storage/API implementation remains a later decision, with no new implementation authority. |
| What may vary, and how may active items change? | **Per-item variation only within declared supported bounds; no silent kind overrides, mutable meaning or automatic item migration.** Use an explicit variant/composition or supported authorized exact rebinding, preserving history and independently resolving current governance. | Keeps shared kind identity truthful. One-off work may use composition; changes may require fresh reliance or assessment. A version does not supply compatibility or a waiver. |
| What fidelity may adapters promise? | **Meaning-preserving supported effect mappings; disclose inspection/projection losses and refuse effects lacking required owner facts.** No packs or registration required for independent Contracts use. | Accepts partial family coverage and the Boolean-only refusal. Rejects workflow-programming expansion and authority from installation. |

These three decisions resolve the product choice and its most consequential
limits. If approved, the next work is bounded native-owner design under separately
authorized preparation, not another general study and not implementation in this
research session.
