# Work compositions and kinds — Packet 2: optional immutable whole-composition kinds

Status: **bounded authority preparation; human-selected recommendation B;
concrete implementation plan awaits human approval** — 2026-10-09.
Part of [one slice](README.md). This document authorizes no product edits and
supplies no runtime project approval or work-registration permission.

After reviewing/discussing Packet 1, the human selected **per-version approval**
and authorized this bounded planning: prepare the concrete plan for review before
implementation. B means the original claimed human separately approves each exact
immutable kind version/content on supported current D/G. It does not establish a
reusable kind-administration grant or reopen selected Packet-1 recommendation A.

## Observable workflow and human checkpoint

Why next: [Packet 1](packet-1.md) already supplies the finite ordinary,
planning-output and requirements-input menu, one composition-aware registration
authority, explicit scope successors and exact recovery. Reusing a named whole
composition now removes repeated composition/schema choices while preserving the
same explicit meaning and independent operating permission.

**Concrete example:** project Q has current local-check D and original human H.
H approves `q.work-kind.planning@1.0.0`: exactly the supported planning-output
descriptor, label/work-question maxima 256/512 bytes, display token `planning`
with omission disallowed, and required-output-question maximum 512 bytes. These
are example supplied values, never defaults. The caller prepares the separate
definition-establishment operation, saves its original locator, invokes it and
obtains the committed exact definition binding K1 and receipt. This may precede
any work-registration grant. It establishes a definition, not permission to work.

Under a separately established current grant R permitting that descriptor and
these actual values, the caller registers item P with K1, label “Plan session
renewal,” work question “What renewal behavior should be specified?”, token
`planning`, and output question “What exact requirements must renewal satisfy?”
The common registration path resolves K1 and admits P as `open`, completion
`unsupported / not assessed`. An explicit planning-output input with the same
values has equal composition meaning. The two forms retain different provenance
and request/receipt fingerprints; different item keys still identify different
items. Reopen resolves K1 and P and recovers lost replies from their saved locators.

**Meaningful refusal:** K1 is committed but R permits only ordinary leaves, or
permits planning questions only up to 256 bytes while P's question is 300 bytes.
Kind-backed registration refuses exactly as the equivalent explicit input would.
K1's definition approval cannot replace R or expand it. A wider K2, a revoked R,
an uncommitted kind, or caller-supplied descriptor override cannot repair that
request. Scope change needs the separate original-human Packet-1 operation.

The selected review increment is **Packet 2**, the final outlined meaningful
slice outcome: usable native/SDK definitions and kind-backed registration,
including refusal, retained truth and original recovery. Internal steps below are
implementation dependencies, not new packet identities or approval gates. If an
earlier independently meaningful outcome is delivered separately, apply the
[lifecycle's earlier checkpoint](../../work-unit-lifecycle.md#human-review-boundaries).
The implementation task returns Packet-2 evidence/commit to meta; meta checks the
complete slice and cross-packet integration once, recaps and stops for human
review before successor planning. This preparation performs no slice closeout.

## Admission, current sources and bounded choices

Hard predecessors are complete: foundation at
`cbfbf7629a0642b8cb08aefea36fa556009d9274`, and Packet 1 under approved plan
`277674e2` at `7129b5c92672624a0314ff6f7904873050d90927`. Supplied meta evidence
confirms foundation closeout and Packet-1 seven-condition validation, focused
causal APPROVE without unresolved P1/P2, human recap and subsequent discussion.
This planning checks source/lineage and reuses that evidence; it does not rerun
product suites or reclose a predecessor. Implementation additionally requires
human approval of this exact bounded plan and verification of the live checkout.

| Current source | Implementation consequence |
| --- | --- |
| [Types](../../../../../crates/engine/src/work_state/types.rs), [scope](../../../../../crates/engine/src/work_state/scope.rs) | `WorkCompositionV1` currently refuses every kind; three exact descriptors and `WorkCompositionClauseV1` already supply closed role/bounds validation. Extend that semantic owner, not generic Artifact Kinds. Retained singleton grants remain ordinary-only. |
| [Transactions](../../../../../crates/engine/src/work_state/transaction.rs) | `Operation::grant()` assumes every effect has a registration grant; `output`, `parsed`, `scan`, `reservation`, `execute` and `close_noncommit` form the existing publication/recovery seam. Separate definition admission needs its own original check/basis route while reusing these durability rules. |
| [Store](../../../../../crates/engine/src/work_state/store.rs) | Bootstrap and `Owner.bootstrap` currently bind a grant, though owner identity derives only from initialized project/claim. Support a closed definition-or-grant bootstrap origin without minting a grant; preserve old anchor/bootstrap bytes and identity. One positive manifest, lock and combined budget remain. |
| [Held authority](../../../../../crates/engine/src/local_authority/work_registration.rs) | Initial/operating/check/tree guards and captured original D/G proof can support a separate fixed definition operation. The seam validates currentness; local-check permission itself authorizes neither definition establishment nor registration. |
| [SDK](../../../../../crates/sdk/src/work_state.rs), [usage](../../../../../crates/sdk/docs/work-state.md), [example](../../../../../crates/sdk/examples/work_state.rs) | Follow typed prepare/save-original-locator/invoke/resolve/recover, sealed prepared inputs and attributed `WorkExecutionV1` outcomes. SDK observes D/G; Engine consumes the exact basis under guards. |
| [Owner tests](../../../../../crates/engine/tests/work_state.rs), [SDK workflow](../../../../../crates/sdk/tests/work_state.rs), [durability](../../../../../crates/sdk/tests/work_state_durability.rs), [integrity](../../../../../crates/sdk/tests/work_state_integrity.rs), [retention](../../../../../crates/sdk/tests/work_state_retained.rs) | Existing prefix, process-serialization, positive inventory, zero-free-slot closure and unchanged original-byte proofs are precedent to extend. Disabled physical-root tests require actual original-writer roots; an ignored test is not evidence. |

The [semantic amendment](../../semantic-model/amendments/work-progression-kinds-and-relations.md)
and [Work Progression ownership](../../semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
remain bounded inputs. [Architecture](../../architecture-context.md#semantic-ownership-and-dependency-direction)
keeps Engine inward of SDK, independent Contracts truth and later Work Progression
eligibility/governance separate. No adjacent semantic or physical-owner expansion.

## Exact definition, reference and parameter contract

Define an Engine-native, project-local immutable whole-composition definition.
The following contracts are the plan's bounded design selections for human review;
local Rust names and record tags may follow repository conventions without changing
their meaning.

- **Content:** a closed definition schema/version, exact name/version reference,
  and exactly one complete supported `WorkCompositionClauseV1`-equivalent template.
  The template fixes descriptor/reference, role schema, initial `open`, empty
  relation support, `unsupported_not_assessed` completion and compatibility version,
  plus all common/role bounds. No multi-composition menu, inheritance, kind-of-kind,
  executable predicate, policy hook, assessment result or transition override.
- **Reference:** a bounded dotted lowercase/hyphen identifier (at most 255 ASCII
  bytes, nonempty segments beginning with a letter and ending in a letter/digit)
  plus exact canonical semver (at most 255 bytes). Use a purpose-specific kind type;
  shared syntax does not make this a Contracts or generic Artifact binding.
  No `latest`, range, family wildcard, default alias, mutable head or implicit
  version selection. Versions are opaque exact identities, not upgrade promises.
- **Binding:** Engine owner/schema tag, initialized project identity, exact
  reference and full definition-content fingerprint. Namespace uniqueness is
  `(project, name, version)` across retained bases/checks/record schemas; a second
  check or D→G does not create a fresh namespace. Physical owner/root proof is
  required at use; a copied other-project binding is not admissible.
- **Fingerprint:** deterministic canonical JSON of a purpose-specific versioned
  domain and the complete definition content above, including reference, exact
  descriptor and bounds. Approval/request/basis/receipt identity is separate and
  includes its own project/provenance. No semantic component hash substitutes for
  full content; list order stays approved content as in Packet 1.
- **Parameters:** item/request keys, label, work question, optional display token
  and matching role value remain the supported per-item inputs. Every text is
  nonblank, NUL-free and within explicit positive UTF-8 byte maxima no greater than
  4,096. Keys/tokens retain current 128-byte ASCII syntax. Omission permission and
  finite unique display tokens are explicit. Ordinary has no role; planning has
  exactly one bounded output question and no future locator; requirements has
  exactly one locator from a nonempty finite unique definition list using the
  current three Contracts family/schema/full-fingerprint shapes. Complete records
  and all derived/preimage/proof records must fit 1 MiB before acceptance.

Definition bounds may be wider than, narrower than, or unrelated to the current
registration grant. Establishment validates supported conformance, not grant fit.
For an item, **both** kind bounds and exact current grant scope must permit every
expanded value. Neither grant nor kind can override the other. A retained ordinary
singleton grant permits a kind-expanded ordinary leaf only with its original
omitted display/no-role/text limits, through the same permission model. Its bytes
and historical request comparison stay unchanged; kind provenance cannot widen it.

The registration input becomes a closed discriminated explicit-or-kind choice
inside the **same** `prepare_work_registration` / `register_work_item` workflow.
Explicit supplies descriptor and role values; kind supplies only exact binding
and matching role values, with common item values outside that choice. Engine
resolves descriptor/schema/bounds from committed retained truth. Kind input cannot
supply a competing descriptor, extra role or bound override. Unknown fields refuse,
including in portable JSON; no permissive drop or ambiguous untagged new choice.

Expansion yields the exact explicit descriptor bytes/fingerprint, role values,
display and lifecycle for the same supported values. Kind identity is provenance,
not a primitive or new item-identity input. Keep `WORK_IDENTITY_SCHEME_V1` and
`work_item_identity_v1(project, item_key)` unchanged. New request/receipt identity
binds submitted form, original grant/basis, exact definition binding/content bytes,
establishment locator/receipt fingerprint and resolved explicit values. Snapshot
exposes effective composition and admitted kind binding; receipt/locator exposes
original form and provenance. Explicit and kind requests need not have equal
fingerprints and cannot occupy the same item via a second registration.

## Separate per-version establishment and retained truth

A purpose-specific fixed Engine operation, external to candidate content, admits
one exact definition. Its request binds supported current check/D/G basis,
initialized project/root/initial/check/Charter lineage, original claimed human
principal and check agent, exact content, stable request key, approval source,
instruction/delegation, `human_approval_asserted`, approved-content fingerprint,
fixed definition-establishment effect, full work-owner reach and retained history.
Approval fingerprints all claimed operation content except the assertion and
fingerprint's own field; its request identity includes those exact claims too.
Actor/approval remain labeled local advisory claims, with no new authentication.

Check the principal/agent against the original current native lineage and the
fixed rule; candidate fields cannot appoint an approver or approve themselves.
Each exact version requires separately supplied human approval, even if its
composition or previous version was approved. Ordinary agents cannot create a
version under a reusable grant. No kind-establishment grant, grant head,
definition revocation/reinstatement, mutable kind or registration-terms effect
is introduced. Registering work separately requires Packet-1 permission.

| Situation | Definition establishment / retained definition | New kind-backed registration |
| --- | --- | --- |
| Current supported D/G, no registration grant | Exact original-human operation can establish a kind, including first owner bootstrap. Committed definition resolves independently. | Refuses until independently approved work-registration terms are committed/current. Kind-only owner history is not a prior grant or permission head. |
| Same version already committed | Exact original request replays its receipt. A different request for that version refuses, even for equal content; resolve/reuse the existing binding. Different content under the same version always conflicts. A fresh version needs fresh approval. | Use that committed exact binding if expanded values fit current grant and kind bounds. No reapproval of retained definition is needed. |
| Pending establishment | Original request/version reserved; definition candidate cannot resolve as committed or be used. Same original resumes/recoveries only. Other versions may proceed if complete inventory/live authority/budget admit them; no global definition head. | Refuses that candidate; unrelated committed kinds/explicit work remain usable under current grant. Definition pending is not registration-head administration. |
| Positive original noncommit | Keep immutable request history; release version reservation only with complete positive noncommit proof. A fresh exact human-approved request may establish that never-committed version; old request cannot be repurposed. | No definition exists from noncommit; permission does not supply it. Uncertainty retains the reservation. |
| Registration scope change or work-grant revocation | Retained kinds remain exact. New versions still use the separate fixed human rule on current D/G, independently of grant scope/revocation/pending administration. | Latest usable grant alone controls new work. Pending administration suspends predecessor; narrowing/removal/revocation blocks affected new work without changing kinds/items. |
| D→G or other original-basis loss | Committed definitions remain retained truth, with their original D provenance. New version establishment must use positively current supported G/D. Pending old D establishment cannot borrow G; recover original commit/noncommit/uncertainty. | Reuse the existing kind after separate unchanged-scope work-grant re-establishment on G. Pending D item cannot borrow G or a successor grant. No kind re-establishment is required. |
| Initial/local-check authority revocation or runtime reset | New establishment refuses absent current native basis/guards. Committed definition/read/history survives supported reset under retained owner proof; recovery cannot recreate owner/locks or reactivate authority. | New effects refuse; committed item/definition truth and original receipt recovery remain independently readable. |

Committed version/content is never removed, rewritten or rebound to a new version.
New K2 affects only new explicit caller selections; items bound to K1 retain K1,
exact expansion and original receipt. Missing/substituted definition or establishment
proof makes dependent historical verification fail closed, not fallback to explicit
meaning or a newest definition. A reference is not proof of commit or permission.

## Durability, owner bootstrap and through-commit admission

Reuse the retained Engine owner, physical root/initialization proof, work lock,
positive slot manifest and transaction publication mechanism. Extend the closed
operation/output variants and admitted transaction names for definitions. Do not
add a separate kind registry/store/lock, parallel registration stream or latest scan.
Records remain in the current retained work-owner partition with explicit schema
dispatch; adding a sibling directory must not bypass its closed inventory.

Generalize only the bootstrap origin/current-authority access needed for a grant
or kind operation. Existing owner identity is derived from project/initialization
claim and remains exact regardless of which operation establishes it. Preserve
grant-origin anchor/bootstrap/manifest bytes and original unresolved bootstrap
recovery; a new closed origin version carries kind establishment without a dummy
grant. Kind-only bootstrap must later allow the normal first grant. Positive prior
footprints, missing anchor/slots, physical root/lock mismatch and partial bootstrap
cannot become fresh owner absence. Original bootstrap reservation and live guards
serialize concurrent kind-first/grant-first establishment; no second owner or lock.

Acquire original initial/operating/check/tree guards before the shared work lock,
in the existing order. For a new kind terminal validate fixed human rule, exact
approved content and original current D/G under held guards. For a kind-backed
item terminal additionally validate the original usable registration head and
scope, kind commit/content/expansion and every item value. Pin exact retained kind
establishment/definition bytes and publication proof through terminal, as current
`execute` pins grant/owner dependencies. SDK observation or preparation is not
permission at commit. Do not release guards between resolution and publication.

Keep acceptance/slot reservation before candidate writes and publication-attempt
evidence before native no-replace terminal installation. Commit means exact
terminal plus matching decision/definition or item snapshot/receipt and original
authority/dependency proof. Candidate/decision/snapshot alone is pending. Definition
and item operations use the existing attributed committed/replayed/refused/pending/
noncommitted/uncertain/unaccepted result semantics.

Recovery takes the saved **original** locator. Exact committed closure replays
history without live permission, new effects or reactivation. A missing terminal
can continue only under that operation's original current rule/basis and, for an
item, original still-usable grant; exclude only its own reservation. A kind's old
approval, successor grant or newly fitting values cannot substitute. Complete
owner/root/lock/inventory and positive never-attempted/not-installed proof can close
original noncommit and release only the affected version/item reservation. Current
implementation proves noncommit only before attempt; retain uncertainty after an
unproven attempted publication. Missing attempt evidence, staged partial proof,
corruption, possible publication or displaced transaction is not absence/noncommit.
Request keys keep their original content forever, including noncommit history.

One lifetime **32-transaction / 128 MiB** budget includes kinds, grants, items,
revocation and all pending/noncommit history, with 1 MiB per record and existing
peak reservations. One initial grant plus one kind leaves at most 30 fresh item
slots in an otherwise empty store. Fresh kinds and fresh revocation refuse at
saturation. Reads/replay and closure of already accepted originals require no
fresh slot; adding definition/provenance bytes must reserve their full peak before
acceptance. No increased constants, slot reuse, GC, index/database redesign or
capacity research. The [human-selected follow-up](../../coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up)
is recommended after this outcome and before committing the next broader Work
Progression increment, before sustained project use. The P3 scan-cost advisory
is distinct; Packet-1's roughly 16-minute Windows durability suite is not an
operation-latency benchmark.

## Native and SDK consumer surface

Expose owner-native typed definition content/binding, exact approval request,
sealed prepared establishment, portable original locator, committed definition
and receipt. A bounded explicit binding read needs no default/list selector.
The SDK forwards these without adjudicating definitions or minting permission.
Illustrative names below identify contract responsibilities, not already existing
methods; implementation may choose consistent purpose-specific spellings.

| Surface | Caller contract |
| --- | --- |
| `prepare_work_kind_establishment(initial, check, supplied_approval)` | Observe exact supported basis, validate inward, return sealed preparation and portable locator; preparation creates no committed kind. No registration grant input. |
| `establish_work_kind(prepared)` | Invoke fixed exact-version operation once; return `WorkExecutionV1` with committed definition/receipt only when proven. Save original locator before invocation. |
| `resolve_work_kind_request(locator)` / `recover_work_kind_request(locator)` | Read/recover original establishment truth using original basis/approval; never select current basis or accept a fresh request during recovery. |
| `resolve_work_kind(binding)` | Exact same-project retained committed definition, template and establishment provenance, independent of live permission. Positive no-match is distinct from pending reserved version and damaged/lost proof; fingerprint/content mismatch refuses. |
| Existing registration/current/history/recovery and completion methods | Explicit or exact kind input through one workflow. Preparation seals resolved meaning/provenance; terminal revalidates. Current snapshot/receipt exposes form and exact binding. Completion stays unsupported/not assessed. |

Normal consumer workflow is: supply actual approved definition content, prepare,
save original locator, establish and inspect disposition; read committed K1;
independently obtain current work grant, prepare K1 item values, save original item
locator, register, reopen/read and resolve/recover both originals. A requirements
kind example copies an owner-produced C@1 family/schema/full fingerprint with the
existing SDK helper, permits that exact locator, then passes it as the item role.
Neither kind nor work registration asserts Contracts availability, completeness,
qualification, governing assignment, coverage or assessment.

Docs/example must explain: committed/replayed definition is retained meaning, not
live permission; refusal needs corrected supported input or separate current
approval; pending kind is unusable; noncommit preserves history and permits only
a fresh separately authorized request; uncertainty means keep/recover original
locator and reservations, never new-key retry or claim of absent/open success.
There is no direct file authoring or private-layout knowledge in the consumer.

## Ordered implementation and acceptance

After human plan approval, work through these internal dependencies in one bounded
Packet-2 assignment. Preserve repository Rust/serde conventions and inward owner
validation; prepared values remain sealed. Do not publish an independently usable
partial outcome past the lifecycle checkpoint.

| Step / likely files | Acceptance and verification before dependent work |
| --- | --- |
| 1. Closed definition/reference/parameter and submitted-form types — Engine `work_state/{types,scope,mod}.rs` or a focused sibling module; owner tests | Exact supported content, canonical full identity and explicit/kind expansion; duplicate/unknown/trailing/overflow refusals; unchanged old byte domains. Codec/equivalence tests, no new primitives. |
| 2. Separate establishment, bootstrap origin and retained read/recovery — Engine `work_state/{store,transaction}.rs`, held local-authority seam; durability/integrity tests | Kind-first and grant-first use one owner/lock/manifest; original-human rule through terminal, immutable-version conflicts and original-key recovery. Test current D and G, every changed bootstrap/acceptance/publication prefix and missing proof. No candidate definition truth or grant creation. |
| 3. Kind-backed common registration — Engine types/scope/transactions; owner/SDK tests | Resolve committed kind, validate both bounds and original current grant through terminal; retain form/binding/expansion/provenance. Prove ordinary/P/I equivalence, narrower grant refusal and unchanged item identity; no replacement by K2 or successor grant. |
| 4. Typed SDK and complete ordinary consumer — SDK facade/docs/example and workflow tests | Normal dependency/default-feature caller uses saved locators, kind-first establishment, explicit/kind work, read/reopen/lost-reply recovery and clear refusals without fault helpers. Engine and example checks pass. |
| 5. Retention/fault/concurrency integration — focused SDK durability/integrity/retention targets | Preserve original foundation and Packet-1 bytes/prefixes; transition/reset and same-version/process races prove only one admitted truth. Combined budget and accepted-original closure tests pass. Review/commit/return; no successor execution. |

Packet-2 acceptance is testable as follows; together with Packet 1 it must satisfy
all seven [slice conditions](README.md#complete-slice-acceptance):

1. Separately approved exact versions establish natively/on normal SDK under
   current D and current G, both without and with an existing work grant. Wrong
   original human/agent/project/basis, changed approval, self-authorization or
   reusable grant substitution refuses. Kind-only bootstrap permits normal first
   grant later and retains old grant-origin bootstrap recovery.
2. Same-version different content conflicts; original request replay is exact;
   another request for a committed equal version refuses. Pending/uncertain version
   reservation cannot be stolen. Proven noncommit alone releases version ownership
   while preserving original request-key/content history. K2 never changes K1/items.
3. Ordinary/P/I kind inputs expand to equivalent explicit meaning and are bounded
   by both template and current grant. Exercise omission/token and UTF-8 byte
   boundaries, exact C@1 versus C@2/full-fingerprint mismatch, malformed family,
   unknown role/result/locator/hooks/relations, unsupported compatibility and every
   override refusal. Planning still has no guessed output reference.
4. Scope change/pending administration/revocation changes work permission, not
   definitions. Committed D kind is reusable on G with independently current G
   work grant; old pending D establishment/item cannot borrow G. Local-check/initial
   revocation and supported reset block effects while exact past truth resolves.
5. Every affected deterministic prefix and real process death separates candidate,
   terminal commit, positive noncommit and uncertainty for both definition and
   kind-backed item. Lost/corrupt kind transaction/manifest/attempt/lock/root or
   substituted definition dependency fails closed. Before-terminal substitution
   cannot commit; after-terminal/lost-reply recovery replays exact original truth.
6. Real processes exercise competing same-version definitions, kind-first versus
   grant-first bootstrap, committed-definition read/use versus pending establishment,
   and kind-backed item versus scope change/revocation/D→G in both serialized orders.
   No double version, owner, head, item or guard gap. Pending unrelated definitions
   do not suspend otherwise valid work; accepted grant administration does.
7. Retained foundation/Packet-1 explicit requests, receipts, grant revisions and
   unresolved original prefixes retain exact old bytes, fingerprints, meaning and
   recovery after support/reopen/transitions/reset. Capture actual physical roots
   using untouched original writers before source edits; JSON round trips alone do
   not prove physical-root recovery. Add equivalent K1 retention across K2/support.
8. Combined transaction/byte/record/peak bounds count definitions/provenance with
   all history. Saturated store refuses fresh kind/item/authority/revocation, yet
   accepted original definition and item closure/replay/reads remain supported
   without fresh slots or history reuse. Normal consumer proves full saved-locator
   workflow independently of fault features; report actual platform/stack limits.

Focused commands (extend these targets or add named narrow targets when needed):

```text
cargo test -p handbook-engine --test work_state
cargo test -p handbook-sdk --test work_state
cargo test -p handbook-sdk --test work_state_durability --config 'profile.test.opt-level=1' -- --test-threads=4
cargo test -p handbook-sdk --test work_state_integrity --config 'profile.test.opt-level=1' -- --test-threads=4
cargo test -p handbook-sdk --test work_state_retained
cargo check -p handbook-engine -p handbook-sdk
cargo check -p handbook-sdk --example work_state
git diff --check
```

Run ordinary default-feature consumer separately from fault instrumentation; run
physical-root retention explicitly with baseline-root inputs rather than counting
ignored tests. Recheck earlier evidence only where changed paths invalidate it.
Report Windows/Unix execution and stack/test-runtime limits honestly; no omnibus
workspace replay or latency promise is required. Before symbol edits perform
upstream GitNexus impact, report direct callers/processes/risk and warn on HIGH/
CRITICAL; run `detect_changes` before commit. This preparation found unavailable
GitNexus MCP, CLI status analyzer-identity failure, and a 62-commits-behind index
with failed FTS queries. Use the Packet-1 documented bounded direct-source fallback
when those limits persist; do not claim graph coverage or start index repair here.

## Format, exclusions and return boundary

The [structured-data standard](../../structured-data-format-standard.md) remains
**PROPOSED** for general defaults, **Contracts JSON fixed**, and existing
owner/versioned contracts controlling. This plan selects closed operation-managed
canonical JSON only for the new work-kind boundary, using Engine duplicate-safe
complete parsing, canonical identity and retained-root primitives. Typed operation
parameters are authoring input; no YAML importer, editable human-view authority,
format migration or global adoption. Use purpose-specific new schema/fingerprint
domains for new shapes and bounded historical dispatch, retaining actual old bytes
and all original preimages. No permanent legacy public API or mixed-binary-write
promise, disposable-fixture rewriting as historical proof, or retained-state loss.

Hierarchy, relations, eligibility, governing completion, CR, close/reopen/removal,
item rebinding, kind defaults/aliases/revocation, generic Artifact Kind reuse,
arbitrary hooks, broad IAM/authentication, CLI/Tauri/host delivery, capacity redesign,
adjacent planning, push/merge and durable orchestration archives are excluded.
No consequential decision remains unresolved within the selected B boundary;
approval of this concrete plan is still required before implementation.

Verify, obtain one focused causal reviewer, remediate P1 Critical/P2 Required and
re-review material remediation; report P3 Optional/Consider and P4 Nit without
turning advisories into gates. Make one coherent local authorized product commit
when implemented, return `PACKET_COMPLETE` to meta with evidence and the final
packet checkpoint, and stop. Only meta performs eventual aggregate slice checks
and recap; a clean verdict does not grant runtime approval or successor authority.
