# Native local JSON conformance checks

The typed local evaluation route asks whether one exact captured JSON payload
conforms to one exact supported offline Draft 2020-12 schema closure through
`handbook.dock.json-schema@1.0.0`. Contracts owns requirements, occurrence,
evidence and explained truth; Engine owns operating authorization; Dock owns
capability and execution facts. The SDK composes their results.

The typed entry points are in [`local_evaluation`](../src/local_evaluation.rs).
The ordinary [`local_evaluation` consumer](../examples/local_evaluation.rs) accepts
supplied files, prints structured owner results and can run as a separate Cargo
program with only normal dependencies. Use the same absolute repository root
that the original native initialization retained.

Supply attributable approval of the actual project setup, Charter/posture,
scoped deferral or governance, and operating terms. The terms must explicitly
permit the selected intent and every required owner subtree and proof read.
An approval of Handbook implementation supplies none of these runtime choices.
Local advisory attribution does not claim authenticated identity or a sandbox.

Supply a complete exact full-check definition and its dependencies, justified
from the question before observing outcomes. The definition fixes one claim,
one case, one observation role, the native producer, all evaluation policies,
an explicit empty metric inventory and compatible retention requirements. Supply
the exact supported horizon with the operating request. Its exact
revision requires actual review by the distinct authorized reviewer and owner
lifecycle establishment. Changing a policy or permitted use requires a new
reviewed revision; the SDK never upgrades a Packet 2 source-seam definition.

The operation order is:

1. Resolve current project applicability through v3 and prepare the exact
   nomination, captured input and requested reach.
2. Form the retained owner Purpose and occurrence without launching a producer.
   Save the original recovery locator before relying on a delivered response.
3. Obtain the independent Engine issuance and owner readiness result. A ready
   attemptable basis may commit; a held or refused result does not launch.
4. Execute at most one native attempt. Owners retain its intent and terminal,
   admit every candidate or refusal, seal collection and close the evidence set.
5. Engine verifies authorization after that exact set exists. Dock re-resolves
   the original native source chain under the declared read reach. Its source
   files and the fixed set stay pinned while Contracts compares the admitted
   origin, closes the final context and persists the immutable result and
   dependency manifest. Missing or substituted source bytes refuse closure.
6. Recover an uncertain response using its original locator, or read the exact
   retained historical result. Neither operation requests a fresh evaluation.

Dock read selectors use exact paths or complete child subtrees (`/**`). Final
closure retains compatible selectors from the original Engine request without
widening its approved read reach. Actual missing read membership refuses.

Use `prepare_local_evaluation_definition`, `append_local_evaluation_definition`
and the lifecycle methods to establish the exact reviewed full definition. Fresh
owner initialization needs the supplied local-check author authority and explicit
original owner-lock creation reach. `prepare_local_evaluation_terms` and
`adopt_local_evaluation_terms` narrow the live predecessor; its supplied approval
must already include the new Engine evaluation subtree, Contracts and Dock
mutation subtrees, native owner locks and all actual proof reads. Include the
actual installed native package read selector returned by
`dock_native_package_read_selector_v1()`; external package availability cannot
borrow repository proof-read reach.

For an active definition, pass `LocalEvaluationPrepareInputV3` to
`prepare_local_evaluation`. Save `prepared.request_locator()` before calling
`form_local_evaluation`. `recover_local_evaluation_request` resolves that original
request even when its allocated occurrence locator was never delivered. It
returns `None` only for owner-proven noncommit; a conflicting or uncertain request
returns its structured owner error.

Create the Engine request from the formed snapshot's `authorization_request`,
using the complete supplied private envelope and declared reach. Save
`local_evaluation_authorization_locator` before issuance. Keep that original
private request with the caller; Contracts retains only the public issuance and
sealed observations. A `LocalEvaluationSessionV1` carries these exact locators
and the original request through the following operations:

```rust,ignore
let capability = sdk.observe_local_evaluation_capability(
    &initial, &authority, &formation, &dock_reach)?;
let readiness = sdk.assess_local_evaluation(&session, &capability, &dock_reach)?;
// Inspect the owner stage and reasons before proceeding.
let committed = sdk.commit_local_evaluation_collection(&session)?;
let attempt = sdk.execute_local_evaluation_once(
    &session, &exact_process_request, &dock_reach, &cancellation)?;
// An uncertain attempt is recovered with its original locator, never relaunched.
let admitted = sdk.admit_local_evaluation_origin(&session, &dock_reach, "packet3-delivery-01")?;
let frontier = sdk.seal_local_evaluation_frontier(&session)?;
let set = sdk.close_local_evaluation_set(&session)?;
let closed = sdk.close_local_evaluation_result(&session)?;
```

The example's `prepare-input` operation creates untrusted typed preparation
input from the supplied question and captured JSON. `form` saves both the original
request and allocated occurrence locators. `authorization-request`, `issue`,
`capability`, `assess`, `commit`, `process-request`, `execute`, `admit`, `frontier`,
`set` and `close` exercise the owner sequence. `recover-request`,
`recover-authorization`, `resume-authorization`, `recover-attempt`, `recover`,
`recover-terms`, `terms-history`, `authorization-history`, `history` and
`result-history` expose the corresponding
recovery and history distinctions. `close` can save an exact immutable result
locator for retrieval in another process. `revoke-evaluation` consumes the
supplied Engine authority, actual revoking principal and declared reach.

Interpret the structured owner result before rendering a summary:

| Owner outcome | Meaning | Supported response |
| --- | --- | --- |
| Decision-seeking held readiness | Valid owner facts establish a limitation or leave a required capability/availability comparison unknown. No attemptable basis or launch is supplied. | Preserve earlier readiness observations. Before collection commitment, reassess only unchanged Purpose, Plan, input and issuance under live authority. Changed bindings require a new occurrence. |
| Formation or authorization refusal | A required input, applicability, exact identity, intent, scope, permission or currentness condition failed. | Preserve its stage, owner, code and available provenance. Repair only within prior terms; wider or reserved choices need their actual authority. |
| Completed conformant observation | The real validator completed and reported conformance. | Report `pass` only after the complete Contracts truth chain closes. |
| Completed nonconformant observation | The real validator completed and reported actual violations. | Preserve violation paths and reasons. A closed `fail` is a valid unfavorable answer. |
| Terminal empty evidence population | The exact allowed no-attempt or failed-attempt population is fully accounted, with no usable observation. | A valid final context may yield `indeterminate`; absence of evidence supplies no pass. |
| Uncertain effect or persistence | The responsible owner cannot establish its exact terminal or committed outcome. | Resolve the original retained locator. Do not guess noncommit, choose another key or relaunch the producer. |
| Unavailable exact history | A required retained dependency or owner verification cannot be resolved. | Preserve the stored result and owner-attributed resolution reasons; state that exact replay is unavailable. Do not substitute a current definition or recompute at now. |

For permitted diagnostic intent, Contracts can retain an attemptable limited or
unknown basis with its reasons. A pinned native unavailable producer then retains
a no-effect refusal terminal under that original basis. It does not acquire a
replacement capability or launch a newly available package. Fully closed empty
accounting can yield `indeterminate` under the same unchanged criteria.

Currentness compares the candidate's retained `observed_at` with the final owner's
`evaluated_at` under the original reviewed freshness policy. A completed conformant
or nonconformant observation can close `indeterminate` when it is stale. Preserve
that finding, its violations and both times; historical retrieval does not
recompute currentness or turn the retained result into a new evaluation.

A locator identifies recovery; it grants no permission. Losing an execution
response is a delivery problem, not a reason to create another attempt. Exact
recovery returns the original owner outcome and time or its honest unresolved
state. Deliberately requesting a new evaluation allocates a new owner occurrence,
even when its content equals an earlier one. Select the intended operation explicitly:

| Caller choice | Required binding and effect |
| --- | --- |
| Read history | Retrieve the retained result and its original dependencies and times. No new currentness assessment, allocation or launch. |
| Recover a response | Supply the original exact formation, issuance, decision or attempt locator. Owner-proven absence, committed delivery and uncertainty are distinct. |
| Deliberately repeat | Supply a new formation key and the same exact capture and reviewed criteria. Owners resolve fresh applicability and allocate a new occurrence, Purpose and independent issuance. |
| Correct captured input | Supply the exact closed prior result and a consistent new subject/case/input binding with a new formation key. Contracts retains additive lineage with formation and preserves the old result. Criteria and dependencies remain exact. |
| Continue before commitment | Reassess the original fixed bindings only if the native owner accepts live facts. After commitment the capability basis and one-attempt sequence remain fixed. |
| Carry an unresolved correction decision | Retain the actual rejected candidate through the owners, then submit the designated decider's response against the exact request head. Only an exact authorized approval can be consumed. |

Routine permitted input corrections proceed directly through owner checks. An
unresolved choice can use the prior-approved `InputCorrection` delegation without
another human approval. Correcting input does not permit changing the schema,
claim, permitted use, policy, freshness horizon, source or declared reach. Such a
change needs its separately justified and actually reviewed exact revision.

The bounded initial delegation names an explicit decision agent and the existing
executor. The decision agent may differ from the executor. The prior terms' human
principal approves that exact immutable binding with an attributable approval
source. The user can approve and provision it before an unresolved request;
native adoption follows establishment of the exact evaluation predecessors.
The grant adds no path, operation class, criterion, executor or authority horizon.
Only one immutable binding is supported per evaluation authority; correction
decision authority supplies no permission to appoint or replace a decider.

The native request reproduces an actual Contracts preformation inconsistency:
the candidate still names the old capture while the proposed correction names
consistent new content. An arbitrary unresolved DTO or claimed refusal supplies
no owner identity. A retained request binds the old exact result, rejected candidate,
missing condition, fixed prospective formation, delegated class and current D/G
basis. The submitter can carry the decision while remaining distinct from both
the deciding actor and the consuming executor.

Approval retains acceptance of only that exact correction and fixed downstream
formation key; fresh Engine admission and Contracts conformance checks remain
required. Denial closes the request without allocation, issuance or launch.
Clarification retains the exact missing information and leaves the request
unresolved. A later submission must bind the current head. Silence and timeout
permit no effect. Exact submission replay returns its original durable record
and time as history; conflicting keys or stale concurrent heads refuse.

An accepted decision selects at most one correction occurrence. If a response is
lost, recover the original decision or Contracts formation request. A new key
cannot conceal an uncertain attempt or create a second consumed correction. D/G
changes and revocation block stale fresh use while leaving old records readable;
a pending D decision cannot be relabeled G. Compatible immutable delegation can
serve a newly retained G request under its own fresh applicable basis.

Local advisory attribution records supplied actor, approval, decision source and
scope claims. It proves neither authenticated identity nor verified delegation,
nor who physically submitted a response. Possessing another agent's locators or
terms supplies no grant. Reserved choices, missing authority without supported
resolution and unresolved material ambiguity preserve an exact escalation need.

Use `LocalEvaluationFollowupInputV1` with an explicit `FreshRepeat` or
`CorrectedInput` action, original `LocalEvaluationResultLocatorV3`, complete exact
`original_capture` and `capture`, and the consistent new
`LocalEvaluationPrepareInputV3`. Captures are the serialized
`DockInlineJsonInputV1`, including its content fingerprint and offline schema
closure. `prepare_local_evaluation_followup` validates these bindings without
allocation; save its `request_locator()` and then call
`form_local_evaluation_followup`. `read_local_evaluation_lineage` reads the
additive relationship. Original V3 snapshots and results retain their existing
canonical preimages.

For an unresolved choice, use the following explicit SDK operations:

```rust,ignore
let prepared = sdk.prepare_local_input_correction_delegation(
    &initial, &authority, supplied_exact_human_approved_binding)?;
let grant = prepared.locator().clone(); // Save before adoption.
let adopted = sdk.adopt_local_input_correction_delegation(&authority, prepared);
// Inspect disposition/outcome/refusal before using the retained binding.
let candidate = sdk.retain_local_input_correction_candidate(
    &initial, &authority, &prepared_followup, actual_rejected_candidate)?;
let request_locator = supplied_request.locator(&adopted.outcome.unwrap().delegation_fingerprint)?;
let request = sdk.retain_local_input_correction_request(
    &initial, &authority, &grant, &supplied_request)?;
let decision_locator = supplied_submission.locator(&request_locator)?;
let decision = sdk.submit_local_input_correction_decision(
    &initial, &authority, &grant, &request_locator, &supplied_submission)?;
// Only an accepted Approve may reach this consuming operation.
let formed = sdk.form_approved_local_input_correction(
    &initial, &authority, &grant, &request_locator, &decision_locator, prepared_followup)?;
```

Save request and submission locators before the owner mutation. Recover their
original records with `recover_local_input_correction_delegation`,
`recover_local_input_correction_request` and
`recover_local_input_correction_decision`. The immutable request record has its
initial head; after clarification, the next submission uses the retained
clarification's `decision_fingerprint` as its expected head. The accepted
formation still needs a separate issuance and the complete existing owner
execution and final-closure sequence.

When delegation adoption retains an uncertain intent, call
`resume_local_input_correction_delegation` with its original saved locator. This
completes only that existing exact intent; absent intent requires explicit new
preparation under current eligibility. After revocation, recover an already
formed correction with `recover_local_evaluation_request` or
`recover_local_evaluation` and inspect its lineage. Historical recovery remains
available and grants no fresh use.

The ordinary example exposes `repeat`, `correct`, `lineage`,
`adopt-correction-delegation`, `correction-candidate`, `request-correction`,
`submit-correction`, `consume-correction` and their recovery commands. All
authority, captures, requests and decisions arrive as supplied files. New
followup files use their bounded canonical JSON codec; new Engine records use
bounded duplicate-safe closed decoding. These argument names demonstrate a Rust
SDK consumer and add no installed Handbook CLI route, routing or LLM call.

The example's history responses place the additive lineage and decision
provenance beside the exact unchanged V3 result. If lineage resolution is
unavailable, its separate owner refusal does not replace the stored result.

Retrying `close` after the result was committed delivers that same retained
snapshot only for the exact original issuance and request. It performs no new
final verification and grants no current use; its observation times stay original.

Historical retrieval resolves the original definition, applicability, issuance,
semantic times, observations and accounting after reopening the repository.
Later governance establishment or revocation does not rewrite that result and
does not turn historical authorization into current permission. A historical
read launches no producer and selects no current policy.

The canonical explanation is the source for the human summary. It preserves
`direct_contract`, the original intent and permitted use, original D/G
applicability and any qualification, independent finding/coverage/currentness/
consistency/uncertainty dimensions, violations and included/excluded/refused/
duplicate accounting. It states **shipment or completion obligation: not
assessed**. Pass supplies no work closure, shipment, present reliance or action
authority. Captured-input conformance makes no claim about later workspace edits.

Use caller-supplied supported retention rather than a numerical default. Every
required owner dependency must have a compatible retained obligation at closure.
Missing or substituted locks and roots refuse; inspection and history do not
initialize locks or repair uncertain intents.

The native executor reports its actual environment, working-directory, time,
output and cancellation controls. A `network=Denied` request is not an OS network
sandbox. This bounded full-definition schema admits no host-enforcement
prerequisite: formation fixes empty prerequisites, and native capability reports
network enforcement as `not_provided`. An altered prerequisite fingerprint cannot
authorize launch. Full-definition construction refuses unsupported dialects;
the offline schema-closure constructor refuses remote references. Neither
unsupported definition can form an evaluation occurrence or launch an attempt.
