# Packet 1: register and resolve native open work

Status: **conditional next plan; unimplemented; authority/recovery choices and
composition-successor compatibility gate pending**, reconciled 2026-10-08 at
`2740e04`. This supersedes the initial-only recommendation at `6aba084`.
This is the sole packet of [work-item-state-foundation](README.md). The slice
owns the outcome, exclusions and eight acceptance conditions; this document
details only its next implementation increment.

## Observable behavior and review boundary

Under the proposed rule, given one positively current local-check project in D
or established-from-D G, newly activated exact human-approved work-registration
terms with sufficient effect/read/write reach, and a valid exact item
request, a normal SDK caller creates one owner-confirmed `open` item. It retains
the original request locator before execution. Current resolution after restart
returns that exact item/snapshot; original-request resolution returns the same
registration outcome. A registration receipt proves creation, not work completion.

The activation API and consuming work permission are **missing product mechanisms**
proposed here, not existing local-check support. [The three next-plan gates](../../coverage-and-landing-map.md#next-plan-selection-gates)
require human selection of activation/population and commit/recovery, and completed
composition/kinds successor preparation before execution. Plan approval then
authorizes this bounded implementation; it supplies no actual project's terms.
The original initial witness still refuses successor footprints and changed
governance. Work permission must be newly established by the selected fixed rule,
not inferred from local-check classes or a Context Resolution envelope.

D→G invalidates an earlier bound/prepared D basis. Proposed explicit human
activation of compatible work terms on current G permits new registration there;
old requests keep their original authority and cannot substitute the new grant.
Existing item/history reads remain separate owner-state reads. A D-only route is
a viable narrower alternative only with a human-accepted G creation limitation.

For example, a current local-check project P activates exact work terms, then the item key
`inspect-session-renewal`, a descriptive label and a
bounded work question select the package-supported explicit leaf composition.
No contract, review result, child, prerequisite or completion standard is invented.
After the item is registered, completion support inspection returns
`unsupported / not assessed` independently of `open`. Requesting local-closeout
eligibility, an authored `completed` status or using a passed direct check as
closure refuses before any Operational Reliance Determination or work-state effect.

The human checkpoint is **Packet 1**. Registration/current/history plus its
authority and completion refusals are a meaningful native/SDK capability. Return
to meta after verification, focused causal review and the local coherent commit;
do not elaborate or implement positive eligibility, close effects or hierarchy.

## Required sources and present product truth

Read the [slice](README.md), [semantic amendment](../../semantic-model/amendments/work-progression-kinds-and-relations.md),
[Work Specification relationship](../../semantic-model/operational-reliance-and-actions.md#work-specification-relationship),
[Work Progression ownership](../../semantic-model/operational-reliance-and-actions.md#work-progression-ownership),
[authority provenance](../../semantic-model/operational-reliance-and-actions.md#authority-provenance-without-a-handbook-iam-platform),
[G1–G6](../../semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules),
[current delivery sequencing](../../phase-and-slice-map.md#current-delivery-sequencing)
and [structured-data standard](../../structured-data-format-standard.md).
General defaults remain PROPOSED; Contracts JSON and existing versioned identities
are fixed. This packet explicitly chooses additive owner-managed JSON only.

| Inspected source through `2740e04` | What may be reused; what it does not supply |
| --- | --- |
| [Engine initial operating authority](../../../../../crates/engine/src/local_authority.rs), `LocalOperatingTermsV1`, `OperatingGuard`, `LocalCurrentAuthorityWitnessV1::with_current` | Exact initial local-advisory terms, current native locks, root/Charter history and bounded OwnerWrite reach. The held witness is crate-private; source reuse inside Engine is possible. Generic `OwnerWrite` and paths do not grant local/parent completion or reinterpret `LocalCheckTermsV1`. |
| [Successor admission](../../../../../crates/engine/src/local_authority/local_check.rs), `with_admission`, `require_predecessor`, `check_current`, `require_basis`; [SDK basis observation](../../../../../crates/sdk/src/local_check.rs), `observe_local_check_basis` | Held callbacks and original/current lineage, revocation, Charter/lock generations and exact relation/tree validation are reusable. SDK supports fresh D/established-from-D G observation outside those guards. Neither supplies work permission; a narrow native work-rule activation and consuming path must be added. |
| [Evaluation/correction authority](../../../../../crates/engine/src/local_authority/local_evaluation.rs), [correction](../../../../../crates/engine/src/local_authority/local_evaluation/correction.rs) | Human-approved domain terms, immutable InputCorrection delegation, exact decisions and original recovery are precedents. They cannot establish/replace a work grant or appoint its decider. No dependency on evaluation/correction heads unless actually governing this operation. |
| [Engine Context Resolution](../../../../../crates/engine/src/context_resolution.rs), `ContextResolutionEnvelope::evaluate_mutation` | Its local branch checks current initial terms and path constraints, but the call returns after the current guard releases. Calling it once before a write is insufficient through-commit authority. Do not introduce generic-artifact authority loading or use a serialized envelope as permission. |
| [Engine durability](../../../../../crates/engine/src/repository_durability.rs) and [tests](../../../../../crates/engine/tests/repository_durability.rs) | Retained physical root, bounded regular reads/inventory, existing native locks, exact/no-replace durable installation and explicit uncertainty. These primitives do not choose a work transaction or prove its new recovery path. |
| [Engine initialization](../../../../../crates/engine/src/repository_initialization.rs), [identity](../../../../../crates/engine/src/repository_invocation_identity.rs) | Existing-project guard and exact retained initialization claim/identity. Registration must use existing truth; ordinary identity allocation/setup cannot repair missing registration or identify a new project during recovery. |
| [Engine definition identity](../../../../../crates/engine/src/definition_identity.rs), `parse_schema_json`, `DefinitionFingerprint` | Bounded complete JSON parsing with recursive duplicate-key rejection, exact value fingerprints and crate-private canonical encoding. Add typed closed-shape/domain validation; do not advertise the parser alone as work admission. |
| [SDK facade](../../../../../crates/sdk/src/lib.rs), [local setup](../../../../../crates/sdk/src/local_setup.rs), [reset](../../../../../crates/sdk/src/route_state.rs) | Typed normal-dependency owner forwarding, explicit supplied terms and response/recovery conventions. Runtime reset recursively targets `.handbook/state`; the new retained store must sit outside it and prove real reset behavior. |
| [Flow packet result](../../../../../crates/flow/src/packet_result.rs), [Pipeline RouteState](../../../../../crates/pipeline/src/route_state.rs), [Contracts closeout composition](../../../../../crates/contracts/src/gate_composition.rs) | Packet selection, routing booleans and legacy pure gate/local-closeout truth respectively. None is the operational work-state owner or the new eligibility/receipt boundary. Preserve them. |
| [Contracts local result/history](../../../../../crates/contracts/src/local_evaluation.rs), [native history reader](../../../../../crates/contracts/src/owner_repository/local_evaluation_repository.rs) | Complete direct check, exact immutable result locator and original history resolution. This packet neither consumes such results as eligibility nor changes their records/readers. |

The [format audit's tool inventory](../../../../research/json-yaml-format-audit-2026-10-02.md#reusable-tooling)
supports the codec/durability selections. Research sources are evidence of bounded
alternatives, not authority to implement their checkout workflow or all examples.

## Owner and native admission

Add a dedicated Engine `work_state` module. Keep owner-defined types, semantic
validation, registration transaction, exact reads and recovery together; separate
its public state/receipt types from the unsupported-completion response. SDK
projects/forwards these types and never constructs verified owner outcomes.
No new crate, registry service, Contracts dependency or broad Engine refactor is
needed for this first composition.

The package-owned immutable registration/support rule permits exactly creation
of a same-project bounded leaf in `open`. It binds the supported composition,
record/identity version, no-relations/no-completion boundary, accepted local
advisory authority mode, request equality and exact commit/recovery requirements.
An untrusted request cannot replace the rule, append executable hooks, choose
another state machine or introduce an unsupported kind. Establish this descriptor through
native fixed conformance, retaining its exact content/binding with each creation;
it assigns no governance and needs no separate kind registry.

**Proposed authority boundary, pending human selection:** Engine local authority
owns a fixed versioned work-registration establishment rule outside candidate
terms. It permits the original claimed human principal to approve exact bounded
terms against positive current initial/check lineage, current Charter and fresh
supported D/G applicability. Native establishment validates that rule and exact
approval/content, current project/root and compatible reach, then durably activates
the grant with original-request recovery. Candidate agent permission cannot
authorize this activation. Advisory attribution remains advisory; no independent
authentication or host enforcement is claimed.

Terms bind original principal/agent/instruction/source, exact registration effect/
composition scope, current lineage/governance, read/full-owner-subtree reach,
reservation, revocation principal and retention. Activation must positively
establish the new owner stream under exact initialized project truth; absent,
damaged, reset or pending state cannot masquerade as never established. Reach for
the new store must be explicitly approved under this new rule, never injected into
old identities. Revocation is supported for the declared principal. Proposed human
re-establishment on G requires exact prior work-grant identity, current G basis and
recovery of pending activation; no generic replacement/delegation or post-revocation
restoration is included. Revoked work grants refuse new use.

SDK composes prepare/activate/resolve/recover work terms and routine registration;
Engine owns native permission. Resolve governance outside consuming locks through
the supported SDK/Contracts basis path and validate exact relation/tree inward.
Keep Engine free of Contracts/SDK dependencies. Routine registration under the
activated grant needs no additional prompt solely for work-owner admission.
Neither old initial terms nor local-check/evaluation/correction/CR action classes
are a fallback. Activation's own exact predecessor/intent/decision/terminal and
revocation recovery must preserve current versus historical authority; it cannot
resume an old activation using the candidate grant as establishment permission.

Use the Engine-native successor-aware held current-authority mechanism while the work owner
validates and commits. A narrow internal consuming callback/admission may be added
for this registration when needed; it must bind the exact project/root, item/key,
composition, request digest, declared write targets and current authority basis.
It is lifetime-sealed, cannot be serialized/reconstructed by SDK, and permits no
other work operation. Acquire the existing initialization/operating guards and
declared authority dependencies, then the work owner's native lock, in one
consistent order. Hold the guards/lock and validate the newly activated work terms
and exact dependencies through terminal publication. Do not acquire operating locks from inside
a held work lock or claim that a preflight mutation decision protects the commit.

Retain the accepted instruction/source, claimed principal/agent/delegation and
advisory assurance mode from the actual authority basis, exact creation input,
native validation and observed current basis separately from the terminal effect.
This proves owner-checked authorization under the supported rule, not independently
authenticated identity, verified delegation or host enforcement. Work registration
has no historical assessment inputs and discharges no completion obligation;
neither a fake completion Determination nor a general permission owner is needed
for this creation effect.

## Identity, finite inventory and persistence

Use separate bounded caller-supplied item and request keys (ASCII tokens, at most
128 bytes). Derive item identity from the exact native project identity, owner/
identity-scheme domain and item key; display names and request delivery do not allocate
identity. A request binds its item, supported explicit composition, bounded label
and work question, expected initial absence and original authority locator/basis.
Use the established 4,096-byte bound for each descriptive/provenance string and
1 MiB for a complete record/request. No arbitrary numeric fields, supplied paths,
external references or open-ended user JSON are needed in this first shape.

Store retained owner truth under `.handbook/work-state/v1/`, including its native
lock, positive owner-establishment identity, exact immutable item-registration record and request-addressed transaction
history. The initial snapshot is version 1, `open`; it binds item/project,
composition descriptor and admitted request. Exact item/request content conflicts
refuse. A separate latest-head selector is unnecessary when the sole supported
item state is its immutable initial state. Each item's exact committed registration
is its current state source; this does not advertise edits or successor selection.

Item identity scheme is independent of codec/release/composition version. A later
composition or capability cannot reinterpret retained leaf meaning. Apply the
[successor compatibility gate](README.md#composition-and-kinds-successor-compatibility-gate)
before freezing descriptor/schema shape. Later relation/definition revisions need
selected owner operations; no editable state setter or automatic rebinding.

No unbounded list operation is exposed. Validate one declared owner inventory
within explicit Engine-compatible file/count/byte bounds; refuse overflow or
unknown/substituted/partial entries rather than truncate or infer absence. Codec
decoding rejects duplicate/unknown keys and trailing documents before typed
validation. Canonical JSON has no trailing newline; semantic fingerprints bind
domain-separated canonical values and byte fingerprints bind retained exact bytes.
No existing owner fingerprint or framing is rewritten.

## Durable prefix and authority-transition contract

This entire contract is **recommended, pending human selection**. Preserve the
terminal-plus-snapshot public boundary: native no-replace durable publication of
one exact registration terminal is the **semantic commit point**. It binds the
retained snapshot/registration receipt, request, item/project, original authority
and admitted effect. Exact snapshot and complete committed terminal are both
necessary for current `open` truth. Intent, decision or installed snapshot alone
creates only a reservation/pending candidate. A decision is not irrevocable commit.
Never expose candidate bytes as current state or report a pending item as absent.
A receipt proves registration, never `local_closeout`.

Retain exact accepted request/intent first; reserve item and request; prove expected
absence; retain the exact decided snapshot/receipt and original authority closure;
install the immutable candidate without replacement; revalidate original live
admission and publish its terminal while native authority/work guards remain held.
New-effect recovery uses those original checks. No gap between returned Allowed
and later publication is permitted. Lost/uncertain terminal write is resolved by
exact proof of installation under the original admitted path, not by publishing
another terminal using another grant. Missing authorization proof stays uncertain/
unverifiable. Physical file layout may not change these public semantics.

| Retained durable prefix | Original authority unchanged/current | Successor adoption/supersession | D→G changes bound basis | Original authority revoked | Runtime-state reset |
| --- | --- | --- | --- | --- | --- |
| No accepted request/intent, positively complete owner inventory | Report unaccepted/absent request, not replayed noncommit. New attempt needs current permission. | No old request to resume; use newly activated current terms. | New G activation required. | No new creation. | No creation from lost operating state. |
| Accepted intent, or decision/reservation, no committed terminal or installed candidate | Same original key may continue with live original permission, or prove noncommit. | No substituted successor permission; prove noncommit or uncertainty. | No publication under stale D; prove noncommit or uncertainty. | No publication; prove noncommit or uncertainty. | Retained historical recovery may prove noncommit; no publication with recreated locks. |
| Exact installed snapshot/receipt, no committed terminal | **Pending, not `open`.** Publishing terminal is a new effect requiring original current authority. Exact proof can instead close noncommit. | Pending snapshot; no historical-proof shortcut. Noncommit proof or uncertainty. | Same: no commit via new G grant. | Same: no post-revocation commit. | Same: no commit via replacement operating state. |
| Exact committed terminal plus matching snapshot/receipt and original proof closure | Current immutable `open` fact or exact committed replay. No second effect. | Historical proof/current item read; no permission restored. | Same committed fact; new creation needs current G terms. | Same committed fact; authority is historical only. | Same fact only through new retained read dependencies, not operating locks. |
| Missing/corrupt/conflicting/partial required evidence, or possible terminal publication with no exact proof | Integrity refusal or commit uncertainty; no manufactured absence or success. | Same; retain reservations. | Same. | Same. | Same; loss is not noncommit. |

The supersession column includes initial-to-check adoption for the alternative
initial route and any selected work-authority successor; the recommended route
already starts on current local-check state. Infer no new operation from that
column. Replaying exact old committed proof is **historical-proof finalization**,
not new registration. Completing an absent terminal is a new effect and cannot
use that historical rule. If terminal installation may have happened but cannot
be proven, missing files do not establish noncommit.

**Reservation and noncommit proof:** reserve only this request/item against
conflicting/new-key registration or future mutation. Unrelated items and exact
committed history remain usable; capacity overflow may refuse new admission
without truncating old recovery. Noncommit needs retained original intent,
positive initialized owner identity, correct root/work-lock generation, complete
bounded inventory of every possible publication target, proof that no committed
terminal was published, and exact pending candidates with no conflicting effect.
Require positive retained proof that terminal publication was never attempted or
that a native attempted installation positively did not apply. Persist the exact
publication-attempt checkpoint before invoking installation. If that checkpoint
exists with no provable result, absent terminal bytes alone cannot close noncommit;
retain uncertainty even if a later grant is current. Missing/damaged attempt
history cannot masquerade as a never-attempted prefix.
Under the owner lock durably close a matching original-key noncommit before
releasing its reservation. This narrowly selected historical recovery effect is
neither work creation nor authority restoration. Retain candidate/decision bytes
as history; later requests cannot overwrite them. Uncertain reservations cannot
be cleared by timeout, absence fallback, grant replacement or a new key.

**Read dependencies versus live authorization:** retain exact initialized project/
physical-root and positive owner-establishment identities, owner lock generation,
request/intent/decision/terminal/snapshot/receipt, descriptor bytes, original
authority/approval/provenance and original admitted dependencies needed for
historical verification. New read-only work resolution and historical recovery
must use retained dependencies outside `.handbook/state`, without creating
operating guards or consulting live Charter/terms as a condition of past truth.
Missing retained proof yields attributed unavailable/unverifiable history, never
fresh setup. New creation still needs current Charter/governance/unrevoked terms
and original operating guards. Reset may remove those live dependencies and block
new writes; it cannot allocate replacement identity, recreate an old lock as proof
or reinterpret retained work. Existing history APIs do not prove this new path.

Activation needs the analogous explicit terminal-publication boundary, exact
pending reservation, same-original establishment rule for unapplied activation,
and noncommit/uncertainty handling. A candidate work grant never finalizes its own
missing activation terminal. Historical committed grant replay never reactivates
it. This is unit 1's selected-rule acceptance, not a general authority-lifecycle
service or permission for CR binding management.

## Typed SDK responses and next steps

Expose typed work-term prepare/activate/resolve/recover/revoke and work
prepare/register, resolve-current-item, resolve-original-request,
recover-original-request and inspect-completion-support operations on
`HandbookSdkV1`. A sealed native prepared value is distinct from its portable
locator; callers retain the locator before invocation. Stable semantic keys are
distinct from correlation/delivery. Callers need no private path knowledge.

| Response | Caller meaning and permitted next step |
| --- | --- |
| Committed / replayed registration with exact receipt | Item was registered `open`; save its exact locator. Replayed delivery performs no creation. Resolve current item separately when present work state is needed. |
| Refused before effect | Named native validation, unsupported composition/relation or current-authority condition failed. Correct the input only under supported prior authority; changed content needs a new request key and still cannot overwrite an existing item. |
| Positively not committed | Native recovery establishes that this request did not apply. A fresh request requires currently valid authority and must retain the old result/history. |
| Uncertain | Retain the original locator and use exact request recovery. Do not assert absent/open/closed success or retry a possibly applied registration under a new key. |
| Current `open` snapshot | Exact owner-established item state at this read. It implies neither operating permission nor eligibility. Missing/corrupt dependencies refuse instead of returning an empty item. |
| Pending registration / reserved item | No public `open` fact or permission; recover the original request. A candidate without terminal cannot appear as absent or committed. |
| Historical registration outcome | Original creation fact and authority provenance only. It restores no revoked/current operating terms. |
| Unsupported completion / not assessed | No governing completion decision is formed and no eligibility/disposition exists. A close/parent action is unavailable. This is a capability/preformation refusal, not `ineligible / block`, `indeterminate / hold`, `not_applicable / no_effect`, or a persisted reliance occurrence. |

Completion inspection is read-only over the exact resolved item; it reports the
package's unsupported completion boundary. No result-submission/import API is
added. A closeout request, asserted state or extra semantic field is refused
at the supported interface rather than accepted and ignored. This packet claims
no positive eligibility implementation or reliance-refusal history protocol.

## Implementation work and verification

Keep the increment within one native state owner and one SDK facade. Add the
owner module/export and focused native tests; add the facade/export, one real SDK
integration test and concise usage/response/recovery documentation alongside the
existing SDK guidance. Other owners may receive only the narrow work-rule activation/currentness and held-admission
reuse necessary for this path; preserve their existing bytes and behavior. No
workspace/crate/catalog/schema-generation expansion is selected.

Before editing any existing function/method, run GitNexus upstream impact and
report direct callers, processes and risk; report HIGH/CRITICAL before edits. If
the same index degradation prevents trustworthy results, record it and supplement
with bounded direct caller/source inspection without repairing the index here.
Run required detect_changes before commit; distinguish this packet's actual delta
from the broad branch-versus-main result.

Planned focused commands (new test targets are proposed, not existing tests):

```text
cargo test -p handbook-engine --test work_state
cargo test -p handbook-sdk --test work_state
cargo check -p handbook-engine -p handbook-sdk
git diff --check
```

Verify real normal-dependency registration/reopen/current/history and receipt;
same-key replay/conflict and duplicate item; unsupported kind/relation/state;
insufficient/revoked/substituted authority; positive new human activation on
post-local-check D and established-from-D G, refusal with local-check terms alone,
D→G invalidation and new explicit G activation without old-request substitution;
actual guards held across terminal publication; exact missing/corrupt history
refusal; concurrent creators/readers; every prefix × transition in the matrix,
including lost replies before/after intent, decision, snapshot and terminal;
same-identity recovery without repeat effect; current open truth after refresh/reset
with new creation refused when required operating state is lost; root/lock
substitution, positive owner bootstrap versus lost state, peak transaction capacity
(including candidates and terminal/noncommit closure), reservation release only
on exact proof, prepared successor composition compatibility, bounds/codec refusals
and unchanged legacy result behavior. Pause registration/activation against real
revocation and D→G transitions to prove serialization and stale refusal.

Use deterministic test-only interruption points around real native writes and
process-level concurrency where it measures the lock/commit contract. They cannot
be enabled or used by the ordinary SDK consumer to mint permission/owner results.
Existing durability/initial-authority tests are precedent; rerun focused existing
targets only if their touched path or a demonstrated regression requires it. Do
not rerun/reclose independent-local-check or claim a full-workspace green result.
Report Windows/Unix coverage and all actual limits separately.

After these checks, one fresh focused causal reviewer evaluates the exact bounded
change and evidence under the [P1–P4 policy](../../review-and-closeout-policy.md#finding-priority-and-disposition).
Fix P1/P2 and obtain the required fresh remediation review; P3/P4 are advisory.
Commit only this packet's coherent reviewed product/documentation delta locally.
Return PACKET_COMPLETE to meta for aggregate acceptance, human recap and stop.
