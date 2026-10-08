# Work hierarchy and adapter discovery experiment

**Status: PROPOSED product recommendation for human review — 2026-10-07.**
This bounded paper experiment adopts no model and authorizes no implementation.
Initial and pre-write checkout: `feat/handbook-contract-membrane-clean`, HEAD
`70019b7356bd2451b2d72d389741bec0f7f1f525`, with other active changes. The target
path was absent. Only this new report is written; previous reports remain untouched.

## Recommendation and deciding evidence

Recommend **A initially: at most one containment parent per item, multiple explicit
prerequisites, multiple grouping memberships and exact references**. Permit
configurable vocabulary and supported composition without a universal taxonomy
or fixed depth. Keep optional immutable kinds compatible with explicit composition,
as proposed by the [kind exercise](work-progression-kind-decision-exercise-2026-10-06.md#decision-summary).
Cardinality is a versioned semantic constraint, not a permanent assertion that all
work is a tree. Reserve genuine multi-parent containment for an explicitly
versioned capability justified by a concrete joint-constituent requirement.

The human's branching example fits A directly. The inspected GSD Core and OpenSpec
sources show dependencies, artifacts and coordination without demonstrating a
need for the **same work identity to be contained by several operational parents**.
Asana supplies the strongest counterexample in this three-system sample: shared
membership and shared progress. Its project completion is a separate status
operation, so membership cannot automatically be interpreted as Handbook's
governed parent closure. This supports typed relations now, shared views where
appropriate, and later semantic extension when needed. It does not establish
universal adapter compatibility or absence of demand elsewhere.

The [program entry point](../specs/handbook-contract-membrane/README.md), approved
[parent boundary](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action),
[owner/reference rules](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#exact-references-do-not-transfer-authority)
and [G1–G6](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules)
control the analysis. They require finite exact boundaries, declared directions,
reach and cycle treatment, complete accounting, independently assigned governance,
current reliance and separate action authority/state receipts. They do **not**
select containment cardinality. The [preceding experiment](work-progression-contracts-governance-design-experiment-2026-10-07.md)
and [initial kinds research](work-progression-configurable-kinds-research-2026-10-06.md)
remain PROPOSED. The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
has proposed general defaults and fixed Contracts JSON; this report selects no
format, schema, layout or physical migration.

## The exact branching goal

This is an illustrative product hierarchy, not repository packet authority:

```text
Feature F: Unified sign-in
├─ Phase P1: Establish authentication
│  ├─ Slice S11: Authentication endpoint
│  │  ├─ Packet T111: Build candidate
│  │  └─ Packet T112: Verify endpoint
│  └─ Slice S12: Session lifecycle
│     ├─ Task T121: Implement renewal
│     └─ Task T122: Verify expiry
└─ Phase P2: Deliver consumers
   ├─ Slice S21: Administration experience
   │  ├─ Packet T211: Implement sign-in
   │  └─ Packet T212: Verify integration
   └─ Slice S22: Mobile experience
      ├─ Task T221: Implement sign-in
      └─ Task T222: Verify integration
```

Every parent has several children; every child has one containment parent. Renaming
Phase to Stage or Slice to Increment changes presentation. Supported compositions
may omit levels, put Packets below a Task, or combine roles in a small item. Each
operation still needs a finite declared boundary; configurable depth does not
authorize open-ended traversal. Labels alone select neither lifecycle nor authority.

| Distinction | Meaning |
| --- | --- |
| Breadth | A parent has many children; A already supports this. |
| Depth | More nested levels; this does not require multiple parents. |
| Multiple parents | One identity is a constituent of two parents, with one actual item state. |
| Grouping | One identity appears in several coordination/reporting populations; no completion effect follows merely from membership. |
| Prerequisite | A dependent operation requires a declared proposition about another item; that item need not be its constituent. |

## Compare the three models fairly

| Model | Genuine benefit | Necessary cost and limit |
| --- | --- | --- |
| **A: containment forest plus typed relations** | Matches the example; supports shared blockers and multiple views while keeping constituent responsibility clear. | Cannot claim one item is a constituent of two parents. A grouped display or prerequisite is a different relation, even if the screen resembles shared containment. |
| **B: genuine multi-parent containment** | Jointly planned work can contribute one identity/state to several constituent populations without an artificial central parent or duplicate items. | Requires parent-relative inclusion, requirements/coverage and removal meaning; shared cancellation consequences; coherent cuts and duplicate accounting. A containment DAG alone supplies none of these. |
| **C: supported configurable cardinality** | Different exact profiles could support exclusive decomposition and shared constituents, with a constrained initial profile. | Must define who establishes/selects the policy, compatible edge endpoints, allowed cardinality/reach, state/closure consequences, cycle checks and profile-change behavior. A configuration flag cannot invent these semantics. |

C can be bounded and is not necessarily a generic graph/workflow engine. However,
if its only initially supported cardinality is zero-or-one, its usable behavior
equals A. Advertising a many-parent setting without its lifecycle semantics is
false flexibility. Supporting B's semantics now but disabling them by default
still incurs their design and validation burden. Therefore select A's exact typed
relations now; add a supported C-style capability later if evidence warrants it.
Do not make cardinality caller-configurable today. Versioning A's rule preserves
an extension boundary without pretending the extension exists.

## Small primary-source sample and mappings

Public sources were read on 2026-10-07. GSD uses the `v1.13.0` tag; OpenSpec uses
commit `9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461`. Neither is asserted to be latest.
Asana documentation is current unpinned evidence. “Mapping” below is a proposal,
not an implemented adapter or proof of available verification/effect routes.

| System and documented relation | Mapping into A / B / C | Preserved meaning, loss and effect boundary |
| --- | --- | --- |
| **GSD Core:** [planner](https://raw.githubusercontent.com/open-gsd/gsd-core/v1.13.0/agents/gsd-planner.md), “PLAN.md Structure,” records a phase, plan, tasks, `wave` and `depends_on` plan IDs. [Commands](https://raw.githubusercontent.com/open-gsd/gsd-core/v1.13.0/docs/COMMANDS.md), `execute-phase` and `complete-milestone`, distinguish plan execution/phase verification from milestone archive/tag effects and document an unstarted-phase guard with a forced path. | **A:** proposed milestone → phase → plan decomposition; tasks can remain authored plan detail unless a selected mapping establishes work instances. Plan dependencies are prerequisites, waves coordination/order. **B:** no shared constituent identity demonstrated. **C:** the constrained profile maps the same facts; wider cardinality adds no demonstrated fidelity. | Inspection can retain exact source names, dependency IDs and execution grouping. Wave-only grouping loses GSD execution scheduling if presented as enforcement. An effect mapping must declare supported operation timing and authoritative prerequisite propositions; source summaries, verification or force flags do not establish Handbook close authority. Refuse any such substitution. |
| **OpenSpec:** [workflows](https://raw.githubusercontent.com/Fission-AI/OpenSpec/9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461/docs/workflows.md), “Parallel Changes” and “Completing a Change,” describe distinct changes, optional/nonblocking verification, and archive warnings for incomplete tasks. [Customization](https://raw.githubusercontent.com/Fission-AI/OpenSpec/9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461/docs/customization.md), “Schema Structure,” defines `requires` as artifact existence dependencies; schema validation rejects circular dependencies. | **A:** Change may map to bounded work; task checklist is source detail or separately established children. Artifact references/readiness are distinct from item closure prerequisites. **B:** several changes touching the same spec target are not evidence of one shared work instance. **C:** extra containment cardinality does not supply artifact-readiness semantics. | A view can preserve artifact dependency graphs without making every artifact a work item. Enforcing readiness requires an explicitly supported exact artifact-existence proposition, not “artifact child closed.” Refuse archive → governed completion, or executing arbitrary custom instructions as native policy. Mapping losses do not grant effect authority. |
| **Asana:** [Tasks API reference](https://developers.asana.com/reference/tasks) distinguishes one `parent` from `projects[]`/`memberships[]` and a task-level `completed` flag. [Multi-homing](https://help.asana.com/s/article/how-to-multi-home-tasks?language=en_US) describes one task updated across projects. [Removal](https://developers.asana.com/reference/removeprojectfortask) preserves the task. [Portfolio management](https://help.asana.com/s/article/portfolio-management) reports project task/milestone completion percentages. [Project completion](https://help.asana.com/s/article/project-progress-and-status-updates?language=en_US) is a separate selectable status; archive does not update that status. | **A:** preserve subtask parent separately from multiple project grouping memberships and shared source state. **B:** could represent shared constituent participation only through an expressly selected interpretation; treating project membership as “all children must close” adds meaning absent from this evidence. **C:** a distinct reporting-membership relation suffices for the documented case; no cardinality switch is required. | Honest inspection retains identity, memberships, state and source progress meaning. Mapping every project to exclusive native containment would lose membership or duplicate identity: refuse it. A selected external membership effect would mutate Asana membership, not native containment; supported authorization/currentness still need separate evidence. Project Complete and task `completed` cannot supply Handbook closure receipts. |

**Inference:** if each Asana project's reported task population includes shared
task Q, Q's single completion state can contribute to both percentages. This is
shared constituent *reporting*, a substantive counterexample to exclusive project
membership. The sources do not establish that either project must wait for Q
before its Complete action. They establish a separate project action, not its
complete precondition set. No actual multi-parent operational-containment need
was found in the selected **GSD/OpenSpec skill-family sources**. The tracker
counterexample cannot establish demand across skill families.

## One shared-work counterexample

All following behavior is proposed. In one project, Features F1 (sign-in) and F2
(account recovery) each contain four direct delivery Tasks. Q is one security
Task assessing the shared authentication core at exact candidate revision R1.
There is one Q identity, state and closure history. A leaves Q as a separate root
and declares F1 → Q and F2 → Q close prerequisites, directed dependent-to-required
item. B contains Q in both Features; each proposed parent policy requires its
actual closure. C initially behaves as A; enabling shared containment later would
need B's complete rules.

| Event/test | A: shared prerequisite | B: shared containment; C when enabled |
| --- | --- | --- |
| Add Q after F2 eligibility | New prerequisite cut invalidates F2's affected reliance. A grouping addition alone changes reporting, unless an explicit policy consumes it. | New constituent cut invalidates F2's parent basis. Adding a closed Q still requires current congruence/coverage, not automatic acceptance. |
| Remove membership/relation | Removing a group membership leaves the required prerequisite. Removing the prerequisite needs authorized scope reconciliation; it cannot erase required security. | Removing Q changes constituent responsibility, but cannot erase independently required feature security. Define remaining responsibility; preserve Q and F1's relation. |
| F1 cancels or closes | Neither action cancels/closes Q or F2. F1 closure, if permitted, needs its own requirements and Q's exact required state proposition. | Same separate state rule is needed. F1 cancellation cannot cancel Q merely because Q is its child; Q may still be required by F2. Closure does not cascade. |
| Q cancels, or scope changes to R2 | Cancelled is not successfully closed. Both required uses fail their declared successful-closure condition. An authorized Q scope change requires affected currentness/coverage reconsideration. | Same shared-state consequence, plus two constituent cuts. Neither parent can retain an obsolete “done” contribution as current truth. Historical R1 receipts remain exact history. |
| Parent-specific requirements | Q's core assessment does not establish F1 browser integration or F2 recovery-abuse coverage. Each Feature closes its own exact assessment roles. | Multiple parents do not expand Q's assessment coverage. If Q's actual scope incorporates both requirements, independently resolve its supported obligations and obtain adequate evidence. Otherwise parent-specific assessments remain separate. |
| Governance and stale currentness | Resolve project/scope applicability independently, including expressly assigned scope rules. Relation/state/requirements changes invalidate affected bases; enforce declared through-commit facts. | Extra ancestry increases candidate/accounting scope, not authority. No automatic inheritance, nearest-parent winner or universal strictest rule. Incompatible required composition refuses. |

If Q is complete for F1's intended use but lacks F2-specific coverage, Q's one
actual state remains unchanged; F2's reliance may refuse or be non-positive under
its selected policy. Do not invent two Q states. Distinct parent-relative
assessment/reliance roles need their own supported semantics. Nor should adding
Q to F2 silently redefine Q's completion question. Unsupported rebinding refuses.

With three of F1's four own Tasks and two of F2's four closed, plus Q closed,
A's constituent progress is 3/4 and 2/4, with the shared prerequisite ready.
B's constituent counts, or an explicitly grouped reporting view in A, can be
4/5 and 3/5. Combining these gives 7/10 **membership contributions**; unique work
is 6/9. Count each relation once within each declared parent boundary; count Q once
in an explicitly unique-item metric. No percentage is an eligibility decision.

Thus prerequisite and containment can both require Q's actual closure, but are
not equivalent: containment declares constituent responsibility and changes the
parent's work population; prerequisite declares a required outside proposition.
A loses genuine joint decomposition. A view preserves shared visibility and
counting, not that operational meaning. Duplicating Q would falsify identity.

## Cycles and prerequisite timing

Reject structural containment cycles: A is a forest; B could be a DAG; C must
declare the supported treatment for each profile. Separately check finite closure
dependencies: parent close → required child close, and dependent close → required
prerequisite close. B's F1 contains Q while Q requires F1 closed has no containment
cycle but creates a closure deadlock; refuse the changed cut. A's F1 → Q and
Q → F1 close prerequisites have the same problem. A tree alone prevents neither.

The preceding experiment's [per-operation timing](work-progression-contracts-governance-design-experiment-2026-10-07.md#concrete-checkout-feature-and-bounded-relations)
matters independently of cardinality. Q may inspect existing candidates from
unfinished Feature work; candidate availability is not that work's closure. A
security prerequisite can guard closing while allowing earlier construction.
Declare the guarded operation/proposition; do not turn every reference or start
condition into a close dependency. Reject unresolved/unsupported timing instead
of promising a general scheduling solver.

Informational F1 ↔ Q citations may form navigation cycles without closure effects.
They cannot bypass the approved [same-occurrence causal rules](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#causal-and-anti-cycle-rules):
an output cannot retroactively become its own requirements input. Later feedback
uses a later revision/new occurrence where required.

## Extension obligations and human decisions

The strongest argument against A is **joint ownership of one constituent**: a
future family may require Q to belong operationally to both Feature plans, with
distinct parent acceptance roles and removal/cancellation obligations. Shared
prerequisites cannot preserve that fact; centralizing Q may impose an unwanted
planning boundary. This would justify reconsideration if supported by primary
documentation plus a concrete intended Handbook use showing why grouping and
prerequisites lose required operational meaning. Shared labels, several children,
multiple project display or hypothetical flexibility alone do not justify it.

Before relaxing cardinality, specify the exact relation/capability version,
selection authority, supported endpoint/profile combinations, parent-relative
coverage and responsibility, closure/cancellation/removal meaning, complete
accounting and coherent currentness/cycle checks. Preserve Q's identity, old
relation cuts and receipts; keep earlier profiles/history resolvable. Existing
items do not silently rebind. An authorized supported conversion must revalidate
affected cuts and independently current governance; an older adapter must refuse
unsupported semantics rather than flatten them. These are conceptual compatibility
obligations, not a selected schema or migration mechanism.

At most three decisions are ready:

| Human decision | Recommended answer |
| --- | --- |
| Initial native relationship model? | A, with finite single-parent containment, multiple direct same-project prerequisites with operation timing, and separate grouping/references; configurable names and supported levels. |
| Configurable containment cardinality now? | No. Permit later explicit versioned capability planning when a demonstrated joint-constituent use supplies the missing lifecycle semantics; do not promise B/C support now. |
| Adapter fidelity boundary? | Preserve exact supported meaning for effects; disclose source semantics/losses in views and refuse unsupported effect translations. Select family mappings individually, without universal compatibility claims. |

## Verification and limits

Verification consists of primary-source reading, agreement with current local
semantic boundaries, local-link/anchor checks, scenario arithmetic and cycle/state
consistency, and report-only scope checks. Help Center facts were available through
indexed first-party extracts; direct Asana Help opens returned a navigation shell.
API relation/removal facts were read directly. No external API execution or runtime
test is claimed. No code exploration/symbol edit was needed; GitNexus MCP tools
were not exposed, and no graph inference or index rebuild is used.

One focused local reviewer challenges source fidelity, the counterexample and
recommendation scope under the [P1–P4 causal review policy](../specs/handbook-contract-membrane/review-and-closeout-policy.md#finding-priority-and-disposition);
its result is returned in chat. Technical review does not adopt the proposal.
Native owners/SDK remain the current stage; CLI/skill-family adapters are future
constraints. Owner placement, lifecycle realization, policies, external effect
verification, schemas and implementation remain deferred. No packet planning,
sequencing/status change, staging or commit follows. This result returns to
meta/human discussion; no further generic study is required before choosing A.
