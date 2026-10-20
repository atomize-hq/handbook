# Provisional CP1 native adapter map

Read-only inspection; no engine qualification. CP1 checkout HEAD was still
`47d667f26a75fa1454dee2d1c50d238aec4cf6bd`, with uncommitted source in progress.
Observed files: `physical_lifetime.rs` SHA-256
`f1d5c03fec00cbda48720fe9a0aafdbcb8e8eaf9375a6848f5be0f809c784406`,
`packing.rs` SHA-256
`5186a8d87d3655a38bdf27cd533472877a2151cc04b454350eae25fdca2b6746`,
`pack.rs` SHA-256
`b452e2d6fe7d794953ed5c8a1e8d0020dd9241eca05a36c55518e7063533f755`.
These observations must be rebound against the eventual exact committed pin.

## Format and independent verification

- STORE versions 1/2 remain legacy. Version 3 adds closed
  `physical: {id, ordinal, plan}` alongside the selected catalog hash.
- Catalog version 2 has instance-bearing descriptors; instance paths are
  `packs/<instance>/<hash>.pack` and `.idx`. Flat legacy paths remain readable
  only under their existing format. Equal content may have different instances.
- FNPIDX01 whole-index format is unchanged at CP1. The independent decoder needs
  explicit STORE/catalog/descriptor versions and paths; it must not remove any
  index ordering, coverage, raw hash, bounds or semantic checks.
- `physical-pins/<id>.lock` stores strict `PinIdentity` JSON (version, store_id,
  id, ordinal, catalog). It is required durable identity metadata, despite its
  suffix. Quiescent backup must preserve its bytes. Do not exclude every `.lock`.
  Global `physical-capture.lock`, `pack-readers.lock`, `pack-maintenance.lock` and
  other already-excluded process gates remain distinct ephemeral lock files.
- `physical-plans/<hash>.json` is immutable selected work authority.
  `physical-progress/<generation>.json` is a mutable checkpoint tied to that plan.
  Progress does not establish activation or deletion credit. Include required work
  and candidate ownership in backup/restore, and retain source schema bindings.

The shared harness commit `01ff272a98fe2cddf01f8ebfc399192f234051af` currently
contains the Windows process driver and baseline controls, not a completed CP1
format decoder. Linux will reuse its reviewed independent decoder when supplied;
this worker does not fork the semantic verifier.

## Barrier and case correspondence

| Existing/new schedule | Provisional CP1 hook and expectation |
| --- | --- |
| Physical/semantic capture interval | `pack-read-lease` and `pack-catalog-captured` now occur after generation pin acquisition, Q release and catalog load, before later semantic work. For legacy conversion a held lease delays conversion; a v3 pin permits successor activation. |
| Initial old-client drain | `pack-reader-drain`; conversion uses nonblocking legacy gate attempts under Q. Timeout is `busy:legacy-readers`, not the old `busy:physical-readers`. Preserve the legacy case and version-scope new expectations. |
| Candidate fully prepared | `pack-before-catalog`, `physical-before-selection`; selected STORE is unchanged before release/selection. |
| Selected before journal | `pack-catalog`, `physical-after-selection`; both are outside Q/W and before `PACKING.json` registered update. Recovery must follow STORE even with installed journal text. |
| Journal update | `pack-accounting`; selected generation must remain authoritative across later successors. |
| Loose deletion | `physical-before-delete`, then `pack-loose-delete` after actual removal; use EXP_NTH to retain first/third prefixes and independent exact oracles. |
| Actual old container deletion | `pack-old-delete` after an actual pack or index remove. Descriptor instance paths must prevent equal-content re-adoption. |
| Cancel/error/death | `EXP_PACK_CANCEL`, `EXP_ERROR`, `EXP_EXIT` and killed held processes; prove only ended process pins release, persistent owners remain. Native outer guard retains escaped descendants and positive closure. |

`cleanup` returns one bounded batch. Drain loops must retain each response, full
pending counts, blockers and actual file inventory. A `complete` response may
still include `protected_metadata_until_successor`: old pin identity metadata is
intentionally retained until a successor omits it. Account it; do not claim zero
files or unprotected backlog from a status string alone. Metrics include raw
`physical_activation_holds_ns`, batch time and native deletion maxima.

CP1 deliberately retains full/frozen cohort discovery. It can establish held
generation and successor lifetimes, but does not complete rolling discovery or
the newly accepted, previously unmapped capture-gap compaction proof. That proof
must be completed at CP2 and rerun on the final pin. No CP1 result may silently
substitute for the final rolling/paged/compatibility matrix.

## Native execution adapter boundary

The integrated new `execution/harness/driver.py` imports Windows NT Job symbols
and starts suspended Windows processes. It is not directly importable on Linux.
A thin native adapter should preserve its explicit executable/output constructor,
raw append-only request/response/event schema and reader/checker limits, while
using the accepted native outer guard and Linux process launch/memory observations.
Any old `Driver(base)` capsule bridge must be explicit and separately source-bound.
No Windows-specific import should be hidden by silently disabling process controls.

Before use: bind exact engine/harness pin and hashes, native binary/dependencies,
guard hash and sealed category claims; freeze operation/stage caps; reserve each
fixture/backup/restore coexistence prospectively. The large cache unit is final-pin
only, with `EXP_PACK_TEST_ROOT` pointing inside admitted runtime fixtures. No
engine source edits, builds or corpus writes occurred during this inspection.
