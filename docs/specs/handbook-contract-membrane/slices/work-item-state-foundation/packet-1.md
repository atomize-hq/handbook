# Packet 1: register and resolve native open work

Status: **prepared for human plan authorization; unimplemented**, 2026-10-08.
This is the sole packet of [work-item-state-foundation](README.md). The slice
owns the outcome, exclusions and eight acceptance conditions; this document
details only its next implementation increment.

## Observable behavior and review boundary

Given one positively registered local project whose original initial authority
remains current over its original positive deferred basis, explicit prior-approved
work-store reach, and a valid exact item
request, a normal SDK caller creates one owner-confirmed `open` item. It retains
the original request locator before execution. Current resolution after restart
returns that exact item/snapshot; original-request resolution returns the same
registration outcome. A registration receipt proves creation, not work completion.

Creation is supported **before local-check successor adoption or D→G**. The
existing initial witness rejects both successor footprints and a changed admission
basis. A project which has already completed local-check setup cannot register
through this route. Do not expand or reinterpret its successor terms to repair
that limitation. Existing item/history reads remain separate owner-state reads
after such changes; they confer no new creation permission.

For example, before local-check adoption on P, the item key
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

| Inspected source at `2ce0871` | What may be reused; what it does not supply |
| --- | --- |
| [Engine initial operating authority](../../../../../crates/engine/src/local_authority.rs), `LocalOperatingTermsV1`, `OperatingGuard`, `LocalCurrentAuthorityWitnessV1::with_current` | Exact initial local-advisory terms, current native locks, root/Charter history and bounded OwnerWrite reach. The held witness is crate-private; source reuse inside Engine is possible. Generic `OwnerWrite` and paths do not grant local/parent completion or reinterpret `LocalCheckTermsV1`. |
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
another state machine or introduce a kind. Establish this descriptor through
native fixed conformance, retaining its exact content/binding with each creation;
it assigns no governance and needs no separate kind registry.

Creation needs the actual prior human-approved initial terms and current native
authority over the original deferred basis, before local-check adoption or D→G,
not a new prompt merely because the work owner performs a check. The initial
witness's successor-footprint and admission-currentness refusals remain controlling.
The exact approved reach must include `.handbook/work-state/v1/` and its required
input/lock paths, and the supported bounded registration effect. Existing terms
that omit that reach refuse. No term replacement, fallback, delegation expansion
or local-check grant interpretation is implemented. Actual runtime terms must be
supplied through the existing setup path; plan approval supplies none.

Use the Engine-native held current-authority mechanism while the work owner
validates and commits. A narrow internal consuming callback/admission may be added
for this registration when needed; it must bind the exact project/root, item/key,
composition, request digest, declared write targets and current authority basis.
It is lifetime-sealed, cannot be serialized/reconstructed by SDK, and permits no
other work operation. Hold and validate the existing initialization/operating
guards and declared authority dependencies through creation, then the work owner's
native lock in one consistent order. Do not acquire operating locks from inside
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
version domain and item key; display names and request delivery do not allocate
identity. A request binds its item, supported explicit composition, bounded label
and work question, expected initial absence and original authority locator/basis.
Use the established 4,096-byte bound for each descriptive/provenance string and
1 MiB for a complete record/request. No arbitrary numeric fields, supplied paths,
external references or open-ended user JSON are needed in this first shape.

Store retained owner truth under `.handbook/work-state/v1/`, including its native
lock, exact immutable item-registration record and request-addressed transaction
history. The initial snapshot is version 1, `open`; it binds item/project,
composition descriptor and admitted request. Exact item/request content conflicts
refuse. A separate latest-head selector is unnecessary when the sole supported
item state is its immutable initial state. Each item's exact committed registration
is its current state source; this does not advertise edits or successor selection.

No unbounded list operation is exposed. Validate one declared owner inventory
within explicit Engine-compatible file/count/byte bounds; refuse overflow or
unknown/substituted/partial entries rather than truncate or infer absence. Codec
decoding rejects duplicate/unknown keys and trailing documents before typed
validation. Canonical JSON has no trailing newline; semantic fingerprints bind
domain-separated canonical values and byte fingerprints bind retained exact bytes.
No existing owner fingerprint or framing is rewritten.

Select a decided-prefix transaction using the existing durable primitives:
retain the exact request and intent before effect installation, prove expected
absence, decide one exact snapshot/receipt, install it without replacement, then
close a matching terminal. Only a complete owner-verified terminal plus its exact
installed snapshot permits a committed response or current item fact. A receipt
names **registration**, not `local_closeout`, and separately binds the exact native
permission and initial state. No in-place replacement or new-key retry may occupy
the item after an uncertain request. File layout details may be settled in scope;
the truthful boundaries and same-identity recovery may not.

Original request resolution is history/replay and creates no new effect. Recovery
may finish the same decided registration or positively resolve noncommit under
the retained original request and supported recovery rule; any remaining permission
needed for a not-yet-applied effect must still be current. A possible application
without exact terminal proof stays uncertain until that recovery resolves it.
Lost acknowledgement is not noncommit. Current resolution must never repair a
missing project identity, owner lock or committed record by allocating or treating
the store as new. Preserve original creation authority snapshots/history so later
revocation or reset can block new creation without relabeling an old receipt as
present permission. Runtime reset may destroy operating state, not this work store;
that loss must not turn existing items into missing/new work.

## Typed SDK responses and next steps

Expose typed prepare/register, resolve-current-item, resolve-original-request,
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
existing SDK guidance. Other owners may receive only the narrow held-admission
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
insufficient/revoked/substituted authority; local-check successor-footprint and
D→G refusal, with existing item/current/history reads still preserving original
work truth after those changes; actual operating guards held across
creation; exact missing/corrupt history refusal; concurrent creators/readers;
lost replies and interrupted prefixes before/after decision, install and terminal;
same-identity recovery without repeat effect; current open truth after refresh/reset
with new creation refused when required operating state is lost; root/lock
substitution, bounds/codec refusals and unchanged legacy result behavior.

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
