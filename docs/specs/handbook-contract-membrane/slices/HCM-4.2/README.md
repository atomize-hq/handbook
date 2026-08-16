# HCM-4.2 Shared DTO and JSON Schema Membrane

This document records the completed outcome authority and single aggregate
closeout for HCM-4.2. It defines the shared result established across the
slice, the invariants every packet preserved, and the packet boundaries. It no
longer delegates active packet work and does not authorize adjacent work.

## Current completion state

**HCM-4.2 is complete. Packets 0 through 5 and the one aggregate closeout are
finished; no HCM-4.2 packet remains eligible for dispatch.**

- Packet 0 is an implemented checkpoint at
  `8605d6539355f350c55b921d95fb450bca30ee18`.
- Packets 1 through 5 completed at `f54dcbd`, `4d70701`, `241b998`, `caf0ef4`,
  and `c1f5d33`. Their linked checkpoint documents record the selected
  generator/schema, operation/admission, bootstrap/catalog, posture, and
  integrated-proof boundaries.
- Aggregate verification passed the three-test Packet-5 integrated proof,
  confirmed the package manifest plus exact 53-schema tree, and built strict
  SDK rustdoc. One aggregate reviewer returned `APPROVE/PROCEED` with no P1 or
  P2. Its P3 stale-status observation is corrected by the current navigation
  update; its P3 bootstrap defense-in-depth observation remains non-blocking.
- The Phase-3 hard predecessor was satisfied under the selected reduced
  boundary before the remaining packet sequence completed.

The [`phase-and-slice map`](../../phase-and-slice-map.md) owns that predecessor
and exit state. A later authority must not reinterpret Packet 0's chronology or
this outcome document as permission to bypass it.

## Responsibility and outcome

HCM-4.2 establishes one bounded shared DTO and JSON Schema membrane through
which semantic owners expose exact public truth without transferring their
authority to the SDK or an adapter.

The completed slice made these surfaces agree:

- exact bounded public types and checked-in schemas;
- authoritative operation definitions and independent owner admission;
- generic ordinary request, response, and admission-refusal envelopes;
- immutable bootstrap and locally closed catalog discovery;
- bounded posture request, outcome, problem, and receipt projections; and
- deterministic schema generation and exact-byte manifest identity.

This is the completed outcome authority and aggregate closeout. Detailed
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
| **0 — implemented typed kernel** | Preserve the bounded owner-neutral types, exact identities, canonicalization, catalogs, redacted raw-key bridge, terminal closure, and sealed verification described above. | Implemented checkpoint at `8605d65`; Packet-0 behavior remains regression-protected; no retrospective authority file. |
| **1 — schema generation and manifest** | Select and pin a generator; define the exact Packet-0 schema inventory and file mapping; produce a checked-in Draft 2020-12 tree and deterministic exact-byte fingerprint manifest; define the parser-versus-schema validation boundary. | Completed at `f54dcbd`; decisions remain recorded in [`packet-1-schema-generation.md`](packet-1-schema-generation.md). |
| **2 — operation definitions and owner admission** | Define the exact operation/admission model and 62-row inventory; assign generic ordinary request, response, and admission-refusal envelopes; resolve source-state inconsistencies. | Completed at `4d70701`; decisions remain recorded in [`packet-2-operation-admission.md`](packet-2-operation-admission.md). |
| **3 — immutable bootstrap and catalogs** | Define descriptor/bootstrap shapes, capability and owner-version entries, production schema/catalog witness, and immutable roots, paging, cursors, and negotiation. | Completed at `241b998`; decisions remain recorded in [`packet-3-immutable-bootstrap.md`](packet-3-immutable-bootstrap.md). |
| **4 — posture projection and direct-Rust admission** | Map current engine-owned outcomes into bounded public requests, results, problems, and receipts without exposing private records or moving authority. | Completed at `caf0ef4`; decisions remain recorded in [`packet-4-posture-projection.md`](packet-4-posture-projection.md). |
| **5 — integrated proof** | After the preceding work, prove regeneration, closed shapes, identity-domain separation, owner parity, replay identity, raw-key absence, catalog closure, and adapter non-authority. | Completed at `c1f5d33`; evidence selection remains recorded in [`packet-5-integrated-proof.md`](packet-5-integrated-proof.md). |

The required dependency order is:

```text
Packet 0 -> Packet 1 -> Packet 2 -> Packet 3 -> Packet 4 -> Packet 5
```

Each completed packet returned control to the meta task under the
[`work-unit lifecycle`](../../work-unit-lifecycle.md). The meta task sequenced
the declared order, and slice closeout occurred once after Packet 5 under the
[`review and closeout policy`](../../review-and-closeout-policy.md).

## Completed just-in-time preparation

The packet-by-packet execution request authorized the meta task to continue
through the ordered sequence above. Packets 2 through 5 used the delegated
preparation boundary and recorded compact decision checkpoints before their
product changes. That delegation is consumed and supplies no continuing
implementation authority.

Every completed preparation phase reconciled its assigned packet against current
product source and tests, this authority, the
[`implemented baseline`](../../implemented-baseline.md),
[`architecture context`](../../architecture-context.md),
[`seam crosswalk`](../../seam-crosswalk.md), and
[`phase-and-slice map`](../../phase-and-slice-map.md). The archived HCM-4.2
`SPEC.md` and `tasks/plan.md` at ref
`archive/handbook-contract-membrane-pre-clean-20260812` are adopted only as
bounded planning input for the assigned packet's intended outcome, constraints,
and candidate design. Their status, authority claims, selectors, proof walls,
handoffs, and closeout procedure remain inactive.

Packet 2 additionally consulted the archived
`research/20260811T031843Z--corrective-owner-admission-matrix.md` under that same
HCM-4.2 ref solely as the candidate 62-row operation, ownership, mutability,
precursor, transport, and discovery classification. It rechecked every
material classification against current source and current Phase-3/Phase-4
boundaries. The archived matrix cannot prove current implementation or decide a
conflict by itself.

The permitted sources yielded one bounded answer for each completed packet.
This historical preparation record does not authorize reopening a packet or
using its adoption rule for HCM-4.3.

## Satisfied slice completion conditions

HCM-4.2 completed after:

- one bounded type/schema/operation membrane agrees with independent owner
  admission, bootstrap/catalog discovery, and posture mapping;
- owner trust and the four terminal outcomes remain closed end to end;
- schema files and their exact-byte manifest regenerate deterministically;
- all Packets 0 through 4 satisfy their authorized boundaries;
- Packet 5 supplies integrated evidence for their shared invariants; and
- one aggregate review and closeout confirms the whole authorized outcome under
  current policy.

No single packet established that result. The aggregate verification and
review recorded above supplied the one slice closeout.

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

## Deferred later-slice decisions

Packet 1's generator and schema boundary, Packet 2's operation/admission model,
Packet 3's bootstrap/catalog representation, Packet 4's posture mapping, and
Packet 5's proof surface are implemented and remain recorded in their linked
checkpoints. Only later-slice concerns remain deferred:

- a future contract owner's crate placement, extraction strategy, or runtime;
- future dock implementation choices; or
- HCM-4.3 CLI parity, HCM-4.4 Tauri, publication, bridge, external adopter, or
  broader Phase-3 posture and gate policy.

Each later decision requires its own current authority. HCM-4.2 historical
material cannot override current source or supply a later consequential choice
by itself.

## Use and drift

Use this document to review the completed HCM-4.2 boundary and cross-packet
consistency. Do not dispatch another HCM-4.2 packet from it.

If executable product truth changes Packet 0, update the implemented baseline
and this authority in the same bounded change or record the discrepancy as
documentation drift. Never use this document to override newer source or tests,
infer an external condition, or expand the slice beyond its stated outcome.
