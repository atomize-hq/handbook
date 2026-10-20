# Packing experiment — working report

Status at 2026-10-20 06:29 UTC: the primary Windows A long-history PZ build,
complete independent verification, matched measurements, physical cleanup and
complete post-cleanup verification passed. Packed operational capture, parent
independent transport/source verification, fresh byte restore, complete restored
oracle and read-only reopen passed. The restored valid successor, representative
N8k A/B publication comparisons, remaining record-corpus controls and native
final-source results are still pending. This is research evidence,
not approval of a production format or migration.

## Exact revisions and scope

Final Rust/Cargo core: `d70f3864f2888d46d9112213b4a03f8d82745a8e` (18 files).
Windows executable SHA-256:
`568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b`.
The original P0 containers were created by `4cb6b9f`; final P0 reads and PZ
creation/reads use d70. The original passing evidence remains attached to its
actual source and executable; it has not been relabeled.

The corrected independent Python oracle is commit
`ba2ce1bbb3e963cff298ec270841c037e0a0a5df`, SHA-256
`94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66`.
Its separate Git DLL is Zstandard **1.5.6**, SHA-256
`f216ea8efb5b3bb9ad3d938fb9ac07e3b8f78c2c92faab03d2c468d0b6798a5a`;
the locked Rust encoder uses C Zstandard 1.5.7. Actual bindings control claims.

The initial H101006 operational image was independently captured, restored,
fully verified and reopened before packing. One separately timed qualified-N
navigation publication yielded H101007 without changing its graph. All lookup
cells retain the same original early/middle/recent/missing/cancelled/lost-reply
IDs. This single primary clone is A; B correctness is separately qualified.

## Primary physical and semantic observations

| Observation | P0 raw | PZ record compression |
|---|---:|---:|
| Mapped exact raw records | 504,833 | 504,833 |
| Exact raw bytes | 579,015,428 | 579,015,428 |
| Packs / indexes / selected catalog | 16 / 16 / 1 | 16 / 16 / 1 |
| Pack + index bytes | 641,986,037 | 283,460,393 |
| Representation data allocation, including catalog | 642,060,288 | 283,537,408 |
| Engine build clock, seconds | 1,323.298 | 129.618 |

PZ uses **44.1537%** of P0 pack/index bytes, passing the frozen 20% reduction
rule. It uses identical role/range/count grouping. Raw inventory digest is
`058be9aea3ff6e4130f2a9f1af545c2f7ce895b6ebe6677d09b63e48d2f43e84`.
The permanent-history digest is
`f398ab10c2020b988d46530c6e93e0c6c6e0188d6e66e9b8e4e07e96c107f63e`.
Independent checks covered all 101,007 accepted publications, all 100,000 ledger
rows, 101 retired publications and the current graph/source/navigation oracle.

Cleanup used only the frozen engine's packing maintenance operation on the
disposable primary clone. It successfully deleted **1,221,001,465 bytes**:
504,833 mapped loose files plus 32 old P0 containers, with zero sharing deferrals.
The selected PZ representation remains 33 files; this is a **99.9935%** reduction
against the original eligible loose file count. Complete raw and semantic
verification passed again after deletion. Historical owners, unselected
catalogs, journals, source/view content, test ledger and unrelated records remain
accounted. The complete post-cleanup store is 5,812 files / 592,459,049 logical
bytes / 601,891,608 regular-file allocation bytes. Excluding the 246,560,316-byte
test ledger, engine storage is 5,811 files / 345,898,733 logical bytes /
355,328,792 allocated data bytes. The bounded reopen observation passed 111
engine events including all 101 explicit retired-content refusals.

The eligible loose records' measured regular-file data allocation was
1,047,946,944 bytes. These immutable duplicates were measured while coexisting;
whole-store loose allocation before activation was not measured. FileStandardInfo
AllocationSize excludes directory/MFT metadata. No physical device write or
device-cold claim follows from logical counters or fresh process starts.

## Matched lookup observations

Warm means 500 repeated samples per category after first lookup in one process.
Fresh means 20 process starts per category with uncontrolled OS/device caches.
The first per-category lookup shares preceding categories in the resident run.

| Category | Qualified L µs | Final P0 µs | P0 / L | Final PZ µs | PZ / P0 |
|---|---:|---:|---:|---:|---:|
| early | 987.50 | 963.35 | 0.976× | 1048.50 | 1.088× |
| middle | 984.90 | 929.95 | 0.944× | 1010.60 | 1.087× |
| recent | 966.05 | 930.70 | 0.963× | 958.15 | 1.029× |
| missing | 611.25 | 608.20 | 0.995× | 590.35 | 0.971× |
| cancelled | 641.65 | 666.55 | 1.039× | 647.25 | 0.971× |
| lost reply | 948.20 | 993.60 | 1.048× | 966.95 | 0.973× |

Both frozen warm 2× gates pass. Fresh-process medians remain about 18–45 ms for
P0/PZ versus 1.0–1.4 ms for qualified L, reflecting the observed cost of loading
indexes in a new process. Complete distributions, instrumented opens/storage and
decoded bytes, cache charge and RSS remain in the raw table archive. Exact
matching outcomes, matching/changed supplied intent and original request replay
were checked separately from the warm samples.

## Correctness qualifications and preserved failures

Final d70 Windows qualification passed five locked/offline Rust workspace tests,
54 packing cases / 275 engine events, 24 strict-input cases and nine focused B
control regressions / 26 engine events / nine ordinary PowerShell processes.
The last set includes a selector captured before later publication,
cleanup/repack, retirement and an ordinary tool-free selected-once exact copy.

Old 4cb cleanup removed the B publication needed by ordinary selected-once shell
discovery. Its immediate failure remains preserved. Final d70 validates and
retains every mapped B publication loose duplicate, including historical and
retired control bindings; these duplicates receive no deletion credit. Missing
or conflicting B control duplicates refuse. The resulting B file-count tradeoff
must be charged to the frozen 95% rule; passing A does not establish B's gate.

The first full PZ Python semantic check failed because concurrent worker calls
shared one non-thread-safe decompression context. All sequential raw records had
already been checked. A diagnostic on 20 exact frames reproduced bad concurrent
results and exited abnormally; that run is not a process pass. The oracle-only
mutex correction then passed 10,000 concurrent requests and the complete oracle,
before any primary cleanup. No Rust rebuild or packed-byte repair was used.

The first artifact gzip capture stopped at its resource bound; its 66,100,201-byte
prefix is retained and never treated as a restorable archive. A separately
bounded XZ archive subsequently preserved and independently stream-verified all
568 artifact files / 440,175,466 bytes. Only the parent-approved 566 exact regular
files were reclaimed, crediting 438,731,626 successful bytes; two required helper
executables remain in place. All binaries, failed prefixes, cache fixtures,
original datasets and original operational captures are retained as recorded.

## Remaining scope and limits

The packed operational image preserves 5,807 files / 592,458,971 raw bytes. Its
gzip transport, inventory and manifest total 314,297,593 bytes. Parent independent
verification streamed every frame/path/size/raw hash/footer, checked exact source
file-set equality and source hashes, and reached gzip EOF. Transport SHA-256 is
`2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b`; inventory
SHA-256 is `0974f4a1dac1f13618105b9fca425b2a56c99528accecb5255f0d4ec6515b9b5`.
Byte restore took 3.469 seconds, with an all-thread prewrite cap and no imported
disposable authority. The complete fresh-restored oracle passed in 89.140 seconds,
then 111 read-only reopen events confirmed the original outcomes, intent checks
and all 101 retired refusals. A new local authority and valid successor still
await the separate conditional admission; no original or primary publication was
added by these restore checks.

The restored valid successor, N8k A/B
ordinary-publication gates, H1k/H10k container controls and final native platform
results must be reported from actual receipts. No pending stage is a pass.
Current additional fixture grants are finite and coordinated globally; prospective
release is never counted before actual deletion. The original functional cutoff
is 07:55:16 UTC and closeout deadline is 08:25:16 UTC on 2026-10-20.

Early individually guarded overlaps did not establish continuous host-wide
aggregate enforcement. Later serialized stages do not repair that historical
qualification. Process exits are not power-loss tests; Windows directory sync
remains unsupported. GitNexus does not register this isolated experimental
worktree, so attempted impact/detect calls remain UNKNOWN with manual traces,
never a claimed clean graph result. No production adoption is authorized.

Recommendation remains pending the required restore/publication/native results.
