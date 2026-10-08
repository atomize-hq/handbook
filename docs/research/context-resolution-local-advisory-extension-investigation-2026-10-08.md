# Extending local-advisory authority to native Context Resolution

Status: **completed investigation; recommendations proposed for human selection**,
2026-10-08. The human selected investigation of extending the existing mechanism,
not adoption of this operation's authority mode, runtime project permission,
implementation, compatibility option A, or a representation contract. This document
is research leading to a possible bounded outcome, not implementation-ready slice
authority. A clean technical review does not change that boundary.

## Recommendation and the need it addresses

Reuse Handbook's local-advisory attribution, exact approved-content binding,
predecessor checks, scope/ceiling checks, revocation, guarded consumption and
original-request recovery. Extend the Engine owner narrowly for **one reusable
native root-envelope authority binding**, with a typed SDK path, on positively
resolved current local state. Start with `RootCreation`, exact constraints and
the existing validation/mutation decision semantics. Keep child, escalation and
memory authority outside the first extension. This is a recommended direction,
not an adopted product rule.

There is already a useful initial local envelope path. If a caller only needs an
in-bounds envelope while initial operating terms remain current, use
`HandbookSdkV1::admit_local_envelope_request`; no new persistent binding is needed.
The remaining need is a reusable, explicitly selected Context Resolution binding
that can be resolved after reopening, independently replaced or revoked within
its established terms, and consumed on **current local-check successor state**.
The existing initial witness deliberately refuses that successor state. A saved
envelope request is not a selected authority stream, and saving a locator does not
create permission. [Initial SDK](../../crates/sdk/src/local_setup.rs),
[local authority](../../crates/engine/src/local_authority.rs),
[local-check authority](../../crates/engine/src/local_authority/local_check.rs).

The binding should select a narrower Context Resolution operating contract under
the existing authority lineage, not duplicate initial project setup or authorize
all Engine operations. Its management permission belongs to the local authority
owner; its definition resolution, admitted envelopes and currentness belong to
Engine Context Resolution. These can be colocated and returned in one owner
outcome without a second general permission platform or one record per noun.

The [earlier design](context-resolution-native-authority-lifecycle-design-2026-10-06.md)
at `70019b7` remains useful for the generic-artifact lifecycle mismatch and legacy
identity/history obligations. Its credential-preserving first route, fresh native
establishment, compatibility A and YAML publication recommendations were proposals.
This investigation recommends an additive local route instead of making credential
acquisition a prerequisite for every Context Resolution use. It does **not** relax
the exercised credential-backed publication contract or complete its migration.

## Evidence and governing boundaries

Source and meaningful test bodies were inspected on
`feat/handbook-contract-membrane-clean` at
`6aba0844d1a04e44fb5d15e28bc310ff966fee60`. The named output was absent and the tracked
tree was clean before writing. Unrelated untracked research/consultation files were
preserved. Direct diffs from `70019b7` to `be7d040` and then to this HEAD show no
changes to `context_resolution.rs`, `artifact_mutation.rs`,
`artifact_lineage_store.rs`, `context_resolution_kernel.rs` or projection tests.
P3/P4 are now committed evidence, rather than the working changes described in
the earlier investigation. Later HEAD changes through this pin are documentation.

The [HCM entry point](../specs/handbook-contract-membrane/README.md),
[lifecycle](../specs/handbook-contract-membrane/work-unit-lifecycle.md#authority-preparation-boundaries),
[review policy](../specs/handbook-contract-membrane/review-and-closeout-policy.md#causal-review),
[preparation discipline](../specs/handbook-contract-membrane/authority-preparation-prompt.md),
and active [routing skill](../../.codex/skills/orchestrate-top-level-tasks/SKILL.md)
bound this research/preparation role. No new slice, packet, roadmap edit or product
authority is created. [Architecture](../specs/handbook-contract-membrane/architecture-context.md#semantic-ownership-and-dependency-direction)
keeps Engine inward of Contracts and every semantic owner inward of SDK.
[Delivery sequencing](../specs/handbook-contract-membrane/phase-and-slice-map.md#current-delivery-sequencing)
keeps the current stage at native owners and usable SDK capabilities; new CLI,
installed product skills, GUI and automatic agent routing remain separate.
The [Context Resolution coverage entry](../specs/handbook-contract-membrane/coverage-and-landing-map.md#successor-readiness-after-independent-local-check)
still lists authority/compatibility/representation as unselected decisions.

Approved [object classification](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#object-classification),
[generic exclusion](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#generic-artifact-inclusion-and-exclusion),
[Engine responsibility](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#semantic-responsibility-map)
and [identity/currentness](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#identity-lifecycle-and-currentness-by-category)
separate category-2 definitions from category-3 envelopes, bindings and owner
determinations. Sharing files, hashing or locks does not share artifact-promotion
authority. The entire legacy capsule is not one category-2 definition.

[G1/G2](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules)
retain initial human decisions, exact supported subsequent delegation, prior-term
authorization and honest advisory provenance. A new grant cannot authorize its
own establishment. [Approval dimensions](../specs/handbook-contract-membrane/semantic-model/amendments/approval-authority-and-assurance.md#independent-dimensions)
separate decision authority, delivery, identity assurance and composition. The
amendment's dated P4 status prose is older than the current entry point and source;
its selected local-check intent is neither universal Context Resolution policy
nor permission for a credential fallback. Exact scope follows the
[shared admission rules](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#policy-admission):
no implicit scope inheritance, directory-order selection or recursive self-admission.

The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
was read before representation reasoning. Its general defaults remain **PROPOSED**;
Contracts JSON is fixed. Existing versioned formats and fingerprint contracts
control. This research authorizes no re-encoding or migration.

## What can actually be reused

“Unchanged” below means the existing responsibility is suitable for reuse; it does
not promise that private symbols can be exposed without a bounded interface change.

| Mechanism and source | Reuse / narrow adaptation / gap |
| --- | --- |
| `LocalOperatingTermsV1::fingerprint_approved_content`, `validate`, locator validation in [local_authority.rs](../../crates/engine/src/local_authority.rs); approval and project binding in [local_charter.rs](../../crates/engine/src/local_charter.rs) | Reuse exact content-bound instruction/principal/agent/delegation/source attribution and human approval assertion. The approval preimage excludes its assertion flag and own fingerprint. Retained owner state is independently checked; claims are still local-advisory, not authenticated identity or delegation. New operation-specific content needs its own versioned identity, never changed bytes under an old identity. |
| `LocalDecisionClassV1`, initial and `LocalCheckTermsV1` reserved/delegated sets | Reuse conflict refusal and exact prior terms. Existing classes are `PathExpansion`, `HorizonExpansion`, `CharterReplacement`, `TermsReplacement`, `InputCorrection`; their names do not implement each successor operation. No current Context Resolution management operation follows from an enum value or broad `OwnerWrite`. |
| `NativeLocalCheckAuthorityV1::with_admission`, `LocalCheckAdmissionV1` in [local_check.rs](../../crates/engine/src/local_authority/local_check.rs) | Adapt the live callback/guard pattern, exact request/content/closure/actor/participant/time and actual read/full-owner-subtree checks. The admission is nonserialized and nonclonable. Its four action classes and fixed action references concern Contracts definition/review/lifecycle/source use; borrowing one as binding-management permission would be false. |
| `admit_local_envelope`, `ContextResolutionEnvelope` in [context_resolution.rs](../../crates/engine/src/context_resolution.rs) | Reuse profile/stack identity, exact constraint population, rank ceilings, subject reach, selector containment and deny precedence. Existing bridge issues only RootCreation on the exact envelope candidate. A successor-aware native witness and persistent selected binding are missing. Do not fabricate registry mappings to make this bridge look credential-backed. |
| `LocalCurrentAuthorityWitnessV1::with_current` and `read_current`; local-check `require_predecessor`, `check_current`, `require_basis` | Reuse currentness responsibilities, adapt the correct lineage. Initial currentness refuses any local-check successor footprint. Local-check checks original unrevoked authority, exact current Charter, selected successor, its revocation, original lock generation and current governance relation/full tree. Never revive the initial witness after successor selection. |
| `NativeLocalEvaluationAuthorityV1`, terms/issuance/verification/history in [local_evaluation.rs](../../crates/engine/src/local_authority/local_evaluation.rs) | Reuse the pattern of human-approved domain terms, exact predecessor and lock generations, purpose/request-bound issuance, live checkpoint callbacks and separate history/current observations. Its private envelope is already retained in issuance history. Its authorization is purpose/plan/occurrence-bound evaluation authority, not a reusable Context Resolution publication or general action permission. |
| `LocalInputCorrectionDelegationV1::validate_under`, request/submission/decision handling in [correction.rs](../../crates/engine/src/local_authority/local_evaluation/correction.rs) | Reuse attribution, exact original/prospective content and prior-delegation checks as a precedent. Actual support is one immutable human-established InputCorrection grant; it binds initial/check/evaluation/Charter predecessors, reach, constraints, participants and retention. InputCorrection must be delegated and unreserved in both prior initial and check terms. It cannot appoint another decider, replace its grant or establish Context Resolution authority. |
| Initial/check/evaluation request recovery, revocation and exact history | Reuse original-request intent/terminal and immutable-history distinctions; adapt transactions for the new binding head and lifecycle. Original-key replay must not reactivate revoked/superseded authority. Existing domain stores prove their own interrupted effects, not a new store's atomicity. |
| [repository_durability.rs](../../crates/engine/src/repository_durability.rs), `RetainedRepositoryRootV1`, exact-file/tree checkpoints, existing native locks | Reuse physical-root identity, bounded regular reads/inventory, no-replace/exact durable writes and explicit uncertainty. Extend the retained authority partition only for actual new authoritative files; do not guard mutable output directories as immutable inputs. New lock order, reservation and recovery still require measured proof. |
| [SDK setup](../../crates/sdk/src/local_setup.rs), [local check](../../crates/sdk/src/local_check.rs), [evaluation](../../crates/sdk/src/local_evaluation.rs) and [workflow](../../crates/sdk/src/local_evaluation/workflow.rs) | Initial typed envelope admission and evaluation/correction workflows exist. Public SDK composition for a persistent successor-state Context Resolution binding does not. The current check SDK can resolve D or established-from-D G basis; initial setup's envelope path supports current D only. Engine must own the added decision; SDK carries supplied choices, outcomes and recovery keys. |

P4 does not impose a universal distinct decision agent/executor rule. Its grant
requires the decider differ from the principal; the human may explicitly name the
executor as decider. Actual selected review composition must still occur. The
[correction acceptance body](../../crates/sdk/tests/local_evaluation_native/correction_acceptance.rs)
checks both executor-as-decider compatibility and refusal of wrong actors or
unsupported delegation replacement. No new quorum or identity-assurance ladder
is justified by that example.

## Existing envelope coverage and the first mapped uses

`NativeLocalAuthorityServiceV1::admit_envelope` currently validates initial owner
state, then constructs a local `ContextResolutionAuthorityAdmission` with
`RootCreation`, the exact candidate binding and local request fingerprint. It
requires the exact selected profile/stack, exact approved active level, no parent
or escalation triggers, exact approved constraints, dimensions no greater than
the ceiling, objective within input reach and all mutation allows contained by
approved write reach. `authorize_validation` separately requires `LocalValidation`;
`evaluate_mutation` separately requires `OwnerWrite` and returns a decision. This
already supports operational authority, rather than just context display.

`projection_authority_view` can consume a current local envelope and preserves
its exact profile/stack/ranks. Its visibility is crate-private. Existing meaningful
projection fixtures use signed legacy admissions; they do not prove a delivered
local projection SDK journey. An Allowed mutation decision also does not hold
authority through some later write: the check's guard has returned. Each mutating
owner still needs a live guarded consuming seam through its own commit.
[Kernel](../../crates/engine/src/context_resolution.rs),
[projection tests](../../crates/engine/src/projection/tests.rs).

The eleven legacy mapped uses serve different propositions. Preserve their
meaning rather than making “native authority” imply all eleven:

| Use(s) | Actual consumer role | Recommended first local extension |
| --- | --- | --- |
| `RootCreation` | Exact root envelope candidate admission | Support under the selected current local binding. Existing local witness proves the shape without credential facts; successor/current-binding sensitivity needs adaptation. |
| `Constraint` | Exact constraint population and same-authority binding in root/child resolution | Support exact constraint consumption inside root admission as today. Do not expose arbitrary standalone Constraint admissions; existing local bridge checks equality to approved terms directly. |
| `Parent` | Exact child candidate and parent binding, same profile/stack/authority, dimension/path non-expansion | Refuse in the first extension. A narrower child is a plausible later extension, but today's local bridge explicitly refuses parents and has no parent authority contract. |
| `Requested`, `TriggerCondition`, `Evidence` | Escalation request, trigger and evidence; Requested/Evidence also participate in promotion | Refuse these independent mapped uses. Local refusal context is an unresolved request, not an admitted escalation or evidence authority. The local witness lacks the legacy candidate/requested mappings. |
| `Approving`, `Decision` | Escalation or promotion disposition and exact terminal decision | Refuse. A binding-management approval is a different decision, not these terminal admissions. P4 correction decisions cannot substitute. |
| `Source`, `Target`, `TargetMemory` | Semantic-memory promotion sources including envelope, target candidate and typed memory request/result | Refuse. `authorize_memory` explicitly refuses local witnesses; filesystem write reach is not memory-promotion authority. |

These roles are exercised by `resolve_root`, `resolve_child`, escalation
request/disposition and promotion request/disposition in
[context_resolution.rs](../../crates/engine/src/context_resolution.rs).
The signed [kernel test](../../crates/engine/tests/context_resolution_kernel.rs),
`real_authority_path_admits_all_eleven_uses_and_replays_exactly`, proves that existing
credential route and changed-subject replay refusal. It does not select eleven
local uses. Bounded validation/mutation decisions and private projection consumption
can remain root-envelope consumers without becoming more mapped authority roles.

## Smallest supported extension: three separate permissions

No current API supports this extension yet. The proposal needs one exact versioned
Engine rule for this operation, outside the binding candidates it decides. Merely
adding `ContextResolution` to a closed enum would leave establishment authority,
subject/participant scope, successor validation and commit semantics undefined.

**Initial human-established authority.** Recommend an explicit local rule allowing
the original claimed human principal to establish operation-specific management
terms on a positively current local-check lineage, with exact content approval.
The new rule must select that eligibility and decision authority explicitly; neither
old action classes nor `TermsReplacement` currently supply it. Require the existing
project/physical-root identity, retained initial and local-check outcomes, current
Charter, exact governance applicability and original lock generations. Keep the
new ceiling, exact constraints and actual proof-read/write reach within compatible
prior operating terms. Missing reach, an incompatible prior condition or a reserved
decision offered by an agent refuses; do not silently widen old terms. Establishing the new domain under that
fixed human rule is distinct from letting its proposed agent grant approve itself.
Whether to support this exact human establishment rule is a consequential product
selection still needed, not a permission already present in the checkout.

**Authority to establish/replace the binding.** The human-approved management
terms must distinguish establishing this exact stream, replacing its selected
binding, revoking it, and using it. Recommend reserving initial establishment and
management-term changes to the named human in the first outcome. A specifically
approved agent may replace the selected binding **within the retained management
grant**, if that bounded operation is selected; naming the executor or granting
path-write reach alone does not grant that decision. No agent may issue/replace
its own management grant in this first extension. A single coherent human review
may approve management terms and the exact first candidate; the owner must retain
the two independently attributable approval propositions. Separate files or a
general grant administration layer are not required.

For every proposed activation, bind and check the following before any effect:

| Binding | Proposed requirement and causal purpose |
| --- | --- |
| Establishment rule and prior authority | Exact rule version/content plus original project/initial/check identities, current unrevoked selected basis and explicit accepted human source for initial establishment. A successor cannot choose its own rule. Evaluation/correction authority is a dependency only when the selected operation actually consumes it. |
| Existing management grant | For replacement, exact committed grant identity/content, named principal/decider/executor, permitted management operation and reservations. Initial activation derives from the supported fixed human establishment rule and exact human approval, not from the grant being installed. |
| Subject and content | Exact project/scope/stream; complete proposed binding content fingerprint; selected definition/profile/stack fingerprints; exact constraint references/fingerprints; root-use policy and ceiling/path bounds. Include the proposed management terms' fingerprint in initial approval. An approval of B1 cannot activate B2. |
| Expected predecessor | Positive never-selected state under complete bounded owner/legacy/transaction inventory for first establishment; exact current binding record and head/generation for replacement or revocation. Do not infer never-selected from missing publication bytes. |
| Actual decision | Attributable human approval or permitted deciding-agent submission with exact grant, candidate, expected head, operation, decision source and participant. Required review must be real; authored or role-based claims cannot manufacture it. |
| Finalization and recovery | Current prior lineage, grant, definitions, exact candidate and predecessor held/rechecked through native compare/commit; exact request key/content and committed outcome/receipt. Replay recognizes original intent and never activates an earlier head again. |

These requirements adapt the concrete terms/predecessor/participant binding in
[local check](../../crates/engine/src/local_authority/local_check.rs), evaluation
`prepare`, `with_terms_admission`, `issuance_locator` and guarded verification in
[local evaluation](../../crates/engine/src/local_authority/local_evaluation.rs), and
P4's exact delegation/candidate/head-bound decisions. They are design obligations
for a new supported operation, not evidence that those old operations authorize it.

**Use of the resulting binding.** Recommend allowing the already named executor
to request exact root candidates inside its use scope without another human
decision. Consumption requires the selected binding and current management/use
basis, exact definitions, subject/constraints, ceiling and non-expansion. A portable
binding projection/locator identifies history; only an owner-created sealed live
witness authorizes use. Refuse attempts to mint a witness from serialized records.
Adapt the witness to the actual selected successor lineage and binding head rather
than tricking initial `read_current` into ignoring local-check state.

Binding replacement changes subsequent admission, invalidates cached/cloned
current witnesses for the old binding, and preserves old envelopes as historical
truth. Grant revocation blocks dependent management/use; binding revocation blocks
this stream. Restoring permission after either requires a supported separately
authorized successor, which is outside the first recommendation. Narrow binding
replacement must preserve the management grant and prior ceilings/constraints;
changes to grants, Charter, profile/policy vocabulary or wider reach remain refused.

## Larger workflow and native fence

Existing setup establishes deliberate D, initial local Charter and operating
terms; existing local-check adoption then supplies its own exact successor and
current D/G applicability. Evaluation/correction can follow independently under
their existing rules. The proposed Context Resolution owner consumes this local
authority lineage and leaves those workflows' history and heads intact.

The human supplies the exact initial Context Resolution management/use choices
and first binding approval. A configured agent may carry that instruction into
the SDK, but native Engine validation decides eligibility/admission and confirms
the effect. Once selected terms authorize an in-bounds root use or bounded binding
replacement, the agent makes that permitted decision and the owner rechecks it;
additional context or a project-agent title supplies no missing permission.

| Scenario | Proposed end-to-end result |
| --- | --- |
| Normal in-bounds use | Current local-check project P selects native local binding B1 for exact profile/stack/constraints and `inputs/**`, with a specified validation ceiling. Executor requests envelope E1 for `inputs/payload.json` within those bounds. Engine resolves P/B1/current prior authority, admits RootCreation and applies the existing validation or mutation decision. Any actual write is finalized by its own live owner admission. No new human prompt for routine authorized use. |
| Permitted replacement | Decider authorized by retained grant G1 submits exact B2, expected B1 and decision source. Owner validates same stream and non-expansion under G1/current local terms, then commits B1→B2 once. B1 history remains exact; its cached admission is no longer current. If replacement was reserved to the human, the agent's submission refuses instead. |
| Insufficient prior authority | Agent has `OwnerWrite`, an InputCorrection grant or additional context but no Context Resolution management decision permission. Binding activation/replacement refuses before effect, identifying the missing operation/grant/reservation. The human may resolve the choice only through the newly selected supported rule; a candidate cannot add its own permission. |
| Revoked or stale inputs | Initial/check/grant/binding revocation, changed Charter, lock replacement, changed governance relation/tree, changed definition identity or stale expected head prevents dependent current admission/commit. Re-resolve and reprepare only when existing authority permits it. Earlier exact outcomes remain history; a fresh candidate cannot repair revoked authority itself. |
| Lost response or interrupted activation | Caller retains the original locator before effects. Read exact request/outcome first; resume only retained original intent under the supported recovery rule. Committed replay returns the same bytes/identity without replacing today's head. Unknown, multiple or damaged state remains uncertain/refused; timeout never authorizes a new activation. |
| Credential-to-advisory fallback attempt | A request selects the existing credential-backed binding; publication/admission cannot obtain valid credential evidence. Refuse on that route. Do not consult a local binding because authentication failed, invent registry state or relabel old publication as advisory. |
| Historical read after later changes | Return retained original inputs, definitions, approvals/claims, predecessor relations and committed output at its recorded semantic time. Missing retained dependencies yield owner-attributed unavailable/unverifiable history. No current witness is returned and no result is rewritten under current definitions. |

The native fence is deterministic eligibility, exact authority/content/scope,
owner conformance, currentness, guarded activation/use, effect verification and
history/recovery. Interviewing, routing to another agent, discovering host profiles,
LLM calls and delivery-channel integration stay outside. A Context Resolution
envelope does not grant evaluation Purpose authority, work completion, Charter
mutation, memory promotion or downstream action authority.

## Compatibility, identity and representation

Three populations need different treatment:

| Population | Proposed compatibility boundary |
| --- | --- |
| Fresh/never-selected native local use | Current initial local envelope admission continues to work as implemented. New persistent native selection requires positive exact eligibility and the supported initial human rule. An absent file alone is not a new project or a never-selected stream. |
| Already-established local state | Prefer an additive selection on a positively current local-check project, including supported D/G applicability. Retain original initial/check/evaluation/correction records and fingerprints. Do not call fresh setup again, rewrite V1 terms or reanimate initial witnesses. Missing permission or new-owner read/write reach refuses until a supported authorized successor is selected; an explicit instruction alone is not an existing transition API. |
| Credential-required legacy binding | Preserve current signed publication, mapped admission, currentness and quarantine requirements and all retained bytes. No conversion, binding takeover or dual authority selection in the first extension. A conflicting legacy footprint at the same proposed stream/scope refuses; a future coexistence or adoption rule must positively disambiguate authority, not choose by lookup failure. |

Fresh-project-only native support is simpler but would miss the actual successor
currentness gap and require another establishment path later. Prefer already-local
state rather than requiring a new repository identity. This recommendation does
not mean arbitrary initialized roots are eligible: existing local owner history
and compatible current authority must positively resolve.

For legacy compatibility, retaining its existing route while adding a separately
selected local stream is the bounded recommendation. It resembles one benefit of
earlier option A, but **does not select A's installed-binding adoption contract**.
Replacing the legacy lifecycle in place would require inventory, transition
authority, exclusive writer reservation and recovery of pending signed effects;
none is supplied here. Even a deliberate explicit transition cannot be promised
before its authorizing rule, exact predecessor and cutover behavior exist.

Recommend versioned owner-managed **JSON** for the new local selection,
management terms, immutable binding/history and request/receipt state, matching
the actual local owner records. A typed request and a derived human explanation
are sufficient initially; no new YAML authoring/importer is necessary. Direct
human-authored YAML candidates could be supported later with an explicitly bounded
normalization/admission route, without two editable masters. Keeping the legacy
capsule/YAML output for a new local binding would require inappropriate credential/
registry fields or a new incompatible capsule contract. Prefer a separate native
local codec and owner namespace, with exact path/framing chosen during authorized
preparation, rather than synthesizing the legacy 53-leaf capsule.

This is a boundary-specific recommendation, not adoption of the format standard.
The existing `.handbook/project/context-resolution-authority.yaml`, generic tuple,
canonical YAML byte hash, declaration/payload JCS tags and semantic binding
preimage remain unchanged. A new local record cannot claim any of those identities.
Keep category-2 owner/name/version/content identities separate from category-3
binding/envelope/decision identities and their exact input closures. Unchanged
definition content may be cited by successive records; altered definitions need
new owner-established identities. Formatting, transport IDs and current-head
pointers do not replace semantic identity. Retain original definitions and
formation dependencies for truthful history; the old historical publisher still
consults live repository/profile/stack bindings, so its displaced-byte tests do
not prove arbitrary historical verification after those inputs change.
[Legacy kernel/publication](../../crates/engine/src/context_resolution.rs),
[generic mutation](../../crates/engine/src/artifact_mutation.rs),
[lineage store](../../crates/engine/src/artifact_lineage_store.rs).

## Actual prerequisites and regression obligations

P3 `2b09faf` delivers complete native evaluation, purpose-bound Engine issuance,
checkpoint verification and retained result/history. P4 `be7d040` delivers fresh
repeat/correction and upfront immutable correction-decision delegation. Their
[packet evidence](../specs/handbook-contract-membrane/slices/independent-local-check/packet-3.md)
and [P4 evidence](../specs/handbook-contract-membrane/slices/independent-local-check/packet-4.md)
record selected focused/native results and limits. The
[aggregate closeout](../specs/handbook-contract-membrane/slices/independent-local-check/README.md#aggregate-slice-closeout--2026-10-08)
and reconciliation at `fd519fb` record completion and human recap. These are
existing delivery evidence, not tests rerun by this research and not authority for
the proposed extension. No blanket wait for P3/P4 or the whole roadmap remains.

| Coupling | Obligation for a later selected extension |
| --- | --- |
| Initial versus successor currentness | Preserve `successor_terms_selected` refusal on the original path. New current-binding witness must validate the actual local-check lineage, original/check revocations and original Charter; include evaluation/grant heads only when they actually govern the requested operation. No “latest terms” search or blanket dependence on unrelated evaluation history. |
| D/G applicability and currentness | Resolve fresh governance basis outside consuming locks, then check exact relation and complete governing tree under native admission. Initial setup's current-D route must not be borrowed for G; SDK check's supported D/G observation supplies precedent, not Context Resolution permission. |
| Shared path/subtree helpers | `local_path_within_selector` and `local_owner_subtree_within_selector` in [context_resolution.rs](../../crates/engine/src/context_resolution.rs) are used by local-check full-definition reads/writes and evaluation envelope/reach checks. Preserve literal-path versus complete-subtree semantics, deny precedence, malformed-selector refusal and actual proof-read bounds. Avoid changing matcher language to simplify the new route. |
| Physical root, retained inputs and locks | Existing order starts with initialization, then promotion/registry/lifecycle/local-operating guards. Resolve governance before these; acquire any new binding lock afterward in one selected order. Do not call public local-check/currentness APIs recursively inside their held guard. P3 added retained Windows child-handle validation in [durability](../../crates/engine/src/repository_durability.rs); no missing/recreated lock can certify old authority. Pin exact authoritative files and preserve their generations through commit. |
| History, recovery and exclusive writer | Native local binding needs its own bounded immutable request/intent/output/head evidence and exact compare/commit/recovery. Preserve historical use after revocation without present permission. Refuse conflict/ambiguous pending footprints; never invoke generic promotion to authorize local binding installation. Reserve only actual native owner targets; this first outcome does not disable legacy writers globally. |
| Schema/profile and SDK | P3's [schema_registry.rs](../../crates/engine/src/schema_registry.rs) delta from `70019b7` is test-only validator-build instrumentation, not a changed authority contract. P3 fixed-profile tests and local Charter/authority helpers are relevant regression seams. Keep exact shipped profile/stack restrictions until compatible replacement is selected. New SDK composition needs normal-dependency proof, usable responses, original-key recovery and explicit unsupported-use errors; no automatic generic operation inventory or CLI/Tauri parity expansion. |
| New shared-owner preparation | [Work-item state foundation](../specs/handbook-contract-membrane/slices/work-item-state-foundation/packet-1.md) is prepared at `6aba084`, not implemented. It overlaps initial `OwnerWrite`, retained root/current guards and live-through-commit requirements. It supplies no Context Resolution management permission and is not a hard predecessor. Reconcile any subsequently landed shared-helper changes at preparation/execution; do not plan that work here. |

Meaningful inspected bodies include initial SDK reopen/Allowed/Denied and
revocation replay tests in
[local_initial_setup.rs](../../crates/sdk/tests/local_initial_setup.rs);
actual paused validation/revocation there; original-key interrupted successor
adoption and lock substitution in
[local_check_authority.rs](../../crates/sdk/tests/local_check_authority.rs);
evaluation [checkpoint races](../../crates/engine/tests/local_evaluation_checkpoint_races.rs)
and [correction acceptance](../../crates/sdk/tests/local_evaluation_native/correction_acceptance.rs);
and signed kernel publication/replacement/quarantine/cached-admission tests.
They establish the intended precedent and refusal bodies, not runtime proof of
the proposed route. Later execution must add focused causal proof for the new
binding lifecycle and integrated currentness/through-commit behavior.

## Smallest useful next outcome and decisions

Recommend human selection of a bounded **native local root-binding lifecycle and
SDK consumption outcome**, starting on current compatible local-check state:
explicit human management/use establishment and first binding, exact in-bounds
root admission, prior-grant-authorized narrow binding replacement if selected,
revocation, original-key recovery and exact history. Its first visible result is
usable/reopenable authority with truthful stale/refused outcomes. That result
deserves a human review checkpoint before any child, escalation, memory or legacy
adoption expansion. This is an outcome recommendation, not packet decomposition,
an implementation plan, or a whole permission-administration backlog.

Acceptance examples for that later bounded plan:

- A current local-check project reopens exact B1 and admits an in-bounds root
  request; unchanged initial-only SDK callers continue using their existing route.
  Validation and mutation decisions retain their independent bounded meaning.
- An executor without binding-management permission cannot activate B2 despite
  broad owner-write paths. A permitted decider's exact B2/B1/G1 submission can
  replace once; substituted candidate, wrong participant and stale predecessor
  refuse. Initial activation never relies on its proposed grant's agent authority.
- Revoke races serialize with actual consuming/committing callbacks. Changed
  Charter, governance basis, definition, physical root or lock generation prevents
  new use; exact historical B1 and original approvals remain readable as history.
- Lost response resolves the original committed receipt or retained uncertain
  intent. Retry cannot create a new head, revive an old binding or manufacture
  absent intent. Damaged/multiple footprint cannot trigger fresh selection.
- Parent/escalation/memory requests and local claims submitted to credential-required
  legacy admission refuse. Legacy signed replay/quarantine and local evaluation/
  correction history retain their existing contracts.

Already determined by current evidence and approved semantics: Engine ownership,
separate definitions/authority records, honest advisory provenance, exact prior
authority and scope, no self-activation, initial versus successor currentness,
history versus present permission, existing initial RootCreation coverage, and no
automatic credential fallback. P3/P4 are complete; they are reuse evidence rather
than a broad new authority grant.

Safe engineering selections for a later authorized preparation include inward
Engine reuse, adapting live callback/retained-file patterns, bounded immutable
history and original-request recovery, using the existing matcher/definition
identities, and typed SDK composition with faithful owner outcomes. Exact public
signatures, storage path/framing and narrow lock/partition changes still need
source-grounded preparation; this research does not freeze their implementation.

Consequential human choices still needed:

1. Adopt the operation-specific local-advisory rule and its proposed eligibility
   on current local-check state, including exact initial human management authority.
   Selecting this investigation did not select that product behavior or any actual
   project's bounds/principal/delegation.
2. Select the first use/management boundary: recommended root-only use and human
   grant establishment, with agent narrow binding replacement only where expressly
   delegated; otherwise reserve replacement to the human. Broader grant administration,
   reach expansion and post-revocation restoration remain separate.
3. Select the native local codec/owner-managed JSON direction and additive
   compatibility boundary. Recommend excluding legacy adoption and ambiguous
   same-stream coexistence from this outcome. Existing historical bytes and
   credential rules remain controlling regardless of that selection.

After selection, one separately authorized preparation may concretize that
outcome and its acceptance/permission boundary. This completed investigation
returns for human discussion and authorizes no implementation or follow-on dispatch.

## Research verification limits

Verification is source/test-body inspection, exact commit-diff comparison, local
reference/anchor checking, whitespace and document-only scope checking. No runtime
proof, build, product suite, external service or installed-consumer census was
rerun/obtained. P3/P4 and signed-kernel evidence retain their recorded limits.

GitNexus query/context were attempted via the existing CLI runner because no
GitNexus MCP tools were exposed. Query failed on missing `Protocol`/`Category`
tables and returned no flows, with a foreign-storage registry warning. Context
returned a resolver entry with empty access references and broad unrelated process
listings; it is not reliable caller/impact evidence. Local metadata pins the index
to `a35297d`, 54 commits behind this source. The status command also failed while
hashing a changed analyzer identity file. No index repair/rebuild was performed.
Direct source/consumer inspection supplements these limits. No product symbols
were edited; symbol impact-before-edit is consequently inapplicable.
GitNexus `detect-changes --scope staged` recognized the single document diff and
reported no indexed-symbol overlap, with the same foreign-storage warning. That
result verifies neither graph completeness nor the proposed runtime behavior.
