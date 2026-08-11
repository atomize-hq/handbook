# HCM-4.2 dependency-ordered planning packets

**Parent outcome:** `HCM-4-2-DOCS-CONTROL-PACK`
**Packet:** `HCM-4.2-PLANNING-CONTROL-PACK`
**Implementation status:** not authorized by this planning slice

Each future packet is independently reviewable and remains inside the HCM-4.2
authority. Candidate paths are future implementation seams, not current edits.
Every packet must repeat upstream GitNexus impact before changing an existing
symbol; GitNexus risk unavailable in this session is not evidence of LOW risk.

## Packet 0 — typed contract module and serialization kernel

- **Dependencies:** reviewed HCM-4.2 selector; live HCM-4.1 SDK/owner truth.
- **Candidate paths/symbols:** `crates/sdk/src/contract_membrane/mod.rs`,
  `dto.rs`, `binding.rs`, `serialization.rs`; `HandbookSdkV1` typed method
  seam in `crates/sdk/src/lib.rs`; existing engine fingerprint utilities only
  as an explicitly mapped dependency.
- **Public/API effect:** adds public typed shared DTOs and exact schema/operation
  identity bindings; no string dispatcher and no public unbounded `Value`.
- **Dependency/version effect:** SDK may depend on the established owner
  crates and approved serializer/generator helpers; owners never add an SDK
  dependency. Any new dependency/version or public SemVer choice stops for
  fresh authority.
- **Compatibility/migration:** `1.0.0` exact refs; full-SemVer major/minor/
  patch rules from SPEC; no legacy artifact-format compatibility promise.
- **Proof gates:** closed discriminants, unknown fields, null/default,
  duplicate-member, bounded-size, canonical JCS, all instance fingerprints,
  raw-key non-disclosure, and direct-owner semantic mapping.
- **GitNexus blast radius:** existing `HandbookSdkV1` and owner facade edits
  require upstream impact; current GitNexus is unavailable, so implementation
  must stop rather than infer risk.
- **Review cadence:** one planning/implementation discovery; consolidated
  remediation; different-fresh closure; at most two directly causal
  supplementals.
- **Risks/stop:** owner-record leakage, DTO schema drift, back-edge, or
  serializer behavior mismatch stops. Rollback removes the candidate module
  before any adapter consumes it.
- **Disallowed shortcuts:** `serde_json::Value`, flattening, field inference,
  visibility-only reachability, schema ranges/latest, or prose-defined errors.

## Packet 1 — checked-in schema generation and manifest

- **Dependencies:** Packet 0 typed DTOs and identity rules.
- **Candidate paths/symbols:** `crates/sdk/schemas/2020-12/`,
  `crates/sdk/schemas/manifest.json`, generator/test seam under
  `crates/sdk/tests/contract_membrane/` or a separately approved SDK tooling
  module; exact generated schema names in SPEC.
- **Public/API effect:** publishes checked-in Draft 2020-12 schema documents
  and a sorted manifest; does not publish a transport or add commands.
- **Dependency/version effect:** generator version and Draft 2020-12 dialect
  are pinned in the reviewed packet; schema bytes and fingerprints become one
  compatibility subject with Rust types.
- **Compatibility/migration:** additive optional fields require minor schema
  versions and absent defaults; breaking/removal/meaning changes require a
  new major; patch cannot alter bytes/meaning.
- **Proof gates:** clean regeneration into a temporary tree is byte-identical;
  no missing/extra files; manifest byte length and fingerprints recompute;
  refs/IDs are unique and full-SemVer; all closed-object/discriminant rules
  are enforced.
- **GitNexus blast radius:** new generated assets have no existing symbol
  impact; any generator integration into existing build/test symbols requires
  upstream impact and honest risk evidence.
- **Risks/stop:** nondeterministic ordering, platform line endings, generator
  drift, or checked-in hand edits stop. Rollback restores the prior generated
  tree and manifest atomically.
- **Disallowed shortcuts:** generated schemas as an unreviewed source of truth,
  OpenAPI-first DTOs, or a schema-only change without the Rust type review.

## Packet 2 — operation definitions and owner-admission catalog

- **Dependencies:** Packets 0-1; exact 62-row inventory in source inventory.
- **Candidate paths/symbols:** SDK-owned operation registry/definition types
  under `crates/sdk/src/contract_membrane/operation.rs` and
  `catalog.rs`; owner adapters remain engine/flow/pipeline/contract seams.
- **Public/API effect:** binds exact operation ID/version, owner, request/
  result/outcome schema refs/fingerprints, capability refs, mutability,
  idempotency, write set, transport targets, and deprecation metadata.
- **Dependency/version effect:** no future Phase-5 `contract.*`/`dock.*`
  owner is admitted early; owner availability is explicit per catalog row.
- **Compatibility/migration:** definitions are exact refs; changes to
  operation meaning require major; replacement/migration refs are exact and
  fingerprinted; no silent removal.
- **Proof gates:** set equality with 05 and HCM-4.1 plan; duplicate/unknown/
  missing definitions; unsupported operation; definition-pin mismatch/stale;
  capability closure; exact write-set/receipt matrix; honest absence of
  unimplemented rows.
- **GitNexus blast radius:** registry additions are new symbols; editing
  existing SDK method routing requires upstream impact. Missing GitNexus blocks
  an implementation claim.
- **Risks/stop:** inventing operation IDs, exposing future owners, or claiming
  direct Rust posture discovery before schema/bootstrap closure stops.
- **Disallowed shortcuts:** deriving catalog from CLI help, source filenames,
  examples, Tauri command names, or ambient repository files.

## Packet 3 — immutable bootstrap and paged catalogs

- **Dependencies:** Packets 0-2 and exact schema manifest.
- **Candidate paths/symbols:** `bootstrap.rs`, `catalog.rs`, and SDK discovery
  methods; checked-in bootstrap descriptor/schema fixtures.
- **Public/API effect:** adds cold bootstrap request/response/refusal and
  immutable capability/profile/schema catalog reads; no CLI/Tauri adapters.
- **Dependency/version effect:** API-major descriptor
  `handbook.bootstrap-descriptor@1.0.0` is compiled/checksum-pinned; a
  breaking descriptor change requires API major 2.
- **Compatibility/migration:** exact same-major descriptor/fingerprint;
  compatible minor only when advertised; no range/latest fallback.
- **Proof gates:** cold/stale/tampered/unsupported-major/cross-major,
  unknown-operation, missing schema/profile, immutable snapshot, pagination,
  cursor, expiry/restart, concurrent registry change, total-count, duplicate,
  gap, and catalog fingerprint cases.
- **GitNexus blast radius:** new discovery seam is new API; any integration
  into existing SDK method routing requires upstream impact and review.
- **Risks/stop:** process-local discovery, mutable pages, or descriptor
  circularity stops; rollback removes discovery assets without changing owner
  truth.
- **Disallowed shortcuts:** CLI `--help`, Tauri command names, examples,
  profiles/enums, ambient files, or latest lookup as capability authority.

## Packet 4 — posture public projection and direct-Rust equivalence

- **Dependencies:** Packets 0-3 and HCM-4.1 owner proof.
- **Candidate paths/symbols:** bounded SDK posture request/result mapping near
  `HandbookSdkV1::apply_posture_transition`; engine
  `PostureTransitionEngineFacadeV1` remains the owner.
- **Public/API effect:** makes `posture.transition.apply@1.0.0` discoverable
  only after exact operation/schema/bootstrap bindings; exposes bounded DTOs,
  not private records or raw canonical bytes.
- **Dependency/version effect:** no owner back-edge, no compiler, no CLI/Tauri
  implementation, no Phase-5 runtime.
- **Compatibility/migration:** exact recommendation/policy/approval/head/
  reassessment/idempotency/document-byte/CAS bindings; future breaking public
  meaning requires a major schema/operation migration artifact.
- **Proof gates:** direct-Rust result equivalence, recommendation/policy/
  approval/head/reassessment/stale/no-op/refusal/error/replay/idempotency/
  exact-receipt proof; private-record non-exposure; unknown operation and
  capability absence before bootstrap.
- **GitNexus blast radius:** HIGH/CRITICAL risk must be warned and reviewed
  before existing SDK/engine method edits; current GitNexus unavailable is a
  hard implementation evidence gap.
- **Risks/stop:** public-private leakage, authority weakening, or manufactured
  transport ingress stops and preserves HCM-4.1.
- **Disallowed shortcuts:** `pub` reachability, test-only callers, recovery or
  startup ingress, raw owner record serialization, or CLI/Tauri branching.

## Packet 5 — complete proof wall and adapter handoff seam

- **Dependencies:** Packets 0-4.
- **Candidate paths/symbols:** SDK contract tests/fixtures only; HCM-4.3 and
  HCM-4.4 adapter paths are consumers and remain untouched.
- **Public/API effect:** no new semantic authority; proves the stable seam.
- **Proof gates:** the complete matrix in `proof/strategy.md`, package/docs/
  manifest/schema regeneration, direct Rust equivalence, and negative
  transport ownership.
- **GitNexus blast radius:** scoped/compare-to-main detection is required
  before commit; unavailable comparison is recorded unavailable, never GREEN.
- **Risks/stop:** any proof gap is a typed proof stop or causal supplemental,
  never a future-program footnote that permits completion.
- **Rollback/recovery:** retain the reviewed primary tip and handoff only;
  closeout artifacts are mechanical and separate.
