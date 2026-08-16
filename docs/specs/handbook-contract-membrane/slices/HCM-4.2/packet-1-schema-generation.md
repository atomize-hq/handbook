# HCM-4.2 Packet 1 — Schema Generation and Manifest

This document records the current implementation authority and completed
decision boundary for HCM-4.2 Packet 1. It resolves the packet's generator,
schema inventory, file mapping, manifest, and parser-versus-schema boundary.

The [HCM-4.2 slice authority](README.md) remains controlling for the complete
slice outcome, cross-packet invariants, ordering, and non-goals. Packet 0 source
and tests remain product truth if this document drifts from them.

## Admission and objective

Packet 1 completed at
`f54dcbd3ec9a50ad586d7952cbec8ca6d508eefe`. Packet 0 and the Phase-3 exit were
its satisfied predecessors. The slice authority now governs automatic
continuation and just-in-time preparation for Packets 2 through 5; this packet
document does not expand their outcomes.

Produce a deterministic, checked-in Draft 2020-12 schema tree for the concrete
Packet-0 public roots and a manifest that binds each exact public ref to the
fingerprint and byte length of its checked-in schema. Prove regeneration and
the boundary between lexical parsing, schema validation, typed invariants, and
owner trust without adding an operation or transport surface.

## Resolved generator decision

Use an in-repository Rust generator built with exactly pinned
`schemars = "=1.2.2"`, behind a non-default SDK feature named
`contract-schema-generation`. Invoke
`SchemaSettings::draft2020_12().for_deserialize()` explicitly; do not depend on
Schemars defaults. Generate schemas from `JsonSchema` implementations that
describe the actual accepted wire form. Types with custom Serde behavior must
use reviewed manual schema implementations or exact wire helper types rather
than deriving from a misleading private representation. Do not use
`schema_for_value`, examples, or checked-in JSON as schema authority.

Schemars documents both the explicit Draft 2020-12 settings and that generated
schema structure may change without a semver-breaking release. The exact pin,
Cargo lock, and byte-for-byte regeneration check therefore form one reviewed
compatibility boundary. See the official
[`SchemaSettings` documentation](https://docs.rs/schemars/1.2.2/schemars/generate/struct.SchemaSettings.html)
and [versioning note](https://docs.rs/schemars/1.2.2/schemars/index.html#versioning-and-stability).

Use `jsonschema = { version = "=0.47.0", default-features = false }` as an SDK
development dependency for Draft 2020-12 meta-schema and instance checks. This
matches the version and no-resolver feature posture already used by the engine;
Packet 1 does not upgrade it or add a runtime dependency. Validation must use
only the supplied in-memory schema tree and must never retrieve refs from the
network or filesystem. The official
[`jsonschema` validator documentation](https://docs.rs/jsonschema/0.47.0/jsonschema/struct.Validator.html)
is the API source.

Normal SDK builds must not generate or rewrite files. Provide one feature-gated
binary named `generate-contract-schemas` with only these modes:

```text
cargo run -p handbook-sdk --features contract-schema-generation --bin generate-contract-schemas -- --check
cargo run -p handbook-sdk --features contract-schema-generation --bin generate-contract-schemas -- --write
```

Both modes generate a complete temporary tree first. `--check` compares it to
the tracked tree. `--write` replaces tracked generated output only after the
temporary tree is complete and internally valid. No `build.rs`, shell, Python,
network access, timestamp, host path, random value, or locale may affect bytes.

## Exact schema inventory and file mapping

Publish exactly these 15 concrete Packet-0 roots. Helper primitives, closed
enums, bounded collections, `ArtifactLocator`, `ExactBinding`, and other nested
wire types live under each document's local `$defs`; they are not independent
manifest entries. Generic `TerminalFields<T>` becomes part of a concrete
operation envelope only in Packet 2. No operation, bootstrap, or posture type
may be pre-generated here.

Paths are relative to `crates/sdk/schemas/2020-12/`:

| Exact ref | Rust root | Generated path |
| --- | --- | --- |
| `handbook.problem@1.0.0` | `Problem` | `handbook.problem/1.0.0.schema.json` |
| `handbook.diagnostic@1.0.0` | `Diagnostic` | `handbook.diagnostic/1.0.0.schema.json` |
| `handbook.next-action@1.0.0` | `NextAction` | `handbook.next-action/1.0.0.schema.json` |
| `handbook.artifact-ref@1.0.0` | `ArtifactRef` | `handbook.artifact-ref/1.0.0.schema.json` |
| `handbook.source-binding@1.0.0` | `SourceBinding` | `handbook.source-binding/1.0.0.schema.json` |
| `handbook.omission@1.0.0` | `Omission` | `handbook.omission/1.0.0.schema.json` |
| `handbook.schema-manifest-entry@1.0.0` | `SchemaManifestEntry` | `handbook.schema-manifest-entry/1.0.0.schema.json` |
| `handbook.catalog-root@1.0.0` | `CatalogRoot` | `handbook.catalog-root/1.0.0.schema.json` |
| `handbook.catalog-cursor@1.0.0` | `CatalogCursor` | `handbook.catalog-cursor/1.0.0.schema.json` |
| `handbook.catalog-page@1.0.0` | `CatalogPage` | `handbook.catalog-page/1.0.0.schema.json` |
| `handbook.recheck-condition@1.0.0` | `RecheckCondition` | `handbook.recheck-condition/1.0.0.schema.json` |
| `handbook.correlation-ref@1.0.0` | `CorrelationRef` | `handbook.correlation-ref/1.0.0.schema.json` |
| `handbook.write-receipt@1.0.0` | `WriteReceipt` | `handbook.write-receipt/1.0.0.schema.json` |
| `handbook.idempotency-state@1.0.0` | `IdempotencyState` | `handbook.idempotency-state/1.0.0.schema.json` |
| `handbook.problem-binding@1.0.0` | `ProblemBinding` | `handbook.problem-binding/1.0.0.schema.json` |

Every root document must be self-contained, use only local `$defs`, and contain:

- `$schema: "https://json-schema.org/draft/2020-12/schema"`;
- an absolute `$id` formed as
  `urn:handbook:schema:<exact-ref identity>:<exact-ref version>`; and
- a closed root shape that describes deserialization, including required-null
  distinctions, unknown-field refusal, tags, enums, bounds, and patterns that
  JSON Schema can express without changing Rust meaning.

For example, `handbook.problem@1.0.0` maps to
`urn:handbook:schema:handbook.problem:1.0.0`. Draft 2020-12 recommends an
absolute root `$id`; the [core specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2.1.1)
is controlling for this URI behavior.

## Manifest and byte identity

Write `crates/sdk/schemas/manifest.json` as a canonical JSON array of the same
15 `SchemaManifestEntry` wire objects, sorted by exact `schema.ref` byte order.
Each entry contains only Packet 0's existing fields:

- `schema`: the exact ref and SHA-256 fingerprint of the schema file;
- `draft`: `"2020-12"`;
- `byte_length`: the exact checked-in schema byte length; and
- `media_type`: `"application/schema+json"`.

The exact-ref-to-path table above is authoritative, so the manifest does not
duplicate paths. The manifest is not an entry in itself and has no recursive
self-fingerprint.

Serialize every schema and the manifest with Packet 0's canonical JSON
serializer, then append exactly one LF byte. A schema fingerprint covers the
complete checked-in schema bytes including that LF. Instance canonical bytes
remain LF-free, and adapter framing remains outside both identity domains.

## Validation responsibility

The four gates remain distinct and run in this order when a later consumer uses
a checked-in schema:

1. **Canonical parser:** Packet 0 owns UTF-8, JSON syntax, duplicate-member,
   lexical number, depth, byte/item bounds, and canonical-byte admission.
2. **JSON Schema:** the selected checked-in root owns declarative closed shape,
   required/null rules, tags, enums, expressible bounds, and patterns.
3. **Typed Rust admission:** constructors and custom `Deserialize` code own
   cross-field invariants, canonical ordering, derived fingerprints, exact
   bindings, portable paths, and any rule JSON Schema cannot express exactly.
4. **Owner witness:** a later semantic owner alone establishes `Verified<T>`;
   structural success at the first three gates is never owner trust.

Schema validation is necessary but intentionally insufficient. It must not
coerce values, apply semantic defaults, accept an unknown field, turn a format
annotation into authority, recompute a fingerprint, or replace typed admission.
A rejection at any gate is final for that admission attempt. Packet 1 proves
this layering in focused tests but does not wire a production adapter.

## Allowed scope

Packet 1 may change only:

- `crates/sdk/Cargo.toml` and `Cargo.lock` for the exact optional generator and
  exact development validator dependencies, feature, and binary declaration;
- Packet-0 contract-membrane modules needed for accurate schema descriptions;
- the feature-gated Rust generator;
- `crates/sdk/schemas/2020-12/` and `crates/sdk/schemas/manifest.json`;
- focused SDK schema-generation tests and existing Packet-0 tests when needed
  to prove parity; and
- current HCM documentation only when implementation uncovers actual drift.

Do not refactor unrelated Packet-0 behavior while adding schema descriptions.
Any product edit to an existing function, class, or method still requires the
repository's impact analysis before editing.

## Acceptance and verification

Packet 1 is complete only when all of the following hold:

1. The exact 15-entry inventory is generated; any missing, extra, duplicate,
   renamed, or unlisted schema fails generation.
2. Every schema meta-validates as Draft 2020-12, has the exact `$id`, is
   self-contained, and has canonical JSON plus exactly one terminal LF.
3. The manifest has exactly 15 sorted entries, and every ref, byte length, and
   fingerprint recomputes from the tracked bytes.
4. A clean temporary regeneration is byte-identical to the complete tracked
   schema tree and manifest on the supported workspace platform.
5. Representative constructor-produced canonical instances and boundary
   fixtures validate against their root schemas and typed parsers.
6. Negative tests preserve unknown-field, tag/enum, required/null, bound,
   duplicate-member, canonical-form, and semantic-invariant refusal at the
   responsible gate.
7. Schemars-generated structure is reviewed against every custom Packet-0 wire
   form; no private representation or example value silently defines a schema.
8. Raw idempotency keys remain absent from schemas, manifest, diagnostics,
   generated output, fixtures, and errors.
9. These focused commands pass:

```text
cargo run -p handbook-sdk --features contract-schema-generation --bin generate-contract-schemas -- --check
cargo test -p handbook-sdk --test contract_schema_generation
cargo test -p handbook-sdk --test contract_membrane
cargo clippy -p handbook-sdk --all-targets --features contract-schema-generation -- -D warnings
```

10. Required repository change detection and one focused causal review approve
    the exact Packet-1 implementation subject.

The completed packet returned `PACKET_COMPLETE` in one coherent local commit.
It did not claim slice completion or perform aggregate closeout. The meta task,
not the Packet-1 task, owns immediate sequencing to Packet 2 under the current
slice authority's delegated preparation rule.

## Non-goals

Packet 1 does not authorize operation definitions or envelopes, bootstrap or
catalog producers, a production owner witness, posture projection, CLI/Tauri
routes, publication, a contract/gate/dock owner, or changes to Packet 0's
semantic and trust boundaries. It creates no schema service, network resolver,
build-time generator, code-generation framework, proof wall, ledger, handoff,
selector, or mandatory second commit.
