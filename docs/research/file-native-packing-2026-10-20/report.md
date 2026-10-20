# Packing experiment — bounded results

Functional closeout: **requested P scope remains incomplete**. The four N8000-B
comparison cells are **unperformed / time-and-coexistence-censored**; their gates
are null, not failed-engine outcomes or passes. Final Mac F qualification is also
time-censored. No functional work continues beyond 07:55:16 UTC.

The primary Windows A long-history PZ build,
complete independent verification, matched measurements, physical cleanup and
complete post-cleanup verification passed. Packed operational capture, parent
independent transport/source verification, fresh byte restore, complete restored
oracle, read-only reopen and valid successor passed. H1k/H10k record controls and
the complete final-source Windows F recovery capsule passed. The authentic
N8000-A baseline and all four valid matched measurement cells passed their
correctness oracles. Its low-history file-count gate failed. The authentic
N8000-B baseline passed, but its comparison cells were not admitted. This is research evidence,
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

## Operational restore and controls

The packed operational image preserves 5,807 files / 592,458,971 raw bytes. Its
gzip transport, inventory and manifest total 314,297,593 bytes. Parent independent
verification streamed every frame/path/size/raw hash/footer, checked exact source
file-set equality and source hashes, and reached gzip EOF. Transport SHA-256 is
`2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b`; inventory
SHA-256 is `0974f4a1dac1f13618105b9fca425b2a56c99528accecb5255f0d4ec6515b9b5`.
Byte restore took 3.469 seconds, with an all-thread prewrite cap and no imported
disposable authority. The complete fresh-restored oracle passed in 89.140 seconds,
then 111 read-only reopen events confirmed the original outcomes, intent checks
and all 101 retired refusals. A new local authority and valid successor
received separate conditional admission. Fresh local setup bound the canonical
restored root and new marker identity, then a real label change published
sequence 101007. Its new publication/result/hint/candidate/allocation stayed
loose. All original outcomes, intent behavior and 101 explicit retirement
refusals passed again. The complete successor oracle verified 101,008
publications, the unchanged original 504,833 packed records, 100,000 ledger rows,
101 retirements and valid current graph/navigation. Its new permanent digest is
`33ed5b72937e57c2496dbdbcca1b606f5ac59bd892f70cab2ef07a1e91944ea2`.

The successor image was independently captured and parent-verified through every
frame/hash and EOF: 5,823 files / 592,508,214 raw bytes; three archive files total
314,328,654 bytes. Transport SHA-256 is
`04a34211ea8efc74127e1d9f69f9e67bde788a17f9dac5c59a44c167cb48b081`.
After separate parent approval, the exact inventoried regular files in each
private clone were reclaimed: 5,807 / 592,458,971 bytes, then 5,823 /
592,508,214 bytes. Both complete verified images, all raw evidence and all
existing excluded control files/directories remain; protected older datasets
and archives were untouched. The read-only first reclamation dry run refused an
incorrectly constructed root-allowlist array before deletion; its source/failure
remain, and only array construction was corrected before the reviewed execution.

| Authenticated record corpus | Records | Raw P0 pack/index bytes | PZ pack/index bytes | PZ / P0 |
|---|---:|---:|---:|---:|
| H1k | 4,800 | 5,885,907 | 2,626,886 | 44.6301% |
| H10k | 49,800 | 62,996,137 | 27,715,268 | 43.9952% |

These corpora contain exact original accepted prefix records and still-retained
finalized attempts from the authenticated post-GC transport. They have no STORE
or SELECTED and are not shortened operational histories. Existing H1k raw data
was retained, not rerun. All new containers passed independent complete tuple
checks and matching P0/PZ role/range/count grouping. The helper uses the actual
frozen pack implementation (SHA-256 `4d854885a54ee95ffed44bfec704f95c5c91d95b98964d419a7607dd2a48b2e3`),
with no Rust rebuild. The protected original ledger/first-publication selector
checks were read-only and SHA-bound before and after.

Final d70 also passed a fresh complete F recovery capsule: 24 cases / 282 engine
events, exactly matching the original case identities and expected status map,
including the impossible-final-namespace expected refusal. Its actual source,
binary, child resources and fresh fixtures are separately preserved from the old
4cb run. No old result was relabeled as a final-source pass.

## Representative collection and research gates

All four valid A cells use the same authentic N8000/H8 operational image, six
original request categories, 500 warm samples per category, 120 fresh starts and
the same seven real operations: payload, batch payload, label, rename, batch
rename, explicit path-only move and relationship. Each reached H15. Their seven
logical state digests match qualified L, and each passed complete semantic,
source/navigation and full operational capture/source equality checks.

| N8000-A cell | Median complete prepare + select, ms | / qualified L | Frozen ordinary gate |
|---|---:|---:|---|
| Qualified L | 370.8002 | 1.000 | pass |
| New loose | 367.0095 | 0.990 | pass |
| P0 | 364.7921 | 0.984 | pass |
| PZ | 386.1715 | 1.041 | pass |

Warm P0/qualified-L ratios are 0.941–1.022; PZ/P0 ratios are 1.006–1.084, so the
two warm 2× gates pass. New loose tails are measured separately and never
substitute for the original keys. Exact distributions/counters/RSS are retained.

The frozen eligible cohort is only 28 records / 1,804,278 raw bytes. Both P0 and
PZ use five packs, five indexes and one catalog: **11 representation files,
60.7143% reduction — FAIL against the unchanged 95% file rule**. P0 pack/index
bytes are 1,807,781; PZ is 412,467 (22.8162% of P0), passing compression. This
negative short-history file result remains separate from the passing long A
history result. Packing owners/journals/control files are additionally charged
to whole-store footprint; omitting them cannot improve this failed criterion
into a pass. B's required loose publication duplicates must also be charged.

The first old-L cell used an invalid explicit path lacking the required
`docs/item-` prefix. The engine correctly refused `semantic:local-I1` after five
accepted publications. Its H13 clone, failed events, timings, source snapshot and
complete lossless archive were preserved separately. No selector was rewound.
The valid `-path-v2` epoch starts from a fresh authenticated baseline image; the
pre-publication tuple inventory was checked equal to the original frozen cohort.
Only the valid complete epoch enters matched tables. Literal source-pin
clarifications distinguish qualified old baseline creation from the frozen d70
candidate observer; original manifests/events/snapshots were not overwritten.

## B censor and native qualification

The N8000-B baseline completed genuine initialization and seven publications,
reached H8, and passed its full 8,000-document oracle, complete capture and both
worker and parent independent source/transport checks. Its image contains
24,515 files / 320,288,418 raw bytes; the three archive files total 13,847,818
bytes. Transport SHA-256 is
`c5a8f54d0e1c4de85b795d9b4da2a4b8deedfa9c1a0b0e79edd312744aa7af14`.
Initialization took 96.266 seconds; actual baseline publication costs included
39.235, 37.187, 37.781 and 38.063 seconds. These are full-generation baseline
costs and cannot establish any packing-attributable B regression.

Its final guard closed with ownership 2,160,637,250 bytes / 60,730 files. Another
600,000,000-byte B cell plus 64 MiB archive and 32 MiB returns exceeds the
2.5 GiB Windows cap while the baseline is retained. Exact reclamation followed
by seven approximately 39-second publications and full checks could not finish
within the original remaining interval. Parent retained both the B source store
and archive and admitted no B comparison cell. No N, history, sample, request or
pin requirement was reduced; no deadline was extended.

Native Linux/ext4 final-candidate evidence reports five units, four immediate
A/B probes, 52 packing cases / 267 events, 24 strict cases, eight focused cases /
25 events, 17 ordinary shell processes and a separate fresh F 24 cases / 282
events. Three Windows-sharing checks are explicit platform SKIPs. The final
Linux executable is SHA-256
`a8501737ca4fdd5cfc177620860c1ae103d56f2e9b2151104a922df511a3d84b`;
all 18 core files match d70. Earlier partial/bootstrap failures remain distinct.
The consolidated native map is retained with exact source, binary, library and
preservation bindings; it does not claim a global P pass.

Parent's final Mac functional status at its fixed 07:52 cutoff: immediate A/B,
52 packing cases / 267 events, 24 strict cases, eight focused cases / 25 events
and all 17 authored Bash processes passed. Fresh final-source F stopped at
37 / 282 events and is TIME-CENSORED. Overall Mac P qualification is incomplete;
older N recovery evidence cannot substitute. Shutdown returned EPERM for PGID
8284 with a zero-RSS row; process-group closure remains unknown pending the
parent's read-only audit. Zero RSS is not proof of closure. Exact final Mac
artifact and executable bindings will be recorded only when supplied by parent.

No absent stage or native partial phase is a pass.
Current additional fixture grants are finite and coordinated globally; prospective
release is never counted before actual deletion. The original functional cutoff
is 07:55:16 UTC and closeout deadline is 08:25:16 UTC on 2026-10-20.

Early individually guarded overlaps did not establish continuous host-wide
aggregate enforcement. Later serialized stages do not repair that historical
qualification. Process exits are not power-loss tests; Windows directory sync
remains unsupported. GitNexus does not register this isolated experimental
worktree, so attempted impact/detect calls remain UNKNOWN with manual traces,
never a claimed clean graph result. No production adoption is authorized.

Recommendation: **adjust; do not adopt as a universal replacement**.
Keep the bounded private PZ prototype and long-history A evidence. The frozen
short-history file criterion fails, fresh-process index startup is materially
slower, B publication-control duplicates are necessary, and remaining native/B
coverage must be reported from actual results. No new activation threshold,
owner migration or production format is approved by these observations.
