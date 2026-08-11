# HCM-4.2 — shared DTO and JSON Schema contract

**Phase:** `HCM-4`
**Packet:** `HCM-4.2-PLANNING-CONTROL-PACK`
**Status:** planning package authored; selector reviewed CLEAN; implementation not authorized
**Selector:** `decision/20260811T001900Z--hcm-4-2-shared-dto-json-schema-selector.md`
**Source/impact inventory:** `research/20260811T001900Z--source-impact-inventory.md`

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

At base commit `818d3662f57fcda867a59b7f9e035755fee1b393`, tree
`6dd3872e93bcad6604e78534d05e2ec14a0f8aea`, HCM-4.1's
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
error` with all four terminal arrays present. `data` is non-null only for
`ok`; blockers, refusals, and errors are mutually exclusive and sorted by the
contract's exact keys. `Problem.category` is closed and selected by its
terminal variant. `Diagnostic` and `NextAction` are non-authoritative and
cannot supply a decision or grant authority.

Public unions use explicit discriminant/tag fields. Unknown fields, unknown
discriminants, unknown enum values, duplicate members, invalid nulls, unsafe
locators, unbounded collections, and non-canonical ordering refuse before a
trusted DTO is constructed. Optional fields are absent-by-default unless the
schema explicitly says nullable; null and omission are never silently
interchanged. Private raw idempotency keys never enter a public DTO, schema
manifest, diagnostic, receipt, or durable public record; only the scoped key
fingerprint may be exposed where the contract permits it.

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
| `handbook.bootstrap-descriptor@1.0.0` | immutable API-major descriptor |
| `handbook.operation-definition@1.0.0` | exact operation definition |
| `handbook.capability-entry@1.0.0` | capability/operation catalog entry |
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

DTO instance fingerprints hash UTF-8 RFC 8785/JCS canonical JSON bytes without
an implicit newline, using lowercase `sha256:<64-hex>`. A wire document is
those canonical bytes followed by one LF at the adapter framing boundary.
Generated schema files are canonical JSON with one terminal LF; their schema
fingerprint hashes the exact checked-in bytes, including that LF. Fingerprint
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

API major `1` has immutable descriptor ref
`handbook.bootstrap-descriptor@1.0.0`. The descriptor is compiled into typed
clients and checksum-pinned by future bridges; it is not fetched from the
operation endpoint it authenticates. Its closure binds the exact bootstrap
request/response/refusal schemas, generic ordinary request/response schemas,
supported API major, positive protocol ceilings, and the exact same-major
`capabilities.describe` definition.

A cold client sends only the descriptor-pinned bootstrap request. A matching
runtime returns immutable `CatalogRoot` values for implemented operations,
schemas, and profiles plus the first bounded capability page. Subsequent
`capabilities.describe`, `schema.list/read`, and `profile.list` requests use
the root's exact catalog fingerprint and cursor. Referenced vocabulary,
Resolution, and Projection definitions are retrieved by exact binding, never
by reading repository files or inferring shipped defaults.

The first list request materializes one immutable content-addressed snapshot.
Every page repeats the root, is duplicate-free and contiguous, and carries a
cursor bound to catalog fingerprint and next sort key. Concurrent registry
changes cannot mix pages. Expiry returns a typed blocked restart condition;
cursor mismatch, gaps, duplicates, count mismatch, stale/tampered
fingerprints, unsupported major, unknown operation, missing schema/profile,
cross-major descriptor substitution, and unparseable bootstrap data refuse or
fail at the exact preselection/adapter boundary defined in `05`.

## Public posture discovery seam

`posture.transition.apply` becomes discoverable only when its future operation
definition binds the public request/result schemas, exact blocker/refusal/error
schemas, capability requirement, mutability/idempotency/write set, and
definition fingerprint into the same-major bootstrap/catalog closure.

The public request is a bounded typed intent containing exact recommendation,
policy, approval, canonical/lifecycle head, reassessment, repository,
idempotency, expected document bytes/fingerprint/length, and compare-and-write
bindings required by HCM-4.1. The public result contains bounded semantic
outcomes and exact receipts/bindings. It does not contain private
`project_posture` records, private transition records, raw canonical bodies,
or recovery-control frames. Engine validation, mutation, and domain refusal
remain authoritative; SDK maps them to the shared closed outcome union.

Direct Rust invocation and future CLI/Tauri adapters must produce equivalent
typed semantic results and fingerprints from the same accepted request and
owner outcome. This proof is a future HCM-4.2 implementation gate. It does not
authorize CLI JSON or Tauri ingress now.

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
