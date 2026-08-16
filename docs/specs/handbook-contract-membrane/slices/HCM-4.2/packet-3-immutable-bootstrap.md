# HCM-4.2 Packet 3 — immutable bootstrap and catalogs

This checkpoint records the decisions selected under the slice's delegated
Packet-3 preparation boundary. It controls only Packet 3. The slice README,
Packet-2 checkpoint, current program sources, and live source/tests remain
controlling where this checkpoint is silent.

## Current decision

Packet 3 exposes one immutable, content-addressed bootstrap snapshot. The SDK
may project checked-in schema assets and independent owner evidence, but it may
not attest to its own semantic authority. Bootstrap therefore remains useful
when the honest capability catalog is empty: it reports the exact schema and
profile catalogs, owner versions, and zero current operations.

The historical candidate's requirement that the descriptor bind an admitted
`capabilities.describe` ordinary-operation definition is not selected. Packet
2 proved that row has no independent semantic owner and incomplete local
closure. Requiring the definition would either make every honest bootstrap
permanently cold or let SDK composition bless its own operation. Bootstrap is
instead a fixed Packet-3 seam, while `capabilities.describe@1.0.0` remains one
of the 62 omitted Packet-2 operations.

## Descriptor and negotiation

- `BootstrapDescriptorV1` has content identity
  `handbook.bootstrap-descriptor@1.0.0`. Its canonical instance fingerprint is
  the binding used by requests and responses.
- The checked-in schema for that content has distinct identity
  `handbook.bootstrap-descriptor-schema@1.0.0`; its fingerprint covers exact
  checked-in schema bytes including the terminal LF. Descriptor construction
  requires the schema binding and rejects equality between the instance and
  schema fingerprints.
- The closed descriptor binds the current API version and major, the exact
  supported same-major API-version list, bootstrap request/response/refusal
  schemas, ordinary request/response/admission-refusal schemas, capability and
  owner-version entry schemas, catalog root/cursor/concrete-page schemas,
  positive ceilings, and the complete declared target list
  `[cli_json, rust_sdk, tauri]`.
- Its ten ceilings are exact positive integers:

  | Ceiling | Value |
  | --- | ---: |
  | `bootstrap_request_max_bytes` | 1,048,576 |
  | `bootstrap_response_max_bytes` | 1,048,576 |
  | `ordinary_request_max_bytes` | 1,048,576 |
  | `ordinary_response_max_bytes` | 1,048,576 |
  | `catalog_page_size_max` | 128 |
  | `catalog_entry_max_bytes` | 65,536 |
  | `catalog_total_entries_max` | 128 |
  | `owner_versions_max_items` | 128 |
  | `idempotency_key_max_bytes` | 65,536 |
  | `bridge_recovery_record_max_bytes` | 1,048,576 |

  The document and raw-key values are Packet-0 maxima. The item values are its
  `BoundedVec` maximum. The entry ceiling is its maximum bounded string size,
  and bridge recovery remains only a framing ceiling: it does not assert that a
  bridge exists. Each exact ceiling is accepted and one over is refused.
- API input is one full `SemVer`, never a range or `latest`. The current
  descriptor advertises only `1.0.0`. A request is accepted only when its exact
  version is advertised and its major equals the descriptor major. An
  advertised higher compatible minor is accepted without changing this rule;
  an unsupported minor or major refuses. A descriptor or cursor from another
  major cannot substitute across snapshots.
- Declared transports are architecture targets, not runtime proof. Current
  transports on a capability require separate, current transport evidence and
  must be a non-empty sorted subset of the operation definition's declared
  targets. Packet 3 proves none, so every current capability page is empty.

## Closed public shapes

- `BootstrapRequestV1` binds its checked-in schema, required-nullable bounded
  request ID, exact descriptor instance, exact API version, page size in
  `1..=128`, and a request fingerprint over every preceding field except the
  request ID. It carries no operation, body, repository, raw key, display, or
  extension field.
- `BootstrapResponseV1` binds the request correlation and fingerprint, has the
  sole status `ok`, repeats the immutable descriptor, exposes exactly the
  operations/schemas/profiles roots, includes the typed first capability page,
  exact owner-version entries and schema manifest, and fingerprints every
  field except its response fingerprint.
- `BootstrapRefusalV1` has the sole status `refused`, normalized nullable
  request ID, a non-null recomputed request fingerprint, one nullable
  `bootstrap_descriptor` binding containing the attempted or selected binding,
  `data=null`, empty blockers and errors, exactly one descriptor-owned refusal
  `Problem`, the exact schema manifest, and a response fingerprint over every
  preceding field. It carries no operation result, idempotency state, receipt,
  or raw key. Failures before the bounded unique-member request preimage exists
  remain framing failures, not Handbook responses.
- `CapabilityEntry` is a bounded projection of one exact Packet-2 definition
  plus an owner-native admission token and independent current-transport
  evidence. It repeats the operation, owner/version/source, request/result and
  three outcome schemas, required capabilities, mutability, idempotency,
  declared/current transports, nullable deprecation, and an entry fingerprint.
- `OwnerVersionEntry` contains exactly owner crate, full SemVer, exact source
  binding, and an entry fingerprint. Engine, Flow, and pipeline supply their
  own crate/version/source bytes; the SDK can hash and project those bytes but
  cannot manufacture the evidence.
- `ProfileCatalogEntry` contains an exact profile binding, an exact checked-in
  source binding, and its entry fingerprint. Profile instance and source-byte
  fingerprints use different preimages.

All shapes deny unknown fields, require explicit null where declared nullable,
recompute derived fingerprints during typed admission, and preserve Packet 2's
raw-key absence.

The newly checked-in public roots are exact and additive:

| Rust/public root | Checked-in schema identity |
| --- | --- |
| `BootstrapDescriptorV1` | `handbook.bootstrap-descriptor-schema@1.0.0` |
| `BootstrapRequestV1` | `handbook.bootstrap-request@1.0.0` |
| `BootstrapResponseV1` | `handbook.bootstrap-response@1.0.0` |
| `BootstrapRefusalV1` | `handbook.bootstrap-refusal@1.0.0` |
| `BootstrapRefusalDetails` | `handbook.bootstrap-refusal-details@1.0.0` |
| `CapabilityEntry` | `handbook.capability-entry@1.0.0` |
| `OwnerVersionEntry` | `handbook.owner-version-entry@1.0.0` |
| `ProfileCatalogEntry` | `handbook.profile-catalog-entry@1.0.0` |
| `CatalogRoots` | `handbook.catalog-roots@1.0.0` |
| `TypedCatalogCursor` | `handbook.typed-catalog-cursor@1.0.0` |
| capability page | `handbook.capability-catalog-page@1.0.0` |
| schema-manifest page | `handbook.schema-catalog-page@1.0.0` |
| profile page | `handbook.profile-catalog-page@1.0.0` |
| `CatalogReadBlockedV1` | `handbook.catalog-read-blocked@1.0.0` |

`BootstrapRefusalDetails` contains exactly a closed reason discriminant and the
descriptor's exact sorted supported-API-version list. The reason set is
`stale_descriptor | tampered_descriptor | unsupported_api_version |
unsupported_api_major | cross_major_descriptor`. Cold or structurally corrupt
production assets prevent creation of a trusted descriptor and therefore fail
the production witness before any Handbook response exists.

## Typed immutable catalogs

Packet 3 selects concrete typed pages. Packet 0's public `CatalogPage` with
`ContractValue` entries remains unchanged for compatibility, but bootstrap and
production catalog APIs never use it. They use additive
`TypedCatalogPage<T>` and `TypedCatalogCursor` shapes, with concrete schema
roots for capability, schema-manifest, and profile pages.

The three exact catalog identities are:

| Catalog | Entry type | Canonical sort key |
| --- | --- | --- |
| `handbook.catalog.operations@1.0.0` | `CapabilityEntry` | operation exact ref |
| `handbook.catalog.schemas@1.0.0` | `SchemaManifestEntry` | schema exact ref |
| `handbook.catalog.profiles@1.0.0` | `ProfileCatalogEntry` | profile exact ref |

The additive `TypedCatalogCursor` has checked-in schema identity
`handbook.typed-catalog-cursor@1.0.0` and exactly these fields: schema binding,
API major, catalog binding, page size, zero-based next index, exact-ref next
sort key, expiry timestamp equal to the root retention timestamp, and cursor
fingerprint. Its fingerprint covers every preceding field. It refuses a
different catalog fingerprint, API major, page size, next index/key pair, or
expiry binding.

The three concrete typed-page schema identities are:

| Rust page | Checked-in schema identity |
| --- | --- |
| capability page | `handbook.capability-catalog-page@1.0.0` |
| schema-manifest page | `handbook.schema-catalog-page@1.0.0` |
| profile page | `handbook.profile-catalog-page@1.0.0` |

Each closed page contains exactly its schema binding, API major, repeated
`CatalogRoot`, page size, zero-based start index, typed entries, explicitly
nullable next cursor, and page fingerprint. The fingerprint covers every
preceding field. An immutable catalog snapshot sorts and rejects duplicate
exact-ref keys before deriving its root fingerprint. Every page contains a
strictly contiguous slice. Its next cursor names the first unreturned index and
key; the final page must end at the root total and have no cursor.

Expiry does not become a refusal. It returns a closed
`CatalogReadBlockedV1` (`handbook.catalog-read-blocked@1.0.0`) with status
`blocked`, null data, one blocker, empty refusals/errors, the expired catalog,
and a `RecheckCondition` whose typed condition is the immutable restart root.
Its response fingerprint covers every preceding field. A non-expired cursor
from another snapshot, catalog, page size, or API major refuses as a typed
`CatalogReadError`. A registry change produces a new root while the old
snapshot remains internally stable, so pages never mix.

## Production witness and current roots

The production witness is constructed only after all of these independent
inputs agree:

1. the exact checked-in schema manifest parses canonically, is strictly sorted,
   and every entry's byte count and SHA-256 fingerprint matches an embedded
   checked-in schema asset including its one terminal LF;
2. the manifest has exact identity coverage for every public schema required
   by the descriptor and all three catalog entry/page types;
3. Engine, Flow, and pipeline each provide owner-native crate/version/source
   evidence through their own `operation_owner_version_evidence()` function;
   each result uses its compiled Cargo package name/version and exact
   `include_bytes!` source for its Packet-2 admission module;
4. profile rows come from exact Engine-owned checked-in profile assets and
   preserve their declared full-SemVer identity; and
5. the witness consumes all 62 Packet-2 inventory rows, rechecks contiguous
   ordinals, uniqueness, full tuples, current `discovery=omit` and transport
   emptiness, and recomputes the frozen digest
   `sha256:ec8eaa6447efd2473340d1de7225acaa8e81e04d6ce9184258b51fdd6c4380a5`;
   it does not accept an SDK-supplied empty list as inventory truth; and
6. every capability has an exact Packet-2 definition, matching manifest
   schemas, owner-native admission, receipt/write closure, and at least one
   separately proven current transport.

The three exact owner-version rows are:

| Owner crate/version | Source exact ref |
| --- | --- |
| `handbook-engine@0.2.0` | `handbook.owner.engine@0.2.0` |
| `handbook-flow@0.1.1` | `handbook.owner.flow@0.1.1` |
| `handbook-pipeline@0.1.2` | `handbook.owner.pipeline@0.1.2` |

Each source fingerprint is SHA-256 over the exact owner-supplied checked-in
source bytes. The entry fingerprint is the canonical instance fingerprint over
owner crate, full SemVer, and source binding, and the three entries sort by
`(owner_crate, owner_version, source.ref, source.fingerprint)`.

Current-transport proof is a separate SDK-sealed trait. Packet 3 supplies no
implementor because no current CLI JSON, Rust SDK ordinary-operation, or Tauri
transport closes definition, owner admission, request/result/outcome schemas,
write receipts, and transport behavior. A definition's declared target list
cannot construct this token, and downstream code cannot implement the sealed
trait. Consequently no capability entry can be constructed in current
production witnessing.

Unknown operations, missing schemas or profiles, duplicate or gapped identity
sets, count mismatches, stale/tampered bytes, and self-attested owner claims
fail construction. At the current Packet-2 boundary item 5 admits no row, so
the operations root total is exactly zero. The schema root covers the complete
new checked-in manifest, and the profiles root covers the three exact shipped
Engine semantic profiles `handbook.profile.shipped-root@1.0.0`, `@1.1.0`, and
`@1.2.0` from `crates/engine/definitions/profiles` through Engine-owned exact
asset evidence. A profile content fingerprint uses the domain-separated exact
ref plus profile bytes; its source fingerprint covers the exact checked-in
bytes, so the two identity domains do not alias.

`core/profiles/**` is deliberately excluded from this root. It contains six
concrete Pipeline-owned mutable workflow packs at `0.1.0` plus the `_template`
directory; those packs have no current `handbook.profile.*` semantic exact
identity and cannot be promoted into Engine profile authority. A future
workflow-pack catalog would require a separately named Pipeline-owned witness.

The witness and snapshots are immutable owned values. They do not read mutable
process registries between pages and expose no adapter hook. This packet adds
no CLI/Tauri route, posture mapping, future owner, publication, bridge, dock,
or crate-placement decision.
