# Register and recover native open work

The [`work_state` SDK surface](../src/work_state.rs) registers ordinary,
planning-output and requirements-input leaves through one typed operation in an
already established local-check project. Supply actual
human-approved work terms under `WORK_REGISTRATION_RULE_V1`; implementation
approval and existing local-check permission supply no runtime work grant.
The native Engine owner validates exact human-approved composition and parameter
scope and retains original project, Charter, governance, principal/agent, effect,
owner reach, reservations and provenance. Decoder recognition never expands scope.
Local advisory attribution supplies no independent authentication.

Both read and write reach use `WORK_STATE_FULL_OWNER_REACH_V1`. This fixed
owner-supported selector covers the retained owner tree and its exact bootstrap,
anchor and staging metadata; callers need no private file layout. A raw subtree
pattern omits required owner targets and refuses.

```rust,ignore
let prepared = sdk.prepare_work_registration_terms(
    &initial, &check, supplied_work_terms, WorkTermsTransitionV1::Initial)?;
save_new(prepared.locator().canonical_json_bytes()?); // Before invocation.
let activation = sdk.activate_work_registration_terms(prepared);
// Inspect disposition, outcome and refusal before preparing work.
let grant = activation.outcome.ok_or("grant was not established")?.locator;
let prepared = sdk.prepare_work_registration(&initial, &grant, supplied_item)?;
let original = prepared.locator().clone();
save_new(original.canonical_json_bytes()?); // Before invocation.
let registration = sdk.register_work_item(prepared);
let current = sdk.resolve_current_work_item(&grant, &original.input.item_key)?;
let history = sdk.resolve_work_registration_request(&original);
let recovery = sdk.recover_work_registration_request(&original);
let support = sdk.inspect_work_completion_support(&original)?;
```

Use separate stable item and request keys. Choose `WorkCompositionV1::ordinary_leaf()`,
`planning_output(question)` or `requirements_input(definition)`. Planning declares
one bounded required-output question before issuance; it contains no guessed future
Contract locator. Requirements-input declares an exact separately defined Contracts
definition, with no assertion of availability, qualification, governance, coverage
or assessment. Each choice establishes a same-project `open` leaf. Optional
`display_level` values are finite approved ASCII tokens and add no hierarchy;
omission is retained as omission. Kind bindings, unknown role fields, relations,
overridden descriptors and requested completed/closed states refuse.

`WorkRegistrationScopeV1::explicit(clauses)` requires a nonempty finite set of
`WorkCompositionClauseV1` values, at most one per exact supported descriptor.
Each clause supplies positive label/work-question maxima within 4,096 bytes,
explicit omission permission and/or exact display tokens, and the matching
`WorkRolePermissionV1`. Planning supplies its output-question maximum;
requirements-input supplies a nonempty finite set of exact permitted definitions.
No wildcard, latest selector, all-future permission or maximum is implied.
These illustrative values are caller choices, not defaults:

```rust,ignore
let definition = work_requirements_definition_from_full_local_v3(&owner_definition);
let clause = WorkCompositionClauseV1 {
    descriptor: WorkDescriptorV1::requirements_input(),
    label_max_bytes: 128,
    work_question_max_bytes: 256,
    allow_omitted_display_level: false,
    display_levels: vec!["implementation".into()],
    role_permission: WorkRolePermissionV1::RequirementsInput {
        definitions: vec![definition.clone()],
    },
};
supplied_work_terms.scope = WorkRegistrationScopeV1::explicit(vec![clause]);
// Obtain explicit human approval of the exact terms and bind its approved content.
supplied_item.composition = WorkCompositionV1::requirements_input(definition);
supplied_item.display_level = Some("implementation".into());
```

The helpers `work_requirements_definition_from_intrinsic_v1`,
`work_requirements_definition_from_local_v2`, and
`work_requirements_definition_from_full_local_v3` copy owner-produced exact
reference, family/schema and full definition fingerprint. V2/V3 use `binding()`;
their embedded intrinsic fingerprint identifies different content. Engine checks
declaration shape and permission without resolving Contracts truth. A well-shaped
permitted declaration can be registered when source truth is unavailable.
The closed JSON codecs reject duplicate keys, unknown fields and trailing data.
ASCII item/request/display tokens are at most 128 bytes; descriptive and provenance strings
are at most 4,096 bytes; a complete record/request is at most 1 MiB. The retained
inventory supports at most 32 transactions and 128 MiB in total, including
candidates and noncommit history. Capacity refusal truncates or replaces nothing;
existing exact recovery and closure of accepted original transactions remain
available even at zero free slots. Initial terms, every scope revision/D→G,
revocation, item and candidate/noncommit history consume the same global budget;
slots are never reused.
The owner retains a positive expected inventory before accepting each transaction.
A missing transaction directory or damaged inventory proof therefore yields an
integrity refusal or uncertain recovery, never invented item absence or permission
to replace the original request. An interrupted owner bootstrap also stays
uncertain when its original establishment proof is incomplete; recovery does not
recreate locks or allocate a replacement project identity.
The identity scheme depends on the native project and item key independently
of codec/release/composition versions; retained descriptor bytes keep their
original meaning during current/history/recovery reads.

| Native response | Meaning and next step |
| --- | --- |
| `Committed` / `Replayed` with receipt | One exact item is registered `open`; replay creates nothing. Save its original locator and inspect current state separately. |
| `Refused` or preparation error | Named input, authority, currentness or integrity condition failed. Changed content needs a new key under valid authority and cannot overwrite an existing item. |
| `NonCommitted` | The owner positively proved no terminal commit. Preserve this history; a fresh effect still requires current permission. Administrative suspension lifts only if the unchanged predecessor remains current and unrevoked. |
| `Uncertain` | Retain the original locator and recover it. Do not retry under another key or claim absent/open success. |
| `Pending` | Only retained candidates/reservations exist. A snapshot without its terminal is pending. Accepted authority change/revocation suspends predecessor use; the candidate supplies no permission. Recover the original request. |
| `Unaccepted` | A complete positive inventory proves no accepted original request. This supplies no creation permission or noncommit replay. |
| Current `Open` | Exact immutable item truth; it supplies no operating permission or eligibility. |
| Current `Absent` | Positive owner inventory proves this item absent. Missing/corrupt owner proof refuses instead. |
| Completion `unsupported` / `not_assessed` | No eligibility, disposition, Determination or close effect is formed. A registration receipt proves creation only. |

Current/new-effect preparation observes Contracts D/G applicability outside
native consuming locks; Engine validates the exact basis while holding its
original live guards through durable terminal publication. For an explicit
same-basis scope successor use `WorkTermsTransitionV1::ScopeChange(&latest_grant)`
with fresh original-human approval. Only composition/parameter scope changes;
actor/agent, original lineage, effect, reach, assurance, reservations, revocation
principal and retention remain fixed. Even narrowing requires an immutable
successor. Acceptance suspends predecessor use and rejects competing administration.
Commit establishes one usable head; historical grants are never unioned.

D to G makes old D preparations stale. Use
`WorkTermsTransitionV1::EstablishFromDeferred(&latest_grant)` with fresh approval
of the unchanged exact scope/fixed fields, including after multiple revisions.
Change scope separately after re-establishment. Committed revocation has no
reinstatement route or fallback. Old requests cannot finish under a successor/G
grant, even if its scope overlaps. Recover the saved original locator to prove
commit/noncommit or retain uncertainty. A retained V1 grant retains exactly its
singleton ordinary scope with omitted display; the same scope-change lifecycle
can name it as predecessor. No old item, request or receipt is rewritten.

Revocation uses `prepare_work_registration_revocation` and
`revoke_work_registration_terms` with the declared principal and actual supplied
source/instruction. Save the revocation locator first. Resolve/recover activation
and revocation through their original request methods. Committed history remains
history and never restores revoked permission.

Item/current/history reads and exact historical recovery retain their own
dependencies outside runtime-reset state. They do not need live operating terms
to verify past truth. A supported runtime reset preserves registered items and
their identities, while fresh writes refuse when required live authority is lost.
Recovery preserves pending reservations where terminal installation cannot be
proven; missing evidence is never permission to rebuild state or allocate identity.

The [`work_state` example](../examples/work_state.rs) accepts supplied files and
prints structured native responses. It can be copied into a standalone Cargo
program with normal `handbook-sdk`, `serde` and `serde_json` dependencies. It needs
no owner-private paths or test feature. Commands `activate`, `change-scope`,
`establish-from-deferred`, `register` and `revoke`
save complete locators using create-new before invocation. `current`, `history`,
`recover`, `terms-history`, `recover-terms`, `revoke-history`, `recover-revoke` and
`completion` reopen and use those exact locators. The example is a Rust consumer;
it adds no installed Handbook CLI route.

The example uses an 8 MiB worker stack for the inherited initial/check native
path in debug builds. Its ordinary dependency proof and the native interruption
tests were exercised on Windows; Unix remains unexecuted.
