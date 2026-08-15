# HCM-4.2 Shared DTO and JSON Schema Membrane

This document owns the compact outcome authority for HCM-4.2. It defines the
shared result to be established across the slice, the invariants every packet
must preserve, and the ordered packet boundaries. It does not authorize packet
execution.

## Current admission state

**HCM-4.2 outcome authority exists after this commit, but no new packet is
admitted.**

- Packet 0 is an implemented checkpoint at
  `8605d6539355f350c55b921d95fb450bca30ee18`; its product and focused-test paths
  remain byte-unchanged on this clean branch.
- No authority file exists for Packets 1 through 5, and none of those packets is
  authorized for execution.
- Phase-3 exit is a hard predecessor to Packet-1 dispatch.
- The latest Phase-3 exit attempt stopped at strict Clippy with nine existing
  violations: `ptr_arg`, three `unnecessary_map_or`, `redundant_closure`,
  `large_enum_variant`, two `type_complexity`, and `unnecessary_sort_by`.
- Packet 1 may be admitted only after separately authorized remediation and a
  successful rerun that exits Phase 3. This document grants neither authority.

The [`phase-and-slice map`](../../phase-and-slice-map.md) owns that predecessor
and exit state. A later authority must not reinterpret Packet 0's chronology or
this outcome document as permission to bypass it.

## Responsibility and outcome

HCM-4.2 establishes one bounded shared DTO and JSON Schema membrane through
which semantic owners expose exact public truth without transferring their
authority to the SDK or an adapter.

The completed slice must make these surfaces agree:

- exact bounded public types and checked-in schemas;
- authoritative operation definitions and independent owner admission;
- generic ordinary request, response, and admission-refusal envelopes;
- immutable bootstrap and locally closed catalog discovery;
- bounded posture request, outcome, problem, and receipt projections; and
- deterministic schema generation and exact-byte manifest identity.

This is outcome authority, not a claim that the outcome exists now. Detailed
current product classification remains in the
[`implemented baseline`](../../implemented-baseline.md), stable ownership and
dependency direction remain in the
[`architecture context`](../../architecture-context.md), and migration routing
remains in the [`seam crosswalk`](../../seam-crosswalk.md).

## Durable cross-packet invariants

Every packet must preserve all of the following:

1. Engine, Flow, and pipeline retain semantic authority. The SDK projects
   bounded public truth, and adapters create no owner back-edges.
2. Public DTOs are bounded projections, never raw private owner records.
3. Stable identity plus full SemVer forms every exact public reference. Ranges,
   `latest`, CLI names, and examples never select identity.
4. Instance canonical fingerprints, exact checked-in schema-byte fingerprints
   with one terminal LF, and adapter framing remain separate identity domains.
5. Unknown fields, discriminants, and enums; invalid nulls; duplicates; unsafe
   locators; bounds violations; noncanonical order; and fingerprint mismatch
   fail before trust.
6. Terminal closure preserves four fields. `data` is non-null only for `ok`,
   and exactly one terminal array is populated for every other outcome.
7. Raw idempotency keys remain bounded and redacted. They never cross response,
   receipt, diagnostic, replay, manifest, log, or public-state boundaries; only
   scoped fingerprints may cross.
8. Adapters own parsing, framing, correlation, presentation, and transport
   failures, never operation identity, DTO shape, owner decisions, or semantic
   branches.
9. Discovery and admission stay honest. A Rust method alone is only a precursor;
   exact definition, schema, owner, receipt, and proven-current transport state
   are required before discovery. That state may honestly be `none`; HCM-4.2
   does not require a CLI or Tauri transport.
10. Structural admission is not semantic trust. A later owner witness must
    preserve Packet 0's sealed structural-to-trusted boundary.

These invariants refine the program boundaries without duplicating their full
rationale. If a packet needs to change one, that change requires explicit new
authority rather than an implementation-time exception.

## Packet 0 implemented boundary

Packet 0 is a transport-free, structurally safe Rust kernel under
[`crates/sdk/src/contract_membrane/`](../../../../../crates/sdk/src/contract_membrane/).
It is an implemented foundation, not an operation surface, owner, or adapter.
No separate retrospective Packet-0 authority file is required.

Its public surface provides:

- bounded values, integers, timestamps, collections, and limit profiles;
- stable exact SemVer references, bindings, and SHA-256 fingerprints;
- portable artifact locators, digest bindings, and artifact references;
- shared problems, diagnostics, actions, omissions, schema-manifest entries,
  receipts, correlations, terminal fields, and idempotency states;
- catalog root, cursor, and page primitives with local structural closure;
- bounded redacted raw-key handling that exposes no public raw-key accessor;
- canonical serialization and instance fingerprinting; and
- sealed witnesses that separate structural admission from owner trust.

The concise export inventory is
[`contract_membrane/mod.rs`](../../../../../crates/sdk/src/contract_membrane/mod.rs),
and the focused regression anchor is
[`contract_membrane.rs`](../../../../../crates/sdk/tests/contract_membrane.rs).
The canonical domain is integer-only and JCS-compatible: it rejects floats and
does not claim unrestricted RFC 8785 number support.

Packet 0 does not provide operation definitions, checked-in schema publication,
schema generation, discovery/bootstrap producers, owner admission, posture
mapping, a CLI/Tauri route, or any future contract, gate, dock, publication,
bridge, or adoption owner.

## Ordered packet decomposition

| Packet | Bounded outcome | Dependency and present status |
| --- | --- | --- |
| **0 — implemented typed kernel** | Preserve the bounded owner-neutral types, exact identities, canonicalization, catalogs, redacted raw-key bridge, terminal closure, and sealed verification described above. | Implemented checkpoint at `8605d65`; byte-unchanged; no retrospective authority file. |
| **1 — schema generation and manifest** | Select and pin a generator; define the exact Packet-0 schema inventory and file mapping; produce a checked-in Draft 2020-12 tree and deterministic exact-byte fingerprint manifest; define the parser-versus-schema validation boundary. | Depends on Packet 0 and Phase-3 exit. Generator, inventory, and validation responsibility are unresolved; no authority file exists. |
| **2 — operation definitions and owner admission** | Define the exact operation/admission model and later 62-row inventory; assign generic ordinary request, response, and admission-refusal envelopes; resolve source-state inconsistencies. The matrix is not part of this authority. | Depends on Packet 1. Envelope ownership, details, and classifications remain unresolved; no authority file exists. |
| **3 — immutable bootstrap and catalogs** | Define descriptor/bootstrap shapes, capability and owner-version entries, production schema/catalog witness, and immutable roots, paging, cursors, and negotiation. | Depends on Packet 2. Catalog representation and production witness remain unresolved; no authority file exists. |
| **4 — posture projection and direct-Rust admission** | Map current engine-owned outcomes into bounded public requests, results, problems, and receipts without exposing private records or moving authority. | Depends on Packet 3. Exact mapping, correlation, and current-transport admission state remain unresolved; `none` remains valid; no authority file exists. |
| **5 — integrated proof** | After the preceding work, prove regeneration, closed shapes, identity-domain separation, owner parity, replay identity, raw-key absence, catalog closure, and adapter non-authority. | Depends on Packets 1–4. It creates no old proof wall or separate closeout ceremony; no authority file exists. |

The required dependency order is:

```text
Packet 0 -> Packet 1 -> Packet 2 -> Packet 3 -> Packet 4 -> Packet 5
```

Each completed packet returns control to the meta task under the
[`work-unit lifecycle`](../../work-unit-lifecycle.md). Packet completion never
selects or authorizes the next packet. Slice closeout occurs once, after Packet
5, under the [`review and closeout policy`](../../review-and-closeout-policy.md).

## Slice completion conditions

HCM-4.2 completes only when:

- one bounded type/schema/operation membrane agrees with independent owner
  admission, bootstrap/catalog discovery, and posture mapping;
- owner trust and the four terminal outcomes remain closed end to end;
- schema files and their exact-byte manifest regenerate deterministically;
- all Packets 0 through 4 satisfy their authorized boundaries;
- Packet 5 supplies integrated evidence for their shared invariants; and
- one aggregate review and closeout confirms the whole authorized outcome under
  current policy.

Packet 0 alone does not satisfy these conditions. Neither the existence of this
document nor a packet-level result establishes slice completion.

## Non-goals

HCM-4.2 completion does not require or authorize:

- CLI or Tauri adapter implementation;
- future contract, evidence, gate, or dock owners;
- publication, a replaceable bridge, or Substrate adoption;
- future HCM-5 contract-crate placement, extraction, runtime, or dock choices;
  or
- deferred Phase-3 gate, broader posture-policy, publication, or adoption work.

Those concerns remain with their later owners even when they consume the shared
membrane.

## Open authority decisions

This document deliberately does not select:

- the Packet-1 generator, exact schema inventory, file mapping, or division of
  parser and schema responsibility;
- the Packet-2 envelope owner or details, operation model, or 62-row
  classifications;
- Packet 3's typed-page versus `ContractValue` representation, production
  witness, or transport list;
- Packet 4's exact public mapping, correlation model, or current-transport
  admission rule;
- a future contract owner's crate placement, extraction strategy, or runtime;
- future dock implementation choices; or
- CLI/Tauri routes, publication, bridge, external adopter, or broader Phase-3
  posture and gate policy.

Each decision belongs in the applicable future authority after its hard
predecessors are satisfied. Until then, implementations, plans, examples, and
historical material cannot supply the missing choice.

## Use and drift

Use this document to author a later bounded packet authority and to review
cross-packet consistency. Use the
[`task-session prompt`](../../task-session-prompt.md) only after the meta task
has selected a packet that current authority actually admits.

If executable product truth changes Packet 0, update the implemented baseline
and this authority in the same bounded change or record the discrepancy as
documentation drift. Never use this document to override newer source or tests,
infer an external condition, or grant permission that its admission state
withholds.
