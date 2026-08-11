# HCM-4.2 — shared DTO and JSON Schema contract

**Phase:** `HCM-4`
**Packet:** `HCM-4.2-PLANNING-CONTROL-PACK`
**Status:** corrective planning primary; authoritative review/closeout status
is carried only by additive evidence; runtime implementation not authorized
**Selector:** `decision/20260811T025558Z--hcm-4-2-causal-remediation-selector.md`
**Source/impact inventories:** historical
`research/20260811T001900Z--source-impact-inventory.md`; authoritative
corrective 62-row matrix
`research/20260811T031843Z--corrective-owner-admission-matrix.md`

## Objective

Freeze the future public, transport-neutral contract shared by typed SDK
consumers: bounded request/result/problem/blocker/refusal/error/reference DTOs;
exact operation definitions; schema IDs, full-SemVer versions, and
fingerprints; checked-in generated Draft 2020-12 schemas; deterministic
serialization; immutable capability/bootstrap discovery; and exact catalog
binding. The package makes HCM-4.1's existing typed Rust entrypoints
implementation-ready for later discovery without turning them into a CLI,
Tauri, skill, Phase-5, or Phase-6 surface in this slice.

## Current truth

At corrective inspection commit `ea2be46856172e503e7f7668d2dbaac9d26d1d39`,
tree `9dbad2c418a331d5b2a0c266c55280d81b676aca`, HCM-4.1's
`HandbookSdkV1::apply_posture_transition` is direct typed Rust linkage to the
engine facade. It is not a discoverable operation, has no public shared
request/result DTO, no operation-definition registry entry, no bootstrap
descriptor or catalog binding, no generated schema, and no CLI JSON or Tauri
surface. `handbook-compiler` is retired from the normal path and must not be
resurrected.

The complete frozen ordinary inventory is 62 operation IDs. The exact set in
`05-contracts-schemas-and-gates.md`, HCM-4.1 `tasks/plan.md`, and the HCM-4.2
source inventory is identical. HCM-4.2 defines common contract machinery and
honest admission, not implementation of every row. `contract.*` and `dock.*`
remain Phase-5-owned and are catalog-deferred.

## Authority boundary

### HCM-4.2 owns

- public shared typed DTO shapes and closed discriminants;
- exact schema/operation/definition identity and compatibility rules;
- canonical serialization and fingerprint preimages;
- generated schema source-of-truth and checked-in layout;
- capability/bootstrap discovery and immutable catalog/page semantics;
- public admission/refusal behavior before and after operation selection;
- operation-owner availability classification for the frozen 62-row inventory;
- future SDK-to-adapter seams and direct-Rust equivalence proof requirements.

### HCM-4.2 does not own

- semantic validation, mutation, repository truth, or domain refusal, which
  remain in engine, flow, pipeline, or the future contracts owner;
- CLI grammar, help, human rendering, stdout/stderr, exit mapping, or JSON
  adapter behavior, which are HCM-4.3;
- Tauri command names, scheduling, or UI, which are HCM-4.4;
- installed-skill behavior, which is HCM-4.5;
- contract runtime, evidence/verdict/gate execution, dock process execution,
  or Phase-5 operation ownership;
- Phase-6 consumers, publication, release, remote, or downstream changes.

## Ownership and dependency direction

The future dependency graph is one-way:

```text
handbook-engine / handbook-flow / handbook-pipeline / handbook-contracts
                              |
                              v
                       handbook-sdk DTO facade
                         /              \
                        v                v
                 handbook-cli       future Tauri adapter
```

Owner crates never depend on `handbook-sdk`; the SDK composes owners but never
becomes canonical semantic authority. SDK DTOs are bounded public projections,
not raw owner records. CLI and Tauri adapters consume the same typed methods
and DTOs; neither may add domain branches or a second DTO shape.

## Public DTO contract

The future public module is planned beneath
`crates/sdk/src/contract_membrane/` with private implementation helpers and
public typed exports only for the approved contract. Its shared closed DTOs
are the 05 contract's `ExactBinding`, `Problem`, `Diagnostic`, `NextAction`,
`ArtifactRef`, `SourceBinding`, `Omission`, `SchemaManifestEntry`,
`CatalogRoot`, `CatalogCursor`, `CatalogPage`, `RecheckCondition`,
`CorrelationRef`, `WriteReceipt`, `IdempotencyState`, and `ProblemBinding`.

The ordinary response is the closed `status` union `ok | blocked | refused |
error` with all four terminal fields present: `data` plus the three terminal
arrays `blockers`, `refusals`, and `errors`. `data` is non-null only for
`ok`; blockers, refusals, and errors are mutually exclusive and sorted by the
contract's exact keys. `Problem.category` is closed and selected by its
terminal variant. `Diagnostic` and `NextAction` are non-authoritative and
cannot supply a decision or grant authority.

Public unions use explicit discriminant/tag fields. Unknown fields, unknown
discriminants, unknown enum values, duplicate members, invalid nulls, unsafe
locators, unbounded collections, and non-canonical ordering refuse before a
trusted DTO is constructed. Optional fields are absent-by-default unless the
schema explicitly says nullable; null and omission are never silently
interchanged. A raw idempotency key is permitted only as bounded input at the
single required typed-body RFC 6901 pointer pinned by the selected mutation
definition. For `posture.transition.apply@1.0.0` that future pointer is
exactly `/body/idempotency_key`. The raw value is prohibited from response/
result DTOs, diagnostics, Problems or details, receipts, replay state, schema
manifests, durable public state, logs, argv/environment, and every output
surface; only the scoped key fingerprint may be exposed. Positive fixtures
must prove a schema-valid key reaches the owner, and negative scans must prove
the literal raw value is absent from every prohibited surface.

`record.list` and `record.read` remain typed closed-union operations. A
`record_family` discriminant selects one exact schema and Rust enum case;
private HCM-3.6 posture records are not exported merely because those generic
operations exist. Every public binding carries exact ref plus fingerprint.

## Exact identity, SemVer, serialization, and fingerprints

The following shared schema IDs are reserved for the HCM-4.2 contract at
version `1.0.0`:

| Schema ref | Purpose |
|---|---|
| `handbook.operation-request@1.0.0` | generic ordinary request envelope |
| `handbook.operation-response@1.0.0` | generic ordinary response envelope |
| `handbook.operation-admission-refusal@1.0.0` | preselection refusal envelope |
| `handbook.bootstrap-request@1.0.0` | cold bootstrap request |
| `handbook.bootstrap-response@1.0.0` | bootstrap descriptor/catalog roots |
| `handbook.bootstrap-refusal@1.0.0` | descriptor/API admission refusal |
| `handbook.bootstrap-descriptor-schema@1.0.0` | schema for the immutable API-major descriptor content object |
| `handbook.operation-definition@1.0.0` | exact operation definition |
| `handbook.capability-entry@1.0.0` | capability/operation catalog entry |
| `handbook.owner-version-entry@1.0.0` | exact owner-crate/version/source binding |
| `handbook.operation-blocker@1.0.0` | blocker outcome |
| `handbook.operation-refusal@1.0.0` | refusal outcome |
| `handbook.operation-error@1.0.0` | error outcome |
| `handbook.problem@1.0.0` | shared Problem DTO |
| `handbook.diagnostic@1.0.0` | shared diagnostic DTO |
| `handbook.next-action@1.0.0` | shared next-action DTO |
| `handbook.artifact-ref@1.0.0` | bounded artifact locator |
| `handbook.source-binding@1.0.0` | exact source/revision/adapter binding |
| `handbook.omission@1.0.0` | bounded omission/proof-effect record |
| `handbook.schema-manifest-entry@1.0.0` | response schema manifest member |
| `handbook.catalog-root@1.0.0` | immutable catalog snapshot root |
| `handbook.catalog-cursor@1.0.0` | catalog-bound page cursor |
| `handbook.catalog-page@1.0.0` | contiguous bounded catalog page |
| `handbook.recheck-condition@1.0.0` | deterministic blocker recheck |
| `handbook.correlation-ref@1.0.0` | opaque non-secret correlation |
| `handbook.write-receipt@1.0.0` | realized owner write-set item |
| `handbook.idempotency-state@1.0.0` | closed idempotency union |
| `handbook.problem-binding@1.0.0` | response-local Problem identity |

Each operation's request/result schema is derived mechanically from the exact
operation ID, for example `artifact.validate` maps to
`handbook.operation.artifact-validate-request@1.0.0` and
`handbook.operation.artifact-validate-result@1.0.0`. No operation may choose a
second spelling or derive an ID from a CLI/Tauri path, profile vocabulary,
custom artifact ID, or example.

`schema_id` is stable identity; `schema_version` and `operation_version` are
full SemVer; an exact ref is `identity + "@" + version`. The legacy two-part
record-routing tag in HCM-0.2/HCM-0.3 records is never a public schema ref.
Breaking field removal/meaning changes require a new major. Additive optional
fields require a new minor and are absent-by-default. Patch versions cannot
change serialized meaning. A higher compatible minor is accepted only when
the immutable capability negotiation advertises it; ranges and `latest` are
refused.

DTO instance fingerprints hash UTF-8 RFC 8785/JCS canonical JSON bytes with no
terminal LF, using lowercase `sha256:<64-hex>`. Generated schema files are
canonical JSON with exactly one terminal LF; their schema fingerprint hashes
the exact checked-in bytes, including that LF. Framing is adapter-owned and is
never part of an instance fingerprint: HCM-4.3 CLI JSON permits the LF-free
document or that document followed by exactly one LF, while HCM-4.4 owns its
Tauri serializer/frame. HCM-4.2 does not impose a universal wire LF.
Fingerprint
preimages exclude only the instance's own derived fingerprint and the exact
presentation/correlation exclusions named by its schema. Every nested binding,
schema manifest member, write receipt, problem, catalog cursor/page, and
operation definition is recomputed before the enclosing fingerprint is
accepted.

## Generated schema source of truth and layout

The future implementation packet uses handwritten typed Rust as the semantic
source of truth and a deterministic in-repository generator as the only
schema producer. The proposed checked-in layout is:

```text
crates/sdk/src/contract_membrane/       # handwritten typed DTOs and definitions
crates/sdk/schemas/2020-12/             # generated *.schema.json documents
crates/sdk/schemas/manifest.json        # sorted exact refs, bytes, fingerprints
crates/sdk/tests/contract_membrane/     # positive/negative/regeneration proof
```

The generator must write a temporary tree, canonicalize each document, add
one LF, recompute byte length/fingerprint, and compare byte-for-byte with the
checked-in tree. It must fail on missing/extra schema files, duplicate IDs,
changed generated ordering, stale manifest entries, schema-ref drift, or
Rust/schema fingerprint disagreement. No hand-edited generated file is an
accepted source of truth; a reviewed type/schema change updates both in one
subject.

## Capability/bootstrap/catalog lifecycle

API major `1` has immutable descriptor-content ref
`handbook.bootstrap-descriptor@1.0.0`. That content object validates against
the separate exact schema binding
`handbook.bootstrap-descriptor-schema@1.0.0`. The content ref/fingerprint
identifies one descriptor instance; the descriptor-schema ref/fingerprint
identifies the checked-in schema file. The two identities are never aliases
and their fingerprints use different preimages.

All bootstrap shapes are closed objects. A field not explicitly marked
nullable is required and non-null; no bootstrap field has an implicit default.
`request_id` is the only nullable request/echo field. Exact bindings contain
only `{ref, fingerprint}`. Identifiers and refs are non-empty UTF-8 strings of
at most 255 bytes; fingerprints have the exact lowercase
`sha256:<64-hex>` form; full versions are SemVer; catalog entries and owner
versions are unique and canonically sorted. Actual object byte lengths, page
counts, string/item counts, raw-key bytes, and bridge-recovery bytes must be
less than or equal to the exact positive ceiling carried by this descriptor
instance; exact ceiling is accepted and one over is refused.

### Closed bootstrap request, response, and refusal

| Type | Exact fields, tags, and bounds |
|---|---|
| `BootstrapRequestV1` | `schema_id="handbook.bootstrap-request"`; `schema_version="1.0.0"`; `request_id: string|null` using `^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$`; `bootstrap_descriptor: ExactBinding`; `api_version="1.0.0"`; `page_size` integer `1..=descriptor.catalog_page_size_max`; `request_fingerprint`. No operation, repository, body, idempotency-key, client-display, or extension member is legal. |
| `BootstrapResponseV1` | `schema_id="handbook.bootstrap-response"`; `schema_version="1.0.0"`; exact echoed `request_id`; `request_fingerprint`; `status="ok"`; `descriptor: BootstrapDescriptorV1`; `catalog_roots` containing exactly `operations`, `schemas`, and `profiles`; `first_capability_page: CatalogPage<CapabilityEntry>`; `owner_versions: OwnerVersionEntry[]`; `schema_manifest`; `response_fingerprint`. The page and arrays obey descriptor ceilings. |
| `BootstrapRefusalV1` | `schema_id="handbook.bootstrap-refusal"`; `schema_version="1.0.0"`; normalized `request_id: string|null`; non-null recomputed `request_fingerprint` after bounded unique-member parsing; attempted or selected `bootstrap_descriptor: ExactBinding|null`; `status="refused"`; `data:null`; `blockers:[]`; `refusals` containing exactly one descriptor-owned `Problem`; `errors:[]`; `schema_manifest`; `response_fingerprint`. It contains no ordinary operation ref/result, idempotency state, or write receipt. Byte/framing/UTF-8/duplicate-member/unparseable input fails before this shape and produces no Handbook response. |

The bootstrap request fingerprint is SHA-256 over LF-free JCS of every request
field except `request_id` and `request_fingerprint`. The response/refusal
fingerprint is SHA-256 over LF-free JCS of the complete emitted object except
only `response_fingerprint`, so it is correlation-sensitive. Different valid
request IDs leave request and semantic content fingerprints unchanged but
change the outer response fingerprint; absent, null, and invalid request IDs
normalize to null and have identical outer bytes after the same refusal is
selected.

### Closed descriptor, owner-version, and capability entry

`BootstrapDescriptorV1` has exactly these fields:

| Field | Exact rule |
|---|---|
| `descriptor_ref` | exactly `handbook.bootstrap-descriptor@1.0.0`; descriptor-content identity |
| `descriptor_schema` | exact binding to `handbook.bootstrap-descriptor-schema@1.0.0`; its fingerprint hashes checked-in schema-file bytes including the one terminal LF |
| `api_version` / `supported_api_major` | exactly `1.0.0` / `1` |
| `bootstrap_request_schema`, `bootstrap_response_schema`, `bootstrap_refusal_schema` | exact bindings to the three bootstrap schemas |
| `ordinary_request_schema`, `ordinary_response_schema`, `admission_refusal_schema` | exact bindings to the generic transport schemas |
| `capabilities_describe_definition` | exact same-major operation-definition binding; descriptor construction fails if the definition is unavailable |
| `operation_definition_schema`, `capability_entry_schema`, `owner_version_entry_schema` | exact schema bindings |
| `catalog_schemas` | exact bindings for `CatalogRoot`, `CatalogCursor`, and `CatalogPage`, in that order |
| `ceilings` | closed object of positive integers: `bootstrap_request_max_bytes`, `bootstrap_response_max_bytes`, `ordinary_request_max_bytes`, `ordinary_response_max_bytes`, `catalog_page_size_max`, `catalog_entry_max_bytes`, `catalog_total_entries_max`, `owner_versions_max_items`, `idempotency_key_max_bytes`, and `bridge_recovery_record_max_bytes`; `idempotency_key_max_bytes <= 65536` |
| `declared_transports` | exactly sorted `[cli_json, rust_sdk, tauri]`; declarations are targets, not availability |
| `descriptor_content_fingerprint` | derived over the complete LF-free JCS descriptor instance except only this field |

`OwnerVersionEntry` has exactly `owner_crate`, full-SemVer `owner_version`,
`source: ExactBinding`, and `entry_fingerprint`. Its fingerprint preimage is
the complete LF-free JCS entry except itself. Entries sort by
`(owner_crate, owner_version, source.ref, source.fingerprint)` and duplicates
refuse.

`CapabilityEntry` has exactly `operation: ExactBinding` (operation ref plus
definition fingerprint), `owner_crate`, `owner_version`, `request_schema`,
`result_schema`, `outcome_schemas` containing exactly `blocker`, `refusal`, and
`error`, `required_capabilities`, `mutability`, `idempotency`,
`declared_transports`, `current_transports`, `deprecation`, and
`entry_fingerprint`. `deprecation` is the only nullable member. Both transport
arrays are unique and sorted subsets of `cli_json | rust_sdk | tauri`;
`declared_transports` is non-empty and `current_transports` is a subset.
`entry_fingerprint` hashes LF-free JCS of every other field. An operation is
included only when all named schemas and owner mappings exist and at least one
declared transport has completed admission proof. The corrective 62-row
matrix has no such row: all current operation transports are empty and every
row is omitted.

The descriptor is compiled into typed clients and separately checksum-pinned
by future bridges; it is not fetched from the endpoint it authenticates. A
cold client sends only `BootstrapRequestV1`. A matching runtime returns the
three immutable roots plus the first bounded capability page. The first list
request materializes one immutable content-addressed snapshot. Every page
repeats the root, is duplicate-free and contiguous, and carries a cursor bound
to catalog fingerprint and next sort key. Concurrent registry changes cannot
mix pages. Expiry returns a typed blocked restart condition; cursor mismatch,
gaps, duplicates, count mismatch, stale/tampered fingerprints, unsupported
major, unknown operation, missing schema/profile, cross-major descriptor
substitution, and malformed bootstrap content terminate at the exact boundary
defined in 05.

## Public posture discovery seam

`posture.transition.apply` becomes discoverable only when its future operation
definition binds the public request/result schemas, exact blocker/refusal/error
schemas, capability requirement, mutability/idempotency/write set, and
definition fingerprint into the same-major bootstrap/catalog closure.

The public request projection must match the live HCM-4.1 owner request
exactly. It contains these seven fields and no caller-supplied policy,
approval set, canonical/lifecycle head, source/result kernel, private record,
or recovery field:

| Field | Exact live meaning |
|---|---|
| `idempotency_key` | required bounded raw input at `/body/idempotency_key`; the engine hashes it and it is prohibited from all outputs and public/durable replay state |
| `repository_identity_fingerprint` | exact repository identity the owner re-reads before composing private records |
| `expected_canonical` | `PostureCanonicalBindingV1` with `reference`, `fingerprint`, `document_sha256`, and `byte_length` |
| `recommendation` | exact `PostureReferenceV1` recommendation pair retained through normal author/approval workflows |
| `reassessment_coverage_ids` | ordered unique coverage IDs, at most 64 items and each at most 256 bytes |
| `change` | `PostureDimensionChangeRequestV1`: `dimension_id`, `dimension_index`, `authority_path`, `expected_stored_value`, `expected_effective_level`, `proposed_effective_level` |
| `effective_at_utc` | bounded normalized UTC timestamp |

The engine, not the caller, resolves policy, approval authority, current
canonical/lifecycle head, current source kernel, profile, and lifecycle
validation. A schema that asks the caller to provide those values is invalid.
The result data for `Applied` or `Replayed` is the exact bounded projection of
all seven live receipt fields: `idempotency_key_fingerprint`,
`repository_identity_fingerprint`, `prior_canonical`, `resulting_canonical`,
`posture_transition`, `lifecycle_transition`, and `resulting_kernel`. It never
contains private records, raw canonical bodies, or recovery-control frames.

The future SDK mapping from every live owner variant is total:

| Live `PostureTransitionApplyResultV1` variant | HCM status / stage / category | Exact terminal data or Problem binding | Idempotency and receipts |
|---|---|---|---|
| `Applied(receipt)` | `ok` | data is the seven-field receipt projection; all three terminal arrays empty | `established`, `replayed=false`, `original_result_fingerprint=null`; exactly two atomic receipts: constitutional-root `canonical_truth` and immutable `PostureTransition` `semantic_record` |
| `Replayed(receipt)` | `ok` | byte-identical semantic data and empty terminal arrays | `established`, `replayed=true`, persisted `original_result_fingerprint`; the same two original receipts, never new writes |
| `Blocked(UnsupportedPlatform)` | `blocked` / `capability_validation` / `prerequisite` | exactly one blocker `posture.unsupported_platform`, details `{owner_variant:"UnsupportedPlatform"}`, descriptor-pinned details schema, and deterministic platform recheck | `not_established`; receipts `[]` |
| `Refused(InvalidIdempotencyKey)` | `refused` / `request_validation` / `schema` | exactly one refusal `posture.invalid_idempotency_key`, details `{owner_variant:"InvalidIdempotencyKey"}` | `not_established`; receipts `[]`; raw key absent |
| `Refused(RepositoryIdentityMismatch)` | `refused` / `authority_validation` / `authority` | exactly one refusal `posture.repository_identity_mismatch`, details `{owner_variant:"RepositoryIdentityMismatch"}` | `not_established`; receipts `[]` |
| `Refused(InvalidFixedDimension)` | `refused` / `precondition_validation` / `precondition` | exactly one refusal `posture.invalid_fixed_dimension`, details `{owner_variant:"InvalidFixedDimension"}` | `not_established`; receipts `[]` |
| `Refused(StaleCanonical)` | `refused` / `precondition_validation` / `precondition` | exactly one refusal `posture.stale_canonical`, details `{owner_variant:"StaleCanonical"}` | `not_established`; receipts `[]` |
| `Refused(MissingLifecycleAuthority)` | `refused` / `authority_validation` / `authority` | exactly one refusal `posture.missing_lifecycle_authority`, details `{owner_variant:"MissingLifecycleAuthority"}` | `not_established`; receipts `[]` |
| `Refused(LifecycleNotCurrent)` | `refused` / `precondition_validation` / `precondition` | exactly one refusal `posture.lifecycle_not_current`, details `{owner_variant:"LifecycleNotCurrent"}` | `not_established`; receipts `[]` |
| `Refused(InvalidSemanticInput)` | `refused` / `precondition_validation` / `precondition` | exactly one refusal `posture.invalid_semantic_input`, details `{owner_variant:"InvalidSemanticInput"}` | `not_established`; receipts `[]` |
| `Refused(AuthorityConflict)` | `refused` / `authority_validation` / `authority` | exactly one refusal `posture.authority_conflict`, details `{owner_variant:"AuthorityConflict"}` | `not_established`; receipts `[]` |
| `Error(UnsafeFilesystem)` | `error` / `execution_start` / `implementation` | exactly one error `posture.unsafe_filesystem`, redacted details `{owner_variant:"UnsafeFilesystem"}`, non-null correlation ref | `not_established`; receipts `[]` |
| `Error(IntegrityOrDurability)` | `error` / `execution_start` / `implementation` | exactly one error `posture.integrity_or_durability`, redacted details `{owner_variant:"IntegrityOrDurability"}`, non-null correlation ref | `not_established`; receipts `[]` |
| `Error(Io)` | `error` / `execution_start` / `implementation` | exactly one error `posture.io`, redacted details `{owner_variant:"Io"}`, non-null correlation ref | `not_established`; receipts `[]` |

The binding is mechanical and leaves no implementation choice. Convert the
owner variant to lowercase kebab case (`InvalidIdempotencyKey` becomes
`invalid-idempotency-key`). Every row's details schema is exactly
`handbook.posture-details.<owner-variant-kebab>@1.0.0`, and `details` is
exactly `{owner_variant:<original Rust variant tag>}`. Each refusal rule is
exactly `handbook.posture-rule.<owner-variant-kebab>@1.0.0`; blocker and error
rules are null as required by the shared Problem contract. Every Problem
subject is the selected exact `posture.transition.apply@1.0.0` definition
binding. `UnsupportedPlatform` alone has recheck schema
`handbook.posture-recheck.unsupported-platform@1.0.0` and condition exactly
`{owner_variant:"UnsupportedPlatform"}`. The descriptor pins every named ref
and fingerprint. The bounded details above are the complete public owner
evidence, not raw owner text. `status`, `data`, all Problems, idempotency, and
receipts are derived only from this table; diagnostics and next actions cannot
change the mapping.

Direct Rust invocation and future CLI/Tauri adapters must agree on status,
semantic data, Problems, realized receipts, replay state, and
`original_result_fingerprint` for the same accepted semantic request and owner
outcome. Each adapter recomputes its correlation-sensitive outer
`response_fingerprint`; outer fingerprints need not equal when valid
`request_id` values differ. Two different valid IDs change only the echo and
outer response fingerprint. Absent, null, or invalid IDs terminate before the
owner as the same `request_id_invalid` refusal with normalized null. These
positive, replay, changed-valid-ID, absent/null/invalid-ID, and raw-key scan
vectors are future HCM-4.2 implementation gates; they authorize no CLI JSON or
Tauri ingress now.

## Future adapter seams

The SDK public method/request/result boundary is transport-neutral. HCM-4.3
will own CLI parsing, `--json`, stdout/stderr, human rendering, and exit
mapping. HCM-4.4 will own Tauri command mapping and scheduling. Both must call
the same typed SDK methods and schema manifest; command names and CLI paths
must not enter operation identity. HCM-4.5 consumes discovery through its
skill workflow. Phase 5 owns `handbook-contracts`/dock operation execution;
Phase 6 owns downstream consumers.

## Stop, rollback, and disallowed shortcuts

Stop for missing owner authority, a new semantic owner, public dependency
back-edge, schema/byte identity disagreement, private-record exposure,
unresolved operation inventory conflict, native/runtime proof unavailable,
GitNexus risk evidence unavailable for an existing-symbol edit, or any scope
expansion. Rollback is deleting the uncommitted planning packet or reverting
the reviewed primary documentation commit before closeout; no runtime data or
external state is changed. Never solve a stop with `pub`, a string dispatcher,
`serde_json::Value`, field-presence inference, schema ranges/latest, CLI-help
parsing, ambient files, generated command names, lint suppression, or a
transport-owned domain branch.

## Exit criteria

The slice exits only when the selector, source inventory, SPEC, plan, todo,
proof strategy, control-pack crosswalk/phase/proof rows, review evidence, and
v1.4 closeout agree; the final complete-subject fresh review is CLEAN; all
applicable documentation/link/path/formatting/whitespace/schema/self-test
checks pass or are honestly recorded unavailable; no P1/P2 remains; and the
two-commit local-only closeout is complete. Completion claims HCM-4.2 planning
package readiness only, not implementation, HCM-3.6, Phase 3, HCM-4.3+, Phase
5, Phase 6, publication, or release.
