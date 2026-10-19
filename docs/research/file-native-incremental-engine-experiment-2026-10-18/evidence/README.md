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
| [Corrected A100k strict audit](strict-large-audit-derived.json) | Completed 7e audit; exact event hash and counters, separately qualified from the independent-renderer censor |

Historical `*-progress*`, `*-development*` and `*-small*` artifacts remain separate
observations. Their filenames do not substitute for source pins or controller
dispositions. In particular, a partial output file does not imply that its outer
controller completed.

## Raw observations and source provenance

The native Linux directories retain main, correction and accounting capsules.
Mac exports use [their own evidence and verifier](../native-macos/), including
source TARs and session ZIPs. [Windows](windows-raw-final/manifest.json) and
WSL [main](native-wsl-main/manifest.json), [audit](native-wsl-audit/manifest.json)
and [accounting](native-wsl-accounting/manifest.json) exports are complete.
[Final transport verification](transport-verification-final.json) checks all 13
lossless manifests; Mac's latest separate result is
[the accounting-inclusive verification](mac-transport-accounting-verification.json).

The [full proof export](history-proof-final/) was censored and has no completed
manifest; its partial gzip is not a restorable capsule. The separately bounded
[ledger-only transport](history-ledger-transport-final/manifest.json) preserves all
100,000 receipts through exact-byte/dense-row checks. Dependent restoration and
long-history collection were unperformed. [Final dispositions](final-dispositions.json)
record that distinction. [Resource census](final-resources.json) separately records
actual file-length sums, reserves and the stated process-filter scope.

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
python3 index_evidence.py --output evidence/inventory-final.json --verify
```

The generic verifier checks lossless manifests, event bytes/types and
session-source/binary associations. Mac has its separate portable verifier;
history proof capsules have `pack_history.py`. A proof capsule is not a complete
restartable mutable-store backup. The final report records whether export and
fresh restoration completed, and any retirement-related proof limitations.

The final inventory is a byte/membership snapshot of standalone source, documents
and retained evidence, excluding itself, Git metadata, caches and build targets.
It authenticates recorded bytes, not semantic conclusions or missing observations.
The local `.gitattributes` disables checkout newline conversion in this standalone
directory so these hashes retain their meaning across exact Git materialization.
