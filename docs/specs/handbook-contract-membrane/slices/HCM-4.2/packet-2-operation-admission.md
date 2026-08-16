# HCM-4.2 Packet 2 — operation definitions and owner admission

This checkpoint records the decisions selected under the slice's delegated
Packet-2 preparation boundary. It controls only Packet 2. The slice README,
current program sources, and live source/tests remain controlling where this
checkpoint is silent.

## Current decision

Packet 2 uses a default-deny two-step model:

1. the SDK structurally admits a closed, exact, fingerprinted operation
   definition and its generic request/response shapes; then
2. the semantic owner independently admits or refuses that definition before
   it may become current or discoverable.

The SDK inventory and an adapter cannot substitute for step 2. Engine, Flow,
and pipeline each expose an owner-native claim function whose successful token
has private fields and can be constructed only in that owner crate. The claim
contains the complete exact definition binding plus the canonical claim fields
the owner compares: owner crate/version/source, definition/request/result/outcome
schema bindings, mutability, exact idempotency fingerprint (including the key
pointer and expected basis), declared-write-set fingerprint, and
required-capability fingerprint. The SDK can project a definition into that
claim and retain the returned token, but it cannot construct a token or turn
its own structural witness into owner admission. Each owner keeps an
owner-native exact-claim registry and compares every claim field by equality
before constructing its private token. All three current registries are empty
because no row has complete closure, so no SDK-authored candidate definition
can reach a token. Such a candidate is a definition mismatch; the inventory's
`incomplete_closure` reason records why the owner has selected no expected
definition, rather than blessing a partial candidate as the expected one.

Current `sdk_composition` rows have no independent semantic owner or selected
multi-owner quorum rule, so their exact current admission is permanent refusal
`independent_owner_absent`; this packet does not invent a composite owner.
Phase-5-deferred rows similarly refuse `owner_deferred`. Only engine, Flow, or
pipeline may return an owner token for their rows, and their current claim
functions still refuse until exact closure exists. This creates no owner-to-SDK
dependency or adapter hook.

No current row has complete operation closure. The exact request and result
schema for each operation, its owner admission, receipts for realized writes,
and a proven current transport are all required before discovery. Therefore
all 62 rows have current transports `none` and discovery `omit`; this is an
honest current result, not a transport failure.

## Exact public model

- Each operation reference is `<stable-operation-id>@<full-SemVer>`. Packet 2
  fixes every current inventory reference at `1.0.0`; ranges, `latest`, CLI
  names, examples, and adapter names are never selectors.
- `OperationDefinition` binds its definition schema, exact operation ref,
  owner crate/version/source, mechanically derived request and result schema
  bindings, the three generic outcome-schema bindings, required capability
  bindings, mutability, idempotency contract, declared write set, declared
  transport targets, nullable deprecation binding, and a canonical instance
  fingerprint over every preceding field.
- Per-operation request/result schema identities are derived by replacing dots
  in the operation ID with hyphens under
  `handbook.operation.<id>-{request|result}@<operation-version>`.
- The ordinary request has distinct wire and admitted views. The closed wire
  envelope binds the generic request schema, exact selected definition, exact
  body schema, raw operation body, required-nullable bounded request ID, and
  request fingerprint. `OperationRequestV1::parse_and_admit` first establishes
  bounded canonical JSON and an exact selected definition, then requires an
  explicit structural schema validator, validates the raw body against that
  definition's request schema, and only then extracts the definition-pinned
  `/body/idempotency_key`. The admitted `OperationRequestV1` stores a sanitized
  body with that member removed, a private `RawIdempotencyKey`, and its scoped
  fingerprint. Debug and public body access expose only the sanitized body;
  owner borrowing uses only the sealed Packet-0 bridge. Request serialization
  is the deliberate wire operation and reconstructs the raw member; response,
  receipt, diagnostic, replay, manifest, log, and public-state types have no
  such serializer or accessor. The admitted request deliberately has no
  generic `Deserialize` implementation that could bypass the selected
  definition. Its sealed key-consumer method is the only route from the
  retained request to Packet 0's owner key bridge.
- The scoped key fingerprint is
  `sha256("handbook.operation-idempotency-key-v1\0" || operation_ref_utf8 ||
  "\0/body/idempotency_key\0" || raw_key_utf8)`. The request fingerprint is
  SHA-256 over LF-free canonical JSON of exactly `{schema, operation,
  body_schema, sanitized_body, idempotency_key_fingerprint}`, where the last
  field is required nullable. It excludes `request_id`, the raw key, and
  `request_fingerprint`. Read-only requests require null; mutation requests
  require the scoped fingerprint.
- `OperationResponseV1` binds the generic response schema, request correlation,
  selected definition, result schema, Packet-0 terminal fields, idempotency
  state, realized write receipts, diagnostics/actions, a semantic result
  fingerprint, and response fingerprint. Construction and
  `parse_and_admit` require the selected definition and structural schema
  validator, validate result data and realized receipt conditions against the
  pinned schemas, and re-run class/idempotency/write-set rules. The admitted
  response has no definition-free `Deserialize`. It preserves exactly
  `ok | blocked | refused | error`; `data` is semantically present only for
  `ok` (the selected result may itself be JSON null), and exactly one non-ok
  terminal array is populated.
- The semantic result fingerprint is SHA-256 over LF-free canonical JSON of
  exactly `{operation, result_schema, status, data, blockers, refusals, errors,
  receipts}`. It excludes request correlation, response schema, idempotency
  replay metadata, diagnostics/actions, and both derived fingerprints. It is
  therefore stable for an original success and its byte-identical replay.
- Its response fingerprint is SHA-256 over LF-free canonical JSON of every
  emitted response field except only `response_fingerprint`; it therefore
  includes `request_id` and is correlation-sensitive. No raw key or unscoped
  key digest is in the preimage.
- `OperationAdmissionRefusalV1` is a distinct pre-owner envelope with exactly
  `status=refused`, `data=null`, `blockers=[]`, one `refusals` item, and
  `errors=[]`. `request_id` is required nullable and normalizes to null when
  absent or invalid. `attempted_operation` is required nullable and is null
  when an exact full-SemVer operation ref was absent or unparseable;
  `attempted_definition` is required nullable and is null unless a complete
  exact binding parsed; `request_fingerprint` is required nullable and is null
  unless the bounded sanitized preimage above can be established. It contains
  no result schema/data, idempotency state, write receipt, owner result, or raw
  key. Its response fingerprint covers every emitted field except only itself,
  including the normalized request ID and all three nullable attempt fields.
- The checked-in Draft 2020-12 tree adds roots for those four public types plus
  the three generic blocker/refusal/error Problem identities. Packet 1's
  deterministic exact-byte manifest rules continue unchanged.

## Mutability, idempotency, and receipts

The closed classifications are:

- `R0`: `read_only` / `safe`; the declared write set and every successful or
  unsuccessful realized receipt set are empty.
- `A1`: `append_only` / `idempotency_key_required`; an admitted definition
  pins `/body/idempotency_key` and a non-empty exact declared write set. `ok`
  returns exactly one realized receipt per declared item; every non-`ok`
  outcome returns none.
- `C1`: `compare_and_write` / `compare_and_write_required`; an admitted
  definition additionally pins a non-empty expected-basis pointer set and one
  atomic write group. `ok` returns exactly the declared atomic write set;
  every non-`ok` outcome returns none. Replay returns the original receipt set
  and never claims new writes.

Each declared write item is exactly `{record_kind, authority_class,
atomic_group, condition_schema}`. Tokens use the Packet-0 token grammar; the
condition schema is an exact binding. Items are unique by
`(atomic_group, record_kind, authority_class)`, sorted by that tuple followed
by `(condition_schema.ref, condition_schema.fingerprint)`, and every item in
one mutation definition uses the same non-empty `atomic_group`. A realized
Packet-0 `WriteReceipt` matches an item by the same three-token tuple; its
runtime `record` binding and condition remain realized owner truth, while the
condition must verify against the item's exact condition schema. Receipt
collections are sorted by the declared-item order and then runtime record ref
and fingerprint; duplicates refuse. Thus `ok` requires set equality under the
three-token key, and any non-`ok` receipt refuses.

For C1, expected-basis entries are canonical RFC 6901 pointers strictly below
`/body`, exclude the key pointer, and are non-empty, unique, and byte-sorted.
They identify the request values the semantic owner must compare atomically;
they do not let the SDK perform the comparison. A replayed response must match
the original response's operation/result binding, semantic data, and complete
receipt bytes, while its `original_result_fingerprint` equals the original
semantic result fingerprint; it never reports a new write.

An established idempotency state's request fingerprint must equal the outer
response request fingerprint. A replay is rejected during construction and
definition-bound parsing unless its `original_result_fingerprint` already
equals the recomputed semantic result fingerprint; replay comparison also
requires the original to be non-replayed and both states to carry identical
request and scoped-key fingerprints.

Packet 2 does not invent the missing operation-specific write sets or expected
bases. A mutation definition without those exact fields is structurally
refused and remains omitted.

## Reconciled current inventory

`live_precursor` means only that the cited Rust symbol still exists. `absent`
means no exact ordinary-operation seam exists even where lower-level public
domain helpers do. `phase5_deferred` means the semantic owner is absent by
current authority. Owner label `sdk_composition` is composition placement, not
semantic authority.

| Proof | Operation | Owner | Availability | Class | Current admission |
| --- | --- | --- | --- | --- | --- |
| M62-001 | `capabilities.describe` | sdk_composition | absent | R0 | independent_owner_absent |
| M62-002 | `profile.list` | engine | absent | R0 | incomplete_closure |
| M62-003 | `profile.resolve` | engine | absent | R0 | incomplete_closure |
| M62-004 | `schema.list` | engine | absent | R0 | incomplete_closure |
| M62-005 | `schema.read` | engine | absent | R0 | incomplete_closure |
| M62-006 | `vocabulary.read` | engine | absent | R0 | incomplete_closure |
| M62-007 | `resolution.stack.read` | engine | absent | R0 | incomplete_closure |
| M62-008 | `projection.definition.read` | flow | absent | R0 | incomplete_closure |
| M62-009 | `artifact.kind.list` | engine | live_precursor | R0 | incomplete_closure |
| M62-010 | `artifact.instance.list` | engine | live_precursor | R0 | incomplete_closure |
| M62-011 | `artifact.read` | engine | live_precursor | R0 | incomplete_closure |
| M62-012 | `artifact.validate` | engine | live_precursor | R0 | incomplete_closure |
| M62-013 | `artifact.render` | engine | absent | R0 | incomplete_closure |
| M62-014 | `intake.definition.read` | engine | live_precursor | R0 | incomplete_closure |
| M62-015 | `intake.coverage.evaluate` | engine | live_precursor | R0 | incomplete_closure |
| M62-016 | `intake.record.append` | engine | live_precursor | A1 | incomplete_closure |
| M62-017 | `record.list` | engine | absent | R0 | incomplete_closure |
| M62-018 | `record.read` | engine | absent | R0 | incomplete_closure |
| M62-019 | `artifact.candidate.validate` | engine | live_precursor | R0 | incomplete_closure |
| M62-020 | `artifact.candidate.append` | engine | live_precursor | A1 | incomplete_closure |
| M62-021 | `artifact.approval.append` | engine | absent | A1 | incomplete_closure |
| M62-022 | `artifact.candidate.promote` | engine | live_precursor | C1 | incomplete_closure |
| M62-023 | `posture.resolve` | engine | absent | R0 | incomplete_closure |
| M62-024 | `posture.recommendation.evaluate` | engine | absent | R0 | incomplete_closure |
| M62-025 | `posture.recommendation.append` | engine | absent | A1 | incomplete_closure |
| M62-026 | `posture.recommendation.acknowledge` | engine | absent | A1 | incomplete_closure |
| M62-027 | `posture.transition.apply` | engine | live_precursor | C1 | incomplete_closure |
| M62-028 | `projection.create` | flow | absent | R0 | incomplete_closure |
| M62-029 | `resolution.escalation.request.append` | engine | live_precursor | A1 | incomplete_closure |
| M62-030 | `resolution.escalation.disposition.append` | engine | live_precursor | A1 | incomplete_closure |
| M62-031 | `memory.promotion.request.append` | engine | live_precursor | A1 | incomplete_closure |
| M62-032 | `memory.promotion.disposition.append` | engine | live_precursor | C1 | incomplete_closure |
| M62-033 | `snapshot.capture` | sdk_composition | absent | A1 | independent_owner_absent |
| M62-034 | `snapshot.read` | engine | absent | R0 | incomplete_closure |
| M62-035 | `snapshot.delta` | engine | absent | R0 | incomplete_closure |
| M62-036 | `snapshot.project` | flow | absent | R0 | incomplete_closure |
| M62-037 | `snapshot.verify_current` | sdk_composition | absent | R0 | independent_owner_absent |
| M62-038 | `snapshot.resolve_applicable` | sdk_composition | absent | R0 | independent_owner_absent |
| M62-039 | `repository.setup.plan` | sdk_composition | absent | R0 | independent_owner_absent |
| M62-040 | `repository.setup.apply` | sdk_composition | live_precursor | C1 | independent_owner_absent |
| M62-041 | `repository.doctor` | sdk_composition | live_precursor | R0 | independent_owner_absent |
| M62-042 | `flow.resolve` | flow | live_precursor | R0 | incomplete_closure |
| M62-043 | `pipeline.catalog.list` | pipeline | live_precursor | R0 | incomplete_closure |
| M62-044 | `pipeline.catalog.read` | pipeline | live_precursor | R0 | incomplete_closure |
| M62-045 | `pipeline.route.resolve` | pipeline | live_precursor | R0 | semantic_mismatch |
| M62-046 | `pipeline.compile` | pipeline | live_precursor | R0 | incomplete_closure |
| M62-047 | `pipeline.capture.plan` | pipeline | live_precursor | R0 | incomplete_closure |
| M62-048 | `pipeline.capture.apply` | pipeline | live_precursor | C1 | incomplete_closure |
| M62-049 | `pipeline.handoff.emit` | pipeline | live_precursor | A1 | incomplete_closure |
| M62-050 | `pipeline.state.apply` | pipeline | live_precursor | C1 | incomplete_closure |
| M62-051 | `contract.definition.list` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-052 | `contract.definition.read` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-053 | `contract.definition.append` | phase5 | phase5_deferred | A1 | owner_deferred |
| M62-054 | `contract.lifecycle.transition` | phase5 | phase5_deferred | C1 | owner_deferred |
| M62-055 | `contract.evidence.list` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-056 | `contract.evidence.read` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-057 | `contract.evidence.append` | phase5 | phase5_deferred | A1 | owner_deferred |
| M62-058 | `contract.verdict.evaluate` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-059 | `contract.gate.evaluate` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-060 | `dock.manifest.list` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-061 | `dock.manifest.read` | phase5 | phase5_deferred | R0 | owner_deferred |
| M62-062 | `dock.run` | phase5 | phase5_deferred | A1 | owner_deferred |

The set is exactly 62 unique rows with contiguous proof ordinals, 50
HCM-4-owned rows, and 12 Phase-5-deferred rows. Current source still resolves
all 26 live-precursor rows and their 28 cited live symbols; the separate
`repository.setup.plan` symbol remains test-only and therefore absent.
`pipeline.route.resolve` still calls
`persist_route_basis`; it remains the sole explicit semantic mismatch and may
not be admitted as read-only without a future matching owner adapter or a new
authorized operation definition.

The focused proof freezes the canonical full-tuple inventory digest as
`sha256:ec8eaa6447efd2473340d1de7225acaa8e81e04d6ce9184258b51fdd6c4380a5`;
changes to any proof ordinal, exact ref, owner, availability, class, or current
admission reason fail independently of the implementation's count checks.
Definition-set validation consumes actual `OperationDefinition` values (not
only references), rechecks complete identity coverage and owner/class binding,
and typed deserialization separately rejects stale instance fingerprints.

## Focused causal decision review

Subject: the default-deny split between SDK structural definitions and
independent owner admission, including the current 62-row classifications.

First-round verdict: **findings**, remediated above. The reviewer required an
unforgeable owner-native token, an exact wire/sanitized raw-key split and
fingerprint domain, full admission-refusal closure, a mechanical declared-write
to receipt comparison, and the corrected live-symbol count. No product edit
began before these corrections.

## Focused causal implementation review

Initial verdict: **findings**. The bounded remediation removed
definition-free request/response deserialization, bound schema validation and
receipt/replay checks to the selected definition, replaced partial owner claim
checks with owner-native exact registries, froze the full inventory digest and
actual-definition coverage, and completed the non-secret typed surface plus
sealed key bridge.

Single causal re-review verdict: **proceed**. No Required or Critical finding
remained, and the reviewer confirmed that the remediation neither transferred
owner authority nor expanded into a later packet.
