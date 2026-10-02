# Owner-native admission-governance establishment

Status: **Single-packet slice complete locally at `aa793c1`; meta aggregate
verification confirms the nine acceptance conditions from the Packet-1 evidence
below. Awaiting the selected human review; no successor work is authorized**.
Prepared 2026-10-01 against `e2c073430e4eb322cf469198947bae5136941799`.
Reconciled 2026-10-02 against `601a080` for the caller-supplied ceiling and selected
file-format boundary. The human authorized local planning updates and review,
followed by top-level Packet-1 implementation dispatch once that review is clean.
Descriptive slice identity: `admission-governance-establishment`; no phase number
is assigned. The human authorized this first bounded preparation after reviewing
the [coverage and landing map](../../coverage-and-landing-map.md). The persistent
meta chat owns adjudication, verification of dispatch conditions, and the human recap.

This document selects the bounded implementation choices for that authorization.
The design and planned-check sections retain their preparation context; the
Packet-1 evidence below records the delivered implementation.
The September 30 semantic approval alone did not select them or authorize coding;
they do not describe existing product behavior. The [lifecycle](../../work-unit-lifecycle.md)
and [review policy](../../review-and-closeout-policy.md) control execution and stops.

## Outcome and next increment

For one newly registered local project and one exact root scope, an explicit
advisory instruction establishes an immutable admission configuration, validates
its one Contracts owner-rule binding, and makes it active through a confirmed
native owner commit. A typed Rust SDK caller can resolve the exact active basis
and replay the original establishment outcome after restart. Invalid or
unresolvable authority refuses; uncertainty stays uncertainty until owner recovery
resolves it. Ordinary policy-candidate admission and successor activation are
outside this slice.

The proposed example is project P, identified by its exact Engine repository
identity fingerprint, at a retained local repository root. Its only scope is
`contracts.evidence-freshness-policy/root`. Configuration
`project.admission.freshness@1.0.0` binds the proposed fixed Contracts rule
`handbook.contracts.freshness-policy-admission@1.0.0` and limits candidate maximum
observation age to a caller-supplied 86,400 seconds in this example only. The caller
must supply a positive JSON-safe integer ceiling in seconds; there is no implicit
24-hour default. Exact source revision is required and allowed future skew is zero,
preserving the existing fixed domain floors. The ceiling constrains later policy
admission; it is not a retention period or a timer that deletes governance records.
These choices do not redefine the whole approved semantic model.

The caller says: establish this exact configuration for P under the standalone
advisory instruction and retain explicit local instruction as the basis for later
replacement. Success reports separately that content validated, advisory permission
was accepted, and activation committed. A subsequent exact resolution returns
the same configuration and owner-rule content identities. Replaying the original
request returns its original committed outcome without another activation.
Changing the ceiling while reusing the configuration identity refuses. Deleting
the active record in an established project refuses resolution and initialization;
it does not produce an empty/new project or authorize another setup.

## Why this is next and what current sources prove

The controlling semantic responsibility is [establishment and replacement of
governing admission requirements](../../semantic-model/objects-and-governance.md#establishment-and-replacement-of-governing-admission-requirements).
The [bounded setup lifecycle](../../semantic-model/operational-reliance-and-actions.md)
does not depend on a future evaluation or a policy admitted under the configuration
being created. The [approval and HR register](../../semantic-model/review-decision-register.md)
settle HR-001 through HR-006 at the semantic level only. Establishment supplies a
real prerequisite for later project-governed policy admission; it does not deliver
that later admission or any evaluation result.

The [program map](../../phase-and-slice-map.md) places this descriptive slice as
the selected first approved-semantics foundation, separate from Phase 6 adoption.
Phase-3/4/5 exits and HCM-4.6 stay complete within their historical boundaries.
There is no external adopter, publication, whole evaluation chain, or generic
artifact correction hard predecessor for the selected route. Its existing
technical foundations are present in source, with the limits below:

| Inspected evidence at the planning baseline | Reusable foundation and limit |
| --- | --- |
| [Contracts exact/canonical primitives](../../../../../crates/contracts/src/canonical.rs), [exports](../../../../../crates/contracts/src/lib.rs), and [freshness policy](../../../../../crates/contracts/src/evaluation_policy.rs#L219) | Exact refs/fingerprints and canonical JSON exist. Freshness construction at lines 228–249 requires positive bounded age, exact source revision, and zero future skew. These are existing domain constraints, not project admission governance. |
| [Contracts repository](../../../../../crates/contracts/src/owner_repository.rs#L348), [owner tests](../../../../../crates/contracts/tests/owner_repository.rs#L63), and [lifecycle tests](../../../../../crates/contracts/tests/lifecycle_chain.rs#L498) | Native semantic inventory, process/native serialization, immutable conflict, exact replay, CAS and restart recovery are precedent. Credential admission, independent lock, and definition lifecycle are different authority and must not be borrowed for advisory establishment. |
| [Engine retained durability](../../../../../crates/engine/src/repository_durability.rs#L237) and [tests](../../../../../crates/engine/tests/repository_durability.rs#L106) | Public bounded reads/listing, durable exclusive create, no-replace publication, exact replacement and native locks are usable mechanisms. The private authority-tree guard at line 332 is tied to other authority paths and is not a governance inventory proof. |
| [SDK setup](../../../../../crates/sdk/src/setup.rs#L563), [identity initialization](../../../../../crates/engine/src/repository_invocation_identity.rs#L367), [approver bootstrap](../../../../../crates/engine/src/approver_registry.rs#L201), and [profile selection](../../../../../crates/engine/src/profile_selection.rs#L82) | Setup/identity, credential mappings and profile loading exist. None establishes HR-001 authority or positively proves that governance was never established. Identity can be created when identity/registry are absent; `Created` alone is insufficient freshness evidence. |
| [SDK reset selection](../../../../../crates/sdk/src/route_state.rs#L83) and [removal](../../../../../crates/sdk/src/route_state.rs#L165) | Reset clears all selected `.handbook/state` children. The sole governance registration/history must not be placed in that tree. |
| [SDK coordinator](../../../../../crates/sdk/src/lib.rs#L71), [membrane exports](../../../../../crates/sdk/src/contract_membrane/mod.rs), [operation catalog](../../../../../crates/sdk/src/contract_membrane/operation.rs#L683), and [owner admission](../../../../../crates/sdk/src/contract_membrane/owner_admission.rs#L64) | Typed owner composition exists. No governance route exists in the closed ordinary inventory. Existing operation admission, setup success, and structural schemas cannot supply governance authority. |

Hard implementation admission conditions are human authorization of this exact
increment and fresh inspection that these mechanisms remain available and compatible.
The new registration, rule, native transaction, and typed consumer are work inside
this increment, not already satisfied prerequisites. Existing roots without positive
native fresh-project registration remain unsupported; this plan does not authorize
conversion of established projects or recovery by generating a replacement identity.
If inspection during implementation exposes a required different owner, scope,
authority rule, or migration, return the exact proposed smaller/different boundary
to meta control before product edits outside this plan.

### Generic-artifact dependency decision

[R-02/E5 and R-05/R-06/E11](../../../../research/hcm-coverage-landing-2026-10-01/rules-and-definitions.md)
were checked against their cited source, tests, and exports. The mismatch is real:
[Context Resolution's private authority tuple](../../../../../crates/engine/src/context_resolution.rs#L30)
names non-family authority content; `load_authority_snapshot` at line 1454 uses
generic repository/lineage recovery, and lines 1526–1595 require the generic
operation tuple and exact committed artifact promotion. [Special promotion
checks](../../../../../crates/engine/src/artifact_mutation.rs#L1726) support that route.
The [test payload](../../../../../crates/engine/tests/context_resolution_kernel.rs#L449)
contains repository/profile/stack/registry authority mappings and declaration
lineage, followed by generic intake/candidate/promotion at lines 503, 529 and 627.
Category-2 definitions and category-3 private authority lineage remain distinct;
the whole capsule is not classified as one definition.

**For the proposed path this correction is outside the exercised dependency
closure.** The new governance responsibility uses native records and retained
durability, not `ContextResolutionAuthorityAdmissionResolver`, generic artifact
intake/promotion, profile-selected admission authority, or credential publication.
Sharing canonicalization/filesystem mechanisms does not share those lifecycles.
The new setup registration must likewise avoid the Context Resolution authority
resolver. Existing generic authority behavior remains a known separate correction;
this slice cannot claim repository-wide six-family lifecycle confinement. If a
necessary call enters that path, the answer changes: stop and report the narrowly
coupled correction before claiming establishment complete. Do not silently perform
a broad artifact rewrite or exempt the called authority capsule.

## Proposed owner, rule and representation

Physical placement: a separately named `project_admission_governance` module in
`handbook-contracts`, inward of SDK, distinct from contract definition lifecycle,
evaluation policy registration, verdict and local closeout. This first realization
supports only the selected Contracts scope and validates its own domain requirements.
It depends on Engine's public retained-root durability and repository identity
mechanisms, with no back-edge from Engine or Contracts into SDK. A later multi-owner
governance realization requires new authority; no registry of arbitrary validators
or services is selected here.

The Handbook establishment rule is proposed as owner-shipped immutable canonical
content `handbook.admission-establishment.standalone@1.0.0`, fingerprinted with an
explicit versioned domain separator. Its implementation embodies the approved
fixed constraints: positive initial-setup eligibility, exact single project/scope,
immutable configuration content, supported exact owner rule, honest explicit local
advisory instruction, recorded prior replacement basis, separate validation and
permission, confirmed cutover, conflict/refusal/replay, and no circular candidate
or operational-policy authority. A request cannot choose a different establishment
rule or relax those constraints. Changing the Handbook rule is outside this slice.

Contracts ships the exact canonical rule descriptor and validator for
`handbook.contracts.freshness-policy-admission@1.0.0`. Its descriptor declares one
candidate kind: the existing `EvidenceFreshnessPolicyV1` vocabulary/schema. The
configuration must contain a caller-supplied maximum-age ceiling in seconds,
an integer greater than zero and no greater than `JSON_SAFE_INTEGER_MAX`.
The request has no default ceiling: 86,400 is only an example/test value, and other
values within that range are supported. Exact source revision remains true and future
skew remains zero, matching the existing domain floor. Unknown fields, kinds,
owners, rules, scope encodings, exceptions or authority modes refuse. The finite
owner-rule binding set is exactly one entry, with no duplicate or omitted entry.
The rule declares no required authored governance revisions and no approval or
exception sources for this standalone initial configuration. That empty source
population is explicit and exact, not a general waiver of later admission closure.
The owner validates the descriptor/content against its shipped rule; accepting a
reference or schema-valid JSON alone is insufficient.

The supplied ceiling is part of immutable configuration content and the exact
request identity. Required typed fields prevent omission at construction; any
supported serialized request boundary also rejects omitted or invalid values,
without substituting 86,400 or another fallback. Changing an activated ceiling
requires a successor configuration and activation under later selected authority;
no parameter-edit or successor operation is provided in this packet.

This descriptor records requirements for a later candidate-admission consumer.
Establishment does not invoke `EvaluationPolicyRegistryV1::register_freshness` or
retrofit current evaluation APIs with a claimed Policy Admission Closure. It
does not relabel existing policies as compliant. Future candidate admission must
mechanically apply the declared criteria and finalize against an active exact
basis; that consumer and its races are separately authorized future work.

Use strict bounded typed Rust inputs and canonical JSON owner records. Proposed
`ProjectBindingV1` binds the Engine repository identity fingerprint, exact retained
root, and native registration identity; a filesystem path is a locator, not project
authority. Configuration identity includes owner, exact name/version and canonical
semantic-content digest; the request digest includes the complete configuration,
project/scope, instruction, replacement terms and expected initial state. Reusing
either configuration identity or idempotency key for different semantic content
refuses. The active relation and outcome have distinct content identities. Paths,
timestamps and advisory display names cannot select a configuration.

This selected JSON representation follows the interface distinction in the
[structured data format standard](../../structured-data-format-standard.md): the
caller supplies typed SDK operation parameters, and the owner writes configuration,
activation and recovery records as ordinary files. It preserves the explicit
Contracts JSON choice and reuses inward canonicalization/durability mechanisms.
This slice does not adopt the standard's other proposed defaults or migrate
existing files. Human-facing Markdown or GUI projections remain compatible future
bounded work derived from the exact owner source. No YAML importer, direct file
authoring interface, dedicated renderer or second editable master is added here.

The local instruction records exact instruction content/digest, local instruction
mode, optional asserted principal/agent/delegation claims, accepted scope and the
owner's validation facts. It explicitly labels those claims advisory. No credential,
authentication, verified delegation, Decision Record, evaluation Binding or policy
waiver is fabricated. No second human prompt is required at runtime where the
authorized explicit instruction already supplies this advisory basis.

Record at establishment the replacement-authority basis: a subsequent explicit
local advisory instruction may request a replacement for this same exact
project/scope, subject to the fixed floor, domain validation, exact predecessor,
and confirmed cutover. It cannot add scopes, change authority mode, change these
terms, or authorize itself through successor content. This basis is immutable
history even though no successor activation entry point is implemented here.

## Initial eligibility, storage and recovery

Positive native registration is a required part of the proposed new-project setup
composition. Under an explicit fresh-project instruction, it must exclusively
reserve a new `.handbook` root, establish and retain its Engine repository identity,
and issue an owner-native registration with a single unused initial-establishment
state. Ordinary setup `Init`, identity `Created`, absence of governance files,
`refresh/reset`, a profile, or an approver registry does not issue this authority.
The new composition cannot turn an existing root into a fresh project. A missing
identity is never allocated by governance establish/resolve/replay/recovery.

Before calling the effectful composition, the caller obtains and retains a proposed
exact project identity and complete request/resolution locator through a nonmutating
SDK preparation method. Reuse Engine's random identity allocation mechanism, with
a new sealed, single-use fresh-creation token and a separately copyable locator.
Allocation proves no initial eligibility or permission. The effectful new-project
path consumes that token, persists that exact proposed identity under exclusive
fresh-root registration, and never substitutes a newly generated identity on error.
The current `initialize_for_setup` regenerating path is not sufficient for this
new contract; add a narrow prepared-identity persistence seam rather than pretend
it accepts a caller's exact identity today. Restoring a locator after restart gives
resolution authority only; it cannot recreate the fresh-creation token or reissue
initialization for an old identity. This lets a caller resolve a lost first reply
without needing a project identity that existed only inside that reply.

The registration reservation and exact request digest are durable before any
activation publication. No standalone registration-success API is selected:
registration is an internal preparatory stage of this one outcome. A partially
created root can only resolve/recover its exact original setup request, never mint
another unused initial state. A competing request or an existing project adding
another scope refuses. If the initial instruction is invalid before reservation,
report a definite refusal with no activation; if reservation happened, retain it
and its exact terminal/refusal or uncertain state. Do not claim generic setup
finished if only native governance preparation occurred.

Proposed fixed layout: an immutable registration/consumption anchor at
`.handbook/admission-governance-project.v1.json`, and native owner storage under
`.handbook/admission-governance/v1/` for registration basis, configurations, exact
outcomes, active relation and bounded pending/committed transaction material.
The process/native lock is also outside `.handbook/state`. The anchor binds the
project, scope, request, original registration and terminal state; it is not a
mutable file that can simply be reset to unused. Its committed consumption must
close with the activation transaction. Preserve these files across ordinary
setup refresh and state reset, and test that actual route. Unknown/substituted
inventory, missing anchor, active relation without its exact committed outcome,
or missing referenced records refuses or remains recovery-unresolved.

Retain immutable rule bytes/descriptors, configuration bytes, accepted instruction
and replacement basis, validation facts and exact outcomes for the lifetime of
this local project registration. No pruning, deletion, relocation, import/export
or finite archival horizon is supported in this increment. Proposed limits are
1 MiB per request/record, 4,096 retained terminal requests, one configuration and
one scope; reaching a bound refuses before another effect. Untrusted symlinks,
reparse paths, hard-link aliases and substituted physical roots must fail closed
under the reused mechanisms and the new owner's complete bounded inventory proof.

Two remaining limits must be honest. This is local retained owner truth, not
cryptographic authentication against an administrator rewriting the entire store.
Total erasure of identity and every registration anchor cannot prove historical
freshness. The old exact project request must remain unavailable/refused; missing
state is never permission to reconstruct it. Deliberately creating a different
fresh project identity is a distinct project, not recovery of or an added scope
for P. This slice supports no such reset/conversion operation for P. If a consumer
requires recovery beyond the declared retained store, that is a new boundary to
return for human selection, not a promise supplied by advisory authority.

Serialize native state decisions under one process lock and one native exclusive
project-governance lock, using a documented fixed lock order with Engine identity
locks during initial registration. Resolve exact identity and registration again
under the locks. After validation, immediately compare the unused registration,
exact request and expected no-active predecessor at the commit boundary. Conflicting
writers cannot both activate. Readers take the same owner lock and validate/recover
the complete inventory before returning a usable active basis.

Use durable staged canonical content and an exact transaction intent, then a
durable commit decision and deterministic installation of the configuration,
outcome, consumed anchor and active relation. Success requires all installed facts
and the terminal transaction to close exactly after durability confirmation.
The active relation alone is not success. Recovery may finish the one already
decided exact transaction; it must never issue new permission or repeat activation
as an ordinary request. A fully verified pre-decision transaction with no installed
effect may be resolved as not committed; mixed/substituted evidence remains
unresolved. The Contracts repository is a mechanism precedent, not a transaction
format to copy without checking these registration/activation invariants.

## Selected consumer and exact replay

Select the transport-free typed Rust SDK route only, in a purpose-named SDK
`admission_governance` module composed by `HandbookSdkV1`. Proposed public methods:

| Method | Input and truthful output |
| --- | --- |
| `prepare_project_admission_governance` | Nonmutating allocation and strict request preparation for an explicit new-project instruction. Returns a sealed single-use creation request/token and copyable exact project/request locator to retain before sending. It grants no initial eligibility or activation authority. |
| `initialize_project_admission_governance` | Explicit fresh-project setup instruction, exact configuration/scope/rule binding, replacement basis and stable request key. Runs the bounded registration/establishment composition. Returns committed outcome, rule-specific refusal with available decisive provenance, or uncertain outcome with exact request/resolution identity. |
| `resolve_project_admission_governance` | Exact ProjectBinding and exact scope, optionally exact expected configuration/outcome binding. Returns one owner-verified active basis or named missing/conflicting/unsupported/unverifiable refusal. Never defaults, merges, selects latest, allocates identity or writes fresh authority. |
| `resolve_admission_governance_request` | Exact project registration and original request identity/digest. Returns the original committed/refused outcome, positively resolved not-committed terminal, or unresolved uncertainty. May finish already-decided owner recovery; does not resubmit activation. |

The SDK accepts untrusted caller DTOs, forwards them to the owner and projects
closed owner outcomes; it does not validate itself into authority. Provide strict
versioned typed request/result/refusal/uncertainty representations and canonical
serialization fixtures for this selected Rust boundary. Preserve validation,
accepted permission, committed effect, historical replay and current active basis
as distinct propositions in those shapes. Historical replay is not a fresh active
selection; exact resolution proves current active state independently.

An exact known terminal request replay returns its original outcome with a replay
delivery disposition and the same owner outcome identity. It must not create a new
registration, cutover or refusal decision. A key/content conflict refuses. After
losing a reply, callers use request resolution; the SDK must not convert an absent
reply, I/O error or timeout to not-committed or perform ordinary retry. If required
history cannot resolve, remain unavailable/uncertain. A later new attempt after
positively established non-commit is not a supported ordinary retry in this slice.
The retained registration/request may only follow its exact owner recovery rules;
otherwise a new action requires later selected authority. A consumed or uncertain
reservation never produces another fresh-creation token.

These are new typed SDK methods, not additions to the current closed 62-row
ordinary operation inventory, immutable capability witness, schema catalogs, or
seven fixed use cases. Do not advertise them through generic capability discovery
or claim CLI/Tauri/installed/external support. Unsupported successor activation,
scope addition, candidate admission, authority-term edits or migration are absent
entry points or explicit unsupported refusals, never placeholder success. If
implementation chooses generic discovery instead, return that expanded exact
catalog/schema/owner-admission closure for human approval first.

## Packet outline, acceptance and verification

There is one packet, **Packet 1 — establish and resolve one native governing
basis**, containing the full vertical behavior above. Its internal order is:
strict owner rule/configuration/authority types; positive project registration;
native durable activation/recovery; typed SDK projection; integrated proofs.
These are implementation steps, not independently dispatched packets. No later
packet is required to make this bounded establishment outcome meaningful. Rule
replacement and ordinary admission are recommendations for later selection, not
packet plans or delegated future work.

Choose a **Packet-1 human review boundary**, coincident with this narrow slice's
completion. It independently delivers an owner/SDK example of authoritative
establishment, exact resolution, refusal and replay. Do not plan an owner-only
public registration/activation packet and defer its review until consumer work.
If an earlier declared implementation outcome becomes independently meaningful,
verify/review/commit that bounded result and return to meta for recap-and-stop
before advancing. A causal verdict or satisfied prerequisite never supplies human
authorization across that boundary.

Packet 1 and slice acceptance are the following finite conditions. All product
checks below are **planned verification**, not tests run during authority preparation.

1. The P/C1 example works through the direct native owner and selected typed SDK;
   reopening resolves the same exact active project/scope/configuration/rule basis.
   Validation, advisory permission and confirmed commit are independently visible.
   A separate fresh-project case with another valid ceiling (for example 7,200
   seconds) persists, resolves and replays that supplied value and the same exact
   identities after reopening; it is not rewritten to 86,400. Configuration and
   request identities bind that value. Omitted values at a supported serialized
   request boundary, zero, out-of-range and invalidly typed values refuse before
   reservation; required Rust input fields expose no omission/default path.
2. The owner-shipped Handbook rule cannot be selected/changed by request content.
   The finite Contracts rule and proposed concrete constraints validate mechanically;
   wrong scope, owner, unknown/duplicate/missing binding, floor relaxation,
   unresolved rule or self-authorizing terms refuse with the violated condition.
3. Initial eligibility uses the positive native registration; existing roots,
   added scopes, missing identity/anchor, consumed initial state and missing
   established governance cannot bootstrap. Root/identity substitution refuses.
4. Immutable identity and idempotency conflicts, including changed ceilings,
   refuse; no supported operation edits an activated configuration. Terminal replay after
   reopen returns the original outcome and has no second activation. Refusal
   provenance never fills an unresolved project/configuration/rule with guesses.
5. Competing same/different-key processes yield one activation or exact replay;
   compare-before-commit rejects stale/consumed initial basis. No reader sees
   a partial installation as usable active governance.
6. Fault injection at reservation, identity retention, registration, staging,
   intent, commit decision, each installation and terminal publication proves
   definite refusal/non-commit only with positive evidence, exact recovery of
   committed outcomes, and unresolved corrupt/non-prefix/missing evidence.
   Include lost reply after complete commit and before consumer serialization;
   neither case produces ordinary retry or a second effect.
7. Registration, rule bytes, configuration, outcomes and replacement basis survive
   the actual SDK refresh/state-reset path. Corruption/deletion, bounds and unsafe
   filesystem entries fail closed; no empty-store reconstruction follows them.
8. The SDK preserves all owner identities, refusal/uncertainty distinctions and
   advisory provenance. Current generic catalogs do not advertise these methods;
   old evaluation policy registration does not claim new admission compliance.
   No Context Resolution generic authority call enters the selected route.
9. Unsupported later operations stay unavailable/refused. Record prior replacement
   authority now, while reporting successor activation and policy admission as
   unimplemented. No evaluation, eligibility, local/parent closeout or mutation
   success is inferred from establishment.

Planned focused checks: new Contracts owner/rule tests in
`crates/contracts/tests/admission_governance.rs`, recovery/process tests in
`crates/contracts/tests/admission_governance_repository.rs` with `test-support`,
and typed SDK composition tests in `crates/sdk/tests/admission_governance.rs`.
Run `cargo test -p handbook-contracts --features test-support --test admission_governance`
and the corresponding repository target; run
`cargo test -p handbook-sdk --test admission_governance`. Check the touched crate
compilation and formatting, applicable setup/reset regressions and existing
retained-durability tests. Recheck the closed catalog only to prove it is unchanged;
do not demand unrelated transport parity or a full Rust workspace replay. Test
targets and new public names above are proposed additions, not current commands
claimed to pass. Before any product-symbol edit use GitNexus impact as AGENTS
requires; before a product commit run change detection and one focused causal
review under the current policy.

The implementation task may resolve local encoding/module/test organization
within these choices just in time, using this authority and cited live sources.
It may not change owner, scope, concrete rule requirements, authority mode,
retention, consumer route or review boundary without returning to meta. Human
authorization of this next increment is recorded above; a clean causal review of
this reconciliation and satisfied current prerequisites are required before
implementation dispatch. Completion still returns to the Packet-1 human boundary.

## Exclusions and planning evidence limits

### Packet-1 implementation evidence — 2026-10-02

The separately named Contracts `project_admission_governance` owner and SDK
`admission_governance` module implement the four selected typed methods. Required
caller ceilings of 86,400 and 7,200 seconds retain their exact configuration,
request, active and outcome identities after reopening. The Engine seam consumes
a sealed prepared identity; a serialized locator restores resolution only.
The fixed shipped rule descriptors have canonical fingerprint fixtures, including
the exact existing freshness-policy schema and candidate ceiling criterion.

The append-only root anchor records the original reservation/request. Its
consumption is a separate exact append inside the activation transaction, closed
with the outcome and active relation by the terminal record; the anchor is never
reset to unused. All owner storage and its native lock are outside `.handbook/state`.
The one-shot route retains one original terminal request, within the selected
4,096-request ceiling, and refuses later creation attempts. Exact retained stage,
intent, decision and installation prefixes determine recovery. Missing, corrupt,
non-prefix or unsafe evidence remains unavailable; no replacement identity is
allocated by any resolution method.

Focused owner/rule, repository, process and SDK tests cover both caller ceilings,
serialized refusals before reservation, immutable/key conflicts, all 18 durable
fault/crash boundaries, competing creators and concurrent resolution, lost reply
and consumer serialization, exact historical replay versus current resolution,
physical-root substitution, hard-link and Windows reparse inventory, and bounds.
The actual SDK refresh/reset preserves every retained governance byte. Engine
identity/durability and SDK setup/reset regressions pass; touched crates compile
and touched sources format cleanly. The unchanged generic catalog proof remains
`62 / 35 / 180 / 7`. Unix-only link/root cases are present but were not executed
on this Windows host. GitNexus impact/reindex tooling remains degraded/unavailable;
source callers and the bounded diff supply the fallback inspection, not graph
conformance evidence. The compact task result carries the controlling causal
verdict and actual commit rather than a durable reviewer record.

Establishment records advisory provenance and prior replacement terms. Ordinary
policy admission, successor activation, added scopes, evaluation, work progression
and generic setup success remain unavailable or outside this outcome.

No product edits are part of this preparation. No adjacent slice, whole-roadmap
decomposition, durable orchestration state or reviewer archive is selected. The
first candidate is limited to local new-project establishment and exact owner/SDK
resolution. Existing-project enrollment, multi-scope inheritance/merge, successor
activation, active-at-policy-decision closure/races, ordinary policy admission,
qualification, evaluation, present reliance and work/action effects remain unfinished.
No seventh authored family, universal IAM/login, additional approval infrastructure,
V1 migration/dual write, API per canonical object or universal transport parity follows.

Preserve existing Dock's native Rust `jsonschema`, offline Draft 2020-12 exact
inline JSON/local-ref closure; no general hostile-code sandbox or tooling roster
is selected. HCM-4.6 grounding remains implemented with external trigger/adopter
and reservations separate. HCM-5.2 local closeout remains historical Contracts
truth with no parent eligibility. Future work must preserve governing assessment
selection before collection or no-collection commitment, exact-policy/scenario
empty/N/A meanings, and historical Engine Binding verification only for consumed
roles binding one; an upstream refusal fabricates no Binding. HCM-6.1–6.5 and
deferred posture/work/grounding/Dock choices remain separate compatibility boundaries.

Planning evidence is source/test/export inspection only; no product runtime suite
was run. GitNexus MCP was unavailable. CLI status failed hashing a cached analyzer
input; local metadata records stale baseline `a35297d` from 2026-08-25. The concept
query returned degraded empty results with missing graph tables/foreign storage
warnings. Graph output is not conformance evidence. The advisory KEEP consultation
and [research limitations](../../../../research/hcm-coverage-landing-2026-10-01/README.md)
informed the boundary; current sources and approved owning semantics control it.
Local Markdown-link checks, documentation diff checks, pre-commit change-detection
attempt and the controlling causal verdict are reported with the committed plan.
`AUTHORITY_READY` means reviewed/committed planning only. Return to meta to verify
the clean review and prerequisites, then dispatch the already-authorized Packet 1.
That authorization does not cross the completion recap-and-stop boundary or permit
successor planning or execution.
