# Work compositions and kinds — Packet 1: unified explicit-composition registration

Status: **recommendation A implemented and verified at `7129b5c`; meta validation,
human recap/review and subsequent discussion complete** — 2026-10-09. Part of [one slice](README.md); no permission to
establish/use kinds. Supersedes the parallel-stream Packet-1 proposal.

The human approved this complete plan at `277674e2` after the practical workflow
and exact Contracts-definition clarification. Selection is recorded in the
[slice](README.md). The implementation task verified the
expected branch/HEAD and clean tracked tree before product edits. No further
plan approval is required for Packet 1.

## Observable behavior and checkpoint

Why next: foundation delivers retained `open` work and original authority/recovery.
The selected flexibility lets a human approve a useful menu through **one
registration authority**, including a continuing ordinary-only development project.
Deliver the two declarations and bounded permission lifecycle together; no
relations, assessment or closing.

**Continuing-project example under recommendation A:** Q has current ordinary-only
R0 and retained ordinary item L. New support recognizes P/I; R0 still refuses them.
The original claimed human approves successor R1 against R0, on the same D/G
basis, permitting three exact descriptors, label/work-question/output-question
maxima of 4,096 bytes, omitted level or tokens `task`, `planning`, `implementation`,
and one exact Contracts C@1 definition with owner-family tag and full fingerprint.
R1 changes no item. A caller uses the same typed prepare/register path to create
P, token `planning`, label “Plan session renewal,” required-output question “What
exact requirements must session renewal satisfy?” P has no guessed contract
locator. It separately creates I, token `implementation`, exact C@1 as requirements
input. Both are `open`, completion `unsupported / not assessed`. Reopen resolves
both; L retains exact original descriptor/request bytes, identity and receipt.

**Fresh-project example:** on positively current supported D or G, first establish
R0 permitting ordinary and planning-output only. One SDK path registers either;
requirements-input refuses until an explicit successor permits its exact descriptor
and definition locator. No legacy enrollment or special upgrade is needed.

**Change example:** R2 removes planning permission; it blocks new P registrations
without removing P. Pending R1 requests cannot complete using R2 even if values
still fit. After D→G, explicitly re-establish the latest unrevoked head on G with
unchanged scope; any scope change is a separate human operation. Old requests keep
their basis and original-key recovery.

The human checkpoint is **Packet 1**: native authority, registration/current/
history/recovery and usable SDK. Return verification/review and coherent local
commit for the recap and stop before kind planning/delivery. An earlier delivered
independently meaningful outcome triggers the lifecycle's earlier checkpoint.

## Admission and exact scope contract

Foundation and its semantic compatibility cases are complete
[predecessors](README.md#predecessor-evidence-and-source-constraints). Before product
edits, the implementation task records the already approved
[recommendation A](README.md#selected-authorization-lifecycle) and plan and confirms
dispatch prerequisites. **One registration model is selected, not a
pending alternative.** The following A details are selected implementation
authority, not runtime project permission.

Terms bind project/root, original initial/check/Charter lineage, current supported
D/G basis, original claimed human/check agent, approval/source/instruction/
delegation/approved-content fingerprint, `register_open_leaf`, fixed full-owner
read/write reach, reserved decisions, revocation principal and retained history.
The fixed external rule authorizes establishment/change; candidates cannot
authorize themselves. Principal/delegation/approval remain local advisory claims.

Selected closed scope has at most one clause per permitted menu descriptor:

| Clause | Human-supplied permission and owner ceiling |
| --- | --- |
| Composition | Exact supported descriptor, role schema and compatibility binding from the three-entry menu. No all-menu/all-future default. |
| Common text | Explicit positive maximum UTF-8 byte lengths for label and work question, each at most 4,096; nonblank owner validation still applies. |
| Display level | Explicit permission for omission and/or a finite exact token list. Each token meets existing ASCII token syntax and 128-byte ceiling. No inferred taxonomy or permission for every bounded token. |
| Planning output | Positive question maximum at most 4,096 bytes; exactly one nonblank question; no locator/fulfilment assertion. |
| Requirements input | Nonempty finite permitted-locator list: fixed Contracts owner, supported definition family/schema version, exact contract reference/version and exact full definition fingerprint. Recognizing a family permits no other definitions. |

Require nonempty composition scope. Reject duplicate/unknown or irrelevant clauses,
unsupported versions, token/locator wildcards and impossible maxima. Finite lists
and nested records fit 1 MiB complete terms/request; overflow refuses without
clipping. All fields are approved content; example values are not defaults.
Match exact descriptor and every common/role parameter inward of SDK, under live
native authority through terminal publication.

Human-approved same-basis successors can add/remove clauses and widen/narrow
parameters; principal/agent, lineage, effect, reach, assurance, reservations,
revocation and retention stay fixed. Exact expected predecessor prevents races;
no union with older grants. D→G preserves entire latest scope/fixed fields with
fresh explicit approval. Reinstatement after committed revocation, reassignment,
general delegation and combined scope-change/D→G refuse next.

Accepted administrative intent reserves the single head and suspends predecessor
use. Candidate and competing administration are unusable. Proven original-key
noncommit releases the reservation; predecessor use resumes only with original
currentness/unrevoked proof. Uncertainty keeps suspension. This availability
consequence is part of the human-selected A; retained item/history reads stay
available with complete proof.

## Meaningful refusals

- Ordinary-only R0 cannot register P/I, add a level or borrow newly recognized
  semantics. Unactivated/self-authorizing terms, wrong actor/project, insufficient
  reach, changed approved content and stale/revoked basis refuse.
- Permission for C@1 refuses C@2, different full fingerprint, latest alias or
  result locator. A semantic component fingerprint cannot replace a complete local
  definition fingerprint. Unapproved token, even below 128 bytes, or a value above
  approved maximum refuses.
- P refuses guessed output locators; I asserts no assessment, qualification,
  governance or coverage. A permitted well-shaped declaration with unavailable
  Contracts truth records only the declaration; later resolution handles absence
  without inferring successful assessment.
- Unknown fields/roles/hooks, relations, kinds, changed descriptor under L's key,
  close and unsupported schema versions refuse without discarded meaning.
- Candidate/predecessor after accepted administration, competing/stale predecessor,
  combined D→G widening or revoked-grant restoration cannot supply permission.
  No second stream or grant substitution repairs these refusals.

## Owner and typed interface boundary

Engine `work_state` owns conformance, scope matching, immutable authority lineage,
work/request identity, persistence, current/history/recovery and unsupported
completion. Extend only the narrow held
[local-authority seam](../../../../../crates/engine/src/local_authority/work_registration.rs)
needed for original-human scope changes. Acquire initial/operating/tree guards
before shared work lock, in one consistent order for authority and item effects;
hold declared dependencies through terminal. SDK observation or a returned Allowed
value outside those guards is insufficient.

SDK revises the public typed surface on `HandbookSdkV1` to one terms lifecycle and
one composition-aware `prepare_work_registration` / `register_work_item` path.
Names are illustrative until implementation; no `register_menu_*` or legacy
operating alternatives. Types distinguish initial establishment, same-basis change
and unchanged-scope D→G using exact predecessor/basis. Expose saved locators/native
outcomes for terms, work and revocation. Current grant resolution differs from
historical request resolution. Prepared native values stay sealed; locators supply
identity, not permission.

The registration input is a closed discriminated choice:

| Choice | Immutable descriptor and parameters |
| --- | --- |
| Ordinary | Exact retained `WorkDescriptorV1::ordinary_leaf()` bytes, no role values. Display/text values do not change its meaning. |
| Planning output | Fixed versioned descriptor and exactly one required-contract-output question; no reference/result. |
| Requirements input | Fixed versioned descriptor and exactly one exact-requirements-input definition locator; no result/coverage assertion. |

Every descriptor fixes initial `open`, empty relations, completion
`unsupported_not_assessed`, role schema and compatibility version. Same-project
leaves only. Omitted level remains omitted; historical items gain no token/taxonomy.
Item/request keys retain 128-byte token syntax; texts have 4,096-byte ceilings and
possibly smaller approved maxima. Duplicate/unknown fields, trailing documents,
excess values and mismatched descriptor/schema refuse.

Engine's neutral definition locator uses closed tags for intrinsic V1, complete
local V2 and full local V3 Contracts definitions; future families refuse. It names
fixed owner, family/schema, exact reference/version and full fingerprint, no path
or latest selector. SDK copies owner-produced fields; Engine validates shape,
binding syntax and permission, not source existence/completeness.
[Contracts identity](../../../../../crates/contracts/src/identity.rs),
[intrinsic definition](../../../../../crates/contracts/src/definition.rs),
[local V2](../../../../../crates/contracts/src/local_contract.rs) and
[full V3](../../../../../crates/contracts/src/local_evaluation_definition.rs)
own those identities. V2/V3 `binding()` fingerprints the whole definition, distinct
from embedded semantic V1. Contracts resolves truth; later Work Progression admits
qualification/governance/coverage. No inward Contracts/SDK dependency or work
prerequisite for independent Contracts use.

## Persistence and minimum retained-state compatibility

Preserve `WORK_IDENTITY_SCHEME_V1` and `work_item_identity_v1(project, item_key)`:
codec/release/descriptor/level/role/kind/delivery are not item-identity inputs.
New initial snapshot/receipt binds exact request/grant revision/basis, descriptor
bytes/fingerprint, role values/display and `open`. No old item edit/rebind/deletion.

Use existing retained owner/root/shared lock and positive expected inventory.
Add closed versioned records/schema dispatch narrowly; preserve old V1 bytes and
fingerprint domains. An original ordinary grant retains singleton scope with
display omitted through the same admission/lifecycle. Retained readers/recovery
are minimum compatibility, not permanent APIs or a separate permission population.
Schema-specific original-request decoding preserves equality/proof before shared
lifecycle checks; it never manufactures new permission.

Inventory/conflicts span all retained schemas. Authority has one unbranched
expected-predecessor chain/reservation; item/request keys reserve across old/new
records. Extend admitted names explicitly, without blind sibling directories,
rewritten slots/history, newest-file scans or legacy permission heads. Older
binaries may refuse new schemas; backwards mixed-store writes are not promised.
Disposable fixtures/API call sites may change; retained-state tests use unchanged
original bytes. Removing decoders/store would lose historical verification/recovery
and is unauthorized, as [the slice explains](README.md#development-compatibility-and-retained-truth).

Budget stays **32 transactions / 128 MiB**, including initial terms, every scope
revision/D→G/revocation, item, candidate/noncommit history and later kind operations.
Slots are not reused. One initial grant, two scope successors and one revocation
consume four slots before items/other history. Preserve 1 MiB record and peak
staging/inventory bounds. Overflow refuses new admission while preserving old
reads/recovery and closure of accepted transactions; closing an accepted original
transaction must not require a fresh slot. No capacity increase, per-schema budget,
garbage collection, database or migration tool. P3 repeated debug scan cost remains
advisory; no throughput claim or optimization project.

## Original-request recovery and transitions

The [foundation prefix contract](../work-item-state-foundation/packet-1.md#durable-prefix-and-authority-transition-contract)
governs every new effect. Retain exact acceptance/reservation and publication-
attempt evidence before writes. Commit requires exact no-replace terminal plus
matching grant or snapshot/receipt and original proof. Decision/candidate alone
supplies no usable permission/current `open` truth.

| Retained proof | Required result and action |
| --- | --- |
| Complete positive inventory, no accepted original request | Unaccepted/absent request; no creation permission/noncommit replay. Recovery never starts a new request. |
| Accepted intent/decision/candidate without terminal, original authority unchanged | Pending. Original-key publication only with original usable head or original fixed establishment/change/revocation rule, current original basis and held guards. Candidate never authorizes activation. |
| Accepted successor/revocation, D→G, currentness loss or reset before item publication | No candidate/successor/G substitution. Prove original commit/noncommit or retain uncertainty and item/request reservations. Scope narrowing/removal changes future permission, not retained items. |
| Pending authority operation | Reserves/suspends head. Continue only original operation/rule/basis/predecessor, excluding its own reservation from its own continuation checks. Other administration/use refuses until commit/noncommit. |
| Exact committed terminal and matching records/proof | Replay original outcome without second effect, reactivation or restored permission. Current item truth is separate from live authority. |
| Positive noncommit proof | Complete retained inventory, original owner/root/lock, and positive proof publication never attempted or did not install permit durable original-key noncommit before reservation release. Keep all history. |
| Possible publication or corrupt/substituted/missing proof | Integrity refusal/uncertainty. Absent terminal bytes alone never prove noncommit. Retain reservation; no timeout/reset/replacement grant/new-key retry. |

Same-basis pending scope-change noncommit can restore predecessor usability only
under its exact still-current unrevoked proof. After D→G/revocation it stays
unusable. Committed successor never substitutes for old requests even with full
scope overlap. Registration committed before serialized change keeps its receipt;
an earlier accepted change blocks predecessor publication. Test both lock orderings.

Retained read/history/recovery dependencies outside `.handbook/state` verify exact
initial/check/Charter/D/G closure, approval/instruction/delegation/advisory claims,
grant/request/descriptor/values and publication evidence. Reset may block effects
through lost operating state, but cannot erase past truth or recreate physical
locks/identity as proof. No bulk cancellation/transfer/store reset is implied.

## SDK workflow and response guidance

1. Observe current supported basis and prepare supplied approved terms for initial
   establishment or exact predecessor successor. Save original locator before
   activation/change; resolve native outcome. Pending candidates are unusable.
2. Prepare ordinary/P/I against exact usable grant, save original locator, then
   invoke the common registration operation.
3. Reopen to resolve current item separately from historical outcome. Recover lost
   reply with saved original locator without a newer grant. Save revocation locators
   before invoking in the same way.

| Response | Caller meaning and next step |
| --- | --- |
| Committed/replayed with receipt | Save locator/receipt. Work establishes `open` only; authority replay is history only. Resolve current permission before new effects. |
| Refused before effect | Correct supported input under current permission or seek explicit scope change. Changed content needs new request key and cannot overwrite committed/reserved item or authority history. |
| Pending | No usable grant or `open` candidate. Recover original request; administrative reservation suspends dependent effects. |
| Positively noncommitted | Keep history; fresh effect needs current permission. Administrative suspension lifts only with unchanged current predecessor proof. |
| Uncertain | Keep/recover original locator; no absent/open/closed assertion or new-key retry. Reservations stay. |
| Unaccepted | Positive proof of no acceptance; no creation permission/noncommit replay. |
| Current item/historical outcome | Exact owner fact, no permission/eligibility/completion. Missing retained proof refuses instead of empty success. |
| Completion unsupported/not assessed | No Determination, eligibility/disposition or close effect. Kinds/relations/closing unsupported. |

## Acceptance and verification

1. Real native/normal SDK paths establish terms initially on D/G and register
   ordinary/P/I through one path. Exact token/text/locator scope and unsupported
   roles/relations/hooks/close refuse; no output locator or assessment assertion.
2. Ordinary-only scope never widens on recognition. Human-approved same-basis
   successors add/remove/widen/narrow clauses, retain immutable history and at most
   one usable head. Candidate/self-authorized, competing/stale predecessor and
   unrelated permissions refuse. Both retained foundation/new-format predecessors
   use the same lifecycle without special upgrade.
3. Pending administration suspends predecessor; positive noncommit restores it
   only if otherwise current; uncertainty keeps suspension. Revocation blocks new
   effects, no reinstatement/fallback. D→G after multiple revisions requires latest
   predecessor/unchanged scope; no old request borrows successor/G permission.
4. Retained L, grants/receipts/request locators and unresolved foundation prefixes
   preserve exact bytes/identity/scope/outcomes through support, changes, reopen,
   D→G/revocation/reset. Cross-schema conflicts and corrupt history fail closed.
   Prove pending continuation with still-live original singleton authority and
   noncommit/uncertainty after its loss, without permanent old public APIs.
5. Deterministic interruption and real process serialization test new scope/P/I
   prefixes against change/revocation/D→G/reset. No candidate truth, grant unions,
   double heads or guard gaps. Missing attempt/inventory/root/lock proof refuses
   or stays uncertain with exact original-key recovery.
6. Combined 32-transaction/128 MiB/1 MiB/peak bounds preserve reads/recovery. Zero
   free slots including authority/candidate/noncommit rows still permits closure
   of accepted originals. No slot reuse or automatic expansion; report scan cost.
7. Ordinary default-feature consumer saves locators, demonstrates initial/scope
   change, P/I reopen/current/history/lost-reply recovery and clear refusal steps,
   without fault helpers/private file knowledge.

Delivered implementation: Engine work-state types/scope/transactions use the
existing store and held authority seam; SDK facade/example/usage and focused
owner/SDK retention/durability/integrity targets exercise the selected boundary.
No Contracts semantic rewrite. Reuse Engine
duplicate-safe complete JSON/canonical identity and retained-root primitives;
no inward SDK dependency. Require the format standard with PROPOSED general
defaults, fixed Contracts JSON and existing identities.

The typed scope/lifecycle, native menu, immutable successor admission and original
recovery, single SDK surface and usage are implemented. These results authorize
no other packet. The subsequent meta validation and human recap/review are
complete; the [slice status](README.md) owns final combined completion.

Focused implementation checks:

```text
cargo test -p handbook-engine --test work_state
cargo test -p handbook-sdk --test work_state
cargo test -p handbook-sdk --test work_state_durability --config 'profile.test.opt-level=1' -- --test-threads=4
cargo test -p handbook-sdk --test work_state_integrity --config 'profile.test.opt-level=1' -- --test-threads=4
cargo check -p handbook-engine -p handbook-sdk
cargo check -p handbook-sdk --example work_state
git diff --check
```

Actual verification: Engine codecs 5/5; plain SDK workflow 10/10 plus the final
added closed-codec test 1/1; optimized durability 11 passed with two intentionally
ignored process-child entrypoints, integrity 9/9, and the freshly rebuilt final
pending-administration/noncommit-key test 1/1. The final retained target passed
3/3 across 14 real physical roots freshly emitted by the untouched original
writer; golden bytes and occupied original records stayed exact. A separate
default-feature consumer passed 21 saved-locator process invocations with normal
dependencies and no fault helpers. Engine/SDK and example checks, scoped formatting,
local Markdown links and diff checks passed. Windows was exercised; Unix and a
full-workspace replay were not. Plain SDK tests used a command-local 32 MiB stack;
the example uses 8 MiB. The optimized four-thread durability run took 959.54 seconds,
so the inherited P3 scan-cost advisory remains. GitNexus change detection was
attempted: the stale index does not map this delta; its CRITICAL branch-versus-main
result covers inherited scope, with direct-source review used for this packet.
One fresh focused causal review returned **APPROVE**, with no unresolved P1/P2
and no new P3/P4 findings; the inherited scan-cost advisory is non-blocking.

Extend focused targets or add narrow new targets; run ordinary consumer with normal
dependencies/default features separately from fault helpers. Report actual Windows/
Unix/debug stack limits. Rerun earlier targets only if changed paths invalidate
evidence; no foundation/local-check reclose or full-workspace claim. Before source
symbol edits run upstream GitNexus impact, report callers/processes/risk and warn
on HIGH/CRITICAL. Attempt detect_changes before commit, distinguish this delta from
inherited branch-versus-main scope, use bounded direct-source fallback for known
stale/foreign index/failed FTS and do not repair the index here.

Verify, use one fresh causal reviewer, fix P1/P2 with fresh delta review after
material remediation and report P3/P4 advisories. Commit one coherent authorized
packet locally; return to meta for human recap and stop. Kinds and adjacent outcome
preparation remain outside this increment.
