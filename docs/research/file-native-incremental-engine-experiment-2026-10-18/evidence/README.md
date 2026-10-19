# Evidence guide

Start with the [report](../report.md), [verification and fresh-path commands](../verification.md)
and [independent review](../review.md). The accepted brief and reconciliation in
`../inputs/` retain the supplied bytes and historical proposal status.

## Measurements and independent checks

| Artifact | Scope |
| --- | --- |
| [Measured tables](measured-tables-final.json) | Exact extracted document/history calls and homogeneous ledger groups; explicit metric scopes and nulls |
| [Contention derivation](contention-derived-final.json) | All requests/attempts, lock clocks, observed latencies, refusal outcomes and verified barrier archives |
| [Pending preparation archives](contention-pending/) | Four exact ZIPs and matching JSON receipts; barrier snapshots, not continuous physical peaks |
| [Genuine imports](genuine-source-final.json), [fresh calibration](genuine-calibration-final.json) | Independently checked selected membership and unequal source variants |
| [Encoding geometry](codec-geometry-final.json) | Independently decoded JSON/C1 logical equality and actual structural geometry |
| [Validation trace](validation-trace-derived.json) | Full-load/semantic-check clocks distinguished from complete incremental preparation |
| [Aggregate projection partial result](independent-projection-final.json) | Four completed matrix roots; original 900-second censor while visiting A100k remains |
| [Calibration continuation](calibration-projection-continuation.json), [controller](calibration-projection-continuation-controller.json), [prospective bound](calibration-projection-amendment.md) | Separate two-root check; no A100k retry |
| [Selected-root census](selected-store-census-final.json), [unselected B census](unselected-B-census-final.json) | File lengths/categories, owner charges and active reservations kept distinct |

Historical `*-progress*`, `*-development*` and `*-small*` artifacts remain separate
observations. Their filenames do not substitute for source pins or controller
dispositions. In particular, a partial output file does not imply that its outer
controller completed.

## Raw observations and source provenance

The native Linux directories retain main, correction and accounting capsules.
Mac exports use [their own evidence and verifier](../native-macos/), including
source TARs and session ZIPs. Windows and WSL final lossless exports are described
in the completed report when their closeout queues finish.

Within a lossless export, `manifest.json` records original length/SHA-256 and
compressed length/SHA-256 for each `.gz` file. Decompression yields the original
bytes. Engine events retain exact stdout as base64, its digest and byte count,
parsed JSON types, raw exit/censor state, controller clocks and executable hash.
Expected exit2 and injected exit86 are not success exits; suite assertions state
why they are expected. Synthetic attribution fixtures use separate filenames.

Session ZIPs capture working source at controller creation. They can differ from
the compiler source associated with an older binary. Use the separate Windows
[F3](windows-f3-source-binding.json), [7ac](windows-final-source-binding.json),
[7e](windows-audit-source-binding.json) and [5a](windows-accounting-source-binding.json)
receipts for those associations. [Original source capsules](source-capsules/) and
[accounting source capsule](source-capsules-accounting/) retain exact transport
bytes. These checks do not claim reproducible executable builds. Early missing
compiler-input captures remain unavailable.

[Setup failures](setup-failures/) and raw failed/censored controllers are retained.
GitNexus receipts document its stale/degraded coverage; absent indexed symbols
mean unknown impact, not zero risk. The exact [5,000-constituent packet](packets/)
has independently verified actual closure counts stated in the report.

## Verification commands

Run from this standalone experiment directory using an explicit Python executable:

```bash
python3 verify_exports.py --root evidence --output /fresh/transport-verification.json
python3 native-macos/helpers/verify_export.py native-macos
```

The generic verifier checks lossless manifests, event bytes/types and
session-source/binary associations. Mac has its separate portable verifier;
history proof capsules have `pack_history.py`. A proof capsule is not a complete
restartable mutable-store backup. The final report records whether export and
fresh restoration completed, and any retirement-related proof limitations.

The final inventory is a byte/membership snapshot of standalone source, documents
and retained evidence, excluding itself, Git metadata, caches and build targets.
It authenticates recorded bytes, not semantic conclusions or missing observations.
